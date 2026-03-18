//! Calendar domain reducer: maps calendar_event signals into CurrentContextV1 schedule/event fields.
//! Sets `next_event_start_ts` and `leave_by_ts` based on the nearest upcoming calendar event.

use vel_core::CurrentContextV1;
use vel_storage::SignalRecord;

use crate::services::inference::SignalReducer;

const DEFAULT_TRAVEL_MINUTES: i64 = 0;

pub struct CalendarReducer;

impl SignalReducer for CalendarReducer {
    fn name(&self) -> &'static str {
        "calendar"
    }

    fn reduce(&self, ctx: CurrentContextV1, signals: &[SignalRecord]) -> CurrentContextV1 {
        let calendar_events: Vec<&SignalRecord> = signals
            .iter()
            .filter(|s| s.signal_type == "calendar_event")
            .collect();

        if calendar_events.is_empty() {
            return ctx;
        }

        let now_ts = ctx.computed_at;
        let next_event = select_next_event(&calendar_events, now_ts);

        let (next_event_start_ts, leave_by_ts) = if let Some(event) = next_event {
            let travel_minutes = event
                .payload_json
                .get("travel_minutes")
                .and_then(|v| v.as_i64())
                .unwrap_or(DEFAULT_TRAVEL_MINUTES);
            (
                Some(event.timestamp),
                Some(event.timestamp - travel_minutes * 60),
            )
        } else {
            (ctx.next_event_start_ts, ctx.leave_by_ts)
        };

        CurrentContextV1 {
            next_event_start_ts,
            leave_by_ts,
            ..ctx
        }
    }
}

fn select_next_event<'a>(
    calendar_events: &'a [&vel_storage::SignalRecord],
    now_ts: i64,
) -> Option<&'a vel_storage::SignalRecord> {
    calendar_events
        .iter()
        .copied()
        .filter(|s| s.timestamp >= now_ts)
        .min_by_key(|s| s.timestamp)
        .or_else(|| {
            calendar_events
                .iter()
                .copied()
                .filter(|s| s.timestamp <= now_ts)
                .max_by_key(|s| s.timestamp)
        })
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

    fn make_ctx(computed_at: i64) -> CurrentContextV1 {
        CurrentContextV1 {
            computed_at,
            ..CurrentContextV1::default()
        }
    }

    #[test]
    fn calendar_reducer_returns_ctx_unchanged_when_no_calendar_signals() {
        let reducer = CalendarReducer;
        let ctx = make_ctx(1_700_000_000);
        let signals = vec![make_signal("sig_health", "health_metric", 1_700_000_000)];

        let result = reducer.reduce(ctx.clone(), &signals);

        assert_eq!(result.next_event_start_ts, ctx.next_event_start_ts);
        assert_eq!(result.leave_by_ts, ctx.leave_by_ts);
    }

    #[test]
    fn calendar_reducer_populates_next_event_start_ts_from_future_calendar_signal() {
        let reducer = CalendarReducer;
        let now_ts = 1_700_000_000;
        let event_ts = now_ts + 3600;
        let ctx = make_ctx(now_ts);
        let signals = vec![make_signal("sig_cal", "calendar_event", event_ts)];

        let result = reducer.reduce(ctx, &signals);

        assert_eq!(result.next_event_start_ts, Some(event_ts));
    }

    #[test]
    fn calendar_reducer_includes_both_events_and_picks_nearest_future() {
        let reducer = CalendarReducer;
        let now_ts = 1_700_000_000;
        let ctx = make_ctx(now_ts);
        let signals = vec![
            make_signal("sig_cal_near", "calendar_event", now_ts + 3600),
            make_signal("sig_cal_far", "calendar_event", now_ts + 7200),
        ];

        let result = reducer.reduce(ctx, &signals);

        // Nearest future event should be selected
        assert_eq!(result.next_event_start_ts, Some(now_ts + 3600));
    }

    #[test]
    fn calendar_reducer_sets_leave_by_ts_using_travel_minutes_from_event_payload() {
        let reducer = CalendarReducer;
        let now_ts = 1_700_000_000;
        let event_ts = now_ts + 3600;
        let ctx = make_ctx(now_ts);
        let mut signal = make_signal("sig_cal", "calendar_event", event_ts);
        signal.payload_json = json!({ "travel_minutes": 20 });
        let signals = vec![signal];

        let result = reducer.reduce(ctx, &signals);

        assert_eq!(result.next_event_start_ts, Some(event_ts));
        assert_eq!(result.leave_by_ts, Some(event_ts - 20 * 60));
    }
}
