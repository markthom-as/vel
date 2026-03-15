use crate::client::ApiClient;

pub async fn run(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.evaluate().await.context("evaluate")?;
    let d = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("inferred_states: {}  nudges_created_or_updated: {}", d.inferred_states, d.nudges_created_or_updated);
    Ok(())
}
