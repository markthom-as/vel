use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RiskFactors {
    pub consequence: f64,
    pub proximity: f64,
    pub dependency_pressure: f64,
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
