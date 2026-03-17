use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RiskFactors {
    pub consequence: f64,
    pub proximity: f64,
    pub dependency_pressure: f64,
    #[serde(default)]
    pub external_anchor: f64,
    #[serde(default)]
    pub stale_open_age: f64,
    #[serde(default)]
    pub reasons: Vec<String>,
    #[serde(default)]
    pub dependency_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RiskSnapshot {
    pub commitment_id: String,
    pub risk_score: f64,
    pub risk_level: String,
    pub factors: RiskFactors,
    pub computed_at: Option<i64>,
}

impl RiskSnapshot {
    pub fn normalized_level(&self) -> &str {
        match self.risk_level.as_str() {
            "low" => "low",
            "medium" => "medium",
            "high" => "high",
            "critical" => "critical",
            _ => "unknown",
        }
    }

    pub fn is_high_or_worse(&self) -> bool {
        matches!(self.normalized_level(), "high" | "critical")
    }

    pub fn severity_rank(&self) -> u8 {
        match self.normalized_level() {
            "critical" => 4,
            "high" => 3,
            "medium" => 2,
            "low" => 1,
            _ => 0,
        }
    }
}

pub fn sort_snapshots_by_priority_desc(snapshots: &mut [RiskSnapshot]) {
    snapshots.sort_by(|left, right| {
        right
            .severity_rank()
            .cmp(&left.severity_rank())
            .then_with(|| right.risk_score.total_cmp(&left.risk_score))
            .then_with(|| right.computed_at.cmp(&left.computed_at))
            .then_with(|| left.commitment_id.cmp(&right.commitment_id))
    });
}

#[cfg(test)]
mod tests {
    use super::{sort_snapshots_by_priority_desc, RiskFactors, RiskSnapshot};

    fn snapshot(level: &str) -> RiskSnapshot {
        RiskSnapshot {
            commitment_id: "com_1".to_string(),
            risk_score: 0.75,
            risk_level: level.to_string(),
            factors: RiskFactors {
                consequence: 0.0,
                proximity: 0.0,
                dependency_pressure: 0.0,
                external_anchor: 0.0,
                stale_open_age: 0.0,
                reasons: Vec::new(),
                dependency_ids: Vec::new(),
            },
            computed_at: Some(123),
        }
    }

    #[test]
    fn normalized_level_falls_back_to_unknown_for_unrecognized_values() {
        assert_eq!(snapshot("danger").normalized_level(), "unknown");
    }

    #[test]
    fn is_high_or_worse_matches_high_and_critical_only() {
        assert!(!snapshot("medium").is_high_or_worse());
        assert!(snapshot("high").is_high_or_worse());
        assert!(snapshot("critical").is_high_or_worse());
    }

    #[test]
    fn severity_rank_orders_levels_from_unknown_to_critical() {
        assert_eq!(snapshot("unknown").severity_rank(), 0);
        assert_eq!(snapshot("low").severity_rank(), 1);
        assert_eq!(snapshot("medium").severity_rank(), 2);
        assert_eq!(snapshot("high").severity_rank(), 3);
        assert_eq!(snapshot("critical").severity_rank(), 4);
    }

    #[test]
    fn sort_snapshots_by_priority_desc_prefers_severity_then_score_then_time() {
        let mut snapshots = vec![
            RiskSnapshot {
                commitment_id: "com_low".to_string(),
                risk_score: 0.95,
                risk_level: "low".to_string(),
                factors: snapshot("low").factors,
                computed_at: Some(5),
            },
            RiskSnapshot {
                commitment_id: "com_high_old".to_string(),
                risk_score: 0.80,
                risk_level: "high".to_string(),
                factors: snapshot("high").factors,
                computed_at: Some(10),
            },
            RiskSnapshot {
                commitment_id: "com_high_new".to_string(),
                risk_score: 0.80,
                risk_level: "high".to_string(),
                factors: snapshot("high").factors,
                computed_at: Some(20),
            },
            RiskSnapshot {
                commitment_id: "com_critical".to_string(),
                risk_score: 0.76,
                risk_level: "critical".to_string(),
                factors: snapshot("critical").factors,
                computed_at: Some(1),
            },
        ];

        sort_snapshots_by_priority_desc(&mut snapshots);

        let ordered_ids: Vec<_> = snapshots
            .iter()
            .map(|snapshot| snapshot.commitment_id.as_str())
            .collect();
        assert_eq!(
            ordered_ids,
            vec!["com_critical", "com_high_new", "com_high_old", "com_low"]
        );
    }
}
