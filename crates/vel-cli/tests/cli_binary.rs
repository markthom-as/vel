use assert_cmd::Command;
use serde::Serialize;
use std::fs;
use tempfile::tempdir;
use time::OffsetDateTime;
use vel_api_types::{
    ArtifactData, BackupCoverageData, BackupFreshnessData, BackupFreshnessStateData,
    BackupStatusData, BackupStatusStateData, BackupTrustData, BackupTrustLevelData,
    CaptureCreateResponse, CommitmentData, ComponentData, ComponentLogEventData, ContextCapture,
    DailyLoopCommitmentActionData, DailyLoopPhaseData, DailyLoopSessionData,
    DailyLoopSessionStateData, DailyLoopStartMetadataData, DailyLoopStartSourceData,
    DailyLoopStatusData, DailyLoopSurfaceData, DailyLoopTurnStateData, DiagnosticCheck,
    DiagnosticStatus, DoctorData, EndOfDayData, GoogleCalendarIntegrationData,
    IntegrationCalendarData, IntegrationLogEventData, IntegrationsData, LocalIntegrationData,
    MorningOverviewStateData, NudgeData, PersonRecordData, ProjectFamilyData,
    ProjectListResponseData, ProjectProvisionRequestData, ProjectRecordData, ProjectRootRefData,
    ProjectStatusData, RiskData, RiskFactorsData, RunDetailData, RunSummaryData, SearchResults,
    SignalData, SuggestionData, SyncResultData, TodoistIntegrationData,
    TodoistWriteCapabilitiesData, UncertaintyData,
};
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn ok_response<T: Serialize>(data: T) -> serde_json::Value {
    serde_json::json!({
        "ok": true,
        "data": data,
        "warnings": [],
        "meta": {
            "request_id": "req_test",
            "degraded": false
        }
    })
}

fn error_response(message: &str) -> serde_json::Value {
    serde_json::json!({
        "ok": false,
        "error": {
            "code": "bad_request",
            "message": message
        },
        "warnings": [],
        "meta": {
            "request_id": "req_test",
            "degraded": false
        }
    })
}

fn vel_command() -> Command {
    Command::cargo_bin("vel").expect("vel binary should build")
}

fn isolated_command() -> (tempfile::TempDir, Command) {
    let dir = tempdir().expect("tempdir");
    let mut cmd = vel_command();
    cmd.current_dir(dir.path());
    cmd.env_remove("VEL_BASE_URL");
    (dir, cmd)
}

fn sample_run_detail() -> RunDetailData {
    RunDetailData {
        id: vel_core::RunId::from("run_123".to_string()),
        kind: "sync".to_string(),
        status: "blocked".to_string(),
        trace_id: "trace_123".to_string(),
        parent_run_id: None,
        automatic_retry_supported: true,
        automatic_retry_reason: Some("retryable".to_string()),
        unsupported_retry_override: false,
        unsupported_retry_override_reason: None,
        input: serde_json::json!({ "source": "calendar" }),
        output: None,
        error: None,
        created_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
        started_at: Some(OffsetDateTime::from_unix_timestamp(1_742_927_260).unwrap()),
        finished_at: None,
        duration_ms: None,
        retry_scheduled_at: Some(OffsetDateTime::from_unix_timestamp(1_742_930_800).unwrap()),
        retry_reason: Some("waiting on dependency".to_string()),
        blocked_reason: Some("calendar token expired".to_string()),
        events: vec![],
        artifacts: vec![],
    }
}

fn sample_run_summary() -> RunSummaryData {
    RunSummaryData {
        id: vel_core::RunId::from("run_sum_1".to_string()),
        kind: "sync".to_string(),
        status: "completed".to_string(),
        trace_id: "trace_sum_1".to_string(),
        parent_run_id: None,
        automatic_retry_supported: true,
        automatic_retry_reason: None,
        unsupported_retry_override: false,
        unsupported_retry_override_reason: None,
        created_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
        started_at: Some(OffsetDateTime::from_unix_timestamp(1_742_927_220).unwrap()),
        finished_at: Some(OffsetDateTime::from_unix_timestamp(1_742_927_260).unwrap()),
        duration_ms: Some(40_000),
        retry_scheduled_at: None,
        retry_reason: None,
        blocked_reason: None,
    }
}

fn sample_commitment() -> CommitmentData {
    CommitmentData {
        id: vel_core::CommitmentId::from("com_bin_1".to_string()),
        text: "Binary harness commitment".to_string(),
        source_type: "manual".to_string(),
        source_id: None,
        status: "open".to_string(),
        due_at: None,
        project: Some("vel".to_string()),
        commitment_kind: Some("todo".to_string()),
        created_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
        resolved_at: None,
        scheduler_rules: vel_api_types::CanonicalScheduleRulesData::default(),
        metadata: serde_json::json!({}),
    }
}

fn sample_nudge() -> NudgeData {
    NudgeData {
        nudge_id: "nud_1".to_string(),
        nudge_type: "follow_up".to_string(),
        level: "high".to_string(),
        state: "active".to_string(),
        related_commitment_id: Some("com_bin_1".to_string()),
        message: "Review the harness result".to_string(),
        created_at: 1_742_927_200,
        snoozed_until: None,
        resolved_at: None,
    }
}

