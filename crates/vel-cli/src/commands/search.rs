use time::format_description::well_known::Rfc3339;
use vel_api_types::{SearchQuery, SearchResult};

use crate::client::ApiClient;

pub async fn run(
    client: &ApiClient,
    query: String,
    capture_type: Option<String>,
    source_device: Option<String>,
    limit: Option<u32>,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .search(SearchQuery {
            q: query,
            capture_type,
            source_device,
            limit,
        })
        .await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let data = response.data.expect("search response missing data");
    if data.results.is_empty() {
        println!("no matching captures");
        return Ok(());
    }

    for result in data.results {
        println!("{}", format_result(&result)?);
    }

    Ok(())
}

fn format_result(result: &SearchResult) -> anyhow::Result<String> {
    let occurred_at = result.occurred_at.format(&Rfc3339)?;
    let mut lines = Vec::with_capacity(3);
    lines.push(format!(
        "{} [{}] {}",
        occurred_at, result.capture_type, result.capture_id
    ));
    if let Some(source_device) = result.source_device.as_deref() {
        lines.push(format!("source: {}", source_device));
    }
    lines.push(result.snippet.replace(['[', ']'], ""));
    Ok(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;
    use vel_core::CaptureId;

    #[test]
    fn format_result_includes_id_and_snippet() {
        let rendered = format_result(&SearchResult {
            capture_id: CaptureId::from("cap_test".to_string()),
            capture_type: "quick_note".to_string(),
            snippet: "remember lidar budget".to_string(),
            occurred_at: OffsetDateTime::UNIX_EPOCH,
            created_at: OffsetDateTime::UNIX_EPOCH,
            source_device: Some("phone".to_string()),
        })
        .unwrap();

        assert!(rendered.contains("cap_test"));
        assert!(rendered.contains("remember lidar budget"));
        assert!(rendered.contains("source: phone"));
    }
}
