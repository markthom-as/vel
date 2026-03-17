//! Risk engine: consequence, proximity, dependency pressure. See vel-risk-engine-spec.md and vel-agent-next-implementation-steps.md.
//! No uncertainty or progress penalty in first version.
//!
//! **Boundary: recompute-and-persist.** [run] must only be called from the evaluate orchestration.
//! Read routes (GET /v1/risk, GET /v1/explain/*) use storage only (list_commitment_risk_*).

use vel_core::{Commitment, CommitmentStatus, RiskFactors, RiskSnapshot};
use vel_storage::Storage;

/// Weights for risk score (consequence, proximity, dependency_pressure only).
const W_CONSEQUENCE: f64 = 0.35;
const W_PROXIMITY: f64 = 0.30;
const W_DEPENDENCY: f64 = 0.20;

/// Thresholds for risk level.
const LOW_MAX: f64 = 0.24;
const MEDIUM_MAX: f64 = 0.49;
const HIGH_MAX: f64 = 0.74;

/// Consequence heuristic: 0.0..=1.0.
fn consequence(commitment: &Commitment) -> f64 {
    let kind = commitment.commitment_kind.as_deref().unwrap_or("");
    let source = commitment.source_type.as_str();
    if kind == "medication" {
        return 0.9;
    }
    if source == "calendar" || kind == "meeting" {
        return 0.9;
    }
    if kind == "prep" || kind == "commute" {
        return 0.8;
    }
    if kind == "todo" || source == "capture" {
        return 0.5;
    }
    0.5
}

/// Proximity from due time: 0.0..=1.0. Buckets: >2h low, 30m–2h medium, <30m high, overdue critical.
fn proximity(due_at_ts: Option<i64>, now_ts: i64) -> f64 {
    let Some(due) = due_at_ts else { return 0.2 };
    let secs = due - now_ts;
    if secs < 0 {
        return 1.0;
    }
    if secs < 30 * 60 {
        return 0.9;
    }
    if secs < 2 * 3600 {
        return 0.5;
    }
    0.2
}

/// One-level dependency pressure: if any parent has high risk, add pressure.
fn dependency_pressure(
    commitment_id: &str,
    parent_risks: &[(String, f64)],
    deps_by_child: &[(String, String)],
) -> f64 {
    let parents: Vec<_> = deps_by_child
        .iter()
        .filter(|(_, c)| c == commitment_id)
        .map(|(p, _)| p.clone())
        .collect();
    for (pid, score) in parent_risks {
        if parents.iter().any(|p| p == pid) && *score >= HIGH_MAX {
            return 0.8;
        }
    }
    0.0
}

fn score_to_level(score: f64) -> &'static str {
    if score > HIGH_MAX {
        "critical"
    } else if score > MEDIUM_MAX {
        "high"
    } else if score > LOW_MAX {
        "medium"
    } else {
        "low"
    }
}

pub fn decode_factors_json(factors_json: &str) -> RiskFactors {
    serde_json::from_str(factors_json).unwrap_or(RiskFactors {
        consequence: 0.0,
        proximity: 0.0,
        dependency_pressure: 0.0,
        reasons: Vec::new(),
        dependency_ids: Vec::new(),
    })
}

pub fn snapshot_from_row(
    commitment_id: String,
    risk_score: f64,
    risk_level: String,
    factors_json: &str,
    computed_at: Option<i64>,
) -> RiskSnapshot {
    RiskSnapshot {
        commitment_id,
        risk_score,
        risk_level,
        factors: decode_factors_json(factors_json),
        computed_at,
    }
}

pub async fn list_latest_snapshots(
    storage: &Storage,
) -> Result<Vec<RiskSnapshot>, crate::errors::AppError> {
    let rows = storage.list_commitment_risk_latest_all().await?;
    Ok(rows
        .into_iter()
        .map(
            |(_, commitment_id, risk_score, risk_level, factors_json, computed_at)| {
                snapshot_from_row(
                    commitment_id,
                    risk_score,
                    risk_level,
                    &factors_json,
                    Some(computed_at),
                )
            },
        )
        .collect())
}