fn sample_suggestion() -> SuggestionData {
    SuggestionData {
        id: "sg_bin_1".to_string(),
        suggestion_type: "triage".to_string(),
        state: "rejected".to_string(),
        title: Some("Review queue".to_string()),
        summary: Some("Defer lower-priority work".to_string()),
        priority: 2,
        confidence: Some("medium".to_string()),
        evidence_count: 1,
        decision_context_summary: Some("Queue is overloaded".to_string()),
        decision_context: None,
        evidence: None,
        latest_feedback_outcome: Some("dismissed".to_string()),
        latest_feedback_notes: Some("not needed".to_string()),
        adaptive_policy: None,
        payload: serde_json::json!({ "reason": "not needed" }),
        created_at: 1_742_927_200,
        resolved_at: Some(1_742_927_260),
    }
}

fn sample_doctor() -> DoctorData {
    DoctorData {
        checks: vec![DiagnosticCheck {
            name: "database".to_string(),
            status: DiagnosticStatus::Ok,
            message: "sqlite is reachable".to_string(),
        }],
        backup: BackupTrustData {
            level: BackupTrustLevelData::Warn,
            status: BackupStatusData {
                state: BackupStatusStateData::Ready,
                last_backup_id: Some("backup_1".to_string()),
                last_backup_at: Some(OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap()),
                output_root: Some("/tmp/vel-backups".to_string()),
                artifact_coverage: Some(BackupCoverageData {
                    included: vec!["var/artifacts".to_string()],
                    omitted: vec!["var/cache".to_string()],
                    notes: vec![],
                }),
                config_coverage: Some(BackupCoverageData {
                    included: vec!["vel.toml".to_string()],
                    omitted: vec![],
                    notes: vec![],
                }),
                verification_summary: None,
                warnings: vec!["backup is older than target".to_string()],
            },
            freshness: BackupFreshnessData {
                state: BackupFreshnessStateData::Stale,
                age_seconds: Some(7_200),
                stale_after_seconds: 3_600,
            },
            guidance: vec!["create a fresh backup".to_string()],
        },
        schema_version: 5,
        version: "0.1.0-test".to_string(),
    }
}

fn sample_uncertainty() -> UncertaintyData {
    UncertaintyData {
        id: "unc_1".to_string(),
        subject_type: "commitment".to_string(),
        subject_id: Some("com_bin_1".to_string()),
        decision_kind: "schedule".to_string(),
        confidence_band: "medium".to_string(),
        confidence_score: Some(0.62),
        reasons: serde_json::json!(["missing due date"]),
        missing_evidence: Some(serde_json::json!(["calendar context"])),
        resolution_mode: "operator_review".to_string(),
        status: "open".to_string(),
        created_at: 1_742_927_200,
        resolved_at: None,
    }
}

fn sample_morning_session() -> DailyLoopSessionData {
    DailyLoopSessionData {
        id: "dls_morning_1".to_string(),
        session_date: "2026-03-25".to_string(),
        phase: DailyLoopPhaseData::MorningOverview,
        status: DailyLoopStatusData::WaitingForInput,
        start: DailyLoopStartMetadataData {
            source: DailyLoopStartSourceData::Manual,
            surface: DailyLoopSurfaceData::Cli,
        },
        turn_state: DailyLoopTurnStateData::WaitingForInput,
        current_prompt: None,
        continuity_summary: "Morning overview continuity is available.".to_string(),
        allowed_actions: vec![
            DailyLoopCommitmentActionData::Accept,
            DailyLoopCommitmentActionData::Choose,
            DailyLoopCommitmentActionData::Close,
        ],
        state: DailyLoopSessionStateData::MorningOverview(MorningOverviewStateData {
            snapshot: "Today starts with one clear priority.".to_string(),
            friction_callouts: vec![],
            signals: vec![],
            check_in_history: vec![],
        }),
        outcome: None,
    }
}

fn sample_person() -> PersonRecordData {
    PersonRecordData {
        id: vel_core::PersonId::from("person_1".to_string()),
        display_name: "Alex Example".to_string(),
        given_name: Some("Alex".to_string()),
        family_name: Some("Example".to_string()),
        relationship_context: Some("teammate".to_string()),
        birthday: None,
        last_contacted_at: Some(OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap()),
        aliases: vec![],
        links: vec![],
    }
}

fn sample_project() -> ProjectRecordData {
    ProjectRecordData {
        id: vel_core::ProjectId::from("proj_1".to_string()),
        slug: "vel".to_string(),
        name: "Vel".to_string(),
        family: ProjectFamilyData::Creative,
        status: ProjectStatusData::Active,
        primary_repo: ProjectRootRefData {
            path: "/code/vel".to_string(),
            label: "repo".to_string(),
            kind: "repo".to_string(),
        },
        primary_notes_root: ProjectRootRefData {
            path: "/notes/vel".to_string(),
            label: "notes".to_string(),
            kind: "notes_root".to_string(),
        },
        secondary_repos: vec![],
        secondary_notes_roots: vec![],
        upstream_ids: [("github".to_string(), "jove/vel".to_string())]
            .into_iter()
            .collect(),
        pending_provision: ProjectProvisionRequestData {
            create_repo: false,
            create_notes_root: true,
        },
        created_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
        updated_at: OffsetDateTime::from_unix_timestamp(1_742_927_260).unwrap(),
        archived_at: None,
    }
}

