use anyhow::{bail, Context};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vel_api_types::{
    AgentInspectData, ApiResponse, BackupExportRequestData, BackupExportResultData,
    BackupExportStatusData, BackupManifestData, BackupStatusData, BatchImportRequest,
    BatchImportResponse, BranchSyncRequestData, CaptureCreateRequest, CaptureCreateResponse,
    ClusterBootstrapData, CommandExecuteRequest, CommandExecutionPlanData,
    CommandExecutionResultData, CommandPlanRequest, CommitmentCreateRequest, CommitmentData,
    CommitmentUpdateRequest, ComponentData, ComponentLogEventData, ConnectAttachData,
    ConnectInstanceData, DailyLoopCheckInSkipRequestData, DailyLoopCheckInSkipResponseData,
    DailyLoopOverdueApplyRequestData, DailyLoopOverdueApplyResponseData,
    DailyLoopOverdueConfirmRequestData, DailyLoopOverdueConfirmResponseData,
    DailyLoopOverdueMenuRequestData, DailyLoopOverdueMenuResponseData,
    DailyLoopOverdueUndoRequestData, DailyLoopOverdueUndoResponseData, DailyLoopPhaseData,
    DailyLoopSessionData, DailyLoopStartRequestData, DailyLoopTurnActionData,
    DailyLoopTurnRequestData, DoctorData, EndOfDayData, EvaluateResultData, ExecutionHandoffData,
    HealthData, IntegrationConnectionData, IntegrationConnectionEventData, IntegrationLogEventData,
    IntegrationsData, LinkScopeData, LinkedNodeData, LlmProfileHealthData, LoopData,
    LoopUpdateRequest, MoodJournalCreateRequest, NowData, NudgeData, NudgeSnoozeRequest,
    PainJournalCreateRequest, PairingTokenData, PersonRecordData,
    PlanningProfileProposalApplyResponseData, PlanningProfileResponseData,
    ProjectCreateRequestData, ProjectCreateResponseData, ProjectFamilyData,
    ProjectListResponseData, ProjectRecordData, QueuedWorkRoutingData, RunUpdateRequest,
    SearchQuery, SearchResults, SignalCreateRequest, SignalData, SyncBootstrapData,
    SyncClusterStateData, SyncResultData, SynthesisWeekData, TodayData, UncertaintyData,
    ValidationRequestData,
};
use vel_core::ResolvedCommand;

#[derive(Debug, Serialize)]
struct IssuePairingTokenRequestData {
    issued_by_node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl_seconds: Option<i64>,
    scopes: LinkScopeData,
}

