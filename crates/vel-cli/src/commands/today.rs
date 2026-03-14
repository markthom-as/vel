use crate::client::ApiClient;

pub async fn run(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.today().await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let data = response.data.expect("today response missing data");
    println!("date: {}", data.date);
    println!("focus:");
    if data.focus_candidates.is_empty() {
        println!("  none yet");
    } else {
        for item in data.focus_candidates {
            println!("  - {}", item);
        }
    }
    println!("reminders:");
    if data.reminders.is_empty() {
        println!("  none yet");
    } else {
        for item in data.reminders {
            println!("  - {}", item);
        }
    }
    println!("recent captures:");
    if data.recent_captures.is_empty() {
        println!("  none today");
    } else {
        for capture in data.recent_captures {
            println!("  - [{}] {}", capture.capture_type, capture.content_text);
        }
    }

    Ok(())
}