fn sample_local_integration(source_kind: &str) -> LocalIntegrationData {
    LocalIntegrationData {
        configured: true,
        guidance: None,
        source_path: Some(format!("/tmp/{source_kind}.json")),
        selected_paths: Vec::new(),
        available_paths: Vec::new(),
        internal_paths: Vec::new(),
        suggested_paths: Vec::new(),
        source_kind: source_kind.to_string(),
        last_sync_at: Some(1_742_927_200),
        last_sync_status: Some("ok".to_string()),
        last_error: None,
        last_item_count: Some(3),
    }
}

fn sample_integrations() -> IntegrationsData {
    IntegrationsData {
        google_calendar: GoogleCalendarIntegrationData {
            configured: true,
            connected: true,
            has_client_id: true,
            has_client_secret: true,
            calendars: vec![IntegrationCalendarData {
                id: "cal_1".to_string(),
                summary: "Primary".to_string(),
                primary: true,
                sync_enabled: true,
                display_enabled: true,
            }],
            all_calendars_selected: false,
            last_sync_at: Some(1_742_927_200),
            last_sync_status: Some("ok".to_string()),
            last_error: None,
            last_item_count: Some(7),
            guidance: None,
        },
        todoist: TodoistIntegrationData {
            configured: true,
            connected: false,
            has_api_token: true,
            last_sync_at: Some(1_742_927_200),
            last_sync_status: Some("warn".to_string()),
            last_error: Some("reauth needed".to_string()),
            last_item_count: Some(4),
            guidance: None,
            write_capabilities: TodoistWriteCapabilitiesData {
                completion_status: true,
                due_date: true,
                tags: false,
            },
        },
        activity: sample_local_integration("activity"),
        health: sample_local_integration("health"),
        git: sample_local_integration("git"),
        messaging: sample_local_integration("messaging"),
        reminders: sample_local_integration("reminders"),
        notes: sample_local_integration("notes"),
        transcripts: sample_local_integration("transcripts"),
    }
}

fn sample_component() -> ComponentData {
    ComponentData {
        id: "evaluate".to_string(),
        name: "Evaluate".to_string(),
        description: "Inference evaluator".to_string(),
        status: "healthy".to_string(),
        last_restarted_at: Some(1_742_927_200),
        last_error: None,
        restart_count: 2,
    }
}

fn sample_component_log() -> ComponentLogEventData {
    ComponentLogEventData {
        id: "clog_1".to_string(),
        component_id: "evaluate".to_string(),
        event_name: "restart".to_string(),
        status: "ok".to_string(),
        message: "component restarted".to_string(),
        payload: serde_json::json!({}),
        created_at: 1_742_927_260,
    }
}

fn sample_integration_log() -> IntegrationLogEventData {
    IntegrationLogEventData {
        id: "ilog_1".to_string(),
        integration_id: "todoist".to_string(),
        event_name: "sync".to_string(),
        status: "error".to_string(),
        message: "token expired".to_string(),
        payload: serde_json::json!({}),
        created_at: 1_742_927_260,
    }
}

fn sample_signal() -> SignalData {
    SignalData {
        signal_id: "sig_1".to_string(),
        signal_type: "activity".to_string(),
        source: "macos".to_string(),
        source_ref: Some("terminal".to_string()),
        timestamp: 1_742_927_200,
        payload: serde_json::json!({ "app": "Terminal" }),
        created_at: 1_742_927_260,
    }
}

fn sample_risk() -> RiskData {
    RiskData {
        commitment_id: "com_bin_1".to_string(),
        risk_score: 0.78,
        risk_level: "high".to_string(),
        factors: RiskFactorsData {
            consequence: 0.8,
            proximity: 0.7,
            dependency_pressure: 0.6,
            external_anchor: 0.4,
            stale_open_age: 0.5,
            reasons: vec!["deadline near".to_string()],
            dependency_ids: vec!["com_dep_1".to_string()],
        },
        computed_at: Some(1_742_927_200),
    }
}

fn sample_artifact() -> ArtifactData {
    ArtifactData {
        artifact_id: vel_core::ArtifactId::from("art_bin_1".to_string()),
        artifact_type: "report".to_string(),
        title: Some("Binary harness artifact".to_string()),
        mime_type: Some("application/json".to_string()),
        storage_uri: "file:///tmp/artifact.json".to_string(),
        storage_kind: "file".to_string(),
        privacy_class: "private".to_string(),
        sync_class: "local".to_string(),
        content_hash: Some("sha256:test".to_string()),
        size_bytes: Some(1536),
        created_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
        updated_at: OffsetDateTime::from_unix_timestamp(1_742_927_260).unwrap(),
    }
}

