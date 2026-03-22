use serde::{Deserialize, Serialize};

use crate::{CalendarId, EventId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarRelationKind {
    BelongsTo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarRelationRef {
    pub object_type: String,
    pub object_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarRelation {
    pub relation_kind: CalendarRelationKind,
    pub from: CalendarRelationRef,
    pub to: CalendarRelationRef,
}

pub fn belongs_to_calendar(event_id: &EventId, calendar_id: &CalendarId) -> CalendarRelation {
    CalendarRelation {
        relation_kind: CalendarRelationKind::BelongsTo,
        from: CalendarRelationRef {
            object_type: "event".to_string(),
            object_id: event_id.to_string(),
        },
        to: CalendarRelationRef {
            object_type: "calendar".to_string(),
            object_id: calendar_id.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{belongs_to_calendar, CalendarRelationKind};
    use crate::{CalendarId, EventId};

    #[test]
    fn belongs_to_calendar_relates_event_to_calendar() {
        let relation = belongs_to_calendar(&EventId::new(), &CalendarId::new());

        assert_eq!(relation.relation_kind, CalendarRelationKind::BelongsTo);
        assert_eq!(relation.from.object_type, "event");
        assert_eq!(relation.to.object_type, "calendar");
    }
}
