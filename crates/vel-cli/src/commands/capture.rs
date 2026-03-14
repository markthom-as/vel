use crate::client::ApiClient;

pub async fn run(client: &ApiClient, text: String) -> anyhow::Result<()> {
    let response = client.capture(text).await?;
    let data = response.data.expect("capture response missing data");
    println!("capture_id: {}", data.capture_id);
    Ok(())
}