#[tokio::test]
async fn health_command_uses_base_url_flag_and_renders_human_output() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/health"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!({
                "status": "ok",
                "db": "ready",
                "version": "0.1.0-test"
            }))),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("health")
        .output()
        .expect("health command should run");

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("veld: ok"), "stdout was: {stdout}");
    assert!(stdout.contains("db: ready"), "stdout was: {stdout}");
    assert!(
        stdout.contains("version: 0.1.0-test"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn health_command_can_use_env_base_url_for_json_output() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/health"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!({
                "status": "ok",
                "db": "ready",
                "version": "0.1.0-test"
            }))),
        )
        .mount(&server)
        .await;

    let dir = tempdir().expect("tempdir");
    let mut cmd = vel_command();
    let output = cmd
        .current_dir(dir.path())
        .env("VEL_BASE_URL", server.uri())
        .arg("health")
        .arg("--json")
        .output()
        .expect("health json command should run");

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid health json");
    assert_eq!(parsed["ok"], true);
    assert_eq!(parsed["data"]["status"], "ok");
    assert_eq!(parsed["data"]["db"], "ready");
}

#[tokio::test]
async fn recent_command_json_hits_expected_endpoint_and_prints_array() {
    let server = MockServer::start().await;
    let capture = ContextCapture {
        capture_id: vel_core::CaptureId::from("cap_123".to_string()),
        capture_type: "quick_note".to_string(),
        content_text: "remember the binary harness".to_string(),
        occurred_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
        source_device: Some("laptop".to_string()),
    };
    Mock::given(method("GET"))
        .and(path("/v1/captures"))
        .and(query_param("limit", "3"))
        .and(query_param("today", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(vec![capture])))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("recent")
        .arg("--limit")
        .arg("3")
        .arg("--today")
        .arg("--json")
        .output()
        .expect("recent command should run");

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("recent output should be valid json");
    assert!(parsed.is_array(), "stdout was: {stdout}");
    assert_eq!(parsed[0]["capture_id"], "cap_123");
    assert_eq!(parsed[0]["content_text"], "remember the binary harness");
}

#[tokio::test]
async fn capture_command_posts_body_and_prints_capture_id() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/captures"))
        .and(body_json(serde_json::json!({
            "content_text": "capture from integration test",
            "capture_type": "todo",
            "source_device": "terminal"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(
            CaptureCreateResponse {
                capture_id: vel_core::CaptureId::from("cap_456".to_string()),
                accepted_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
            },
        )))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("capture")
        .arg("capture from integration test")
        .arg("--type")
        .arg("todo")
        .arg("--source")
        .arg("terminal")
        .output()
        .expect("capture command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("capture_id: cap_456"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn capture_command_returns_nonzero_and_surfaces_api_error_message() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/captures"))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_json(error_response("capture rejected for testing")),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("capture")
        .arg("this should fail")
        .output()
        .expect("capture command should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("capture rejected for testing"),
        "stderr was: {stderr}"
    );
}

