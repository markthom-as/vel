use crate::client::ApiClient;

pub async fn run(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.end_of_day().await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let data = response.data.expect("end-of-day response missing data");
    println!("date: {}", data.date);
    println!("what was done:");
    if data.what_was_done.is_empty() {
        println!("  nothing captured today");
    } else {
        for capture in data.what_was_done {
            println!("  - [{}] {}", capture.capture_type, capture.content_text);
        }
    }
    println!("what remains open:");
    if data.what_remains_open.is_empty() {
        println!("  none");
    } else {
        for item in data.what_remains_open {
            println!("  - {}", item);
        }
    }
    println!("what may matter tomorrow:");
    if data.what_may_matter_tomorrow.is_empty() {
        println!("  none");
    } else {
        for item in data.what_may_matter_tomorrow {
            println!("  - {}", item);
        }
    }
    Ok(())
}
