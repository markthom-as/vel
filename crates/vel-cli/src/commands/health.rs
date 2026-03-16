use crate::client::ApiClient;

pub async fn run(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.health().await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let data = response.data.expect("health response missing data");
    println!("veld: {}", data.status);
    println!("db: {}", data.db);
    println!("version: {}", data.version);
    Ok(())
}