#[test]
fn config_show_reads_local_vel_toml_without_server_dependency() {
    let (dir, mut cmd) = isolated_command();
    fs::write(
        dir.path().join("vel.toml"),
        r#"
base_url = "http://127.0.0.1:9999"
node_id = "node_cli"
node_display_name = "CLI Node"
tailscale_preferred = true
"#,
    )
    .expect("write vel.toml");

    let output = cmd
        .arg("config")
        .arg("show")
        .output()
        .expect("config show should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("base_url: http://127.0.0.1:9999"),
        "stdout was: {stdout}"
    );
    assert!(stdout.contains("node_id: node_cli"), "stdout was: {stdout}");
    assert!(
        stdout.contains("node_display_name: CLI Node"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn search_command_json_uses_query_parameters_and_prints_response_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/search"))
        .and(query_param("q", "lidar"))
        .and(query_param("capture_type", "quick_note"))
        .and(query_param("source_device", "laptop"))
        .and(query_param("limit", "2"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(SearchResults {
                results: vec![vel_api_types::SearchResult {
                    capture_id: vel_core::CaptureId::from("cap_search_1".to_string()),
                    capture_type: "quick_note".to_string(),
                    snippet: "remember lidar budget".to_string(),
                    occurred_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
                    created_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
                    source_device: Some("laptop".to_string()),
                }],
            })),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("search")
        .arg("lidar")
        .arg("--capture-type")
        .arg("quick_note")
        .arg("--source-device")
        .arg("laptop")
        .arg("--limit")
        .arg("2")
        .arg("--json")
        .output()
        .expect("search command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid search json");
    assert_eq!(parsed["ok"], true);
    assert_eq!(parsed["data"]["results"][0]["capture_id"], "cap_search_1");
}

#[tokio::test]
async fn recent_command_human_output_prints_table_and_capture_content() {
    let server = MockServer::start().await;
    let capture = ContextCapture {
        capture_id: vel_core::CaptureId::from("cap_human_1".to_string()),
        capture_type: "quick_note".to_string(),
        content_text: "human recent output still matters".to_string(),
        occurred_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
        source_device: Some("laptop".to_string()),
    };
    Mock::given(method("GET"))
        .and(path("/v1/captures"))
        .and(query_param("limit", "1"))
        .and(query_param("today", "false"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(vec![capture])))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("recent")
        .arg("--limit")
        .arg("1")
        .output()
        .expect("recent command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CAPTURE ID"), "stdout was: {stdout}");
    assert!(stdout.contains("cap_human_1"), "stdout was: {stdout}");
    assert!(
        stdout.contains("human recent output still matters"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn run_status_command_patches_run_and_prints_followup_details() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/v1/runs/run_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(sample_run_detail())))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("run")
        .arg("status")
        .arg("run_123")
        .arg("blocked")
        .arg("--retry-after-seconds")
        .arg("900")
        .arg("--retry-at")
        .arg("2025-03-26T01:00:00Z")
        .arg("--reason")
        .arg("waiting on dependency")
        .arg("--blocked-reason")
        .arg("calendar token expired")
        .output()
        .expect("run status command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Run run_123 status -> blocked"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("Retry scheduled at"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn commitments_command_lists_human_output() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/commitments"))
        .and(query_param("limit", "5"))
        .and(query_param("status", "open"))
        .and(query_param("project", "vel"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(vec![sample_commitment()])),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("commitments")
        .arg("--limit")
        .arg("5")
        .arg("--project")
        .arg("vel")
        .output()
        .expect("commitments command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("com_bin_1"), "stdout was: {stdout}");
    assert!(
        stdout.contains("Binary harness commitment"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn nudges_command_lists_human_output() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/nudges"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(vec![sample_nudge()])))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("nudges")
        .output()
        .expect("nudges command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nud_1"), "stdout was: {stdout}");
    assert!(
        stdout.contains("Review the harness result"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn runs_list_json_prints_response_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/runs"))
        .and(query_param("limit", "2"))
        .and(query_param("today", "true"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(vec![sample_run_summary()])),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("run")
        .arg("list")
        .arg("--today")
        .arg("--limit")
        .arg("2")
        .arg("--json")
        .output()
        .expect("run list command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid runs json");
    assert_eq!(parsed["ok"], true);
    assert_eq!(parsed["data"][0]["id"], "run_sum_1");
}

#[tokio::test]
async fn doctor_command_prints_backup_guidance_and_succeeds_without_fail_checks() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/doctor"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(sample_doctor())))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("doctor")
        .output()
        .expect("doctor command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("advanced trust and runtime checks:"),
        "stdout was: {stdout}"
    );
    assert!(stdout.contains("database: ok"), "stdout was: {stdout}");
    assert!(
        stdout.contains("backup_guidance: create a fresh backup"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn suggestion_reject_command_posts_and_prints_reason() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/suggestions/sg_bin_1/reject"))
        .and(body_json(serde_json::json!({ "reason": "not needed" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(sample_suggestion())))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("suggestion")
        .arg("reject")
        .arg("sg_bin_1")
        .arg("--reason")
        .arg("not needed")
        .output()
        .expect("suggestion reject command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Rejected suggestion sg_bin_1"),
        "stdout was: {stdout}"
    );
    assert!(stdout.contains("reason=not needed"), "stdout was: {stdout}");
}

#[tokio::test]
async fn journal_mood_command_posts_and_prints_capture_id() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/journal/mood"))
        .and(body_json(serde_json::json!({
            "score": 7,
            "label": "steady",
            "note": "good enough",
            "source_device": "terminal"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(
            CaptureCreateResponse {
                capture_id: vel_core::CaptureId::from("cap_mood_1".to_string()),
                accepted_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
            },
        )))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("journal")
        .arg("mood")
        .arg("7")
        .arg("--label")
        .arg("steady")
        .arg("--note")
        .arg("good enough")
        .arg("--source")
        .arg("terminal")
        .output()
        .expect("journal mood command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("capture_id: cap_mood_1"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn loops_command_lists_human_output() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/loops"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(vec![
            vel_api_types::LoopData {
                kind: "morning".to_string(),
                enabled: true,
                interval_seconds: 3600,
                last_started_at: None,
                last_finished_at: None,
                last_status: Some("ok".to_string()),
                last_error: None,
                next_due_at: Some(1_742_930_800),
            },
        ])))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("loops")
        .output()
        .expect("loops command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("LOOP"), "stdout was: {stdout}");
    assert!(stdout.contains("morning"), "stdout was: {stdout}");
    assert!(stdout.contains("3600s"), "stdout was: {stdout}");
}

#[tokio::test]
async fn uncertainty_list_command_prints_human_output() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/uncertainty"))
        .and(query_param("status", "open"))
        .and(query_param("limit", "50"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(vec![sample_uncertainty()])),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("uncertainty")
        .arg("list")
        .arg("--status")
        .arg("open")
        .output()
        .expect("uncertainty list command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("unc_1"), "stdout was: {stdout}");
    assert!(stdout.contains("operator_review"), "stdout was: {stdout}");
}

#[tokio::test]
async fn morning_command_starts_session_when_no_active_session_exists() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/daily-loop/sessions/active"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(serde_json::Value::Null)),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/daily-loop/sessions"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(sample_morning_session())),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("morning")
        .output()
        .expect("morning command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("session: dls_morning_1"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("Today starts with one clear priority."),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn inspect_capture_command_prints_capture_fields() {
    let server = MockServer::start().await;
    let capture = ContextCapture {
        capture_id: vel_core::CaptureId::from("cap_inspect_1".to_string()),
        capture_type: "quick_note".to_string(),
        content_text: "inspect path coverage".to_string(),
        occurred_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
        source_device: Some("laptop".to_string()),
    };
    Mock::given(method("GET"))
        .and(path("/v1/captures/cap_inspect_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(capture)))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("inspect")
        .arg("capture")
        .arg("cap_inspect_1")
        .output()
        .expect("inspect capture should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("capture_id: cap_inspect_1"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("content_text: inspect path coverage"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn signals_list_command_prints_signal_summary() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/signals"))
        .and(query_param("limit", "5"))
        .and(query_param("signal_type", "activity"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(vec![sample_signal()])))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("signals")
        .arg("list")
        .arg("--signal-type")
        .arg("activity")
        .arg("--limit")
        .arg("5")
        .output()
        .expect("signals list should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("sig_1"), "stdout was: {stdout}");
    assert!(stdout.contains("activity"), "stdout was: {stdout}");
    assert!(stdout.contains("src=macos"), "stdout was: {stdout}");
}

#[tokio::test]
async fn people_list_command_prints_person_summary() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/people"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(vec![sample_person()])))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("people")
        .arg("list")
        .output()
        .expect("people list should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("person_1"), "stdout was: {stdout}");
    assert!(stdout.contains("Alex"), "stdout was: {stdout}");
    assert!(stdout.contains("last_contact="), "stdout was: {stdout}");
}

#[tokio::test]
async fn project_list_command_json_prints_project_records() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(
            ProjectListResponseData {
                projects: vec![sample_project()],
            },
        )))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("project")
        .arg("list")
        .arg("--json")
        .output()
        .expect("project list should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("project list output should be valid json");
    assert_eq!(parsed[0]["id"], "proj_1");
    assert_eq!(parsed[0]["slug"], "vel");
    assert_eq!(parsed[0]["name"], "Vel");
}

#[tokio::test]
async fn project_inspect_command_prints_project_details() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/proj_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(sample_project())))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("project")
        .arg("inspect")
        .arg("proj_1")
        .output()
        .expect("project inspect should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("slug:                vel"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("family:              creative"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("primary_repo:        /code/vel"),
        "stdout was: {stdout}"
    );
    assert!(stdout.contains("github=jove/vel"), "stdout was: {stdout}");
}

#[tokio::test]
async fn project_families_command_json_prints_family_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/families"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(vec![
            ProjectFamilyData::Personal,
            ProjectFamilyData::Creative,
            ProjectFamilyData::Work,
        ])))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("project")
        .arg("families")
        .arg("--json")
        .output()
        .expect("project families should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("project families output should be valid json");
    assert_eq!(parsed, serde_json::json!(["personal", "creative", "work"]));
}

#[tokio::test]
async fn project_create_command_posts_payload_and_prints_summary() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects"))
        .and(body_json(serde_json::json!({
            "slug": "vel",
            "name": "Vel",
            "family": "creative",
            "status": "active",
            "primary_repo": {
                "path": "/code/vel",
                "label": "repo",
                "kind": "repo"
            },
            "primary_notes_root": {
                "path": "/notes/vel",
                "label": "notes",
                "kind": "notes_root"
            },
            "secondary_repos": [],
            "secondary_notes_roots": [],
            "upstream_ids": {},
            "pending_provision": {
                "create_repo": false,
                "create_notes_root": true
            }
        })))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!({
                "project": sample_project()
            }))),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("project")
        .arg("create")
        .arg("--slug")
        .arg("vel")
        .arg("--name")
        .arg("Vel")
        .arg("--family")
        .arg("creative")
        .arg("--status")
        .arg("active")
        .arg("--repo-path")
        .arg("/code/vel")
        .arg("--notes-path")
        .arg("/notes/vel")
        .arg("--create-notes-root")
        .output()
        .expect("project create should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("project_id:          proj_1"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("slug:                vel"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("family:              creative"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn settings_show_command_prints_operator_settings_summary() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/settings"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!({
                "timezone": "America/Denver",
                "node_display_name": "desk",
                "writeback_enabled": true,
                "tailscale_preferred": true,
                "tailscale_base_url": "https://100.64.0.2:4000",
                "tailscale_base_url_auto_discovered": false,
                "lan_base_url": "http://192.168.1.20:4000",
                "lan_base_url_auto_discovered": true,
                "llm": {
                    "models_dir": "/models",
                    "default_chat_profile_id": "default",
                    "fallback_chat_profile_id": "fallback",
                    "profiles": [
                        {
                            "id": "default",
                            "provider": "openai",
                            "base_url": "http://localhost:11434/v1",
                            "model": "gpt-5",
                            "context_window": 32768,
                            "enabled": true,
                            "editable": true,
                            "has_api_key": true
                        }
                    ]
                },
                "core_settings": {
                    "user_display_name": "Jove",
                    "client_location_label": "Office",
                    "developer_mode": true,
                    "bypass_setup_gate": false,
                    "agent_profile": {
                        "role": null,
                        "preferences": null,
                        "constraints": null,
                        "freeform": null
                    }
                },
                "web_settings": {
                    "dense_rows": true,
                    "tabular_numbers": true,
                    "reduced_motion": false,
                    "strong_focus": true,
                    "docked_action_bar": true,
                    "semantic_aliases": {}
                }
            }))),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("settings")
        .arg("show")
        .output()
        .expect("settings show should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("timezone:                    America/Denver"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("llm_profiles:                1"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("core_developer_mode:         true"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn integrations_show_command_prints_provider_summary() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/integrations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(sample_integrations())))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("integrations")
        .arg("show")
        .output()
        .expect("integrations show should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("google_calendar"), "stdout was: {stdout}");
    assert!(stdout.contains("todoist"), "stdout was: {stdout}");
    assert!(stdout.contains("messaging"), "stdout was: {stdout}");
}

#[tokio::test]
async fn integration_logs_command_prints_log_events() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/integrations/todoist/logs"))
        .and(query_param("limit", "12"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(vec![sample_integration_log()])),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("integration")
        .arg("logs")
        .arg("todoist")
        .arg("--limit")
        .arg("12")
        .output()
        .expect("integration logs should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("token expired"), "stdout was: {stdout}");
    assert!(stdout.contains("sync"), "stdout was: {stdout}");
}

#[tokio::test]
async fn components_list_command_prints_component_summary() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/components"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(vec![sample_component()])),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("components")
        .arg("list")
        .output()
        .expect("components list should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("evaluate"), "stdout was: {stdout}");
    assert!(stdout.contains("restarts=2"), "stdout was: {stdout}");
}

#[tokio::test]
async fn component_logs_command_prints_component_log_events() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/components/evaluate/logs"))
        .and(query_param("limit", "20"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(vec![sample_component_log()])),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("component")
        .arg("logs")
        .arg("evaluate")
        .arg("--limit")
        .arg("20")
        .output()
        .expect("component logs should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("component restarted"),
        "stdout was: {stdout}"
    );
    assert!(stdout.contains("restart"), "stdout was: {stdout}");
}

#[tokio::test]
async fn component_restart_command_posts_and_prints_component_summary() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/components/evaluate/restart"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(sample_component())))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("component")
        .arg("restart")
        .arg("evaluate")
        .output()
        .expect("component restart should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("evaluate"), "stdout was: {stdout}");
    assert!(stdout.contains("healthy"), "stdout was: {stdout}");
}

#[tokio::test]
async fn llm_profile_health_command_prints_health_summary() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/llm/profiles/default/health"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!({
                "profile_id": "default",
                "healthy": true,
                "message": "Provider handshake succeeded."
            }))),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("llm")
        .arg("profile-health")
        .arg("default")
        .output()
        .expect("llm profile-health should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("profile_id: default"),
        "stdout was: {stdout}"
    );
    assert!(stdout.contains("healthy:    true"), "stdout was: {stdout}");
}

