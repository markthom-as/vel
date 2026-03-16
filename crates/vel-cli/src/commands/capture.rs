use crate::client::ApiClient;
use vel_api_types::CaptureCreateRequest;

fn read_stdin() -> anyhow::Result<String> {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

pub async fn run(
    client: &ApiClient,
    text: Option<String>,
    stdin: bool,
    capture_type: Option<String>,
    source: Option<String>,
) -> anyhow::Result<()> {
    let content_text = if stdin || text.as_deref() == Some("-") {
        read_stdin()?
    } else {
        text.ok_or_else(|| anyhow::anyhow!("provide capture text or use --stdin (or -)"))?
    };
    let content_text = content_text.trim().to_string();
    if content_text.is_empty() {
        anyhow::bail!("capture text must not be empty");
    }
    let request = CaptureCreateRequest {
        content_text,
        capture_type: capture_type.unwrap_or_else(|| "quick_note".to_string()),
        source_device: source.or_else(|| Some("vel-cli".to_string())),
    };
    let response = client.capture(request).await?;
    let data = response.data.expect("capture response missing data");
    println!("capture_id: {}", data.capture_id);
    Ok(())
}
