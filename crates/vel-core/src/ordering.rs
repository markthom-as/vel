use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use crate::NodeIdentity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderingStamp {
    pub physical_ts: i64,
    pub logical_counter: u32,
    pub node_id: NodeIdentity,
}

impl OrderingStamp {
    pub fn new(physical_ts: i64, logical_counter: u32, node_id: NodeIdentity) -> Self {
        Self {
            physical_ts,
            logical_counter,
            node_id,
        }
    }
}

impl PartialOrd for OrderingStamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderingStamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.physical_ts
            .cmp(&other.physical_ts)
            .then_with(|| self.logical_counter.cmp(&other.logical_counter))
            .then_with(|| self.node_id.cmp(&other.node_id))
    }
}

#[cfg(test)]
mod tests {
    use super::OrderingStamp;
    use crate::NodeIdentity;

    #[test]
    fn ordering_stamp_sorts_deterministically_on_node_tiebreak() {
        let later = OrderingStamp::new(
            1_710_000_000,
            4,
            NodeIdentity::from("123e4567-e89b-12d3-a456-426614174001".to_string()),
        );
        let earlier = OrderingStamp::new(
            1_710_000_000,
            4,
            NodeIdentity::from("123e4567-e89b-12d3-a456-426614174000".to_string()),
        );
        let mut values = vec![later.clone(), earlier.clone()];

        values.sort();

        assert_eq!(values, vec![earlier, later]);
    }
}