#[tokio::test]
async fn risk_command_json_prints_risk_payload() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/risk"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(vec![sample_risk()])))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("risk")
        .arg("--json")
        .output()
        .expect("risk command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid risk json");
    assert_eq!(parsed[0]["commitment_id"], "com_bin_1");
    assert_eq!(parsed[0]["risk_level"], "high");
}

#[tokio::test]
async fn sync_activity_command_prints_ingest_count() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/sync/activity"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(SyncResultData {
                source: "activity".to_string(),
                signals_ingested: 4,
            })),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("sync")
        .arg("activity")
        .output()
        .expect("sync activity should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("activity: 4 signals ingested"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn artifact_latest_command_json_prints_artifact_payload() {
    let server = MockServer::start().await;
    let mut artifact = sample_artifact();
    artifact.artifact_id = vel_core::ArtifactId::from("art_latest_1".to_string());
    artifact.artifact_type = "daily_plan".to_string();
    artifact.title = Some("Daily plan".to_string());
    Mock::given(method("GET"))
        .and(path("/v1/artifacts/latest"))
        .and(query_param("type", "daily_plan"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(Some(artifact))))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("artifact")
        .arg("latest")
        .arg("--type")
        .arg("daily_plan")
        .arg("--json")
        .output()
        .expect("artifact latest command should run");

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("artifact latest output should be valid json");
    assert_eq!(parsed["artifact_id"], "art_latest_1");
    assert_eq!(parsed["artifact_type"], "daily_plan");
}

