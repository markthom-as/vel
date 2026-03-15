//! Risk engine: consequence, proximity, dependency pressure. See vel-risk-engine-spec.md and vel-agent-next-implementation-steps.md.
//! No uncertainty or progress penalty in first version.

use time::OffsetDateTime;
use vel_core::{Commitment, CommitmentStatus};
use vel_storage::Storage;

/// Weights for risk score (consequence, proximity, dependency_pressure only).
const W_CONSEQUENCE: f64 = 0.35;
const W_PROXIMITY: f64 = 0.30;
const W_DEPENDENCY: f64 = 0.20;

/// Thresholds for risk level.
const LOW_MAX: f64 = 0.24;
const MEDIUM_MAX: f64 = 0.49;
const HIGH_MAX: f64 = 0.74;

#[derive(Debug, Clone)]
pub struct RiskSnapshot {
    pub commitment_id: String,
    pub risk_score: f64,
    pub risk_level: String,
    pub factors_json: String,
}

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

/// Compute risk for all open commitments and persist. Returns snapshots.
/// Two passes: first consequence+proximity, then add dependency pressure from parent scores.
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
        let score = W_CONSEQUENCE * consequence_val
            + W_PROXIMITY * proximity_val
            + W_DEPENDENCY * dep_val;
        let score = score.min(1.0);
        let level = score_to_level(score).to_string();
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
        let factors = serde_json::json!({
            "consequence": consequence_val,
            "proximity": proximity_val,
            "dependency_pressure": dep_val,
            "reasons": reasons,
            "dependency_ids": []
        });
        let factors_str = factors.to_string();
        storage
            .insert_commitment_risk(c.id.as_ref(), score, &level, &factors_str, now_ts)
            .await?;
        snapshots.push(RiskSnapshot {
            commitment_id: c.id.as_ref().to_string(),
            risk_score: score,
            risk_level: level,
            factors_json: factors_str,
        });
    }
    Ok(snapshots)
}
