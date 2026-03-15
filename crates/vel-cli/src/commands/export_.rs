//! `vel export` — export captures, runs, and/or artifacts as JSON.

use crate::client::ApiClient;

pub async fn run(
    client: &ApiClient,
    captures: bool,
    runs: bool,
    artifacts: bool,
    format: &str,
    json: bool,
) -> anyhow::Result<()> {
    let captures = captures || (!captures && !runs && !artifacts);
    if !captures && !runs && !artifacts {
        println!("Specify --captures, --runs, and/or --artifacts to export.");
        return Ok(());
    }
    if format != "json" {
        println!("Only --format json is supported.");
        return Ok(());
    }
    let mut out = serde_json::Map::new();
    if captures {
        let resp = client.list_captures_recent(500, false).await?;
        let data = resp.data.expect("list_captures_recent missing data");
        out.insert(
            "captures".to_string(),
            serde_json::to_value(&data).unwrap(),
        );
    }
    if runs {
        let resp = client.list_runs().await?;
        let data = resp.data.expect("list_runs missing data");
        out.insert("runs".to_string(), serde_json::to_value(&data).unwrap());
    }
    if artifacts {
        let resp = client.list_artifacts(500).await?;
        let data = resp.data.expect("list_artifacts missing data");
        out.insert("artifacts".to_string(), serde_json::to_value(&data).unwrap());
    }
    if json {
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("{}", serde_json::to_string(&out)?);
    }
    Ok(())
}