#[tokio::test]
async fn end_of_day_command_prints_summary_sections() {
    let server = MockServer::start().await;
    let capture = ContextCapture {
        capture_id: vel_core::CaptureId::from("cap_eod_1".to_string()),
        capture_type: "note".to_string(),
        content_text: "Closed the CLI coverage gap".to_string(),
        occurred_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
        source_device: Some("terminal".to_string()),
    };
    Mock::given(method("GET"))
        .and(path("/v1/context/end-of-day"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(EndOfDayData {
                date: "2026-03-25".to_string(),
                what_was_done: vec![capture],
                what_remains_open: vec!["Cover export command".to_string()],
                what_may_matter_tomorrow: vec!["Verify coverage in CI".to_string()],
            })),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("end-of-day")
        .output()
        .expect("end-of-day command should run");

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("date: 2026-03-25"), "stdout was: {stdout}");
    assert!(stdout.contains("what was done:"), "stdout was: {stdout}");
    assert!(
        stdout.contains("Closed the CLI coverage gap"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("what remains open:"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("Verify coverage in CI"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn evaluate_command_posts_and_prints_summary_counts() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/evaluate"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!({
                "inferred_states": 4,
                "nudges_created_or_updated": 2
            }))),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("evaluate")
        .output()
        .expect("evaluate command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("inferred_states: 4"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("nudges_created_or_updated: 2"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn explain_context_command_prints_reason_summary() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/explain/context"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!({
                "computed_at": 1742927200,
                "mode": "morning",
                "morning_state": "focused",
                "context": {},
                "source_summaries": {
                    "signals": [],
                    "captures": [],
                    "commitments": [],
                    "calendar_blocks": [],
                    "todo_items": [],
                    "messages": []
                },
                "adaptive_policy_overrides": [],
                "signals_used": ["sig_1"],
                "signal_summaries": [],
                "commitments_used": ["com_bin_1"],
                "risk_used": ["com_bin_1"],
                "reasons": ["Recent capture mentioned CLI coverage work"]
            }))),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("explain")
        .arg("context")
        .output()
        .expect("explain context command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Mode: morning"), "stdout was: {stdout}");
    assert!(
        stdout.contains("Signals used: sig_1"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("Recent capture mentioned CLI coverage work"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn explain_command_json_includes_local_and_daemon_plan() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/command/plan"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!({
                "operation": "capture",
                "target_kinds": ["capture"],
                "mode": "ready",
                "summary": "Capture the note directly.",
                "steps": [
                    {
                        "title": "Create capture",
                        "detail": "Persist the note as a capture record."
                    }
                ],
                "intent_hints": {
                    "target_kind": "capture",
                    "mode": "direct",
                    "suggestions": ["add tags"]
                },
                "delegation_hints": null,
                "planned_records": [],
                "validation": {
                    "is_valid": true,
                    "issues": []
                }
            }))),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("explain")
        .arg("command")
        .arg("--json")
        .arg("should")
        .arg("capture")
        .arg("remember")
        .arg("coverage")
        .output()
        .expect("explain command should run");

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("explain command output should be valid json");
    assert_eq!(
        parsed["input"],
        serde_json::json!(["should", "capture", "remember", "coverage"])
    );
    assert_eq!(
        parsed["daemon_plan"]["summary"],
        "Capture the note directly."
    );
    assert!(parsed["local_explanation"].is_string());
}

