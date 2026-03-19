use crate::client::ApiClient;
use vel_api_types::DailyLoopPhaseData;

pub async fn run(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    crate::commands::daily_loop::run_start(client, DailyLoopPhaseData::MorningOverview, json).await
}
