use crate::client::ApiClient;
use anyhow::Context;

pub async fn run_calendar(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_calendar().await.context("sync calendar")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("calendar: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_todoist(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_todoist().await.context("sync todoist")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("todoist: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_activity(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_activity().await.context("sync activity")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("activity: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_git(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_git().await.context("sync git")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("git: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_notes(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_notes().await.context("sync notes")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("notes: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_transcripts(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client
        .sync_transcripts()
        .await
        .context("sync transcripts")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("transcripts: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_messaging(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client
        .sync_messaging()
        .await
        .context("sync messaging")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("messaging: {} signals ingested", d.signals_ingested);
    Ok(())
}
