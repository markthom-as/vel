//! `vel inspect` — inspect a single capture, artifact, or other entity by ID.

use crate::client::ApiClient;

fn format_size(bytes: i64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

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

pub async fn run_artifact(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let response = client.get_artifact(id).await?;
    let a = response.data.expect("get_artifact response missing data");
    println!("artifact_id: {}", a.artifact_id);
    println!("artifact_type: {}", a.artifact_type);
    println!("storage_kind: {}", a.storage_kind);
    println!("storage_uri: {}", a.storage_uri);
    if let Some(ref size) = a.size_bytes {
        println!("size: {}", format_size(*size));
    }
    if let Some(ref h) = a.content_hash {
        println!("content_hash: {}", h);
    }
    if let Some(ref t) = a.title {
        println!("title: {}", t);
    }
    println!("created_at: {}", a.created_at);
    println!("updated_at: {}", a.updated_at);
    Ok(())
}
