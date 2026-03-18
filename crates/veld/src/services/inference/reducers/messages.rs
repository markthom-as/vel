//! Messages domain reducer: maps message_thread signals into CurrentContextV1 message summary fields.

use vel_core::CurrentContextV1;
use vel_storage::SignalRecord;

use crate::services::inference::SignalReducer;

pub struct MessagesReducer;

impl SignalReducer for MessagesReducer {
    fn name(&self) -> &'static str {
        "messages"
    }

    fn reduce(&self, ctx: CurrentContextV1, signals: &[SignalRecord]) -> CurrentContextV1 {
        let message_threads: Vec<&SignalRecord> = signals
            .iter()
            .filter(|s| s.signal_type == "message_thread")
            .collect();

        if message_threads.is_empty() {
            return ctx;
        }

        let waiting_on_me_threads: Vec<&SignalRecord> = message_threads
            .iter()
            .copied()
            .filter(|s| {
                s.payload_json
                    .get("waiting_state")
                    .and_then(|v| v.as_str())
                    == Some("me")
            })
            .collect();
        let waiting_on_others_count = message_threads
            .iter()
            .filter(|s| {
                s.payload_json
                    .get("waiting_state")
                    .and_then(|v| v.as_str())
                    == Some("others")
            })
            .count();
        let scheduling_thread_count = message_threads
            .iter()
            .filter(|s| {
                s.payload_json
                    .get("scheduling_related")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
            })
            .count();
        let urgent_thread_count = message_threads
            .iter()
            .filter(|s| {
                s.payload_json
                    .get("urgent")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
            })
            .count();
        let top_threads: Vec<serde_json::Value> = waiting_on_me_threads
            .iter()
            .take(3)
            .map(|s| {
                serde_json::json!({
                    "thread_id": s.payload_json.get("thread_id").and_then(|v| v.as_str()),
                    "platform": s.payload_json.get("platform").and_then(|v| v.as_str()),
                    "title": s.payload_json.get("title").and_then(|v| v.as_str()),
                    "waiting_state": s.payload_json.get("waiting_state").and_then(|v| v.as_str()),
                    "scheduling_related": s.payload_json.get("scheduling_related").and_then(|v| v.as_bool()),
                    "urgent": s.payload_json.get("urgent").and_then(|v| v.as_bool()),
                    "latest_timestamp": s.payload_json.get("latest_timestamp").and_then(|v| v.as_i64()),
                    "snippet": s.payload_json.get("snippet").and_then(|v| v.as_str()),
                })
            })
            .collect();

        let waiting_on_me_count = waiting_on_me_threads.len();
        let message_summary_json = serde_json::json!({
            "waiting_on_me_count": waiting_on_me_count,
            "waiting_on_others_count": waiting_on_others_count,
            "scheduling_thread_count": scheduling_thread_count,
            "urgent_thread_count": urgent_thread_count,
            "top_threads": top_threads,
        });

        CurrentContextV1 {
            message_waiting_on_me_count: Some(waiting_on_me_count as u64),
            message_waiting_on_others_count: Some(waiting_on_others_count as u64),
            message_scheduling_thread_count: Some(scheduling_thread_count as u64),
            message_urgent_thread_count: Some(urgent_thread_count as u64),
            message_summary: Some(message_summary_json),
            ..ctx
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_signal(signal_id: &str, signal_type: &str, timestamp: i64) -> SignalRecord {
        SignalRecord {
            signal_id: signal_id.to_string(),
            signal_type: signal_type.to_string(),
            source: "test".to_string(),
            source_ref: None,
            timestamp,
            payload_json: json!({}),
            created_at: timestamp,
        }
    }

    fn make_ctx() -> CurrentContextV1 {
        CurrentContextV1::default()
    }

    #[test]
    fn messages_reducer_returns_ctx_unchanged_when_no_message_signals() {
        let reducer = MessagesReducer;
        let ctx = make_ctx();
        let signals: Vec<SignalRecord> = vec![
            make_signal("sig_cal", "calendar_event", 1_700_000_000),
        ];

        let result = reducer.reduce(ctx.clone(), &signals);

        assert_eq!(result.message_waiting_on_me_count, ctx.message_waiting_on_me_count);
        assert!(result.message_summary.is_none());
    }

    #[test]
    fn messages_reducer_populates_message_fields_from_message_thread_signal() {
        let reducer = MessagesReducer;
        let ctx = make_ctx();
        let mut signal = make_signal("sig_msg", "message_thread", 1_700_000_000);
        signal.payload_json = json!({
            "waiting_state": "me",
            "scheduling_related": false,
            "urgent": true,
            "thread_id": "th_1",
            "platform": "slack",
            "title": "Urgent thing",
        });
        let signals = vec![signal];

        let result = reducer.reduce(ctx, &signals);

        assert_eq!(result.message_waiting_on_me_count, Some(1));
        assert_eq!(result.message_urgent_thread_count, Some(1));
        assert!(result.message_summary.is_some());
    }

    #[test]
    fn messages_reducer_counts_waiting_on_others_separately() {
        let reducer = MessagesReducer;
        let ctx = make_ctx();
        let mut sig1 = make_signal("sig_me", "message_thread", 1_700_000_000);
        sig1.payload_json = json!({ "waiting_state": "me" });
        let mut sig2 = make_signal("sig_others", "message_thread", 1_700_000_001);
        sig2.payload_json = json!({ "waiting_state": "others" });
        let signals = vec![sig1, sig2];

        let result = reducer.reduce(ctx, &signals);

        assert_eq!(result.message_waiting_on_me_count, Some(1));
        assert_eq!(result.message_waiting_on_others_count, Some(1));
    }
}
