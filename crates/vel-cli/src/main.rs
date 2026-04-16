mod client;
mod command_lang;
mod commands;

use anyhow::Context;
use clap::{Parser, Subcommand};
use vel_config::AppConfig;

#[derive(Debug, Parser)]
#[command(name = "vel", about = "Vel operator shell for now, setup, and threads")]
struct Cli {
    #[arg(long)]
    base_url: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
#[allow(clippy::enum_variant_names)]
enum Command {
    #[command(about = "Run advanced trust and runtime checks")]
    Doctor {
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Show published docs for daily use, setup, and deeper detail")]
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
    #[command(about = "Show the daily-use Now lane summary")]
    Today {
        #[arg(long)]
        json: bool,
    },
    Morning {
        #[arg(long)]
        json: bool,
    },
    Standup {
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
    Settings {
        #[command(subcommand)]
        command: SettingsCommand,
    },
    Policy {
        #[command(subcommand)]
        command: PolicyCommand,
    },
    Inspect {
        #[command(subcommand)]
        command: InspectCommand,
    },
    Agent {
        #[command(subcommand)]
        command: AgentCommand,
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
    Components {
        #[command(subcommand)]
        command: ComponentsCommand,
    },
    Connect {
        #[command(subcommand)]
        command: ConnectCommand,
    },
    Node {
        #[command(subcommand)]
        command: NodeCommand,
    },
    Integration {
        #[command(subcommand)]
        command: IntegrationCommand,
    },
    Component {
        #[command(subcommand)]
        command: ComponentCommand,
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
        #[arg(long)]
        create: bool,
        #[arg(long)]
        export: bool,
        #[arg(long = "export-status")]
        export_status: bool,
        #[arg(long)]
        output_root: Option<String>,
        #[arg(long)]
        target_root: Option<String>,
        #[arg(long = "domain")]
        domains: Vec<String>,
        #[arg(long)]
        inspect: Option<String>,
        #[arg(long)]
        verify: Option<String>,
        #[arg(long = "dry-run-restore")]
        dry_run_restore: Option<String>,
        #[arg(long)]
        json: bool,
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
    #[command(
        about = "Recompute and persist runtime evaluation",
        long_about = "Recompute and persist context, risk, nudges, and downstream runtime outputs.\n\nFor deterministic fixture replay and regression reports, use the standalone eval path:\n  cargo run -p veld-evals -- run --fixtures crates/veld-evals/fixtures/sample-day-context.json --report /tmp/vel-eval-report.json\n\nThat runner uses the vel-sim harness; vel evaluate does not embed veld-evals or change replay behavior."
    )]
    Evaluate {},
    #[command(
        about = "Prepare, review, and launch supervised execution handoffs",
        long_about = "Prepare, review, and launch supervised execution handoffs.\n\nThe reference client for external/runtime envelopes is vel-agent-sdk. Its live-envelope helpers are AgentSdkClient::manifest_reference(...) and AgentSdkClient::connect_launch_request(...)."
    )]
    Exec {
        #[command(subcommand)]
        command: ExecCommand,
    },
    Context {
        #[command(subcommand)]
        command: ContextCommand,
    },
    DailyLoop {
        #[command(subcommand)]
        command: DailyLoopCommand,
    },
    Explain {
        #[command(subcommand)]
        command: ExplainCommand,
    },
    #[command(about = "List and inspect the continuity/archive thread lane")]
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
    #[command(about = "Inspect the canonical planning profile used by day-plan and reflow")]
    PlanningProfile {
        #[arg(long)]
        json: bool,
        /// Apply a staged proposal by ID
        #[arg(long, value_name = "ID")]
        apply_proposal: Option<String>,
    },
    #[command(about = "List and inspect person records")]
    People {
        #[command(subcommand)]
        command: PeopleCommand,
    },
    #[command(about = "List and inspect project records")]
    Project {
        #[command(subcommand)]
        command: ProjectCommand,
    },
    #[command(about = "List and create signals")]
    Signals {
        #[command(subcommand)]
        command: SignalCommand,
    },
    #[command(about = "Inspect LLM profile health")]
    Llm {
        #[command(subcommand)]
        command: LlmCommand,
    },
}

#[derive(Debug, Subcommand)]
enum PeopleCommand {
    List {
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
enum ProjectCommand {
    List {
        #[arg(long)]
        json: bool,
    },
    Families {
        #[arg(long)]
        json: bool,
    },
    Inspect {
        id: String,
        #[arg(long)]
        json: bool,
    },
    Create {
        #[arg(long)]
        slug: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        family: String,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        repo_path: String,
        #[arg(long)]
        notes_path: String,
        #[arg(long)]
        create_repo: bool,
        #[arg(long)]
        create_notes_root: bool,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum SignalCommand {
    List {
        #[arg(long)]
        signal_type: Option<String>,
        #[arg(long)]
        since_ts: Option<i64>,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        json: bool,
    },
    Create {
        #[arg(long)]
        signal_type: String,
        #[arg(long)]
        source: String,
        #[arg(long)]
        source_ref: Option<String>,
        #[arg(long)]
        payload: Option<String>,
        #[arg(long)]
        json: bool,
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
enum ExecCommand {
    Show {
        project_id: String,
        #[arg(long)]
        json: bool,
    },
    Save {
        project_id: String,
        #[arg(long)]
        objective: String,
        #[arg(long)]
        repo_brief: Option<String>,
        #[arg(long)]
        notes_brief: Option<String>,
        #[arg(long = "constraint")]
        constraints: Vec<String>,
        #[arg(long = "expected-output")]
        expected_outputs: Vec<String>,
        #[arg(long)]
        json: bool,
    },
    Preview {
        project_id: String,
        #[arg(long)]
        output_dir: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Export {
        project_id: String,
        #[arg(long)]
        output_dir: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Review {
        #[arg(long)]
        project_id: Option<String>,
        #[arg(long)]
        state: Option<String>,
        #[arg(long)]
        json: bool,
    },
    #[command(
        about = "Preview whether an approved handoff can launch",
        long_about = "Preview whether an approved handoff can launch.\n\nUse this before runtime handoff launch to inspect blockers, write scope, and the vel-agent-sdk reference envelope helpers: AgentSdkClient::manifest_reference(...) and AgentSdkClient::connect_launch_request(...)."
    )]
    LaunchPreview {
        handoff_id: String,
        #[arg(long)]
        json: bool,
    },
    Approve {
        handoff_id: String,
        #[arg(long)]
        reviewed_by: Option<String>,
        #[arg(long)]
        reason: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Reject {
        handoff_id: String,
        #[arg(long)]
        reviewed_by: Option<String>,
        #[arg(long)]
        reason: Option<String>,
        #[arg(long)]
        json: bool,
    },
    #[command(
        about = "Launch an approved handoff through the supervised runtime lane",
        long_about = "Launch an approved handoff through the supervised runtime lane.\n\nFor external agent clients, treat vel-agent-sdk as the reference implementation for the manifest reference and connect-launch request envelopes."
    )]
    Launch {
        handoff_id: String,
        #[arg(long, default_value = "local_command")]
        runtime_kind: String,
        #[arg(long)]
        actor_id: Option<String>,
        #[arg(long)]
        display_name: Option<String>,
        #[arg(long)]
        working_dir: Option<String>,
        #[arg(long = "writable-root")]
        writable_roots: Vec<String>,
        #[arg(long)]
        lease_seconds: Option<i64>,
        #[arg(long)]
        json: bool,
        #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
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
    Follow {
        id: String,
        #[arg(long)]
        after_id: Option<i64>,
        #[arg(long, default_value = "200")]
        limit: u32,
        #[arg(long, default_value = "500")]
        poll_ms: u64,
        #[arg(long)]
        once: bool,
    },
    Reply {
        id: String,
        #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
        input: Vec<String>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum IntegrationsCommand {
    Show {
        #[arg(long)]
        json: bool,
    },
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
enum ComponentsCommand {
    List {
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
    Logs {
        id: String,
        #[arg(long, default_value = "10")]
        limit: u32,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum ComponentCommand {
    Logs {
        id: String,
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        json: bool,
    },
    Restart {
        id: String,
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
    Launch {
        #[arg(long, default_value = "local_command")]
        runtime_kind: String,
        #[arg(long)]
        actor_id: String,
        #[arg(long)]
        display_name: Option<String>,
        #[arg(long)]
        working_dir: Option<String>,
        #[arg(long = "writable-root")]
        writable_roots: Vec<String>,
        #[arg(long)]
        lease_seconds: Option<i64>,
        #[arg(long)]
        json: bool,
        #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },
    Inspect {
        id: String,
        #[arg(long)]
        json: bool,
    },
    Attach {
        id: String,
        #[arg(long)]
        json: bool,
    },
    Heartbeat {
        id: String,
        #[arg(long, default_value = "healthy")]
        status: String,
        #[arg(long)]
        json: bool,
    },
    Terminate {
        id: String,
        #[arg(long, default_value = "operator_requested")]
        reason: String,
        #[arg(long)]
        json: bool,
    },
    Stdin {
        id: String,
        input: String,
        #[arg(long)]
        json: bool,
    },
    Events {
        id: String,
        #[arg(long)]
        after_id: Option<i64>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        json: bool,
    },
    Tail {
        id: String,
        #[arg(long)]
        after_id: Option<i64>,
        #[arg(long, default_value = "200")]
        limit: u32,
        #[arg(long, default_value = "500")]
        poll_ms: u64,
        #[arg(long)]
        once: bool,
    },
    Stream {
        id: String,
        #[arg(long)]
        after_id: Option<i64>,
        #[arg(long, default_value = "200")]
        limit: u32,
        #[arg(long, default_value = "500")]
        poll_ms: u64,
        #[arg(long)]
        max_events: Option<u32>,
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
enum AgentCommand {
    Inspect {
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum RunCommand {
    /// Execute an intent-driven run path via command-language orchestration.
    Intent {
        #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
        input: Vec<String>,
        #[arg(long)]
        json: bool,
    },
    /// Preview an intent-driven run path without mutating side effects.
    DryRun {
        #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
        input: Vec<String>,
        #[arg(long)]
        json: bool,
    },
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
enum SettingsCommand {
    Show {
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum PolicyCommand {
    Check {
        #[arg(long)]
        path: Option<String>,
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
enum LlmCommand {
    ProfileHealth {
        profile_id: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum NodeCommand {
    Link {
        #[command(subcommand)]
        command: NodeLinkCommand,
    },
    Status {
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum NodeLinkCommand {
    Issue {
        #[arg(long)]
        issued_by_node_id: Option<String>,
        #[arg(long = "expires-seconds")]
        expires_seconds: Option<i64>,
        #[arg(long)]
        read_context: bool,
        #[arg(long)]
        write_safe_actions: bool,
        #[arg(long)]
        execute_repo_tasks: bool,
        #[arg(long)]
        json: bool,
    },
    Redeem {
        token_code: String,
        #[arg(long)]
        node_id: String,
        #[arg(long)]
        node_display_name: String,
        #[arg(long)]
        transport_hint: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Revoke {
        node_id: String,
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
    Run {
        id: String,
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
    #[command(name = "codex-workspace")]
    CodexWorkspace { path: String },
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
    Reminders,
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
    /// Explain a run from persisted events, artifacts, and status metadata.
    Run {
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

#[derive(Debug, Subcommand)]
enum DailyLoopCommand {
    Reply {
        session_id: String,
        text: String,
        #[arg(long)]
        json: bool,
    },
    Skip {
        session_id: String,
        #[arg(long)]
        json: bool,
    },
    SkipCheckIn {
        check_in_event_id: String,
        #[arg(long)]
        reason_code: Option<String>,
        #[arg(long)]
        reason_text: Option<String>,
        #[arg(long)]
        source: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Overdue {
        #[command(subcommand)]
        command: DailyLoopOverdueCommand,
    },
}

#[derive(Debug, Subcommand)]
enum DailyLoopOverdueCommand {
    Menu {
        #[arg(long, default_value = "50")]
        limit: u32,
        #[arg(long)]
        json: bool,
    },
    Confirm {
        commitment_id: String,
        #[arg(long)]
        action: String,
        #[arg(long)]
        due_at: Option<String>,
        #[arg(long)]
        reason: Option<String>,
        #[arg(long)]
        use_vel_guess: bool,
        #[arg(long)]
        json: bool,
    },
    Apply {
        proposal_id: String,
        #[arg(long)]
        confirmation_token: String,
        #[arg(long)]
        idempotency_key: Option<String>,
        #[arg(long)]
        json: bool,
    },
    Undo {
        action_event_id: String,
        #[arg(long)]
        idempotency_key: Option<String>,
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
        Command::Standup { json } => {
            commands::daily_loop::run_start(
                &client,
                vel_api_types::DailyLoopPhaseData::Standup,
                json,
            )
            .await
        }
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
        Command::Settings { command } => match command {
            SettingsCommand::Show { json } => commands::settings::run_show(&client, json).await,
        },
        Command::Policy { command } => match command {
            PolicyCommand::Check { path, json } => {
                commands::policy::run_check(path.as_deref(), json)
            }
        },
        Command::Inspect { command } => match command {
            InspectCommand::Capture { id } => commands::inspect::run_capture(&client, &id).await,
            InspectCommand::Artifact { id } => commands::inspect::run_artifact(&client, &id).await,
        },
        Command::Agent { command } => match command {
            AgentCommand::Inspect { json } => commands::agent::run_inspect(&client, json).await,
        },
        Command::Run { command } => match command {
            RunCommand::Intent { input, json } => {
                command_lang::run(&client, input, false, json).await
            }
            RunCommand::DryRun { input, json } => {
                command_lang::run(&client, input, true, json).await
            }
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
            IntegrationsCommand::Show { json } => {
                commands::integrations::run_show(&client, json).await
            }
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
        Command::Components { command } => match command {
            ComponentsCommand::List { json } => commands::components::run_list(&client, json).await,
        },
        Command::Connect { command } => match command {
            ConnectCommand::Instances { json } => {
                commands::connect::run_list_instances(&client, json).await
            }
            ConnectCommand::Launch {
                runtime_kind,
                actor_id,
                display_name,
                command,
                working_dir,
                writable_roots,
                lease_seconds,
                json,
            } => {
                commands::connect::run_launch_instance(
                    &client,
                    crate::client::ConnectLaunchRequestData {
                        runtime_kind,
                        actor_id,
                        display_name,
                        command,
                        working_dir,
                        writable_roots,
                        capability_allowlist: Vec::new(),
                        lease_seconds,
                    },
                    json,
                )
                .await
            }
            ConnectCommand::Inspect { id, json } => {
                commands::connect::run_inspect_instance(&client, &id, json).await
            }
            ConnectCommand::Attach { id, json } => {
                commands::connect::run_attach_instance(&client, &id, json).await
            }
            ConnectCommand::Heartbeat { id, status, json } => {
                commands::connect::run_heartbeat_instance(&client, &id, &status, json).await
            }
            ConnectCommand::Terminate { id, reason, json } => {
                commands::connect::run_terminate_instance(&client, &id, &reason, json).await
            }
            ConnectCommand::Stdin { id, input, json } => {
                commands::connect::run_stdin_instance(&client, &id, &input, json).await
            }
            ConnectCommand::Events {
                id,
                after_id,
                limit,
                json,
            } => commands::connect::run_events_instance(&client, &id, after_id, limit, json).await,
            ConnectCommand::Tail {
                id,
                after_id,
                limit,
                poll_ms,
                once,
            } => {
                commands::connect::run_tail_instance(&client, &id, after_id, limit, poll_ms, once)
                    .await
            }
            ConnectCommand::Stream {
                id,
                after_id,
                limit,
                poll_ms,
                max_events,
            } => {
                commands::connect::run_stream_instance(
                    &client, &id, after_id, limit, poll_ms, max_events,
                )
                .await
            }
        },
        Command::Node { command } => match command {
            NodeCommand::Link { command } => match command {
                NodeLinkCommand::Issue {
                    issued_by_node_id,
                    expires_seconds,
                    read_context,
                    write_safe_actions,
                    execute_repo_tasks,
                    json,
                } => {
                    commands::node::run_link_issue(
                        &client,
                        issued_by_node_id.as_deref(),
                        config.node_id.as_deref(),
                        expires_seconds,
                        read_context,
                        write_safe_actions,
                        execute_repo_tasks,
                        json,
                    )
                    .await
                }
                NodeLinkCommand::Redeem {
                    token_code,
                    node_id,
                    node_display_name,
                    transport_hint,
                    json,
                } => {
                    commands::node::run_link_redeem(
                        &client,
                        &token_code,
                        &node_id,
                        &node_display_name,
                        transport_hint.as_deref(),
                        json,
                    )
                    .await
                }
                NodeLinkCommand::Revoke { node_id } => {
                    commands::node::run_link_revoke(&client, &node_id).await
                }
            },
            NodeCommand::Status { json } => commands::node::run_status(&client, json).await,
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
            IntegrationCommand::Logs { id, limit, json } => {
                commands::integrations::run_logs(&client, &id, limit, json).await
            }
        },
        Command::Component { command } => match command {
            ComponentCommand::Logs { id, limit, json } => {
                commands::components::run_logs(&client, &id, limit, json).await
            }
            ComponentCommand::Restart { id, json } => {
                commands::components::run_restart(&client, &id, json).await
            }
        },
        Command::Artifact { command } => match command {
            ArtifactCommand::Latest { r#type: t, json } => {
                commands::artifact::run_latest(&client, &t, json).await
            }
            ArtifactCommand::Run { id, json } => {
                commands::artifact::run_for_run(&client, &id, json).await
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
            ImportCommand::CodexWorkspace { path } => {
                commands::import_::run_codex_workspace(&client, &path).await
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
        Command::Backup {
            create,
            export,
            export_status,
            output_root,
            target_root,
            domains,
            inspect,
            verify,
            dry_run_restore,
            json,
        } => {
            commands::backup::run(
                &client,
                &config,
                create,
                export,
                export_status,
                output_root.as_deref(),
                target_root.as_deref(),
                domains,
                inspect.as_deref(),
                verify.as_deref(),
                dry_run_restore.as_deref(),
                json,
            )
            .await
        }
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
            SyncCommand::Reminders => commands::sync::run_reminders(&client).await,
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
        Command::Exec { command } => match command {
            ExecCommand::Show { project_id, json } => {
                commands::exec::run_show_context(&client, &project_id, json).await
            }
            ExecCommand::Save {
                project_id,
                objective,
                repo_brief,
                notes_brief,
                constraints,
                expected_outputs,
                json,
            } => {
                commands::exec::run_save_context(
                    &client,
                    &project_id,
                    crate::client::ExecutionContextSaveRequestData {
                        objective,
                        repo_brief: repo_brief.unwrap_or_default(),
                        notes_brief: notes_brief.unwrap_or_default(),
                        constraints,
                        expected_outputs,
                    },
                    json,
                )
                .await
            }
            ExecCommand::Preview {
                project_id,
                output_dir,
                json,
            } => {
                commands::exec::run_preview_context(
                    &client,
                    &project_id,
                    output_dir.as_deref(),
                    json,
                )
                .await
            }
            ExecCommand::Export {
                project_id,
                output_dir,
                json,
            } => {
                commands::exec::run_export_context(
                    &client,
                    &project_id,
                    output_dir.as_deref(),
                    json,
                )
                .await
            }
            ExecCommand::Review {
                project_id,
                state,
                json,
            } => {
                commands::exec::run_review_handoffs(
                    &client,
                    project_id.as_deref(),
                    state.as_deref(),
                    json,
                )
                .await
            }
            ExecCommand::LaunchPreview { handoff_id, json } => {
                commands::exec::run_preview_handoff_launch(&client, &handoff_id, json).await
            }
            ExecCommand::Approve {
                handoff_id,
                reviewed_by,
                reason,
                json,
            } => {
                commands::exec::run_approve_handoff(
                    &client,
                    &handoff_id,
                    reviewed_by.as_deref().unwrap_or("operator_shell"),
                    reason,
                    json,
                )
                .await
            }
            ExecCommand::Reject {
                handoff_id,
                reviewed_by,
                reason,
                json,
            } => {
                commands::exec::run_reject_handoff(
                    &client,
                    &handoff_id,
                    reviewed_by.as_deref().unwrap_or("operator_shell"),
                    reason,
                    json,
                )
                .await
            }
            ExecCommand::Launch {
                handoff_id,
                runtime_kind,
                actor_id,
                display_name,
                working_dir,
                writable_roots,
                lease_seconds,
                json,
                command,
            } => {
                commands::exec::run_launch_handoff(
                    &client,
                    &handoff_id,
                    crate::client::LaunchExecutionHandoffRequestData {
                        runtime_kind,
                        actor_id,
                        display_name,
                        command,
                        working_dir,
                        writable_roots,
                        capability_allowlist: Vec::new(),
                        lease_seconds,
                    },
                    json,
                )
                .await
            }
        },
        Command::Context { command } => match command {
            ContextCommand::Show { json } => commands::context::run_current(&client, json).await,
            ContextCommand::Timeline { limit, json } => {
                commands::context::run_timeline(&client, limit, json).await
            }
        },
        Command::DailyLoop { command } => match command {
            DailyLoopCommand::Reply {
                session_id,
                text,
                json,
            } => commands::daily_loop::run_reply(&client, &session_id, text, json).await,
            DailyLoopCommand::Skip { session_id, json } => {
                commands::daily_loop::run_skip(&client, &session_id, json).await
            }
            DailyLoopCommand::SkipCheckIn {
                check_in_event_id,
                reason_code,
                reason_text,
                source,
                json,
            } => {
                commands::daily_loop::run_skip_check_in(
                    &client,
                    &check_in_event_id,
                    reason_code,
                    reason_text,
                    source,
                    json,
                )
                .await
            }
            DailyLoopCommand::Overdue { command } => match command {
                DailyLoopOverdueCommand::Menu { limit, json } => {
                    commands::daily_loop::run_overdue_menu(&client, limit, json).await
                }
                DailyLoopOverdueCommand::Confirm {
                    commitment_id,
                    action,
                    due_at,
                    reason,
                    use_vel_guess,
                    json,
                } => {
                    let action = match action.as_str() {
                        "close" => vel_api_types::DailyLoopOverdueActionData::Close,
                        "reschedule" => vel_api_types::DailyLoopOverdueActionData::Reschedule,
                        "back_to_inbox" => vel_api_types::DailyLoopOverdueActionData::BackToInbox,
                        "tombstone" => vel_api_types::DailyLoopOverdueActionData::Tombstone,
                        other => {
                            return Err(anyhow::anyhow!(
                                "unsupported overdue action '{}'; use close|reschedule|back_to_inbox|tombstone",
                                other
                            ))
                        }
                    };
                    commands::daily_loop::run_overdue_confirm(
                        &client,
                        &commitment_id,
                        action,
                        due_at,
                        reason,
                        use_vel_guess,
                        json,
                    )
                    .await
                }
                DailyLoopOverdueCommand::Apply {
                    proposal_id,
                    confirmation_token,
                    idempotency_key,
                    json,
                } => {
                    commands::daily_loop::run_overdue_apply(
                        &client,
                        &proposal_id,
                        &confirmation_token,
                        idempotency_key,
                        json,
                    )
                    .await
                }
                DailyLoopOverdueCommand::Undo {
                    action_event_id,
                    idempotency_key,
                    json,
                } => {
                    commands::daily_loop::run_overdue_undo(
                        &client,
                        &action_event_id,
                        idempotency_key,
                        json,
                    )
                    .await
                }
            },
        },
        Command::Explain { command } => match command {
            ExplainCommand::Nudge { id, json } => {
                commands::explain::run_nudge(&client, &id, json).await
            }
            ExplainCommand::Context { json } => commands::explain::run_context(&client, json).await,
            ExplainCommand::Commitment { id, json } => {
                commands::explain::run_commitment(&client, &id, json).await
            }
            ExplainCommand::Run { id, json } => {
                commands::runs::run_inspect(&client, &id, json).await
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
            ThreadCommand::Close { id } => commands::threads::run_close(&client, &id).await,
            ThreadCommand::Reopen { id } => commands::threads::run_reopen(&client, &id).await,
            ThreadCommand::Follow {
                id,
                after_id,
                limit,
                poll_ms,
                once,
            } => commands::threads::run_follow(&client, &id, after_id, limit, poll_ms, once).await,
            ThreadCommand::Reply { id, input, json } => {
                commands::threads::run_reply(&client, &id, input, json).await
            }
        },
        Command::Risk { id, json } => match id {
            Some(ref id) => commands::risk::run_commitment(&client, id, json).await,
            None => commands::risk::run_list(&client, json).await,
        },
        Command::Suggestions { state, json } => {
            commands::suggestions::run_list(&client, state.as_deref(), json).await
        }
        Command::PlanningProfile {
            json,
            apply_proposal,
        } => {
            if let Some(id) = apply_proposal {
                commands::planning_profile::run_apply_proposal(&client, &id, json).await
            } else {
                commands::planning_profile::run(&client, json).await
            }
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
        Command::People { command } => match command {
            PeopleCommand::List { json } => commands::people::run_list(&client, json).await,
            PeopleCommand::Inspect { id, json } => {
                commands::people::run_inspect(&client, &id, json).await
            }
        },
        Command::Project { command } => match command {
            ProjectCommand::List { json } => commands::projects::run_list(&client, json).await,
            ProjectCommand::Families { json } => {
                commands::projects::run_families(&client, json).await
            }
            ProjectCommand::Inspect { id, json } => {
                commands::projects::run_inspect(&client, &id, json).await
            }
            ProjectCommand::Create {
                slug,
                name,
                family,
                status,
                repo_path,
                notes_path,
                create_repo,
                create_notes_root,
                json,
            } => {
                commands::projects::run_create(
                    &client,
                    &slug,
                    &name,
                    &family,
                    status.as_deref(),
                    &repo_path,
                    &notes_path,
                    create_repo,
                    create_notes_root,
                    json,
                )
                .await
            }
        },
        Command::Signals { command } => match command {
            SignalCommand::List {
                signal_type,
                since_ts,
                limit,
                json,
            } => {
                commands::signals::run_list(&client, signal_type.as_deref(), since_ts, limit, json)
                    .await
            }
            SignalCommand::Create {
                signal_type,
                source,
                source_ref,
                payload,
                json,
            } => {
                commands::signals::run_create(
                    &client,
                    &signal_type,
                    &source,
                    source_ref.as_deref(),
                    payload.as_deref(),
                    json,
                )
                .await
            }
        },
        Command::Llm { command } => match command {
            LlmCommand::ProfileHealth { profile_id, json } => {
                commands::llm::run_profile_health(&client, &profile_id, json).await
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
    fn cli_parses_exec_export() {
        let cli = Cli::try_parse_from([
            "vel",
            "exec",
            "export",
            "proj_123",
            "--output-dir",
            ".planning/vel",
        ])
        .unwrap();

        match cli.command {
            Command::Exec {
                command:
                    ExecCommand::Export {
                        project_id,
                        output_dir,
                        json,
                    },
            } => {
                assert_eq!(project_id, "proj_123");
                assert_eq!(output_dir.as_deref(), Some(".planning/vel"));
                assert!(!json);
            }
            _ => panic!("expected exec export command"),
        }
    }

    #[test]
    fn cli_parses_agent_inspect_json() {
        let cli = Cli::try_parse_from(["vel", "agent", "inspect", "--json"]).unwrap();
        match cli.command {
            Command::Agent {
                command: AgentCommand::Inspect { json },
            } => assert!(json),
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn cli_parses_backup_export() {
        let cli = Cli::try_parse_from([
            "vel",
            "backup",
            "--export",
            "--target-root",
            "/tmp/vel-export",
            "--domain",
            "calendar",
            "--domain",
            "tasks",
        ])
        .unwrap();

        match cli.command {
            Command::Backup {
                export,
                target_root,
                domains,
                ..
            } => {
                assert!(export);
                assert_eq!(target_root.as_deref(), Some("/tmp/vel-export"));
                assert_eq!(domains, vec!["calendar".to_string(), "tasks".to_string()]);
            }
            other => panic!("unexpected command: {:?}", other),
        }

        let status = Cli::try_parse_from(["vel", "backup", "--export-status"]).unwrap();
        match status.command {
            Command::Backup { export_status, .. } => assert!(export_status),
            other => panic!("unexpected command: {:?}", other),
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
    fn cli_parses_settings_show() {
        let cli = Cli::try_parse_from(["vel", "settings", "show", "--json"]).unwrap();
        match cli.command {
            Command::Settings {
                command: SettingsCommand::Show { json },
            } => assert!(json),
            _ => panic!("expected settings show command"),
        }
    }

    #[test]
    fn cli_parses_components_list() {
        let cli = Cli::try_parse_from(["vel", "components", "list", "--json"]).unwrap();
        match cli.command {
            Command::Components {
                command: ComponentsCommand::List { json },
            } => assert!(json),
            _ => panic!("expected components list command"),
        }
    }

    #[test]
    fn cli_parses_project_list() {
        let cli = Cli::try_parse_from(["vel", "project", "list", "--json"]).unwrap();
        match cli.command {
            Command::Project {
                command: ProjectCommand::List { json },
            } => assert!(json),
            _ => panic!("expected project list command"),
        }
    }

    #[test]
    fn cli_parses_project_inspect() {
        let cli = Cli::try_parse_from(["vel", "project", "inspect", "proj_123", "--json"]).unwrap();
        match cli.command {
            Command::Project {
                command: ProjectCommand::Inspect { id, json },
            } => {
                assert_eq!(id, "proj_123");
                assert!(json);
            }
            _ => panic!("expected project inspect command"),
        }
    }

    #[test]
    fn cli_parses_project_families() {
        let cli = Cli::try_parse_from(["vel", "project", "families", "--json"]).unwrap();
        match cli.command {
            Command::Project {
                command: ProjectCommand::Families { json },
            } => assert!(json),
            _ => panic!("expected project families command"),
        }
    }

    #[test]
    fn cli_parses_project_create() {
        let cli = Cli::try_parse_from([
            "vel",
            "project",
            "create",
            "--slug",
            "vel",
            "--name",
            "Vel",
            "--family",
            "creative",
            "--status",
            "active",
            "--repo-path",
            "/code/vel",
            "--notes-path",
            "/notes/vel",
            "--create-notes-root",
            "--json",
        ])
        .unwrap();
        match cli.command {
            Command::Project {
                command:
                    ProjectCommand::Create {
                        slug,
                        name,
                        family,
                        status,
                        repo_path,
                        notes_path,
                        create_repo,
                        create_notes_root,
                        json,
                    },
            } => {
                assert_eq!(slug, "vel");
                assert_eq!(name, "Vel");
                assert_eq!(family, "creative");
                assert_eq!(status.as_deref(), Some("active"));
                assert_eq!(repo_path, "/code/vel");
                assert_eq!(notes_path, "/notes/vel");
                assert!(!create_repo);
                assert!(create_notes_root);
                assert!(json);
            }
            _ => panic!("expected project create command"),
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
    fn cli_parses_explain_run_command() {
        let cli = Cli::try_parse_from(["vel", "explain", "run", "run_123", "--json"]).unwrap();
        match cli.command {
            Command::Explain {
                command: ExplainCommand::Run { id, json },
            } => {
                assert_eq!(id, "run_123");
                assert!(json);
            }
            _ => panic!("expected explain run command"),
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
    fn cli_parses_planning_profile_json() {
        let cli = Cli::try_parse_from(["vel", "planning-profile", "--json"]).unwrap();
        match cli.command {
            Command::PlanningProfile { json, .. } => assert!(json),
            _ => panic!("expected planning-profile command"),
        }
    }

    #[test]
    fn cli_help_uses_shell_taxonomy_framing() {
        let help = Cli::command().render_long_help().to_string();
        assert!(help.contains("Vel operator shell for now, setup, and threads"));
        assert!(help.contains("Run advanced trust and runtime checks"));
        assert!(help.contains("Show published docs for daily use, setup, and deeper detail"));
        assert!(help.contains("Show the daily-use Now lane summary"));
        assert!(help.contains("Inspect the canonical planning profile used by day-plan and reflow"));
        assert!(help.contains("List and inspect the continuity/archive thread lane"));
    }

    #[test]
    fn evaluate_help_points_to_fixture_replay() {
        let mut command = Cli::command();
        let evaluate = command
            .find_subcommand_mut("evaluate")
            .expect("evaluate subcommand");
        let help = evaluate.render_long_help().to_string();

        assert!(help.contains(
            "Recompute and persist context, risk, nudges, and downstream runtime outputs"
        ));
        assert!(help.contains("cargo run -p veld-evals -- run"));
        assert!(help.contains("crates/veld-evals/fixtures/sample-day-context.json"));
        assert!(help.contains("vel-sim"));
        assert!(help.contains("does not embed veld-evals"));
    }

    #[test]
    fn exec_help_points_to_agent_sdk_reference_client() {
        let mut command = Cli::command();
        let exec = command
            .find_subcommand_mut("exec")
            .expect("exec subcommand");
        let help = exec.render_long_help().to_string();

        assert!(help.contains("Prepare, review, and launch supervised execution handoffs"));
        assert!(help.contains("reference client for external/runtime envelopes is vel-agent-sdk"));
        assert!(help.contains("AgentSdkClient::manifest_reference(...)"));
        assert!(help.contains("AgentSdkClient::connect_launch_request(...)"));
    }

    #[test]
    fn exec_launch_help_points_to_agent_sdk_envelopes() {
        let mut command = Cli::command();
        let exec = command
            .find_subcommand_mut("exec")
            .expect("exec subcommand");
        let launch = exec
            .find_subcommand_mut("launch")
            .expect("exec launch subcommand");
        let help = launch.render_long_help().to_string();

        assert!(help.contains("supervised runtime lane"));
        assert!(help.contains("vel-agent-sdk"));
        assert!(help.contains("manifest reference"));
        assert!(help.contains("connect-launch request envelopes"));
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
    fn cli_parses_sync_reminders() {
        let cli = Cli::try_parse_from(["vel", "sync", "reminders"]).unwrap();
        match cli.command {
            Command::Sync {
                command: SyncCommand::Reminders,
            } => {}
            _ => panic!("expected sync reminders command"),
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
    fn cli_parses_integrations_show() {
        let cli = Cli::try_parse_from(["vel", "integrations", "show", "--json"]).unwrap();
        match cli.command {
            Command::Integrations {
                command: IntegrationsCommand::Show { json },
            } => assert!(json),
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn cli_parses_integration_logs() {
        let cli = Cli::try_parse_from([
            "vel",
            "integration",
            "logs",
            "todoist",
            "--limit",
            "25",
            "--json",
        ])
        .unwrap();
        match cli.command {
            Command::Integration {
                command: IntegrationCommand::Logs { id, limit, json },
            } => {
                assert_eq!(id, "todoist");
                assert_eq!(limit, 25);
                assert!(json);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn cli_parses_component_restart() {
        let cli =
            Cli::try_parse_from(["vel", "component", "restart", "evaluate", "--json"]).unwrap();
        match cli.command {
            Command::Component {
                command: ComponentCommand::Restart { id, json },
            } => {
                assert_eq!(id, "evaluate");
                assert!(json);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn cli_parses_llm_profile_health() {
        let cli =
            Cli::try_parse_from(["vel", "llm", "profile-health", "default", "--json"]).unwrap();
        match cli.command {
            Command::Llm {
                command: LlmCommand::ProfileHealth { profile_id, json },
            } => {
                assert_eq!(profile_id, "default");
                assert!(json);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn cli_parses_node_link_issue() {
        let cli = Cli::try_parse_from([
            "vel",
            "node",
            "link",
            "issue",
            "--issued-by-node-id",
            "node_alpha",
            "--expires-seconds",
            "900",
            "--read-context",
            "--write-safe-actions",
            "--json",
        ])
        .unwrap();
        match cli.command {
            Command::Node {
                command:
                    NodeCommand::Link {
                        command:
                            NodeLinkCommand::Issue {
                                issued_by_node_id,
                                expires_seconds,
                                read_context,
                                write_safe_actions,
                                execute_repo_tasks,
                                json,
                            },
                    },
            } => {
                assert_eq!(issued_by_node_id.as_deref(), Some("node_alpha"));
                assert_eq!(expires_seconds, Some(900));
                assert!(read_context);
                assert!(write_safe_actions);
                assert!(!execute_repo_tasks);
                assert!(json);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn cli_parses_node_link_redeem() {
        let cli = Cli::try_parse_from([
            "vel",
            "node",
            "link",
            "redeem",
            "paircode_123",
            "--node-id",
            "node_beta",
            "--node-display-name",
            "Beta",
            "--transport-hint",
            "tailscale",
        ])
        .unwrap();
        match cli.command {
            Command::Node {
                command:
                    NodeCommand::Link {
                        command:
                            NodeLinkCommand::Redeem {
                                token_code,
                                node_id,
                                node_display_name,
                                transport_hint,
                                json,
                            },
                    },
            } => {
                assert_eq!(token_code, "paircode_123");
                assert_eq!(node_id, "node_beta");
                assert_eq!(node_display_name, "Beta");
                assert_eq!(transport_hint.as_deref(), Some("tailscale"));
                assert!(!json);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn cli_parses_node_status() {
        let cli = Cli::try_parse_from(["vel", "node", "status", "--json"]).unwrap();
        match cli.command {
            Command::Node {
                command: NodeCommand::Status { json },
            } => assert!(json),
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
    fn cli_parses_policy_check_command() {
        let cli = Cli::try_parse_from([
            "vel",
            "policy",
            "check",
            "--path",
            "config/policies.yaml",
            "--json",
        ])
        .unwrap();
        match cli.command {
            Command::Policy {
                command: PolicyCommand::Check { path, json },
            } => {
                assert_eq!(path.as_deref(), Some("config/policies.yaml"));
                assert!(json);
            }
            _ => panic!("expected policy check command"),
        }
    }

    #[test]
    fn cli_parses_artifact_run_command() {
        let cli = Cli::try_parse_from(["vel", "artifact", "run", "run_777", "--json"]).unwrap();
        match cli.command {
            Command::Artifact {
                command: ArtifactCommand::Run { id, json },
            } => {
                assert_eq!(id, "run_777");
                assert!(json);
            }
            _ => panic!("expected artifact run command"),
        }
    }

    #[test]
    fn cli_parses_run_intent_and_dry_run_commands() {
        let intent = Cli::try_parse_from([
            "vel", "run", "intent", "--json", "should", "capture", "remember", "this",
        ])
        .expect("intent run command should parse");
        match intent.command {
            Command::Run {
                command: RunCommand::Intent { input, json },
            } => {
                assert!(json);
                assert_eq!(input, vec!["should", "capture", "remember", "this"]);
            }
            _ => panic!("expected run intent command"),
        }

        let dry = Cli::try_parse_from(["vel", "run", "dry-run", "should", "review", "today"])
            .expect("dry-run command should parse");
        match dry.command {
            Command::Run {
                command: RunCommand::DryRun { input, json },
            } => {
                assert!(!json);
                assert_eq!(input, vec!["should", "review", "today"]);
            }
            _ => panic!("expected run dry-run command"),
        }
    }

    #[test]
    fn cli_parses_daily_loop_overdue_commands() {
        let menu =
            Cli::try_parse_from(["vel", "daily-loop", "overdue", "menu", "--limit", "25"]).unwrap();
        match menu.command {
            Command::DailyLoop {
                command:
                    DailyLoopCommand::Overdue {
                        command: DailyLoopOverdueCommand::Menu { limit, json },
                    },
            } => {
                assert_eq!(limit, 25);
                assert!(!json);
            }
            _ => panic!("expected daily-loop overdue menu command"),
        }

        let confirm = Cli::try_parse_from([
            "vel",
            "daily-loop",
            "overdue",
            "confirm",
            "com_123",
            "--action",
            "close",
            "--json",
        ])
        .unwrap();
        match confirm.command {
            Command::DailyLoop {
                command:
                    DailyLoopCommand::Overdue {
                        command:
                            DailyLoopOverdueCommand::Confirm {
                                commitment_id,
                                action,
                                json,
                                ..
                            },
                    },
            } => {
                assert_eq!(commitment_id, "com_123");
                assert_eq!(action, "close");
                assert!(json);
            }
            _ => panic!("expected daily-loop overdue confirm command"),
        }

        let apply = Cli::try_parse_from([
            "vel",
            "daily-loop",
            "overdue",
            "apply",
            "ovdp_123",
            "--confirmation-token",
            "confirm:ovdp_123",
            "--idempotency-key",
            "ovd:test:apply",
        ])
        .unwrap();
        match apply.command {
            Command::DailyLoop {
                command:
                    DailyLoopCommand::Overdue {
                        command:
                            DailyLoopOverdueCommand::Apply {
                                proposal_id,
                                confirmation_token,
                                idempotency_key,
                                json,
                            },
                    },
            } => {
                assert_eq!(proposal_id, "ovdp_123");
                assert_eq!(confirmation_token, "confirm:ovdp_123");
                assert_eq!(idempotency_key.as_deref(), Some("ovd:test:apply"));
                assert!(!json);
            }
            _ => panic!("expected daily-loop overdue apply command"),
        }

        let undo = Cli::try_parse_from([
            "vel",
            "daily-loop",
            "overdue",
            "undo",
            "ovda_123",
            "--idempotency-key",
            "ovd:test:undo",
            "--json",
        ])
        .unwrap();
        match undo.command {
            Command::DailyLoop {
                command:
                    DailyLoopCommand::Overdue {
                        command:
                            DailyLoopOverdueCommand::Undo {
                                action_event_id,
                                idempotency_key,
                                json,
                            },
                    },
            } => {
                assert_eq!(action_event_id, "ovda_123");
                assert_eq!(idempotency_key.as_deref(), Some("ovd:test:undo"));
                assert!(json);
            }
            _ => panic!("expected daily-loop overdue undo command"),
        }
    }

    #[test]
    fn cli_parses_daily_loop_check_in_skip_command() {
        let parsed = Cli::try_parse_from([
            "vel",
            "daily-loop",
            "skip-check-in",
            "dci_123",
            "--reason-code",
            "not_applicable",
            "--reason-text",
            "in a meeting",
            "--json",
        ])
        .unwrap();
        match parsed.command {
            Command::DailyLoop {
                command:
                    DailyLoopCommand::SkipCheckIn {
                        check_in_event_id,
                        reason_code,
                        reason_text,
                        source,
                        json,
                    },
            } => {
                assert_eq!(check_in_event_id, "dci_123");
                assert_eq!(reason_code.as_deref(), Some("not_applicable"));
                assert_eq!(reason_text.as_deref(), Some("in a meeting"));
                assert!(source.is_none());
                assert!(json);
            }
            _ => panic!("expected daily-loop skip-check-in command"),
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
