use crate::client::ApiClient;
use vel_api_types::{MoodJournalCreateRequest, PainJournalCreateRequest};

pub async fn run_mood(
    client: &ApiClient,
    score: u8,
    label: Option<String>,
    note: Option<String>,
    source: Option<String>,
) -> anyhow::Result<()> {
    let response = client
        .journal_mood(&MoodJournalCreateRequest {
            score,
            label,
            note,
            source_device: source.or_else(|| Some("vel-cli".to_string())),
        })
        .await?;
    let data = response.data.expect("journal mood response missing data");
    println!("capture_id: {}", data.capture_id);
    Ok(())
}

pub async fn run_pain(
    client: &ApiClient,
    severity: u8,
    location: Option<String>,
    note: Option<String>,
    source: Option<String>,
) -> anyhow::Result<()> {
    let response = client
        .journal_pain(&PainJournalCreateRequest {
            severity,
            location,
            note,
            source_device: source.or_else(|| Some("vel-cli".to_string())),
        })
        .await?;
    let data = response.data.expect("journal pain response missing data");
    println!("capture_id: {}", data.capture_id);
    Ok(())
}
