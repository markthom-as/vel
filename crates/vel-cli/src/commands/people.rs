//! vel people — list and inspect person records.

use crate::client::ApiClient;
use anyhow::Context;

pub async fn run_list(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client.list_people().await.context("list people")?;
    let people = resp.data.ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&people)?);
        return Ok(());
    }
    if people.is_empty() {
        println!("No people records.");
        return Ok(());
    }
    for p in &people {
        let name = p.given_name.as_deref().unwrap_or(&p.display_name);
        let last_contacted = p
            .last_contacted_at
            .map(|t| t.to_string())
            .unwrap_or_else(|| "—".to_string());
        println!("{}  {}  last_contact={}", p.id, name, last_contacted);
    }
    Ok(())
}

pub async fn run_inspect(client: &ApiClient, id: &str, json: bool) -> anyhow::Result<()> {
    let resp = client.get_person(id).await.context("get person")?;
    let p = resp.data.ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&p)?);
        return Ok(());
    }
    println!("id:               {}", p.id);
    println!("display_name:     {}", p.display_name);
    println!("given_name:       {}", p.given_name.as_deref().unwrap_or("—"));
    println!("family_name:      {}", p.family_name.as_deref().unwrap_or("—"));
    println!(
        "relationship:     {}",
        p.relationship_context.as_deref().unwrap_or("—")
    );
    println!("birthday:         {}", p.birthday.as_deref().unwrap_or("—"));
    let last = p
        .last_contacted_at
        .map(|t| t.to_string())
        .unwrap_or_else(|| "—".to_string());
    println!("last_contacted:   {}", last);
    if !p.aliases.is_empty() {
        println!("aliases:");
        for a in &p.aliases {
            println!("  {} ({}: {})", a.display, a.platform, a.handle);
        }
    }
    Ok(())
}
