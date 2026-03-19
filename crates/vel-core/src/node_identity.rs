use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeIdentity(pub(crate) String);

impl NodeIdentity {
    pub fn new() -> Self {
        Self(Uuid::new_v4().hyphenated().to_string())
    }
}

impl Default for NodeIdentity {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for NodeIdentity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for NodeIdentity {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for NodeIdentity {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::NodeIdentity;

    #[test]
    fn node_identity_round_trips_uuid_string() {
        let original = "123e4567-e89b-12d3-a456-426614174000".to_string();
        let value = NodeIdentity::from(original.clone());
        let encoded = serde_json::to_string(&value).expect("node identity should serialize");
        let decoded: NodeIdentity =
            serde_json::from_str(&encoded).expect("node identity should deserialize");

        assert_eq!(decoded.as_ref(), original);
    }
}
