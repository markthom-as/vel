//! `vel inspect` — inspect a single capture or other entity by ID.

use crate::client::ApiClient;

pub async fn run_capture(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let response = client.get_capture(id).await?;
    let capture = response.data.expect("get_capture response missing data");
    println!("capture_id: {}", capture.capture_id);
    println!("capture_type: {}", capture.capture_type);
    println!("occurred_at: {}", capture.occurred_at);
    if let Some(ref d) = capture.source_device {
        println!("source_device: {}", d);
    }
    println!("content_text: {}", capture.content_text);
    Ok(())
}
