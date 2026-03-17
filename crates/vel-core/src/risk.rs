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
}

#[cfg(test)]
mod tests {
    use super::{RiskFactors, RiskSnapshot};

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
}