/// **Recompute-and-persist.** Compute risk for all open commitments and persist. Returns snapshots.
/// Two passes: first consequence+proximity, then add dependency pressure from parent scores.
/// Only call from evaluate orchestration; read routes use storage list_commitment_risk_*.
pub async fn run(
    storage: &Storage,
    now_ts: i64,
) -> Result<Vec<RiskSnapshot>, crate::errors::AppError> {
    let open = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 500)
        .await?;
    let mut deps_by_child: Vec<(String, String)> = Vec::new();
    for c in &open {
        let rows = storage
            .list_commitment_dependencies_by_parent(c.id.as_ref())
            .await
            .unwrap_or_default();
        for (_, child_id, _, _) in rows {
            deps_by_child.push((c.id.as_ref().to_string(), child_id));
        }
    }
    let mut first_pass: Vec<(String, f64, f64, f64)> = Vec::new();
    for c in &open {
        let consequence_val = consequence(c);
        let due_ts = c.due_at.map(|t| t.unix_timestamp());
        let proximity_val = proximity(due_ts, now_ts);
        first_pass.push((
            c.id.as_ref().to_string(),
            consequence_val,
            proximity_val,
            0.0,
        ));
    }
    let parent_scores: Vec<(String, f64)> = first_pass
        .iter()
        .map(|(id, c, p, _)| (id.clone(), W_CONSEQUENCE * c + W_PROXIMITY * p))
        .collect();
    let mut snapshots = Vec::new();
    for (i, c) in open.iter().enumerate() {
        let (_, consequence_val, proximity_val, _) = &first_pass[i];
        let dep_val = dependency_pressure(c.id.as_ref(), &parent_scores, &deps_by_child);
        let score =
            W_CONSEQUENCE * consequence_val + W_PROXIMITY * proximity_val + W_DEPENDENCY * dep_val;
        let score = score.min(1.0);
        let level = score_to_level(score).to_string();
        let dependency_ids: Vec<String> = deps_by_child
            .iter()
            .filter(|(_, child)| child == c.id.as_ref())
            .map(|(parent, _)| parent.clone())
            .collect();
        let reasons: Vec<&str> = [
            if *consequence_val >= 0.8 {
                Some("high consequence commitment")
            } else {
                None
            },
            if *proximity_val >= 0.5 {
                Some("due time approaching")
            } else {
                None
            },
            if dep_val > 0.0 {
                Some("parent commitment at risk")
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .collect();
        let factors = RiskFactors {
            consequence: *consequence_val,
            proximity: *proximity_val,
            dependency_pressure: dep_val,
            reasons: reasons.iter().map(|reason| (*reason).to_string()).collect(),
            dependency_ids,
        };
        let factors_str = serde_json::to_string(&factors)
            .map_err(|error| crate::errors::AppError::internal(error.to_string()))?;
        storage
            .insert_commitment_risk(c.id.as_ref(), score, &level, &factors_str, now_ts)
            .await?;
        snapshots.push(snapshot_from_row(
            c.id.as_ref().to_string(),
            score,
            level,
            &factors_str,
            Some(now_ts),
        ));
    }
    Ok(snapshots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_from_row_decodes_typed_factors() {
        let snapshot = snapshot_from_row(
            "com_1".to_string(),
            0.91,
            "critical".to_string(),
            r#"{"consequence":0.9,"proximity":1.0,"dependency_pressure":0.8,"reasons":["due now"],"dependency_ids":["com_parent"]}"#,
            Some(123),
        );

        assert_eq!(snapshot.commitment_id, "com_1");
        assert_eq!(snapshot.risk_level, "critical");
        assert_eq!(snapshot.factors.consequence, 0.9);
        assert_eq!(snapshot.factors.proximity, 1.0);
        assert_eq!(snapshot.factors.dependency_pressure, 0.8);
        assert_eq!(snapshot.factors.reasons, vec!["due now".to_string()]);
        assert_eq!(
            snapshot.factors.dependency_ids,
            vec!["com_parent".to_string()]
        );
        assert_eq!(snapshot.computed_at, Some(123));
    }
}
