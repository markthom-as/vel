mod client;
mod command_lang;
mod commands;

use anyhow::Context;
use clap::{Args, Parser, Subcommand};
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
    Docs {
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
    Journal {
        #[command(subcommand)]
        command: JournalCommand,
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
    Command {
        #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
        input: Vec<String>,
        #[arg(long)]
        dry_run: bool,
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
    Integrations {
        #[command(subcommand)]
        command: IntegrationsCommand,
    },
    Connect {
        #[command(subcommand)]
        command: ConnectCommand,
    },
    Integration {
        #[command(subcommand)]
        command: IntegrationCommand,
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
    Backup {
        #[command(subcommand)]
        command: Option<BackupCommand>,
    },
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
enum BackupCommand {
    Manifest {
        #[command(subcommand)]
        command: BackupManifestCommand,
    },
}

#[derive(Debug, Subcommand)]
enum BackupManifestCommand {
    Create(BackupManifestCreateArgs),
    Verify(BackupManifestVerifyArgs),
}

#[derive(Debug, Args)]
struct BackupManifestCreateArgs {
    #[arg(long)]
    output: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct BackupManifestVerifyArgs {
    #[arg(long)]
    manifest: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Subcommand)]
enum JournalCommand {
    Mood {
        score: u8,
        #[arg(long)]
        label: Option<String>,
        #[arg(long)]
        note: Option<String>,
        #[arg(long)]
        source: Option<String>,
    },
    Pain {
        severity: u8,
        #[arg(long)]
        location: Option<String>,
        #[arg(long)]
        note: Option<String>,
        #[arg(long)]
        source: Option<String>,
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
    Status {
        id: String,
        status: String,
    },
    Close {
        id: String,
    },
    Reopen {
        id: String,
    },
}

#[derive(Debug, Subcommand)]
enum IntegrationsCommand {
    Connections {
        #[arg(long)]
        family: Option<String>,
        #[arg(long)]
        provider_key: Option<String>,
        #[arg(long)]
        include_disabled: bool,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum IntegrationCommand {
    Inspect {
        id: String,
        #[arg(long, default_value = "10")]
        events_limit: u32,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum ConnectCommand {
    Instances {
        #[arg(long)]
        json: bool,
    },
    Inspect {
        id: String,
        #[arg(long)]
        json: bool,
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
    Bootstrap {
        #[arg(long)]
        json: bool,
    },
    Cluster {
        #[arg(long)]
        json: bool,
    },
    Workers {
        #[arg(long)]
        json: bool,
    },
    BranchSync {
        branch: String,
        #[arg(long)]
        remote: Option<String>,
        #[arg(long)]
        base_branch: Option<String>,
        #[arg(long)]
        mode: Option<String>,
        #[arg(long)]
        requested_by: Option<String>,
        #[arg(long)]
        via_cluster: bool,
        #[arg(long)]
        json: bool,
    },
    Validation {
        profile_id: String,
        #[arg(long)]
        branch: Option<String>,
        #[arg(long)]
        environment: Option<String>,
        #[arg(long)]
        requested_by: Option<String>,
        #[arg(long)]
        via_cluster: bool,
        #[arg(long)]
        json: bool,
    },
    Calendar,
    Todoist,
    Activity,
    Health,
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
    /// Explain a command-language input (parse + inferred resolution + daemon plan)
    Command {
        #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
        input: Vec<String>,
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
        Command::Docs { json } => commands::docs::run(json),
        Command::Health { json } => commands::health::run(&client, json).await,
        Command::Capture {
            text,
            stdin,
            r#type: capture_type,
            source,
        } => {
            commands::capture::run(&client, text, stdin, capture_type.clone(), source.clone()).await
        }
        Command::Journal { command } => match command {
            JournalCommand::Mood {
                score,
                label,
                note,
                source,
            } => commands::journal::run_mood(&client, score, label, note, source).await,
            JournalCommand::Pain {
                severity,
                location,
                note,
                source,
            } => commands::journal::run_pain(&client, severity, location, note, source).await,
        },
        Command::Command {
            input,
            dry_run,
            json,
        } => command_lang::run(&client, input, dry_run, json).await,
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
        Command::Integrations { command } => match command {
            IntegrationsCommand::Connections {
                family,
                provider_key,
                include_disabled,
                json,
            } => {
                commands::integrations::run_list_connections(
                    &client,
                    family.as_deref(),
                    provider_key.as_deref(),
                    include_disabled,
                    json,
                )
                .await
            }
        },
        Command::Connect { command } => match command {
            ConnectCommand::Instances { json } => {
                commands::connect::run_list_instances(&client, json).await
            }
            ConnectCommand::Inspect { id, json } => {
                commands::connect::run_inspect_instance(&client, &id, json).await
            }
        },
        Command::Integration { command } => match command {
            IntegrationCommand::Inspect {
                id,
                events_limit,
                json,
            } => {
                commands::integrations::run_inspect_connection(&client, &id, events_limit, json)
                    .await
            }
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
        Command::Backup { command } => match command {
            None => commands::backup::run_guide(&config).await,
            Some(BackupCommand::Manifest { command }) => match command {
                BackupManifestCommand::Create(args) => {
                    commands::backup::run_manifest_create(
                        &config,
                        args.output.as_deref(),
                        args.json,
                    )
                    .await
                }
                BackupManifestCommand::Verify(args) => {
                    commands::backup::run_manifest_verify(
                        &config,
                        args.manifest.as_deref(),
                        args.json,
                    )
                    .await
                }
            },
        },
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
            SyncCommand::Bootstrap { json } => commands::sync::run_bootstrap(&client, json).await,
            SyncCommand::Cluster { json } => commands::sync::run_cluster(&client, json).await,
            SyncCommand::Workers { json } => commands::sync::run_workers(&client, json).await,
            SyncCommand::BranchSync {
                branch,
                remote,
                base_branch,
                mode,
                requested_by,
                via_cluster,
                json,
            } => {
                commands::sync::run_branch_sync_request(
                    &client,
                    &branch,
                    remote.as_deref(),
                    base_branch.as_deref(),
                    mode.as_deref(),
                    requested_by.as_deref(),
                    via_cluster,
                    json,
                )
                .await
            }
            SyncCommand::Validation {
                profile_id,
                branch,
                environment,
                requested_by,
                via_cluster,
                json,
            } => {
                commands::sync::run_validation_request(
                    &client,
                    &profile_id,
                    branch.as_deref(),
                    environment.as_deref(),
                    requested_by.as_deref(),
                    via_cluster,
                    json,
                )
                .await
            }
            SyncCommand::Calendar => commands::sync::run_calendar(&client).await,
            SyncCommand::Todoist => commands::sync::run_todoist(&client).await,
            SyncCommand::Activity => commands::sync::run_activity(&client).await,
            SyncCommand::Health => commands::sync::run_health(&client).await,
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
            ExplainCommand::Command { input, json } => {
                commands::explain::run_command(&client, input, json).await
            }
        },
        Command::Thread { command } => match command {
            ThreadCommand::List {
                status,
                limit,
                json,
            } => commands::threads::run_list(&client, status.as_deref(), limit, json).await,
            ThreadCommand::Inspect { id } => commands::threads::run_inspect(&client, &id).await,
            ThreadCommand::Status { id, status } => {
                commands::threads::run_status(&client, &id, &status).await
            }
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
    fn cli_parses_journal_mood() {
        let cli = Cli::try_parse_from([
            "vel",
            "journal",
            "mood",
            "7",
            "--label",
            "steady",
            "--note",
            "good enough",
        ])
        .unwrap();
        match cli.command {
            Command::Journal {
                command:
                    JournalCommand::Mood {
                        score,
                        label,
                        note,
                        source,
                    },
            } => {
                assert_eq!(score, 7);
                assert_eq!(label.as_deref(), Some("steady"));
                assert_eq!(note.as_deref(), Some("good enough"));
                assert!(source.is_none());
            }
            _ => panic!("expected journal mood command"),
        }
    }

    #[test]
    fn cli_parses_journal_pain() {
        let cli = Cli::try_parse_from([
            "vel",
            "journal",
            "pain",
            "4",
            "--location",
            "lower-back",
            "--source",
            "watch",
        ])
        .unwrap();
        match cli.command {
            Command::Journal {
                command:
                    JournalCommand::Pain {
                        severity,
                        location,
                        note,
                        source,
                    },
            } => {
                assert_eq!(severity, 4);
                assert_eq!(location.as_deref(), Some("lower-back"));
                assert!(note.is_none());
                assert_eq!(source.as_deref(), Some("watch"));
            }
            _ => panic!("expected journal pain command"),
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
    fn cli_parses_command_dry_run_json() {
        let cli = Cli::try_parse_from([
            "vel",
            "command",
            "--dry-run",
            "--json",
            "inspect",
            "capture",
            "cap_123",
        ])
        .unwrap();
        match cli.command {
            Command::Command {
                input,
                dry_run,
                json,
            } => {
                assert_eq!(input, vec!["inspect", "capture", "cap_123"]);
                assert!(dry_run);
                assert!(json);
            }
            _ => panic!("expected command command"),
        }
    }

    #[test]
    fn cli_parses_explain_command() {
        let cli = Cli::try_parse_from([
            "vel", "explain", "command", "--json", "should", "capture", "remember", "this",
        ])
        .unwrap();
        match cli.command {
            Command::Explain {
                command: ExplainCommand::Command { input, json },
            } => {
                assert_eq!(input, vec!["should", "capture", "remember", "this"]);
                assert!(json);
            }
            _ => panic!("expected explain command subcommand"),
        }
    }

    #[test]
    fn cli_parses_docs_json() {
        let cli = Cli::try_parse_from(["vel", "docs", "--json"]).unwrap();
        match cli.command {
            Command::Docs { json } => assert!(json),
            _ => panic!("expected docs command"),
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
    fn cli_parses_sync_bootstrap() {
        let cli = Cli::try_parse_from(["vel", "sync", "bootstrap"]).unwrap();
        match cli.command {
            Command::Sync {
                command: SyncCommand::Bootstrap { json },
            } => assert!(!json),
            _ => panic!("expected sync bootstrap command"),
        }
    }

    #[test]
    fn cli_parses_sync_cluster() {
        let cli = Cli::try_parse_from(["vel", "sync", "cluster", "--json"]).unwrap();
        match cli.command {
            Command::Sync {
                command: SyncCommand::Cluster { json },
            } => assert!(json),
            _ => panic!("expected sync cluster command"),
        }
    }

    #[test]
    fn cli_parses_sync_workers() {
        let cli = Cli::try_parse_from(["vel", "sync", "workers"]).unwrap();
        match cli.command {
            Command::Sync {
                command: SyncCommand::Workers { json },
            } => assert!(!json),
            _ => panic!("expected sync workers command"),
        }
    }

    #[test]
    fn cli_parses_sync_branch_sync() {
        let cli = Cli::try_parse_from([
            "vel",
            "sync",
            "branch-sync",
            "feature/swarm",
            "--remote",
            "origin",
            "--base-branch",
            "main",
            "--mode",
            "pull",
            "--requested-by",
            "cli",
            "--via-cluster",
            "--json",
        ])
        .unwrap();
        match cli.command {
            Command::Sync {
                command:
                    SyncCommand::BranchSync {
                        branch,
                        remote,
                        base_branch,
                        mode,
                        requested_by,
                        via_cluster,
                        json,
                    },
            } => {
                assert_eq!(branch, "feature/swarm");
                assert_eq!(remote.as_deref(), Some("origin"));
                assert_eq!(base_branch.as_deref(), Some("main"));
                assert_eq!(mode.as_deref(), Some("pull"));
                assert_eq!(requested_by.as_deref(), Some("cli"));
                assert!(via_cluster);
                assert!(json);
            }
            _ => panic!("expected sync branch-sync command"),
        }
    }

    #[test]
    fn cli_parses_sync_validation() {
        let cli = Cli::try_parse_from([
            "vel",
            "sync",
            "validation",
            "repo-verify",
            "--branch",
            "main",
            "--environment",
            "repo",
            "--requested-by",
            "cli",
        ])
        .unwrap();
        match cli.command {
            Command::Sync {
                command:
                    SyncCommand::Validation {
                        profile_id,
                        branch,
                        environment,
                        requested_by,
                        via_cluster,
                        json,
                    },
            } => {
                assert_eq!(profile_id, "repo-verify");
                assert_eq!(branch.as_deref(), Some("main"));
                assert_eq!(environment.as_deref(), Some("repo"));
                assert_eq!(requested_by.as_deref(), Some("cli"));
                assert!(!via_cluster);
                assert!(!json);
            }
            _ => panic!("expected sync validation command"),
        }
    }

    #[test]
    fn cli_parses_sync_health() {
        let cli = Cli::try_parse_from(["vel", "sync", "health"]).unwrap();
        match cli.command {
            Command::Sync {
                command: SyncCommand::Health,
            } => {}
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn cli_parses_integrations_connections() {
        let cli = Cli::try_parse_from([
            "vel",
            "integrations",
            "connections",
            "--family",
            "messaging",
            "--provider-key",
            "imessage",
            "--include-disabled",
            "--json",
        ])
        .unwrap();
        match cli.command {
            Command::Integrations {
                command:
                    IntegrationsCommand::Connections {
                        family,
                        provider_key,
                        include_disabled,
                        json,
                    },
            } => {
                assert_eq!(family.as_deref(), Some("messaging"));
                assert_eq!(provider_key.as_deref(), Some("imessage"));
                assert!(include_disabled);
                assert!(json);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn cli_parses_integration_inspect() {
        let cli = Cli::try_parse_from([
            "vel",
            "integration",
            "inspect",
            "conn_123",
            "--events-limit",
            "25",
            "--json",
        ])
        .unwrap();
        match cli.command {
            Command::Integration {
                command:
                    IntegrationCommand::Inspect {
                        id,
                        events_limit,
                        json,
                    },
            } => {
                assert_eq!(id, "conn_123");
                assert_eq!(events_limit, 25);
                assert!(json);
            }
            other => panic!("unexpected command: {:?}", other),
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

        let disable = Cli::try_parse_from(["vel", "loop", "disable", "sync_messaging"]).unwrap();
        match disable.command {
            Command::Loop {
                command: LoopCommand::Disable { kind },
            } => assert_eq!(kind, "sync_messaging"),
            _ => panic!("expected loop disable command"),
        }
    }

    #[test]
    fn cli_parses_uncertainty_commands() {
        let list =
            Cli::try_parse_from(["vel", "uncertainty", "list", "--status", "resolved"]).unwrap();
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
