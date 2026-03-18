//! Git activity reducer: maps git_activity signals into CurrentContextV1 git summary fields.

use vel_core::CurrentContextV1;
use vel_storage::SignalRecord;

use crate::services::inference::SignalReducer;

const RECENT_GIT_ACTIVITY_WINDOW_SECS: i64 = 90 * 60;

pub struct GitActivityReducer;

impl SignalReducer for GitActivityReducer {
    fn name(&self) -> &'static str {
        "git_activity"
    }

    fn reduce(&self, ctx: CurrentContextV1, signals: &[SignalRecord]) -> CurrentContextV1 {
        let latest_git = signals
            .iter()
            .filter(|s| s.signal_type == "git_activity")
            .max_by_key(|s| s.timestamp);

        let Some(signal) = latest_git else {
            return ctx;
        };

        // Only include git activity within the recent window
        let now_ts = ctx.computed_at;
        if now_ts - signal.timestamp > RECENT_GIT_ACTIVITY_WINDOW_SECS {
            return ctx;
        }

        let payload = &signal.payload_json;
        let Some(repo) = payload
            .get("repo_name")
            .and_then(serde_json::Value::as_str)
            .map(ToString::to_string)
            .or_else(|| {
                payload
                    .get("repo")
                    .and_then(serde_json::Value::as_str)
                    .and_then(repo_basename)
            })
        else {
            return ctx;
        };

        let git_activity_summary = serde_json::json!({
            "timestamp": signal.timestamp,
            "repo": repo,
            "branch": payload.get("branch").and_then(|v| v.as_str()),
            "operation": payload.get("operation").and_then(|v| v.as_str()),
            "message": payload.get("message").and_then(|v| v.as_str()),
            "files_changed": payload.get("files_changed").and_then(|v| v.as_u64()),
            "insertions": payload.get("insertions").and_then(|v| v.as_u64()),
            "deletions": payload.get("deletions").and_then(|v| v.as_u64()),
        });

        CurrentContextV1 {
            git_activity_summary: Some(git_activity_summary),
            ..ctx
        }
    }
}

fn repo_basename(path: &str) -> Option<String> {
    path.rsplit('/')
        .find(|segment| !segment.trim().is_empty())
        .map(ToString::to_string)
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
    fn git_reducer_returns_ctx_unchanged_when_no_git_signals() {
        let reducer = GitActivityReducer;
        let ctx = make_ctx(1_700_000_000);
        let signals: Vec<SignalRecord> = vec![
            make_signal("sig_cal", "calendar_event", 1_700_000_000),
        ];

        let result = reducer.reduce(ctx.clone(), &signals);

        assert!(result.git_activity_summary.is_none());
    }

    #[test]
    fn git_reducer_populates_git_summary_from_recent_git_activity_signal() {
        let reducer = GitActivityReducer;
        let now_ts = 1_700_000_000;
        let ctx = make_ctx(now_ts);
        let mut signal = make_signal("sig_git", "git_activity", now_ts - 60);
        signal.payload_json = json!({
            "repo_name": "vel",
            "branch": "main",
            "operation": "commit",
            "message": "feat: add thing",
            "files_changed": 3,
            "insertions": 50,
            "deletions": 10,
        });
        let signals = vec![signal];

        let result = reducer.reduce(ctx, &signals);

        let summary = result.git_activity_summary.expect("git_activity_summary should be set");
        assert_eq!(summary["repo"], "vel");
        assert_eq!(summary["branch"], "main");
        assert_eq!(summary["files_changed"], 3);
    }

    #[test]
    fn git_reducer_skips_stale_git_activity_outside_window() {
        let reducer = GitActivityReducer;
        let now_ts = 1_700_000_000;
        let ctx = make_ctx(now_ts);
        // Signal is older than 90 minutes
        let stale_ts = now_ts - RECENT_GIT_ACTIVITY_WINDOW_SECS - 60;
        let mut signal = make_signal("sig_git_stale", "git_activity", stale_ts);
        signal.payload_json = json!({ "repo_name": "vel" });
        let signals = vec![signal];

        let result = reducer.reduce(ctx, &signals);

        assert!(result.git_activity_summary.is_none());
    }
}
