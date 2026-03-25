use std::process::Stdio;
use std::time::Duration;
use tokio::time::timeout;
use vel_core::{ToolInvocationOutcome, ToolInvocationRequest};

fn to_command_line(request: &ToolInvocationRequest) -> Option<(String, Vec<String>)> {
    let executable = request.tool_name.trim();
    if executable.is_empty() {
        return None;
    }
    Some((executable.to_string(), request.args.clone()))
}

pub async fn run_tool(request: &ToolInvocationRequest) -> ToolInvocationOutcome {
    let Some((executable, args)) = to_command_line(request) else {
        return ToolInvocationOutcome::Refusal {
            reason: "tool_name must not be empty".to_string(),
        };
    };

    let mut command = tokio::process::Command::new(executable);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let spawn = command.spawn();
    let mut child = match spawn {
        Ok(child) => child,
        Err(error) => {
            return ToolInvocationOutcome::Failure {
                error: format!("spawn failed: {error}"),
            };
        }
    };

    let timeout_ms = request.timeout_ms.unwrap_or(5_000);
    match timeout(Duration::from_millis(timeout_ms), child.wait()).await {
        Ok(wait_result) => match wait_result {
            Ok(status) => ToolInvocationOutcome::Success {
                stdout: String::new(),
                stderr: String::new(),
                exit_code: status.code().unwrap_or(-1),
            },
            Err(error) => ToolInvocationOutcome::Failure {
                error: format!("wait failed: {error}"),
            },
        },
        Err(_) => {
            let _ = child.kill().await;
            ToolInvocationOutcome::Timeout { timeout_ms }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run_tool;
    use vel_core::{ToolInvocationOutcome, ToolInvocationRequest};

    #[tokio::test]
    async fn shell_runner_returns_success_outcome() {
        let outcome = run_tool(&ToolInvocationRequest {
            tool_name: "sh".to_string(),
            args: vec!["-lc".to_string(), "printf hello".to_string()],
            timeout_ms: Some(1_000),
            metadata_json: serde_json::json!({}),
        })
        .await;
        match outcome {
            ToolInvocationOutcome::Success { exit_code, .. } => {
                assert_eq!(exit_code, 0);
            }
            other => panic!("unexpected outcome: {other:?}"),
        }
    }

    #[tokio::test]
    async fn shell_runner_distinguishes_timeout() {
        let outcome = run_tool(&ToolInvocationRequest {
            tool_name: "sh".to_string(),
            args: vec!["-lc".to_string(), "sleep 1".to_string()],
            timeout_ms: Some(5),
            metadata_json: serde_json::json!({}),
        })
        .await;
        match outcome {
            ToolInvocationOutcome::Timeout { timeout_ms } => assert_eq!(timeout_ms, 5),
            other => panic!("unexpected outcome: {other:?}"),
        }
    }
}
