//! `vel import` — import file, lines from stdin, or capture URL.

use crate::client::ApiClient;
use std::path::Path;
use vel_api_types::CaptureCreateRequest;

fn read_stdin() -> anyhow::Result<String> {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

pub async fn run_file(client: &ApiClient, path: &str, capture_type: &str) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(Path::new(path))?;
    let content = content.trim();
    if content.is_empty() {
        anyhow::bail!("file is empty");
    }
    let request = CaptureCreateRequest {
        content_text: content.to_string(),
        capture_type: capture_type.to_string(),
        source_device: Some("import-file".to_string()),
    };
    let response = client.capture(request).await?;
    let data = response.data.expect("capture response missing data");
    println!("capture_id: {}", data.capture_id);
    Ok(())
}

pub async fn run_lines(client: &ApiClient, capture_type: &str) -> anyhow::Result<()> {
    let stdin = read_stdin()?;
    let lines: Vec<&str> = stdin
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if lines.is_empty() {
        anyhow::bail!("no non-empty lines on stdin");
    }
    let mut ids = Vec::new();
    for line in lines {
        let request = CaptureCreateRequest {
            content_text: line.to_string(),
            capture_type: capture_type.to_string(),
            source_device: Some("import-lines".to_string()),
        };
        let response = client.capture(request).await?;
        let data = response.data.expect("capture response missing data");
        ids.push(data.capture_id.to_string());
    }
    println!("Created {} capture(s): {}", ids.len(), ids.join(", "));
    Ok(())
}

pub async fn run_capture_url(client: &ApiClient, url: &str) -> anyhow::Result<()> {
    let request = CaptureCreateRequest {
        content_text: url.trim().to_string(),
        capture_type: "url".to_string(),
        source_device: Some("vel-cli".to_string()),
    };
    let response = client.capture(request).await?;
    let data = response.data.expect("capture response missing data");
    println!("capture_id: {}", data.capture_id);
    Ok(())
}
