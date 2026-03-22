use serde_json::{json, Value as JsonValue};
use vel_core::AvailabilityWindow;

pub struct CalendarExplainService;

impl CalendarExplainService {
    pub fn explain_availability(
        window: &AvailabilityWindow,
        candidate_start: &str,
        candidate_end: &str,
    ) -> JsonValue {
        let accepted = window.blocking_intervals.is_empty();

        json!({
            "candidate": {
                "start": candidate_start,
                "end": candidate_end,
            },
            "result": window.result,
            "accepted": accepted,
            "blocking_intervals": window.blocking_intervals,
            "sources_consulted": window.sources_consulted,
            "filters_applied": window.filters_applied,
            "basis": window.basis,
            "confidence": window.confidence,
            "decision_reason": if accepted {
                "candidate accepted because no blocking intervals overlapped the evaluated window"
            } else {
                "candidate rejected because one or more blocking intervals overlapped the evaluated window"
            }
        })
    }
}