#[derive(Debug, Serialize)]
struct RedeemPairingTokenRequestData {
    token_code: String,
    node_id: String,
    node_display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    transport_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_scopes: Option<LinkScopeData>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBackupRequestData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_root: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BackupRootRequestData {
    pub backup_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCreateResultData {
    pub manifest: BackupManifestData,
    pub status: BackupStatusData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInspectResultData {
    pub manifest: BackupManifestData,
    pub status: BackupStatusData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupVerifyResultData {
    pub manifest: BackupManifestData,
    pub status: BackupStatusData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContextData {
    pub project_id: String,
    pub project_slug: String,
    pub project_name: String,
    pub objective: String,
    pub repo_brief: String,
    pub notes_brief: String,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub expected_outputs: Vec<String>,
    #[serde(default)]
    pub repo_roots: Vec<ExecutionRootData>,
    #[serde(default)]
    pub notes_roots: Vec<ExecutionRootData>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRootData {
    pub path: String,
    pub label: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContextSaveRequestData {
    pub objective: String,
    #[serde(default)]
    pub repo_brief: String,
    #[serde(default)]
    pub notes_brief: String,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub expected_outputs: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionArtifactRequestData {
    #[serde(default)]
    pub output_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionArtifactFileData {
    pub relative_path: String,
    pub contents: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionArtifactPackData {
    pub project_id: String,
    pub project_slug: String,
    pub repo_root: String,
    pub output_dir: String,
    #[serde(default)]
    pub files: Vec<ExecutionArtifactFileData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionExportResultData {
    pub pack: ExecutionArtifactPackData,
    #[serde(default)]
    pub written_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRoutingReasonData {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRoutingDecisionData {
    pub task_kind: String,
    pub agent_profile: String,
    pub token_budget: String,
    pub review_gate: String,
    #[serde(default)]
    pub read_scopes: Vec<String>,
    #[serde(default)]
    pub write_scopes: Vec<String>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub reasons: Vec<ExecutionRoutingReasonData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionHandoffRecordData {
    pub id: String,
    pub project_id: String,
    pub origin_kind: String,
    pub review_state: String,
    pub handoff: ExecutionHandoffData,
    pub routing: ExecutionRoutingDecisionData,
    #[serde(default)]
    pub manifest_id: Option<String>,
    pub requested_by: String,
    #[serde(default)]
    pub reviewed_by: Option<String>,
    #[serde(default)]
    pub decision_reason: Option<String>,
    #[serde(default)]
    pub reviewed_at: Option<String>,
    #[serde(default)]
    pub launched_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionLaunchPreviewData {
    pub handoff_id: String,
    pub review_state: String,
    pub launch_ready: bool,
    #[serde(default)]
    pub blockers: Vec<String>,
    pub handoff: ExecutionHandoffData,
    pub routing: ExecutionRoutingDecisionData,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviewExecutionHandoffRequestData {
    pub reviewed_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaunchExecutionHandoffRequestData {
    pub runtime_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default)]
    pub command: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    #[serde(default)]
    pub writable_roots: Vec<String>,
    #[serde(default)]
    pub capability_allowlist: Vec<vel_core::CapabilityDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectHeartbeatResponseData {
    pub id: String,
    pub status: String,
    pub lease_expires_at: i64,
    pub trace_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectRunEventData {
    pub id: i64,
    pub run_id: String,
    pub stream: String,
    pub chunk: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectStdinWriteAckData {
    pub run_id: String,
    pub accepted_bytes: u32,
    pub event_id: i64,
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ConnectHeartbeatRequestData {
    pub status: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ConnectTerminateRequestData {
    pub reason: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ConnectStdinRequestData {
    pub input: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectLaunchRequestData {
    pub runtime_kind: String,
    pub actor_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default)]
    pub command: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    #[serde(default)]
    pub writable_roots: Vec<String>,
    #[serde(default)]
    pub capability_allowlist: Vec<vel_core::CapabilityDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_seconds: Option<i64>,
}

#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        let http = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("reqwest client");
        Self { http, base_url }
    }

    pub async fn health(&self) -> anyhow::Result<ApiResponse<HealthData>> {
        self.get("/v1/health").await
    }

    pub async fn doctor(&self) -> anyhow::Result<ApiResponse<DoctorData>> {
        self.get("/v1/doctor").await
    }

    pub async fn planning_profile(
        &self,
    ) -> anyhow::Result<ApiResponse<PlanningProfileResponseData>> {
        self.get("/v1/planning-profile").await
    }

    pub async fn apply_planning_profile_proposal(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<PlanningProfileProposalApplyResponseData>> {
        self.post_empty(&format!("/v1/planning-profile/proposals/{}/apply", id))
            .await
    }

    pub async fn list_people(&self) -> anyhow::Result<ApiResponse<Vec<PersonRecordData>>> {
        self.get("/v1/people").await
    }

    pub async fn get_person(&self, id: &str) -> anyhow::Result<ApiResponse<PersonRecordData>> {
        self.get(&format!("/v1/people/{}", id)).await
    }

    pub async fn list_signals(
        &self,
        signal_type: Option<&str>,
        since_ts: Option<i64>,
        limit: u32,
    ) -> anyhow::Result<ApiResponse<Vec<SignalData>>> {
        let mut path = format!("/v1/signals?limit={}", limit);
        if let Some(t) = signal_type {
            path.push_str(&format!("&signal_type={}", t));
        }
        if let Some(ts) = since_ts {
            path.push_str(&format!("&since_ts={}", ts));
        }
        self.get(&path).await
    }

    pub async fn create_signal(
        &self,
        req: SignalCreateRequest,
    ) -> anyhow::Result<ApiResponse<SignalData>> {
        self.post_json("/v1/signals", &req).await
    }

    pub async fn backup_status(&self) -> anyhow::Result<ApiResponse<BackupStatusData>> {
        self.get("/v1/backup/status").await
    }

    pub async fn backup_export_status(
        &self,
    ) -> anyhow::Result<ApiResponse<BackupExportStatusData>> {
        self.get("/v1/backup/export/status").await
    }

    pub async fn create_backup(
        &self,
        output_root: Option<&str>,
    ) -> anyhow::Result<ApiResponse<BackupCreateResultData>> {
        self.post_json(
            "/v1/backup/create",
            &CreateBackupRequestData {
                output_root: output_root.map(ToString::to_string),
            },
        )
        .await
    }

    pub async fn export_backup(
        &self,
        target_root: &str,
        domains: Vec<String>,
    ) -> anyhow::Result<ApiResponse<BackupExportResultData>> {
        self.post_json(
            "/v1/backup/export",
            &BackupExportRequestData {
                target_root: Some(target_root.to_string()),
                domains,
                include_parquet: false,
            },
        )
        .await
    }

    pub async fn inspect_backup(
        &self,
        backup_root: &str,
    ) -> anyhow::Result<ApiResponse<BackupInspectResultData>> {
        self.post_json(
            "/v1/backup/inspect",
            &BackupRootRequestData {
                backup_root: backup_root.to_string(),
            },
        )
        .await
    }

    pub async fn verify_backup(
        &self,
        backup_root: &str,
    ) -> anyhow::Result<ApiResponse<BackupVerifyResultData>> {
        self.post_json(
            "/v1/backup/verify",
            &BackupRootRequestData {
                backup_root: backup_root.to_string(),
            },
        )
        .await
    }

    pub async fn get_capture(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::ContextCapture>> {
        self.get(&format!("/v1/captures/{}", id)).await
    }

    pub async fn list_captures_recent(
        &self,
        limit: u32,
        today: bool,
    ) -> anyhow::Result<ApiResponse<Vec<vel_api_types::ContextCapture>>> {
        let path = format!("/v1/captures?limit={}&today={}", limit, today);
        self.get(&path).await
    }

    pub async fn list_runs(
        &self,
        limit: Option<u32>,
        kind: Option<&str>,
        today: bool,
    ) -> anyhow::Result<ApiResponse<Vec<vel_api_types::RunSummaryData>>> {
        let mut path = "/v1/runs".to_string();
        let mut params = Vec::new();
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(k) = kind.filter(|s| !s.is_empty()) {
            params.push(format!("kind={}", k));
        }
        if today {
            params.push("today=true".to_string());
        }
        if !params.is_empty() {
            path.push('?');
            path.push_str(&params.join("&"));
        }
        self.get(&path).await
    }

    pub async fn get_run(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::RunDetailData>> {
        self.get(&format!("/v1/runs/{}", id)).await
    }

    pub async fn update_run(
        &self,
        id: &str,
        body: &RunUpdateRequest,
    ) -> anyhow::Result<ApiResponse<vel_api_types::RunDetailData>> {
        let response = self
            .http
            .patch(format!("{}/v1/runs/{}", self.base_url, id))
            .json(body)
            .send()
            .await
            .context("sending update run status request")?;
        crate::client::decode_response(response).await
    }

    pub async fn get_artifact(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::ArtifactData>> {
        self.get(&format!("/v1/artifacts/{}", id)).await
    }

    pub async fn get_artifact_latest(
        &self,
        artifact_type: &str,
    ) -> anyhow::Result<ApiResponse<Option<vel_api_types::ArtifactData>>> {
        let path = format!("/v1/artifacts/latest?type={}", artifact_type);
        self.get(&path).await
    }

    pub async fn list_artifacts(
        &self,
        limit: u32,
    ) -> anyhow::Result<ApiResponse<Vec<vel_api_types::ArtifactData>>> {
        let path = format!("/v1/artifacts?limit={}", limit);
        self.get(&path).await
    }

    pub async fn capture(
        &self,
        request: CaptureCreateRequest,
    ) -> anyhow::Result<ApiResponse<CaptureCreateResponse>> {
        self.post_json("/v1/captures", &request).await
    }

    pub async fn journal_mood(
        &self,
        request: &MoodJournalCreateRequest,
    ) -> anyhow::Result<ApiResponse<CaptureCreateResponse>> {
        self.post_json("/v1/journal/mood", request).await
    }

    pub async fn journal_pain(
        &self,
        request: &PainJournalCreateRequest,
    ) -> anyhow::Result<ApiResponse<CaptureCreateResponse>> {
        self.post_json("/v1/journal/pain", request).await
    }

    pub async fn plan_command(
        &self,
        command: &ResolvedCommand,
        persist_preview: bool,
    ) -> anyhow::Result<ApiResponse<CommandExecutionPlanData>> {
        let response = self
            .http
            .post(format!("{}{}", self.base_url, "/v1/command/plan"))
            .json(&CommandPlanRequest {
                command: command.clone(),
                persist_preview,
            })
            .send()
            .await
            .context("sending command plan request")?;
        decode_response(response).await
    }

    pub async fn execute_command(
        &self,
        command: &ResolvedCommand,
        dry_run: bool,
        approve: bool,
        idempotency_key: Option<String>,
        write_scope: Vec<String>,
    ) -> anyhow::Result<ApiResponse<CommandExecutionResultData>> {
        let response = self
            .http
            .post(format!("{}{}", self.base_url, "/v1/command/execute"))
            .json(&CommandExecuteRequest {
                command: command.clone(),
                dry_run,
                approve,
                idempotency_key,
                write_scope,
            })
            .send()
            .await
            .context("sending command execute request")?;
        decode_response(response).await
    }

    pub async fn list_commitments(
        &self,
        status: Option<&str>,
        project: Option<&str>,
        kind: Option<&str>,
        limit: u32,
    ) -> anyhow::Result<ApiResponse<Vec<CommitmentData>>> {
        let mut path = format!("/v1/commitments?limit={}", limit);
        if let Some(s) = status {
            path.push_str(&format!("&status={}", s));
        }
        if let Some(p) = project {
            path.push_str(&format!("&project={}", p));
        }
        if let Some(k) = kind {
            path.push_str(&format!("&kind={}", k));
        }
        self.get(&path).await
    }

    pub async fn get_commitment(&self, id: &str) -> anyhow::Result<ApiResponse<CommitmentData>> {
        self.get(&format!("/v1/commitments/{}", id)).await
    }

    pub async fn create_commitment(
        &self,
        request: CommitmentCreateRequest,
    ) -> anyhow::Result<ApiResponse<CommitmentData>> {
        let response = self
            .http
            .post(format!("{}{}", self.base_url, "/v1/commitments"))
            .json(&request)
            .send()
            .await
            .context("sending create commitment request")?;
        decode_response(response).await
    }

    pub async fn update_commitment(
        &self,
        id: &str,
        request: CommitmentUpdateRequest,
    ) -> anyhow::Result<ApiResponse<CommitmentData>> {
        let response = self
            .http
            .patch(format!("{}/v1/commitments/{}", self.base_url, id))
            .json(&request)
            .send()
            .await
            .context("sending update commitment request")?;
        decode_response(response).await
    }

    pub async fn list_commitment_dependencies(
        &self,
        commitment_id: &str,
    ) -> anyhow::Result<ApiResponse<Vec<vel_api_types::CommitmentDependencyData>>> {
        self.get(&format!("/v1/commitments/{}/dependencies", commitment_id))
            .await
    }

    pub async fn add_commitment_dependency(
        &self,
        parent_id: &str,
        child_id: &str,
        dependency_type: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::CommitmentDependencyData>> {
        let body = serde_json::json!({
            "child_commitment_id": child_id,
            "dependency_type": dependency_type
        });
        let response = self
            .http
            .post(format!(
                "{}/v1/commitments/{}/dependencies",
                self.base_url, parent_id
            ))
            .json(&body)
            .send()
            .await
            .context("add commitment dependency")?;
        decode_response(response).await
    }

    pub async fn search(&self, query: SearchQuery) -> anyhow::Result<ApiResponse<SearchResults>> {
        let response = self
            .http
            .get(format!("{}{}", self.base_url, "/v1/search"))
            .query(&query)
            .send()
            .await
            .context("sending search request")?;

        decode_response(response).await
    }

    pub async fn today(&self) -> anyhow::Result<ApiResponse<TodayData>> {
        self.get("/v1/context/today").await
    }

    pub async fn start_daily_loop_session(
        &self,
        request: &DailyLoopStartRequestData,
    ) -> anyhow::Result<ApiResponse<DailyLoopSessionData>> {
        self.post_json("/v1/daily-loop/sessions", request).await
    }

    pub async fn active_daily_loop_session(
        &self,
        session_date: &str,
        phase: DailyLoopPhaseData,
    ) -> anyhow::Result<ApiResponse<Option<DailyLoopSessionData>>> {
        let phase = match phase {
            DailyLoopPhaseData::MorningOverview => "morning_overview",
            DailyLoopPhaseData::Standup => "standup",
        };
        self.get(&format!(
            "/v1/daily-loop/sessions/active?session_date={session_date}&phase={phase}"
        ))
        .await
    }

    pub async fn submit_daily_loop_turn(
        &self,
        session_id: &str,
        action: DailyLoopTurnActionData,
        response_text: Option<String>,
    ) -> anyhow::Result<ApiResponse<DailyLoopSessionData>> {
        self.post_json(
            &format!("/v1/daily-loop/sessions/{session_id}/turn"),
            &DailyLoopTurnRequestData {
                session_id: session_id.to_string(),
                action,
                response_text,
            },
        )
        .await
    }

    pub async fn daily_loop_check_in_skip(
        &self,
        check_in_event_id: &str,
        request: &DailyLoopCheckInSkipRequestData,
    ) -> anyhow::Result<ApiResponse<DailyLoopCheckInSkipResponseData>> {
        self.post_json(
            &format!("/v1/daily-loop/check-ins/{check_in_event_id}/skip"),
            request,
        )
        .await
    }

    pub async fn daily_loop_overdue_menu(
        &self,
        session_id: &str,
        request: &DailyLoopOverdueMenuRequestData,
    ) -> anyhow::Result<ApiResponse<DailyLoopOverdueMenuResponseData>> {
        self.post_json(
            &format!("/v1/daily-loop/sessions/{session_id}/overdue/menu"),
            request,
        )
        .await
    }

    pub async fn daily_loop_overdue_confirm(
        &self,
        session_id: &str,
        request: &DailyLoopOverdueConfirmRequestData,
    ) -> anyhow::Result<ApiResponse<DailyLoopOverdueConfirmResponseData>> {
        self.post_json(
            &format!("/v1/daily-loop/sessions/{session_id}/overdue/confirm"),
            request,
        )
        .await
    }

    pub async fn daily_loop_overdue_apply(
        &self,
        session_id: &str,
        request: &DailyLoopOverdueApplyRequestData,
    ) -> anyhow::Result<ApiResponse<DailyLoopOverdueApplyResponseData>> {
        self.post_json(
            &format!("/v1/daily-loop/sessions/{session_id}/overdue/apply"),
            request,
        )
        .await
    }

    pub async fn daily_loop_overdue_undo(
        &self,
        session_id: &str,
        request: &DailyLoopOverdueUndoRequestData,
    ) -> anyhow::Result<ApiResponse<DailyLoopOverdueUndoResponseData>> {
        self.post_json(
            &format!("/v1/daily-loop/sessions/{session_id}/overdue/undo"),
            request,
        )
        .await
    }

    pub async fn end_of_day(&self) -> anyhow::Result<ApiResponse<EndOfDayData>> {
        self.get("/v1/context/end-of-day").await
    }

    pub async fn list_loops(&self) -> anyhow::Result<ApiResponse<Vec<LoopData>>> {
        self.get("/v1/loops").await
    }

    pub async fn get_loop(&self, kind: &str) -> anyhow::Result<ApiResponse<LoopData>> {
        self.get(&format!("/v1/loops/{}", kind)).await
    }

    pub async fn update_loop(
        &self,
        kind: &str,
        body: &LoopUpdateRequest,
    ) -> anyhow::Result<ApiResponse<LoopData>> {
        let response = self
            .http
            .patch(format!("{}/v1/loops/{}", self.base_url, kind))
            .json(body)
            .send()
            .await
            .context("sending update loop request")?;
        decode_response(response).await
    }

    pub async fn list_uncertainty(
        &self,
        status: Option<&str>,
        limit: Option<u32>,
    ) -> anyhow::Result<ApiResponse<Vec<UncertaintyData>>> {
        let mut path = "/v1/uncertainty".to_string();
        let mut params = Vec::new();
        if let Some(status) = status.filter(|value| !value.is_empty()) {
            params.push(format!("status={}", status));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if !params.is_empty() {
            path.push('?');
            path.push_str(&params.join("&"));
        }
        self.get(&path).await
    }

    pub async fn get_uncertainty(&self, id: &str) -> anyhow::Result<ApiResponse<UncertaintyData>> {
        self.get(&format!("/v1/uncertainty/{}", id)).await
    }

    pub async fn resolve_uncertainty(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<UncertaintyData>> {
        let response = self
            .http
            .post(format!("{}/v1/uncertainty/{}/resolve", self.base_url, id))
            .send()
            .await
            .context("resolve uncertainty")?;
        decode_response(response).await
    }

    pub async fn sync_calendar(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/calendar").await
    }

    pub async fn sync_todoist(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/todoist").await
    }

    pub async fn sync_activity(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/activity").await
    }

    pub async fn sync_health(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/health").await
    }

    pub async fn sync_git(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/git").await
    }

    pub async fn sync_notes(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/notes").await
    }

    pub async fn sync_transcripts(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/transcripts").await
    }

    pub async fn sync_messaging(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/messaging").await
    }

    pub async fn sync_reminders(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/reminders").await
    }

    pub async fn sync_bootstrap(&self) -> anyhow::Result<ApiResponse<SyncBootstrapData>> {
        self.get("/v1/sync/bootstrap").await
    }

    pub async fn cluster_bootstrap(&self) -> anyhow::Result<ApiResponse<ClusterBootstrapData>> {
        self.get("/v1/cluster/bootstrap").await
    }

    pub async fn issue_pairing_token(
        &self,
        issued_by_node_id: &str,
        ttl_seconds: Option<i64>,
        scopes: LinkScopeData,
    ) -> anyhow::Result<ApiResponse<PairingTokenData>> {
        let body = IssuePairingTokenRequestData {
            issued_by_node_id: issued_by_node_id.to_string(),
            ttl_seconds,
            scopes,
        };
        self.post_json("/v1/linking/tokens", &body).await
    }

    pub async fn redeem_pairing_token(
        &self,
        token_code: &str,
        node_id: &str,
        node_display_name: &str,
        transport_hint: Option<&str>,
    ) -> anyhow::Result<ApiResponse<LinkedNodeData>> {
        let body = RedeemPairingTokenRequestData {
            token_code: token_code.to_string(),
            node_id: node_id.to_string(),
            node_display_name: node_display_name.to_string(),
            transport_hint: transport_hint.map(ToString::to_string),
            requested_scopes: None,
        };
        self.post_json("/v1/linking/redeem", &body).await
    }

    pub async fn load_linking_status(&self) -> anyhow::Result<ApiResponse<Vec<LinkedNodeData>>> {
        self.get("/v1/linking/status").await
    }

    pub async fn revoke_link(
        &self,
        node_id: &str,
    ) -> anyhow::Result<ApiResponse<serde_json::Value>> {
        self.post_empty(&format!("/v1/linking/revoke/{}", node_id))
            .await
    }

    pub async fn list_connect_instances(
        &self,
    ) -> anyhow::Result<ApiResponse<Vec<ConnectInstanceData>>> {
        self.get("/v1/connect/instances").await
    }

    pub async fn get_connect_instance(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<ConnectInstanceData>> {
        self.get(&format!("/v1/connect/instances/{}", id)).await
    }

    pub async fn attach_connect_instance(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<ConnectAttachData>> {
        self.get(&format!("/v1/connect/instances/{id}/attach"))
            .await
    }

    pub async fn launch_connect_instance(
        &self,
        body: &ConnectLaunchRequestData,
    ) -> anyhow::Result<ApiResponse<ConnectInstanceData>> {
        self.post_json("/v1/connect/instances", body).await
    }

    pub async fn heartbeat_connect_instance(
        &self,
        id: &str,
        body: &ConnectHeartbeatRequestData,
    ) -> anyhow::Result<ApiResponse<ConnectHeartbeatResponseData>> {
        self.post_json(&format!("/v1/connect/instances/{id}/heartbeat"), body)
            .await
    }

    pub async fn terminate_connect_instance(
        &self,
        id: &str,
        body: &ConnectTerminateRequestData,
    ) -> anyhow::Result<ApiResponse<ConnectInstanceData>> {
        self.post_json(&format!("/v1/connect/instances/{id}/terminate"), body)
            .await
    }

    pub async fn write_connect_instance_stdin(
        &self,
        id: &str,
        body: &ConnectStdinRequestData,
    ) -> anyhow::Result<ApiResponse<ConnectStdinWriteAckData>> {
        self.post_json(&format!("/v1/connect/instances/{id}/stdin"), body)
            .await
    }

    pub async fn list_connect_instance_events(
        &self,
        id: &str,
        after_id: Option<i64>,
        limit: Option<u32>,
    ) -> anyhow::Result<ApiResponse<Vec<ConnectRunEventData>>> {
        let mut path = format!("/v1/connect/instances/{id}/events");
        let mut query = Vec::new();
        if let Some(after_id) = after_id {
            query.push(format!("after_id={after_id}"));
        }
        if let Some(limit) = limit {
            query.push(format!("limit={limit}"));
        }
        if !query.is_empty() {
            path.push('?');
            path.push_str(&query.join("&"));
        }
        self.get(&path).await
    }

    pub async fn stream_connect_instance_events(
        &self,
        id: &str,
        after_id: Option<i64>,
        limit: Option<u32>,
        poll_ms: Option<u64>,
        max_events: Option<u32>,
    ) -> anyhow::Result<reqwest::Response> {
        let mut path = format!("/v1/connect/instances/{id}/events/stream");
        let mut query = Vec::new();
        if let Some(after_id) = after_id {
            query.push(format!("after_id={after_id}"));
        }
        if let Some(limit) = limit {
            query.push(format!("limit={limit}"));
        }
        if let Some(poll_ms) = poll_ms {
            query.push(format!("poll_ms={poll_ms}"));
        }
        if let Some(max_events) = max_events {
            query.push(format!("max_events={max_events}"));
        }
        if !query.is_empty() {
            path.push('?');
            path.push_str(&query.join("&"));
        }
        let response = self
            .http
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await
            .with_context(|| format!("sending GET {}", path))?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.context("reading stream error body")?;
            bail!("request failed with status {}: {}", status, body);
        }
        Ok(response)
    }

    pub async fn sync_cluster_state(&self) -> anyhow::Result<ApiResponse<SyncClusterStateData>> {
        self.get("/v1/sync/cluster").await
    }

    pub async fn list_integration_connections(
        &self,
        family: Option<&str>,
        provider_key: Option<&str>,
        include_disabled: bool,
    ) -> anyhow::Result<ApiResponse<Vec<IntegrationConnectionData>>> {
        let mut path = "/api/integrations/connections".to_string();
        let mut params = Vec::new();
        if let Some(family) = family.filter(|value| !value.is_empty()) {
            params.push(format!("family={family}"));
        }
        if let Some(provider_key) = provider_key.filter(|value| !value.is_empty()) {
            params.push(format!("provider_key={provider_key}"));
        }
        if include_disabled {
            params.push("include_disabled=true".to_string());
        }
        if !params.is_empty() {
            path.push('?');
            path.push_str(&params.join("&"));
        }
        self.get(&path).await
    }

    pub async fn get_integration_connection(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<IntegrationConnectionData>> {
        self.get(&format!("/api/integrations/connections/{}", id))
            .await
    }

    pub async fn list_integration_connection_events(
        &self,
        id: &str,
        limit: Option<u32>,
    ) -> anyhow::Result<ApiResponse<Vec<IntegrationConnectionEventData>>> {
        let mut path = format!("/api/integrations/connections/{id}/events");
        if let Some(limit) = limit {
            path.push_str(&format!("?limit={limit}"));
        }
        self.get(&path).await
    }

    pub async fn get_integrations(&self) -> anyhow::Result<ApiResponse<IntegrationsData>> {
        self.get("/api/integrations").await
    }

    pub async fn list_integration_logs(
        &self,
        id: &str,
        limit: Option<u32>,
    ) -> anyhow::Result<ApiResponse<Vec<IntegrationLogEventData>>> {
        let mut path = format!("/api/integrations/{id}/logs");
        if let Some(limit) = limit {
            path.push_str(&format!("?limit={limit}"));
        }
        self.get(&path).await
    }

    pub async fn get_settings(&self) -> anyhow::Result<ApiResponse<serde_json::Value>> {
        self.get("/api/settings").await
    }

    pub async fn get_llm_profile_health(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<LlmProfileHealthData>> {
        self.get(&format!("/api/llm/profiles/{id}/health")).await
    }

    pub async fn list_components(&self) -> anyhow::Result<ApiResponse<Vec<ComponentData>>> {
        self.get("/api/components").await
    }

    pub async fn get_component_logs(
        &self,
        id: &str,
        limit: Option<u32>,
    ) -> anyhow::Result<ApiResponse<Vec<ComponentLogEventData>>> {
        let mut path = format!("/api/components/{id}/logs");
        if let Some(limit) = limit {
            path.push_str(&format!("?limit={limit}"));
        }
        self.get(&path).await
    }

    pub async fn restart_component(&self, id: &str) -> anyhow::Result<ApiResponse<ComponentData>> {
        let path = format!("/api/components/{id}/restart");
        self.post_empty(&path).await
    }

    pub async fn sync_branch_sync_request(
        &self,
        body: &BranchSyncRequestData,
    ) -> anyhow::Result<ApiResponse<QueuedWorkRoutingData>> {
        self.post_json("/v1/sync/branch-sync", body).await
    }

    pub async fn sync_validation_request(
        &self,
        body: &ValidationRequestData,
    ) -> anyhow::Result<ApiResponse<QueuedWorkRoutingData>> {
        self.post_json("/v1/sync/validation", body).await
    }

    pub async fn cluster_branch_sync_request(
        &self,
        body: &BranchSyncRequestData,
    ) -> anyhow::Result<ApiResponse<QueuedWorkRoutingData>> {
        self.post_json("/v1/cluster/branch-sync", body).await
    }

    pub async fn cluster_validation_request(
        &self,
        body: &ValidationRequestData,
    ) -> anyhow::Result<ApiResponse<QueuedWorkRoutingData>> {
        self.post_json("/v1/cluster/validation", body).await
    }

    pub async fn list_nudges(&self) -> anyhow::Result<ApiResponse<Vec<NudgeData>>> {
        self.get("/v1/nudges").await
    }

    pub async fn get_nudge(&self, id: &str) -> anyhow::Result<ApiResponse<NudgeData>> {
        self.get(&format!("/v1/nudges/{}", id)).await
    }

    pub async fn nudge_done(&self, id: &str) -> anyhow::Result<ApiResponse<NudgeData>> {
        let response = self
            .http
            .post(format!("{}/v1/nudges/{}/done", self.base_url, id))
            .send()
            .await
            .context("nudge done")?;
        decode_response(response).await
    }

    pub async fn nudge_snooze(
        &self,
        id: &str,
        minutes: u32,
    ) -> anyhow::Result<ApiResponse<NudgeData>> {
        let body = NudgeSnoozeRequest { minutes };
        let response = self
            .http
            .post(format!("{}/v1/nudges/{}/snooze", self.base_url, id))
            .json(&body)
            .send()
            .await
            .context("nudge snooze")?;
        decode_response(response).await
    }

    pub async fn evaluate(&self) -> anyhow::Result<ApiResponse<EvaluateResultData>> {
        self.post_empty("/v1/evaluate").await
    }

    pub async fn synthesis_week(&self) -> anyhow::Result<ApiResponse<SynthesisWeekData>> {
        self.post_empty("/v1/synthesis/week").await
    }

    pub async fn synthesis_project(
        &self,
        project_slug: &str,
    ) -> anyhow::Result<ApiResponse<SynthesisWeekData>> {
        let path = format!("/v1/synthesis/project/{}", project_slug);
        self.post_empty(&path).await
    }

    pub async fn get_current_context(
        &self,
    ) -> anyhow::Result<ApiResponse<Option<vel_api_types::CurrentContextData>>> {
        self.get("/v1/context/current").await
    }

    pub async fn get_now(&self) -> anyhow::Result<ApiResponse<NowData>> {
        self.get("/v1/now").await
    }

    pub async fn get_agent_inspect(&self) -> anyhow::Result<ApiResponse<AgentInspectData>> {
        self.get("/v1/agent/inspect").await
    }

    pub async fn list_projects(&self) -> anyhow::Result<ApiResponse<ProjectListResponseData>> {
        self.get("/v1/projects").await
    }

    pub async fn get_project(&self, id: &str) -> anyhow::Result<ApiResponse<ProjectRecordData>> {
        self.get(&format!("/v1/projects/{}", id)).await
    }

    pub async fn create_project(
        &self,
        body: &ProjectCreateRequestData,
    ) -> anyhow::Result<ApiResponse<ProjectCreateResponseData>> {
        self.post_json("/v1/projects", body).await
    }

    pub async fn list_project_families(
        &self,
    ) -> anyhow::Result<ApiResponse<Vec<ProjectFamilyData>>> {
        self.get("/v1/projects/families").await
    }

    pub async fn get_execution_context(
        &self,
        project_id: &str,
    ) -> anyhow::Result<ApiResponse<ExecutionContextData>> {
        self.get(&format!("/v1/execution/projects/{}/context", project_id))
            .await
    }

    pub async fn save_execution_context(
        &self,
        project_id: &str,
        body: &ExecutionContextSaveRequestData,
    ) -> anyhow::Result<ApiResponse<ExecutionContextData>> {
        self.post_json(
            &format!("/v1/execution/projects/{}/context", project_id),
            body,
        )
        .await
    }

    pub async fn preview_execution_artifacts(
        &self,
        project_id: &str,
        body: &ExecutionArtifactRequestData,
    ) -> anyhow::Result<ApiResponse<ExecutionArtifactPackData>> {
        self.post_json(
            &format!("/v1/execution/projects/{}/preview", project_id),
            body,
        )
        .await
    }

    pub async fn export_execution_artifacts(
        &self,
        project_id: &str,
        body: &ExecutionArtifactRequestData,
    ) -> anyhow::Result<ApiResponse<ExecutionExportResultData>> {
        self.post_json(
            &format!("/v1/execution/projects/{}/export", project_id),
            body,
        )
        .await
    }

    pub async fn list_execution_handoffs(
        &self,
        project_id: Option<&str>,
        state: Option<&str>,
    ) -> anyhow::Result<ApiResponse<Vec<ExecutionHandoffRecordData>>> {
        let mut path = "/v1/execution/handoffs".to_string();
        let mut params = Vec::new();
        if let Some(project_id) = project_id.filter(|value| !value.trim().is_empty()) {
            params.push(format!("project_id={project_id}"));
        }
        if let Some(state) = state.filter(|value| !value.trim().is_empty()) {
            params.push(format!("state={state}"));
        }
        if !params.is_empty() {
            path.push('?');
            path.push_str(&params.join("&"));
        }
        self.get(&path).await
    }

    pub async fn preview_execution_handoff_launch(
        &self,
        handoff_id: &str,
    ) -> anyhow::Result<ApiResponse<ExecutionLaunchPreviewData>> {
        self.get(&format!(
            "/v1/execution/handoffs/{handoff_id}/launch-preview"
        ))
        .await
    }

    pub async fn approve_execution_handoff(
        &self,
        handoff_id: &str,
        body: &ReviewExecutionHandoffRequestData,
    ) -> anyhow::Result<ApiResponse<ExecutionHandoffRecordData>> {
        self.post_json(
            &format!("/v1/execution/handoffs/{handoff_id}/approve"),
            body,
        )
        .await
    }

    pub async fn reject_execution_handoff(
        &self,
        handoff_id: &str,
        body: &ReviewExecutionHandoffRequestData,
    ) -> anyhow::Result<ApiResponse<ExecutionHandoffRecordData>> {
        self.post_json(&format!("/v1/execution/handoffs/{handoff_id}/reject"), body)
            .await
    }

    pub async fn launch_execution_handoff(
        &self,
        handoff_id: &str,
        body: &LaunchExecutionHandoffRequestData,
    ) -> anyhow::Result<ApiResponse<ConnectInstanceData>> {
        self.post_json(&format!("/v1/execution/handoffs/{handoff_id}/launch"), body)
            .await
    }

    pub async fn get_explain_nudge(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::NudgeExplainData>> {
        self.get(&format!("/v1/explain/nudge/{}", id)).await
    }

    pub async fn get_context_timeline(
        &self,
        limit: u32,
    ) -> anyhow::Result<ApiResponse<Vec<vel_api_types::ContextTimelineEntry>>> {
        self.get(&format!("/v1/context/timeline?limit={}", limit))
            .await
    }

    pub async fn get_explain_context(
        &self,
    ) -> anyhow::Result<ApiResponse<vel_api_types::ContextExplainData>> {
        self.get("/v1/explain/context").await
    }

    pub async fn get_explain_commitment(
        &self,
        commitment_id: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::CommitmentExplainData>> {
        self.get(&format!("/v1/explain/commitment/{}", commitment_id))
            .await
    }

    pub async fn get_explain_drift(
        &self,
    ) -> anyhow::Result<ApiResponse<vel_api_types::DriftExplainData>> {
        self.get("/v1/explain/drift").await
    }

    pub async fn list_threads(
        &self,
        status: Option<&str>,
        limit: u32,
    ) -> anyhow::Result<ApiResponse<Vec<vel_api_types::ThreadData>>> {
        let path = match status {
            Some(s) => format!("/v1/threads?status={}&limit={}", s, limit),
            None => format!("/v1/threads?limit={}", limit),
        };
        self.get(&path).await
    }

    pub async fn get_thread(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::ThreadData>> {
        self.get(&format!("/v1/threads/{}", id)).await
    }

    pub async fn get_risk_list(&self) -> anyhow::Result<ApiResponse<Vec<vel_api_types::RiskData>>> {
        self.get("/v1/risk").await
    }

    pub async fn get_risk_commitment(
        &self,
        commitment_id: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::RiskData>> {
        self.get(&format!("/v1/risk/{}", commitment_id)).await
    }

    pub async fn list_suggestions(
        &self,
        state: Option<&str>,
        limit: Option<u32>,
    ) -> anyhow::Result<ApiResponse<Vec<vel_api_types::SuggestionData>>> {
        let limit = limit.unwrap_or(50);
        let path = match state {
            Some(s) => format!("/v1/suggestions?state={}&limit={}", s, limit),
            None => format!("/v1/suggestions?limit={}", limit),
        };
        self.get(&path).await
    }

    pub async fn get_suggestion(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::SuggestionData>> {
        self.get(&format!("/v1/suggestions/{}", id)).await
    }

    pub async fn get_suggestion_evidence(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<Vec<vel_api_types::SuggestionEvidenceData>>> {
        self.get(&format!("/v1/suggestions/{}/evidence", id)).await
    }

    pub async fn update_suggestion(
        &self,
        id: &str,
        state: &str,
        payload: Option<serde_json::Value>,
    ) -> anyhow::Result<ApiResponse<vel_api_types::SuggestionData>> {
        let body = vel_api_types::SuggestionUpdateRequest {
            state: Some(state.to_string()),
            payload,
        };
        let response = self
            .http
            .patch(format!("{}/v1/suggestions/{}", self.base_url, id))
            .json(&body)
            .send()
            .await
            .context("PATCH suggestion")?;
        decode_response(response).await
    }

    pub async fn accept_suggestion(
        &self,
        id: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::SuggestionData>> {
        let response = self
            .http
            .post(format!("{}/v1/suggestions/{}/accept", self.base_url, id))
            .json(&vel_api_types::SuggestionActionRequest::default())
            .send()
            .await
            .context("POST accept suggestion")?;
        decode_response(response).await
    }

    pub async fn reject_suggestion(
        &self,
        id: &str,
        reason: Option<&str>,
    ) -> anyhow::Result<ApiResponse<vel_api_types::SuggestionData>> {
        let response = self
            .http
            .post(format!("{}/v1/suggestions/{}/reject", self.base_url, id))
            .json(&vel_api_types::SuggestionActionRequest {
                reason: reason.map(ToString::to_string),
            })
            .send()
            .await
            .context("POST reject suggestion")?;
        decode_response(response).await
    }

    pub async fn update_thread(
        &self,
        id: &str,
        status: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::ThreadData>> {
        let response = self
            .http
            .patch(format!("{}/v1/threads/{}", self.base_url, id))
            .json(&serde_json::json!({ "status": status }))
            .send()
            .await
            .context("PATCH thread")?;
        decode_response(response).await
    }

    pub async fn import_batch(
        &self,
        request: BatchImportRequest,
    ) -> anyhow::Result<ApiResponse<BatchImportResponse>> {
        let response = self
            .http
            .post(format!("{}/v1/import/batch", self.base_url))
            .timeout(std::time::Duration::from_secs(120))
            .json(&request)
            .send()
            .await
            .context("POST /v1/import/batch")?;
        decode_response(response).await
    }

    async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<ApiResponse<T>> {
        let response = self
            .http
            .post(format!("{}{}", self.base_url, path))
            .send()
            .await
            .with_context(|| format!("POST {}", path))?;
        decode_response(response).await
    }

    async fn post_json<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<ApiResponse<T>> {
        let response = self
            .http
            .post(format!("{}{}", self.base_url, path))
            .json(body)
            .send()
            .await
            .with_context(|| format!("POST {}", path))?;
        decode_response(response).await
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<ApiResponse<T>> {
        let response = self
            .http
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await
            .with_context(|| format!("sending GET {}", path))?;

        decode_response(response).await
    }
}

async fn decode_response<T: DeserializeOwned>(
    response: reqwest::Response,
) -> anyhow::Result<ApiResponse<T>> {
    let status = response.status();
    let body = response.text().await.context("reading response body")?;
    let parsed: ApiResponse<T> = serde_json::from_str(&body).context("parsing api response")?;

    if !status.is_success() || !parsed.ok {
        let message = parsed
            .error
            .as_ref()
            .map(|error| error.message.clone())
            .unwrap_or_else(|| format!("request failed with status {}", status));
        bail!(message);
    }

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use serde::Serialize;
    use time::OffsetDateTime;
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

    fn make_client(server: &MockServer) -> ApiClient {
        ApiClient::new(server.uri())
    }

    fn sample_commitment() -> CommitmentData {
        CommitmentData {
            id: vel_core::CommitmentId::from("com_123".to_string()),
            text: "Ship client coverage".to_string(),
            source_type: "manual".to_string(),
            source_id: None,
            status: "open".to_string(),
            due_at: None,
            project: Some("vel".to_string()),
            commitment_kind: Some("todo".to_string()),
            created_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
            resolved_at: None,
            scheduler_rules: vel_api_types::CanonicalScheduleRulesData::default(),
            metadata: serde_json::json!({ "lane": "cli" }),
        }
    }

    fn sample_run_detail() -> vel_api_types::RunDetailData {
        vel_api_types::RunDetailData {
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

    fn sample_suggestion() -> vel_api_types::SuggestionData {
        vel_api_types::SuggestionData {
            id: "sg_123".to_string(),
            suggestion_type: "adaptive_policy".to_string(),
            state: "pending".to_string(),
            title: Some("Delay sync".to_string()),
            summary: Some("Push sync later".to_string()),
            priority: 3,
            confidence: Some("medium".to_string()),
            evidence_count: 1,
            decision_context_summary: Some("Recent failures detected".to_string()),
            decision_context: Some(serde_json::json!({ "source": "health" })),
            evidence: None,
            latest_feedback_outcome: None,
            latest_feedback_notes: None,
            adaptive_policy: None,
            payload: serde_json::json!({ "minutes": 30 }),
            created_at: 1_742_927_200,
            resolved_at: None,
        }
    }

    fn sample_thread() -> vel_api_types::ThreadData {
        vel_api_types::ThreadData {
            id: "thr_123".to_string(),
            thread_type: "planning".to_string(),
            title: "Client coverage follow-up".to_string(),
            status: "open".to_string(),
            planning_kind: None,
            lifecycle_stage: None,
            created_at: 1_742_927_200,
            updated_at: 1_742_927_260,
            continuation: None,
            metadata: Some(serde_json::json!({ "owner": "cli" })),
            links: None,
            project_id: Some(vel_core::ProjectId::from("proj_vel".to_string())),
            project_label: Some("Vel".to_string()),
        }
    }

    #[tokio::test]
    async fn decode_response_returns_parsed_payload_for_success_status() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/ok"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(ok_response(serde_json::json!({ "value": 7 }))),
            )
            .mount(&server)
            .await;

        let response = reqwest::Client::new()
            .get(format!("{}/ok", server.uri()))
            .send()
            .await
            .unwrap();
        let parsed = decode_response::<serde_json::Value>(response)
            .await
            .unwrap();

        assert!(parsed.ok);
        assert_eq!(parsed.data.unwrap()["value"], 7);
    }

    #[tokio::test]
    async fn decode_response_uses_api_error_message_for_failed_status() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/fail"))
            .respond_with(
                ResponseTemplate::new(400).set_body_json(error_response("invalid request payload")),
            )
            .mount(&server)
            .await;

        let response = reqwest::Client::new()
            .get(format!("{}/fail", server.uri()))
            .send()
            .await
            .unwrap();
        let err = decode_response::<serde_json::Value>(response)
            .await
            .unwrap_err();

        assert!(err.to_string().contains("invalid request payload"));
    }

    #[tokio::test]
    async fn list_runs_includes_limit_kind_and_today_query_parameters() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/runs"))
            .and(query_param("limit", "5"))
            .and(query_param("kind", "sync"))
            .and(query_param("today", "true"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!([]))),
            )
            .mount(&server)
            .await;

        let response = make_client(&server)
            .list_runs(Some(5), Some("sync"), true)
            .await
            .unwrap();

        assert_eq!(response.data.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn list_uncertainty_omits_blank_status_and_keeps_limit() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/uncertainty"))
            .and(query_param("limit", "20"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!([]))),
            )
            .mount(&server)
            .await;

        let response = make_client(&server)
            .list_uncertainty(Some(""), Some(20))
            .await
            .unwrap();

        assert_eq!(response.data.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn list_integration_connections_applies_only_non_empty_filters() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/integrations/connections"))
            .and(query_param("provider_key", "google"))
            .and(query_param("include_disabled", "true"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!([]))),
            )
            .mount(&server)
            .await;

        let response = make_client(&server)
            .list_integration_connections(Some(""), Some("google"), true)
            .await
            .unwrap();

        assert_eq!(response.data.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn list_integration_logs_includes_optional_limit() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/integrations/google/logs"))
            .and(query_param("limit", "12"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!([]))),
            )
            .mount(&server)
            .await;

        let response = make_client(&server)
            .list_integration_logs("google", Some(12))
            .await
            .unwrap();

        assert_eq!(response.data.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn get_llm_profile_health_targets_profile_route() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/llm/profiles/default/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(
                serde_json::json!({
                    "profile_id": "default",
                    "healthy": true,
                    "message": "Provider handshake succeeded."
                }),
            )))
            .mount(&server)
            .await;

        let response = make_client(&server)
            .get_llm_profile_health("default")
            .await
            .unwrap();

        let data = response.data.unwrap();
        assert_eq!(data.profile_id, "default");
        assert!(data.healthy);
    }

    #[tokio::test]
    async fn get_component_logs_includes_optional_limit() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/components/evaluate/logs"))
            .and(query_param("limit", "50"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!([]))),
            )
            .mount(&server)
            .await;

        let response = make_client(&server)
            .get_component_logs("evaluate", Some(50))
            .await
            .unwrap();

        assert_eq!(response.data.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn list_connect_instance_events_builds_optional_query_string() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/connect/instances/run_123/events"))
            .and(query_param("after_id", "9"))
            .and(query_param("limit", "25"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_response(serde_json::json!([]))),
            )
            .mount(&server)
            .await;

        let response = make_client(&server)
            .list_connect_instance_events("run_123", Some(9), Some(25))
            .await
            .unwrap();

        assert_eq!(response.data.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn post_json_serializes_body_and_decodes_response() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/example"))
            .and(body_json(serde_json::json!({
                "note": "keep this stable",
                "count": 2
            })))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(ok_response(serde_json::json!({ "saved": true }))),
            )
            .mount(&server)
            .await;

        let response = make_client(&server)
            .post_json::<serde_json::Value, _>(
                "/v1/example",
                &serde_json::json!({
                    "note": "keep this stable",
                    "count": 2
                }),
            )
            .await
            .unwrap();

        assert_eq!(response.data.unwrap()["saved"], true);
    }

    #[tokio::test]
    async fn stream_connect_instance_events_returns_response_for_successful_stream() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/connect/instances/run_123/events/stream"))
            .and(query_param("after_id", "4"))
            .and(query_param("limit", "10"))
            .and(query_param("poll_ms", "250"))
            .and(query_param("max_events", "3"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "text/event-stream")
                    .set_body_string("event: connect_event\ndata: {}\n\n"),
            )
            .mount(&server)
            .await;

        let response = make_client(&server)
            .stream_connect_instance_events("run_123", Some(4), Some(10), Some(250), Some(3))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn stream_connect_instance_events_surfaces_error_body_for_failed_stream() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/connect/instances/run_404/events/stream"))
            .respond_with(ResponseTemplate::new(404).set_body_string("connect instance not found"))
            .mount(&server)
            .await;

        let err = make_client(&server)
            .stream_connect_instance_events("run_404", None, None, None, None)
            .await
            .unwrap_err();

        assert!(err.to_string().contains("connect instance not found"));
    }

    #[tokio::test]
    async fn capture_posts_typed_request_and_parses_capture_response() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/captures"))
            .and(body_json(serde_json::json!({
                "content_text": "remember the edge case",
                "capture_type": "todo",
                "source_device": "laptop"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(
                CaptureCreateResponse {
                    capture_id: vel_core::CaptureId::from("cap_123".to_string()),
                    accepted_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
                },
            )))
            .mount(&server)
            .await;

        let response = make_client(&server)
            .capture(CaptureCreateRequest {
                content_text: "remember the edge case".to_string(),
                capture_type: "todo".to_string(),
                source_device: Some("laptop".to_string()),
            })
            .await
            .unwrap();

        let capture = response.data.unwrap();
        assert_eq!(capture.capture_id.as_ref(), "cap_123");
    }

    #[tokio::test]
    async fn export_backup_posts_expected_payload() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/backup/export"))
            .and(body_json(serde_json::json!({
                "target_root": "/tmp/nas/google",
                "domains": ["calendar", "tasks"],
                "include_parquet": false
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(
                serde_json::json!({
                    "manifest": {
                        "export_id": "bex_test",
                        "created_at": "2026-04-16T09:00:00Z",
                        "target_root": "/tmp/nas/google",
                        "export_root": "/tmp/nas/google/runs/bex_test",
                        "included_domains": ["calendar", "tasks"],
                        "omitted_domains": [],
                        "files": [],
                        "derivatives": [],
                        "verification_summary": {
                            "verified": true,
                            "checksum_algorithm": "sha256",
                            "checksum": "abc123",
                            "checked_paths": ["/tmp/nas/google/manifest.json"],
                            "notes": []
                        }
                    }
                }),
            )))
            .mount(&server)
            .await;

        let response = make_client(&server)
            .export_backup(
                "/tmp/nas/google",
                vec!["calendar".to_string(), "tasks".to_string()],
            )
            .await
            .unwrap();

        let result = response.data.unwrap();
        assert_eq!(result.manifest.export_id, "bex_test");
        assert_eq!(
            result.manifest.included_domains,
            vec!["calendar".to_string(), "tasks".to_string()]
        );
    }

    #[tokio::test]
    async fn backup_export_status_targets_status_route() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/backup/export/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(
                serde_json::json!({
                    "state": "ready",
                    "last_export_id": "bex_test",
                    "last_export_at": "2026-04-16T09:00:00Z",
                    "target_root": "/tmp/nas/google",
                    "included_domains": ["tasks"],
                    "omitted_domains": [],
                    "warnings": []
                }),
            )))
            .mount(&server)
            .await;

        let response = make_client(&server).backup_export_status().await.unwrap();
        let status = response.data.unwrap();
        assert_eq!(status.last_export_id.as_deref(), Some("bex_test"));
        assert_eq!(status.included_domains, vec!["tasks".to_string()]);
    }

    #[tokio::test]
    async fn update_loop_uses_patch_with_expected_body() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/v1/loops/morning"))
            .and(body_json(serde_json::json!({
                "enabled": true,
                "interval_seconds": 3600
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(
                serde_json::json!({
                    "kind": "morning",
                    "enabled": true,
                    "interval_seconds": 3600,
                    "last_started_at": null,
                    "last_finished_at": null,
                    "last_status": null,
                    "last_error": null,
                    "next_due_at": null
                }),
            )))
            .mount(&server)
            .await;

        let response = make_client(&server)
            .update_loop(
                "morning",
                &LoopUpdateRequest {
                    enabled: Some(true),
                    interval_seconds: Some(3600),
                },
            )
            .await
            .unwrap();

        let loop_data = response.data.unwrap();
        assert_eq!(loop_data.kind, "morning");
        assert!(loop_data.enabled);
        assert_eq!(loop_data.interval_seconds, 3600);
    }

    #[tokio::test]
    async fn sync_activity_uses_post_empty_helper() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/sync/activity"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(
                serde_json::json!({
                    "source": "activity",
                    "signals_ingested": 4
                }),
            )))
            .mount(&server)
            .await;

        let response = make_client(&server).sync_activity().await.unwrap();

        let result = response.data.unwrap();
        assert_eq!(result.source, "activity");
        assert_eq!(result.signals_ingested, 4);
    }

    #[tokio::test]
    async fn restart_component_uses_post_empty_helper() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/components/evaluate/restart"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(
                serde_json::json!({
                    "id": "evaluate",
                    "name": "Evaluate",
                    "description": "Inference evaluator",
                    "status": "healthy",
                    "last_restarted_at": 1742927200,
                    "last_error": null,
                    "restart_count": 2
                }),
            )))
            .mount(&server)
            .await;

        let response = make_client(&server)
            .restart_component("evaluate")
            .await
            .unwrap();

        let component = response.data.unwrap();
        assert_eq!(component.id, "evaluate");
        assert_eq!(component.restart_count, 2);
    }

    #[tokio::test]
    async fn create_commitment_posts_typed_body_and_parses_response() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/commitments"))
            .and(body_json(serde_json::json!({
                "text": "Ship client coverage",
                "source_type": "manual",
                "source_id": null,
                "due_at": null,
                "project": "vel",
                "commitment_kind": "todo",
                "metadata": { "lane": "cli" }
            })))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_response(sample_commitment())),
            )
            .mount(&server)
            .await;

        let response = make_client(&server)
            .create_commitment(CommitmentCreateRequest {
                text: "Ship client coverage".to_string(),
                source_type: "manual".to_string(),
                source_id: None,
                due_at: None,
                project: Some("vel".to_string()),
                commitment_kind: Some("todo".to_string()),
                metadata: serde_json::json!({ "lane": "cli" }),
            })
            .await
            .unwrap();

        let commitment = response.data.unwrap();
        assert_eq!(commitment.id.as_ref(), "com_123");
        assert_eq!(commitment.project.as_deref(), Some("vel"));
    }

    #[tokio::test]
    async fn update_run_uses_patch_and_preserves_retry_fields() {
        let server = MockServer::start().await;
        let retry_at = OffsetDateTime::from_unix_timestamp(1_742_930_800).unwrap();
        let request = RunUpdateRequest {
            status: "blocked".to_string(),
            retry_at: Some(retry_at),
            retry_after_seconds: Some(900),
            reason: Some("waiting on dependency".to_string()),
            allow_unsupported_retry: false,
            blocked_reason: Some("calendar token expired".to_string()),
        };
        Mock::given(method("PATCH"))
            .and(path("/v1/runs/run_123"))
            .and(body_json(serde_json::to_value(&request).unwrap()))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_response(sample_run_detail())),
            )
            .mount(&server)
            .await;

        let response = make_client(&server)
            .update_run("run_123", &request)
            .await
            .unwrap();

        let run = response.data.unwrap();
        assert_eq!(run.id.as_ref(), "run_123");
        assert_eq!(
            run.blocked_reason.as_deref(),
            Some("calendar token expired")
        );
    }

    #[tokio::test]
    async fn update_suggestion_uses_patch_with_optional_payload() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/v1/suggestions/sg_123"))
            .and(body_json(serde_json::json!({
                "state": "accepted",
                "payload": { "minutes": 30 }
            })))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_response(sample_suggestion())),
            )
            .mount(&server)
            .await;

        let response = make_client(&server)
            .update_suggestion(
                "sg_123",
                "accepted",
                Some(serde_json::json!({ "minutes": 30 })),
            )
            .await
            .unwrap();

        let suggestion = response.data.unwrap();
        assert_eq!(suggestion.id, "sg_123");
        assert_eq!(suggestion.payload["minutes"], 30);
    }

    #[tokio::test]
    async fn update_thread_uses_patch_with_status_body() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/v1/threads/thr_123"))
            .and(body_json(serde_json::json!({ "status": "closed" })))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_response(sample_thread())))
            .mount(&server)
            .await;

        let response = make_client(&server)
            .update_thread("thr_123", "closed")
            .await
            .unwrap();

        let thread = response.data.unwrap();
        assert_eq!(thread.id, "thr_123");
        assert_eq!(thread.project_label.as_deref(), Some("Vel"));
    }
}
