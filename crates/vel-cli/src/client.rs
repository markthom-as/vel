use anyhow::{bail, Context};
use reqwest::Client;
use serde::de::DeserializeOwned;
use vel_api_types::{
    ApiResponse, CaptureCreateRequest, CaptureCreateResponse, EndOfDayData, HealthData, MorningData,
    SearchQuery, SearchResults, TodayData,
};

#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            http: Client::new(),
            base_url,
        }
    }

    pub async fn health(&self) -> anyhow::Result<ApiResponse<HealthData>> {
        self.get("/v1/health").await
    }

    pub async fn capture(&self, text: String) -> anyhow::Result<ApiResponse<CaptureCreateResponse>> {
        let response = self
            .http
            .post(format!("{}{}", self.base_url, "/v1/captures"))
            .json(&CaptureCreateRequest {
                content_text: text,
                capture_type: "quick_note".to_string(),
                source_device: Some("vel-cli".to_string()),
            })
            .send()
            .await
            .context("sending capture request")?;

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

async fn decode_response<T: DeserializeOwned>(response: reqwest::Response) -> anyhow::Result<ApiResponse<T>> {
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
