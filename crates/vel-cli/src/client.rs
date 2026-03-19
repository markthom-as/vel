use anyhow::{bail, Context};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vel_api_types::{
    ApiResponse, BackupManifestData, BackupStatusData, BranchSyncRequestData,
    CaptureCreateRequest, CaptureCreateResponse, ClusterBootstrapData, CommandExecuteRequest,
    CommandExecutionPlanData, CommandExecutionResultData, CommandPlanRequest,
    CommitmentCreateRequest, CommitmentData, CommitmentUpdateRequest, ConnectInstanceData,
    DoctorData, EndOfDayData, EvaluateResultData, ExecutionHandoffData, HealthData,
    IntegrationConnectionData, IntegrationConnectionEventData, LinkScopeData, LinkedNodeData,
    LoopData, LoopUpdateRequest, MoodJournalCreateRequest, MorningData, NowData, NudgeData,
    NudgeSnoozeRequest, PainJournalCreateRequest, PairingTokenData, ProjectListResponseData,
    QueuedWorkRoutingData, RunUpdateRequest, SearchQuery, SearchResults, SyncBootstrapData,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateExecutionHandoffRequestData {
    pub project_id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub origin_kind: String,
    pub objective: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_budget: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_gate: Option<String>,
    #[serde(default)]
    pub read_scopes: Vec<String>,
    #[serde(default)]
    pub write_scopes: Vec<String>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub inputs: serde_json::Value,
    #[serde(default)]
    pub expected_output_schema: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviewExecutionHandoffRequestData {
    pub reviewed_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_reason: Option<String>,
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

    pub async fn backup_status(&self) -> anyhow::Result<ApiResponse<BackupStatusData>> {
        self.get("/v1/backup/status").await
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
    ) -> anyhow::Result<ApiResponse<CommandExecutionPlanData>> {
        let response = self
            .http
            .post(format!("{}{}", self.base_url, "/v1/command/plan"))
            .json(&CommandPlanRequest {
                command: command.clone(),
            })
            .send()
            .await
            .context("sending command plan request")?;
        decode_response(response).await
    }

    pub async fn execute_command(
        &self,
        command: &ResolvedCommand,
    ) -> anyhow::Result<ApiResponse<CommandExecutionResultData>> {
        let response = self
            .http
            .post(format!("{}{}", self.base_url, "/v1/command/execute"))
            .json(&CommandExecuteRequest {
                command: command.clone(),
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

    pub async fn morning(&self) -> anyhow::Result<ApiResponse<MorningData>> {
        self.get("/v1/context/morning").await
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

    pub async fn list_projects(&self) -> anyhow::Result<ApiResponse<ProjectListResponseData>> {
        self.get("/v1/projects").await
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

    pub async fn create_execution_handoff(
        &self,
        body: &CreateExecutionHandoffRequestData,
    ) -> anyhow::Result<ApiResponse<ExecutionHandoffRecordData>> {
        self.post_json("/v1/execution/handoffs", body).await
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
