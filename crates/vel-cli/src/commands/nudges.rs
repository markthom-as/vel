use crate::client::ApiClient;

pub async fn run_list(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client.list_nudges().await.context("list nudges")?;
    let data = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(data)?);
        return Ok(());
    }
    if data.is_empty() {
        println!("No active nudges.");
        return Ok(());
    }
    for n in data {
        println!("{}  {}  {}  {}", n.nudge_id, n.nudge_type, n.level, n.message);
    }
    Ok(())
}

pub async fn run_done(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client.nudge_done(id).await.context("nudge done")?;
    let d = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("{}  resolved  {}", d.nudge_id, d.message);
    Ok(())
}

pub async fn run_snooze(client: &ApiClient, id: &str, minutes: u32) -> anyhow::Result<()> {
    let resp = client.nudge_snooze(id, minutes).await.context("nudge snooze")?;
    let d = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("{}  snoozed {}m  {}", d.nudge_id, minutes, d.message);
    Ok(())
}