#[tokio::test]
async fn export_command_json_combines_requested_sections() {
    let server = MockServer::start().await;
    let capture = ContextCapture {
        capture_id: vel_core::CaptureId::from("cap_export_1".to_string()),
        capture_type: "note".to_string(),
        content_text: "capture export".to_string(),
        occurred_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
        source_device: Some("terminal".to_string()),
    };
    let mut artifact = sample_artifact();
    artifact.artifact_id = vel_core::ArtifactId::from("art_export_1".to_string());
    Mock::given(method("GET"))
        .and(path("/v1/captures"))
        .and(query_param("limit", "500"))
        .and(query_param("today", "false"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(vec![capture])))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/runs"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(vec![sample_run_summary()])),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/artifacts"))
        .and(query_param("limit", "500"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(vec![artifact])))
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("export")
        .arg("--captures")
        .arg("--runs")
        .arg("--artifacts")
        .arg("--json")
        .output()
        .expect("export command should run");

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("export output should be valid json");
    assert_eq!(parsed["captures"][0]["capture_id"], "cap_export_1");
    assert_eq!(parsed["runs"][0]["id"], "run_sum_1");
    assert_eq!(parsed["artifacts"][0]["artifact_id"], "art_export_1");
}

#[tokio::test]
async fn synthesize_week_command_prints_run_and_artifact_ids() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/synthesis/week"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!({
                "run_id": "run_syn_1",
                "artifact_id": "art_syn_1"
            }))),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("synthesize")
        .arg("week")
        .output()
        .expect("synthesize week command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("run_id: run_syn_1"), "stdout was: {stdout}");
    assert!(
        stdout.contains("artifact_id: art_syn_1"),
        "stdout was: {stdout}"
    );
}

#[tokio::test]
async fn thread_list_command_json_prints_thread_records() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/threads"))
        .and(query_param("status", "open"))
        .and(query_param("limit", "2"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!([
                {
                    "id": "thr_1",
                    "thread_type": "continuity",
                    "title": "Finish coverage follow-through",
                    "status": "open",
                    "created_at": 1742927200,
                    "updated_at": 1742927260,
                    "links": [
                        {
                            "id": "thr_link_1",
                            "entity_type": "commitment",
                            "entity_id": "com_bin_1",
                            "relation_type": "tracks"
                        }
                    ]
                }
            ]))),
        )
        .mount(&server)
        .await;

    let (_dir, mut cmd) = isolated_command();
    let output = cmd
        .arg("--base-url")
        .arg(server.uri())
        .arg("thread")
        .arg("list")
        .arg("--status")
        .arg("open")
        .arg("--limit")
        .arg("2")
        .arg("--json")
        .output()
        .expect("thread list command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("thread list output should be valid json");
    assert_eq!(parsed[0]["id"], "thr_1");
    assert_eq!(parsed[0]["title"], "Finish coverage follow-through");
}
