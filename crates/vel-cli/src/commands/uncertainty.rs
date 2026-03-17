use crate::client::ApiClient;
use anyhow::Context;

pub async fn run_list(client: &ApiClient, status: Option<&str>, json: bool) -> anyhow::Result<()> {
    let resp = client
        .list_uncertainty(status, Some(50))
        .await
        .context("list uncertainty records")?;
    let data = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(data)?);
        return Ok(());
    }
    if data.is_empty() {
        println!("No uncertainty records.");
        return Ok(());
    }
    for record in data {
        println!(
            "{}  {}  {}  {}  mode={}  subject={}",
            record.id,
            record.status,
            record.decision_kind,
            record.confidence_band,
            record.resolution_mode,
            record.subject_id.as_deref().unwrap_or("-")
        );
    }
    Ok(())
}

pub async fn run_inspect(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client
        .get_uncertainty(id)
        .await
        .context("get uncertainty record")?;
    let record = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("id:               {}", record.id);
    println!("status:           {}", record.status);
    println!("subject_type:     {}", record.subject_type);
    println!(
        "subject_id:       {}",
        record.subject_id.as_deref().unwrap_or("-")
    );
    println!("decision_kind:    {}", record.decision_kind);
    println!("confidence_band:  {}", record.confidence_band);
    println!("confidence_score: {:?}", record.confidence_score);
    println!("resolution_mode:  {}", record.resolution_mode);
    println!("created_at:       {}", record.created_at);
    println!("resolved_at:      {:?}", record.resolved_at);
    println!(
        "reasons:          {}",
        serde_json::to_string_pretty(&record.reasons)?
    );
    if let Some(missing) = &record.missing_evidence {
        println!(
            "missing_evidence: {}",
            serde_json::to_string_pretty(missing)?
        );
    }
    Ok(())
}

pub async fn run_resolve(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client
        .resolve_uncertainty(id)
        .await
        .context("resolve uncertainty record")?;
    let record = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("Resolved uncertainty {} (status: {})", record.id, record.status);
    Ok(())
}
