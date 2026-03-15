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

pub async fn run_inspect(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client.get_nudge(id).await.context("get nudge")?;
    let d = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("nudge_id: {}", d.nudge_id);
    println!("nudge_type: {}", d.nudge_type);
    println!("level: {}", d.level);
    println!("state: {}", d.state);
    println!("message: {}", d.message);
    if let Some(ref c) = d.related_commitment_id {
        println!("related_commitment_id: {}", c);
    }
    if let Some(ts) = d.snoozed_until {
        println!("snoozed_until: {}", ts);
    }
    if let Some(ts) = d.resolved_at {
        println!("resolved_at: {}", ts);
    }
    println!("created_at: {}", d.created_at);
    Ok(())
}
