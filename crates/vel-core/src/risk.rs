use serde::{Deserialize, Serialize};

pub fn normalize_risk_level(level: &str) -> &str {
    match level {
        "low" => "low",
        "medium" => "medium",
        "high" => "high",
        "critical" => "critical",
        _ => "unknown",
    }
}

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
    pub fn new(
        commitment_id: String,
        risk_score: f64,
        risk_level: String,
        factors: RiskFactors,
        computed_at: Option<i64>,
    ) -> Self {
        Self {
            commitment_id,
            risk_score,
            risk_level: normalize_risk_level(&risk_level).to_string(),
            factors,
            computed_at,
        }
    }

    pub fn normalized_level(&self) -> &str {
        normalize_risk_level(&self.risk_level)
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
    use super::{normalize_risk_level, sort_snapshots_by_priority_desc, RiskFactors, RiskSnapshot};

    fn snapshot(level: &str) -> RiskSnapshot {
        RiskSnapshot::new(
            "com_1".to_string(),
            0.75,
            level.to_string(),
            RiskFactors {
                consequence: 0.0,
                proximity: 0.0,
                dependency_pressure: 0.0,
                external_anchor: 0.0,
                stale_open_age: 0.0,
                reasons: Vec::new(),
                dependency_ids: Vec::new(),
            },
            Some(123),
        )
    }

    #[test]
    fn normalized_level_falls_back_to_unknown_for_unrecognized_values() {
        assert_eq!(snapshot("danger").normalized_level(), "unknown");
    }

    #[test]
    fn normalize_risk_level_falls_back_to_unknown_for_unrecognized_values() {
        assert_eq!(normalize_risk_level("danger"), "unknown");
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
            RiskSnapshot::new(
                "com_low".to_string(),
                0.95,
                "low".to_string(),
                snapshot("low").factors,
                Some(5),
            ),
            RiskSnapshot::new(
                "com_high_old".to_string(),
                0.80,
                "high".to_string(),
                snapshot("high").factors,
                Some(10),
            ),
            RiskSnapshot::new(
                "com_high_new".to_string(),
                0.80,
                "high".to_string(),
                snapshot("high").factors,
                Some(20),
            ),
            RiskSnapshot::new(
                "com_critical".to_string(),
                0.76,
                "critical".to_string(),
                snapshot("critical").factors,
                Some(1),
            ),
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

    #[test]
    fn new_normalizes_unrecognized_levels_to_unknown() {
        let snapshot = RiskSnapshot::new(
            "com_x".to_string(),
            0.4,
            "danger".to_string(),
            snapshot("low").factors,
            Some(1),
        );

        assert_eq!(snapshot.risk_level, "unknown");
        assert_eq!(snapshot.normalized_level(), "unknown");
    }
}
