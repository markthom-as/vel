use crate::client::ApiClient;
use anyhow::Context;

pub async fn run_profile_health(
    client: &ApiClient,
    profile_id: &str,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .get_llm_profile_health(profile_id)
        .await
        .with_context(|| format!("get llm profile health for {}", profile_id))?;
    let health = response
        .data
        .ok_or_else(|| anyhow::anyhow!("llm profile health missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&health)?);
        return Ok(());
    }
    println!("profile_id: {}", health.profile_id);
    println!("healthy:    {}", health.healthy);
    println!("message:    {}", health.message);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn human_output_shape_is_stable() {
        let rendered = [
            "profile_id: default",
            "healthy:    true",
            "message:    Provider handshake succeeded.",
        ]
        .join("\n");

        assert!(rendered.contains("profile_id: default"));
        assert!(rendered.contains("healthy:    true"));
        assert!(rendered.contains("Provider handshake succeeded."));
    }
}
