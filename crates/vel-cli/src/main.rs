mod client;
mod commands;

use anyhow::Context;
use clap::{Parser, Subcommand};
use vel_config::AppConfig;

#[derive(Debug, Parser)]
#[command(name = "vel", about = "Vel operator shell")]
struct Cli {
    #[arg(long)]
    base_url: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Doctor {
        #[arg(long)]
        json: bool,
    },
    Health {
        #[arg(long)]
        json: bool,
    },
    Capture {
        text: Option<String>,
        #[arg(long)]
        stdin: bool,
        #[arg(long)]
        r#type: Option<String>,
        #[arg(long)]
        source: Option<String>,
    },
    Recent {
        #[arg(long, default_value = "20")]
        limit: u32,
        #[arg(long)]
        today: bool,
        #[arg(long)]
        json: bool,
    },
    Today {
        #[arg(long)]
        json: bool,
    },
    Morning {
        #[arg(long)]
        json: bool,
    },
    EndOfDay {
        #[arg(long)]
        json: bool,
    },
    Search {
        query: String,
        #[arg(long)]
        capture_type: Option<String>,
        #[arg(long)]
        source_device: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        json: bool,
    },
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    Inspect {
        #[command(subcommand)]
        command: InspectCommand,
    },
    Run {
        #[command(subcommand)]
        command: RunCommand,
    },
    Loops {
        #[arg(long)]
        json: bool,
    },
    Loop {
        #[command(subcommand)]
        command: LoopCommand,
    },
    Review {
        #[command(subcommand)]
        command: ReviewCommand,
    },
    Artifact {
        #[command(subcommand)]
        command: ArtifactCommand,
    },
    Import {
        #[command(subcommand)]
        command: ImportCommand,
    },
    Export {
        #[arg(long)]
        captures: bool,
        #[arg(long)]
        runs: bool,
        #[arg(long)]
        artifacts: bool,
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long)]
        json: bool,
    },
    Backup {},
    Synthesize {
        #[command(subcommand)]
        command: SynthesizeCommand,
    },
    Commitments {
        /// Filter by status (default: open)
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        project: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        json: bool,
    },
    Commitment {
        #[command(subcommand)]
        command: CommitmentCommand,
    },
    Sync {
        #[command(subcommand)]
        command: SyncCommand,
    },
    Nudges {
        #[arg(long)]
        json: bool,
    },
    Nudge {
        #[command(subcommand)]
        command: NudgeCommand,
    },
    Uncertainty {
        #[command(subcommand)]
        command: UncertaintyCommand,
    },
    Evaluate {},
    Context {
        #[command(subcommand)]
        command: ContextCommand,
    },
    Explain {
        #[command(subcommand)]
        command: ExplainCommand,
    },
    Thread {
        #[command(subcommand)]
        command: ThreadCommand,
    },
    Risk {
        #[arg(required = false)]
        id: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Suggestions {
        #[arg(long)]
        state: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Suggestion {
        #[command(subcommand)]
        command: SuggestionCommand,
    },
}

#[derive(Debug, Subcommand)]
enum SuggestionCommand {
    Inspect {
        id: String,
    },
    Accept {
        id: String,
    },
    Reject {
        id: String,
        #[arg(long)]
        reason: Option<String>,
    },
    Modify {
        id: String,
        #[arg(long)]
        payload: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum UncertaintyCommand {
    List {
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Inspect {
        id: String,
    },
    Resolve {
        id: String,
    },
}

#[derive(Debug, Subcommand)]
enum ThreadCommand {
    List {
        #[arg(long)]
        status: Option<String>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        json: bool,
    },
    Inspect {
        id: String,
    },
    Close {
        id: String,
    },
    Reopen {
        id: String,
    },
}

#[derive(Debug, Subcommand)]
enum ContextCommand {
    /// Show current context (default)
    Show {
        #[arg(long)]
        json: bool,
    },
    /// Show recent material context transitions
    Timeline {
        #[arg(long, default_value = "20")]
        limit: u32,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum InspectCommand {
    Capture { id: String },
    Artifact { id: String },
}

#[derive(Debug, Subcommand)]
enum RunCommand {
    List {
        #[arg(long)]
        kind: Option<String>,
        #[arg(long)]
        today: bool,
        #[arg(long, default_value = "20")]
        limit: u32,
        #[arg(long)]
        json: bool,
    },
    Inspect {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Set run status (e.g. retry_scheduled, blocked)
    Status {
        id: String,
        status: String,
        #[arg(long)]
        retry_after_seconds: Option<u32>,
        #[arg(long)]
        retry_at: Option<String>,
        #[arg(long)]
        reason: Option<String>,
        #[arg(long)]
        allow_unsupported_retry: bool,
        #[arg(long)]
        blocked_reason: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    Show {
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum ReviewCommand {
    Today {
        #[arg(long)]
        json: bool,
    },
    Week {
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum LoopCommand {
    Inspect {
        kind: String,
        #[arg(long)]
        json: bool,
    },
    Enable {
        kind: String,
    },
    Disable {
        kind: String,
    },
}

#[derive(Debug, Subcommand)]
enum ArtifactCommand {
    Latest {
        #[arg(long)]
        r#type: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum ImportCommand {
    File {
        path: String,
        #[arg(long, default_value = "note")]
        r#type: String,
    },
    Lines {
        #[arg(long, default_value = "note")]
        r#type: String,
    },
    #[command(name = "url")]
    CaptureUrl { url: String },
}

#[derive(Debug, Subcommand)]
enum SynthesizeCommand {
    Week {
        #[arg(long)]
        json: bool,
    },
    Project {
        name: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum CommitmentCommand {
    Add {
        text: String,
        #[arg(long)]
        kind: Option<String>,
        #[arg(long)]
        project: Option<String>,
    },
    Done {
        id: String,
    },
    Cancel {
        id: String,
    },
    Inspect {
        id: String,
    },
    /// List dependencies (children) of a commitment
    Dependencies {
        id: String,
    },
    /// Add a dependency: child commitment required by parent
    AddDependency {
        parent_id: String,
        child_id: String,
        #[arg(long, default_value = "blocks")]
        r#type: String,
    },
}

#[derive(Debug, Subcommand)]
enum SyncCommand {
    Calendar,
    Todoist,
    Activity,
    Git,
    Notes,
    Transcripts,
    Messaging,
}

#[derive(Debug, Subcommand)]
enum NudgeCommand {
    Done {
        id: String,
    },
    Snooze {
        id: String,
        #[arg(long, default_value = "10")]
        minutes: u32,
    },
    Inspect {
        id: String,
    },
}

#[derive(Debug, Subcommand)]
enum ExplainCommand {
    Nudge {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Explain current context (what shaped it)
    Context {
        #[arg(long)]
        json: bool,
    },
    /// Explain a commitment (risk, why in context)
    Commitment {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Explain current drift/attention state
    Drift {
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = AppConfig::load().context("loading config")?;
    let base_url = cli.base_url.unwrap_or_else(|| config.base_url.clone());
    let client = client::ApiClient::new(base_url);

    match cli.command {
        Command::Doctor { json } => commands::doctor::run(&client, json).await,
        Command::Health { json } => commands::health::run(&client, json).await,
        Command::Capture {
            text,
            stdin,
            r#type: capture_type,
            source,
        } => {
            commands::capture::run(&client, text, stdin, capture_type.clone(), source.clone()).await
        }
        Command::Recent { limit, today, json } => {
            commands::recent::run(&client, limit, today, json).await
        }
        Command::Today { json } => commands::today::run(&client, json).await,
        Command::Morning { json } => commands::morning::run(&client, json).await,
        Command::EndOfDay { json } => commands::end_of_day::run(&client, json).await,
        Command::Search {
            query,
            capture_type,
            source_device,
            limit,
            json,
        } => commands::search::run(&client, query, capture_type, source_device, limit, json).await,
        Command::Config { command } => match command {
            ConfigCommand::Show { json } => commands::config::run(&config, json),
        },
        Command::Inspect { command } => match command {
            InspectCommand::Capture { id } => commands::inspect::run_capture(&client, &id).await,
            InspectCommand::Artifact { id } => commands::inspect::run_artifact(&client, &id).await,
        },
        Command::Run { command } => match command {
            RunCommand::List {
                kind,
                today,
                limit,
                json,
            } => commands::runs::run_list(&client, kind.as_deref(), today, limit, json).await,
            RunCommand::Inspect { id, json } => {
                commands::runs::run_inspect(&client, &id, json).await
            }
            RunCommand::Status {
                id,
                status,
                retry_after_seconds,
                retry_at,
                reason,
                allow_unsupported_retry,
                blocked_reason,
            } => {
                commands::runs::run_status(
                    &client,
                    &id,
                    &status,
                    retry_after_seconds,
                    retry_at.as_deref(),
                    reason.as_deref(),
                    allow_unsupported_retry,
                    blocked_reason.as_deref(),
                )
                .await
            }
        },
        Command::Loops { json } => commands::loops::run_list(&client, json).await,
        Command::Loop { command } => match command {
            LoopCommand::Inspect { kind, json } => {
                commands::loops::run_inspect(&client, &kind, json).await
            }
            LoopCommand::Enable { kind } => commands::loops::run_enable(&client, &kind).await,
            LoopCommand::Disable { kind } => commands::loops::run_disable(&client, &kind).await,
        },
        Command::Review { command } => match command {
            ReviewCommand::Today { json } => commands::review::run_today(&client, json).await,
            ReviewCommand::Week { json } => commands::review::run_week(&client, json).await,
        },
        Command::Artifact { command } => match command {
            ArtifactCommand::Latest { r#type: t, json } => {
                commands::artifact::run_latest(&client, &t, json).await
            }
        },
        Command::Import { command } => match command {
            ImportCommand::File { path, r#type: t } => {
                commands::import_::run_file(&client, &path, &t).await
            }
            ImportCommand::Lines { r#type: t } => commands::import_::run_lines(&client, &t).await,
            ImportCommand::CaptureUrl { url } => {
                commands::import_::run_capture_url(&client, &url).await
            }
        },
        Command::Export {
            captures,
            runs,
            artifacts,
            format,
            json,
        } => {
            commands::export_::run(&client, captures, runs, artifacts, format.as_str(), json).await
        }
        Command::Backup {} => commands::backup::run(&config).await,
        Command::Synthesize { command } => match command {
            SynthesizeCommand::Week { json } => commands::synthesize::run_week(&client, json).await,
            SynthesizeCommand::Project { name, json } => {
                commands::synthesize::run_project(&client, &name, json).await
            }
        },
        Command::Commitments {
            status,
            project,
            limit,
            json,
        } => {
            commands::commitments::run_list(
                &client,
                status.as_deref(),
                project.as_deref(),
                limit,
                json,
            )
            .await
        }
        Command::Commitment { command } => match command {
            CommitmentCommand::Add {
                text,
                kind,
                project,
            } => {
                commands::commitments::run_add(&client, &text, kind.as_deref(), project.as_deref())
                    .await
            }
            CommitmentCommand::Done { id } => commands::commitments::run_done(&client, &id).await,
            CommitmentCommand::Cancel { id } => {
                commands::commitments::run_cancel(&client, &id).await
            }
            CommitmentCommand::Inspect { id } => {
                commands::commitments::run_inspect(&client, &id).await
            }
            CommitmentCommand::Dependencies { id } => {
                commands::commitments::run_dependencies(&client, &id).await
            }
            CommitmentCommand::AddDependency {
                parent_id,
                child_id,
                r#type: t,
            } => {
                commands::commitments::run_add_dependency(&client, &parent_id, &child_id, &t).await
            }
        },
        Command::Sync { command } => match command {
            SyncCommand::Calendar => commands::sync::run_calendar(&client).await,
            SyncCommand::Todoist => commands::sync::run_todoist(&client).await,
            SyncCommand::Activity => commands::sync::run_activity(&client).await,
            SyncCommand::Git => commands::sync::run_git(&client).await,
            SyncCommand::Notes => commands::sync::run_notes(&client).await,
            SyncCommand::Transcripts => commands::sync::run_transcripts(&client).await,
            SyncCommand::Messaging => commands::sync::run_messaging(&client).await,
        },
        Command::Nudges { json } => commands::nudges::run_list(&client, json).await,
        Command::Nudge { command } => match command {
            NudgeCommand::Done { id } => commands::nudges::run_done(&client, &id).await,
            NudgeCommand::Snooze { id, minutes } => {
                commands::nudges::run_snooze(&client, &id, minutes).await
            }
            NudgeCommand::Inspect { id } => commands::nudges::run_inspect(&client, &id).await,
        },
        Command::Uncertainty { command } => match command {
            UncertaintyCommand::List { status, json } => {
                commands::uncertainty::run_list(&client, status.as_deref(), json).await
            }
            UncertaintyCommand::Inspect { id } => {
                commands::uncertainty::run_inspect(&client, &id).await
            }
            UncertaintyCommand::Resolve { id } => {
                commands::uncertainty::run_resolve(&client, &id).await
            }
        },
        Command::Evaluate {} => commands::evaluate::run(&client).await,
        Command::Context { command } => match command {
            ContextCommand::Show { json } => commands::context::run_current(&client, json).await,
            ContextCommand::Timeline { limit, json } => {
                commands::context::run_timeline(&client, limit, json).await
            }
        },
        Command::Explain { command } => match command {
            ExplainCommand::Nudge { id, json } => {
                commands::explain::run_nudge(&client, &id, json).await
            }
            ExplainCommand::Context { json } => commands::explain::run_context(&client, json).await,
            ExplainCommand::Commitment { id, json } => {
                commands::explain::run_commitment(&client, &id, json).await
            }
            ExplainCommand::Drift { json } => commands::explain::run_drift(&client, json).await,
        },
        Command::Thread { command } => match command {
            ThreadCommand::List {
                status,
                limit,
                json,
            } => commands::threads::run_list(&client, status.as_deref(), limit, json).await,
            ThreadCommand::Inspect { id } => commands::threads::run_inspect(&client, &id).await,
            ThreadCommand::Close { id } => commands::threads::run_close(&client, &id).await,
            ThreadCommand::Reopen { id } => commands::threads::run_reopen(&client, &id).await,
        },
        Command::Risk { id, json } => match id {
            Some(ref id) => commands::risk::run_commitment(&client, id, json).await,
            None => commands::risk::run_list(&client, json).await,
        },
        Command::Suggestions { state, json } => {
            commands::suggestions::run_list(&client, state.as_deref(), json).await
        }
        Command::Suggestion { command } => match command {
            SuggestionCommand::Inspect { id } => {
                commands::suggestions::run_inspect(&client, &id).await
            }
            SuggestionCommand::Accept { id } => {
                commands::suggestions::run_accept(&client, &id).await
            }
            SuggestionCommand::Reject { id, reason } => {
                commands::suggestions::run_reject(&client, &id, reason.as_deref()).await
            }
            SuggestionCommand::Modify { id, payload } => {
                commands::suggestions::run_modify(&client, &id, payload.as_deref()).await
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_parses_capture() {
        let cli = Cli::try_parse_from(["vel", "capture", "remember lidar"]).unwrap();
        match cli.command {
            Command::Capture { text, .. } => assert_eq!(text, Some("remember lidar".to_string())),
            _ => panic!("expected capture command"),
        }
    }

    #[test]
    fn cli_parses_search() {
        let cli = Cli::try_parse_from(["vel", "search", "--limit", "5", "lidar"]).unwrap();
        match cli.command {
            Command::Search { query, limit, .. } => {
                assert_eq!(query, "lidar");
                assert_eq!(limit, Some(5));
            }
            _ => panic!("expected search command"),
        }
    }

    #[test]
    fn cli_parses_today() {
        let cli = Cli::try_parse_from(["vel", "today", "--json"]).unwrap();
        match cli.command {
            Command::Today { json } => assert!(json),
            _ => panic!("expected today command"),
        }
    }

    #[test]
    fn cli_parses_sync_messaging() {
        let cli = Cli::try_parse_from(["vel", "sync", "messaging"]).unwrap();
        match cli.command {
            Command::Sync {
                command: SyncCommand::Messaging,
            } => {}
            _ => panic!("expected sync messaging command"),
        }
    }

    #[test]
    fn cli_parses_loops_list() {
        let cli = Cli::try_parse_from(["vel", "loops", "--json"]).unwrap();
        match cli.command {
            Command::Loops { json } => assert!(json),
            _ => panic!("expected loops command"),
        }
    }

    #[test]
    fn cli_parses_loop_inspect() {
        let cli =
            Cli::try_parse_from(["vel", "loop", "inspect", "evaluate_current_state", "--json"])
                .unwrap();
        match cli.command {
            Command::Loop {
                command: LoopCommand::Inspect { kind, json },
            } => {
                assert_eq!(kind, "evaluate_current_state");
                assert!(json);
            }
            _ => panic!("expected loop inspect command"),
        }
    }

    #[test]
    fn cli_parses_loop_enable_and_disable() {
        let enable = Cli::try_parse_from(["vel", "loop", "enable", "sync_calendar"]).unwrap();
        match enable.command {
            Command::Loop {
                command: LoopCommand::Enable { kind },
            } => assert_eq!(kind, "sync_calendar"),
            _ => panic!("expected loop enable command"),
        }

        let disable =
            Cli::try_parse_from(["vel", "loop", "disable", "sync_messaging"]).unwrap();
        match disable.command {
            Command::Loop {
                command: LoopCommand::Disable { kind },
            } => assert_eq!(kind, "sync_messaging"),
            _ => panic!("expected loop disable command"),
        }
    }

    #[test]
    fn cli_parses_uncertainty_commands() {
        let list = Cli::try_parse_from(["vel", "uncertainty", "list", "--status", "resolved"])
            .unwrap();
        match list.command {
            Command::Uncertainty {
                command: UncertaintyCommand::List { status, json },
            } => {
                assert_eq!(status.as_deref(), Some("resolved"));
                assert!(!json);
            }
            _ => panic!("expected uncertainty list command"),
        }

        let inspect = Cli::try_parse_from(["vel", "uncertainty", "inspect", "unc_123"]).unwrap();
        match inspect.command {
            Command::Uncertainty {
                command: UncertaintyCommand::Inspect { id },
            } => assert_eq!(id, "unc_123"),
            _ => panic!("expected uncertainty inspect command"),
        }

        let resolve = Cli::try_parse_from(["vel", "uncertainty", "resolve", "unc_456"]).unwrap();
        match resolve.command {
            Command::Uncertainty {
                command: UncertaintyCommand::Resolve { id },
            } => assert_eq!(id, "unc_456"),
            _ => panic!("expected uncertainty resolve command"),
        }
    }

    #[test]
    fn cli_parses_run_status_with_retry_flags() {
        let cli = Cli::try_parse_from([
            "vel",
            "run",
            "status",
            "run_123",
            "retry_scheduled",
            "--retry-after-seconds",
            "120",
            "--retry-at",
            "2026-03-16T22:10:00Z",
            "--reason",
            "transient_failure",
        ])
        .unwrap();

        match cli.command {
            Command::Run {
                command:
                    RunCommand::Status {
                        id,
                        status,
                        retry_after_seconds,
                        retry_at,
                        reason,
                        allow_unsupported_retry,
                        blocked_reason,
                    },
            } => {
                assert_eq!(id, "run_123");
                assert_eq!(status, "retry_scheduled");
                assert_eq!(retry_after_seconds, Some(120));
                assert_eq!(retry_at.as_deref(), Some("2026-03-16T22:10:00Z"));
                assert_eq!(reason.as_deref(), Some("transient_failure"));
                assert!(!allow_unsupported_retry);
                assert!(blocked_reason.is_none());
            }
            _ => panic!("expected run status command"),
        }
    }

    #[test]
    fn cli_parses_run_status_with_blocked_reason() {
        let cli = Cli::try_parse_from([
            "vel",
            "run",
            "status",
            "run_456",
            "blocked",
            "--blocked-reason",
            "awaiting dependency",
        ])
        .unwrap();

        match cli.command {
            Command::Run {
                command:
                    RunCommand::Status {
                        id,
                        status,
                        retry_after_seconds,
                        retry_at,
                        reason,
                        allow_unsupported_retry,
                        blocked_reason,
                    },
            } => {
                assert_eq!(id, "run_456");
                assert_eq!(status, "blocked");
                assert!(retry_after_seconds.is_none());
                assert!(retry_at.is_none());
                assert!(reason.is_none());
                assert!(!allow_unsupported_retry);
                assert_eq!(blocked_reason.as_deref(), Some("awaiting dependency"));
            }
            _ => panic!("expected run status command"),
        }
    }

    #[test]
    fn cli_parses_run_status_with_unsupported_retry_override() {
        let cli = Cli::try_parse_from([
            "vel",
            "run",
            "status",
            "run_789",
            "retry_scheduled",
            "--allow-unsupported-retry",
        ])
        .unwrap();

        match cli.command {
            Command::Run {
                command:
                    RunCommand::Status {
                        id,
                        status,
                        retry_after_seconds,
                        retry_at,
                        reason,
                        allow_unsupported_retry,
                        blocked_reason,
                    },
            } => {
                assert_eq!(id, "run_789");
                assert_eq!(status, "retry_scheduled");
                assert!(retry_after_seconds.is_none());
                assert!(retry_at.is_none());
                assert!(reason.is_none());
                assert!(allow_unsupported_retry);
                assert!(blocked_reason.is_none());
            }
            _ => panic!("expected run status command"),
        }
    }

    #[test]
    fn cli_parses_suggestion_reject_with_reason() {
        let cli = Cli::try_parse_from([
            "vel",
            "suggestion",
            "reject",
            "sugg_123",
            "--reason",
            "not useful",
        ])
        .unwrap();

        match cli.command {
            Command::Suggestion {
                command: SuggestionCommand::Reject { id, reason },
            } => {
                assert_eq!(id, "sugg_123");
                assert_eq!(reason.as_deref(), Some("not useful"));
            }
            _ => panic!("expected suggestion reject command"),
        }
    }

    #[test]
    fn cli_command_tree_builds() {
        Cli::command().debug_assert();
    }
}
