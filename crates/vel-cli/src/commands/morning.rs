use crate::client::ApiClient;

pub async fn run(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.morning().await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let data = response.data.expect("morning response missing data");
    println!("date: {}", data.date);
    match data.suggested_focus {
        Some(focus) => println!("suggested focus: {}", focus),
        None => println!("suggested focus: none yet"),
    }
    println!("top active threads:");
    if data.top_active_threads.is_empty() {
        println!("  none yet");
    } else {
        for item in data.top_active_threads {
            println!("  - {}", item);
        }
    }
    println!("pending commitments:");
    if data.pending_commitments.is_empty() {
        println!("  none yet");
    } else {
        for item in data.pending_commitments {
            println!("  - {}", item);
        }
    }
    Ok(())
}
