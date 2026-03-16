//! `vel artifact` — artifact-related commands.

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

pub async fn run_latest(client: &ApiClient, artifact_type: &str, json: bool) -> anyhow::Result<()> {
    let response = client.get_artifact_latest(artifact_type).await?;
    let opt = response
        .data
        .expect("get_artifact_latest response missing data");
    if json {
        println!("{}", serde_json::to_string_pretty(&opt)?);
        return Ok(());
    }
    match opt {
        None => println!("No artifact of type '{}' found.", artifact_type),
        Some(a) => {
            println!("artifact_id: {}", a.artifact_id);
            println!("artifact_type: {}", a.artifact_type);
            println!("storage_kind: {}", a.storage_kind);
            println!("storage_uri: {}", a.storage_uri);
            if let Some(size) = a.size_bytes {
                println!("size: {}", format_size(size));
            }
            if let Some(ref h) = a.content_hash {
                println!("content_hash: {}", h);
            }
            println!("created_at: {}", a.created_at);
        }
    }
    Ok(())
}
