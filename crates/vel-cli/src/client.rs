use anyhow::{bail, Context};
use reqwest::Client;
use serde::de::DeserializeOwned;
use vel_api_types::{
    ApiResponse, CaptureCreateRequest, CaptureCreateResponse, CommitmentCreateRequest,
    CommitmentData, CommitmentUpdateRequest, DoctorData, EndOfDayData, EvaluateResultData,
    HealthData, MorningData, NudgeData, NudgeSnoozeRequest, SearchQuery, SearchResults,
    SyncResultData, SynthesisWeekData, TodayData,
};

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

    pub async fn update_run_status(
        &self,
        id: &str,
        status: &str,
    ) -> anyhow::Result<ApiResponse<vel_api_types::RunDetailData>> {
        let body = serde_json::json!({ "status": status });
        let response = self
            .http
            .patch(format!("{}/v1/runs/{}", self.base_url, id))
            .json(&body)
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
        let response = self
            .http
            .post(format!("{}{}", self.base_url, "/v1/captures"))
            .json(&request)
            .send()
            .await
            .context("sending capture request")?;

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

    pub async fn sync_calendar(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/calendar").await
    }

    pub async fn sync_todoist(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/todoist").await
    }

    pub async fn sync_activity(&self) -> anyhow::Result<ApiResponse<SyncResultData>> {
        self.post_empty("/v1/sync/activity").await
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
