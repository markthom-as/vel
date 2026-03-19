use crate::client::ApiClient;

fn render_today_text(data: &vel_api_types::TodayData) -> String {
    let mut lines = vec![
        format!("now: {}", data.date),
        "focus right now:".to_string(),
    ];

    if data.focus_candidates.is_empty() {
        lines.push("  none yet".to_string());
    } else {
        for item in &data.focus_candidates {
            lines.push(format!("  - {item}"));
        }
    }

    lines.push("inbox reminders:".to_string());
    if data.reminders.is_empty() {
        lines.push("  none yet".to_string());
    } else {
        for item in &data.reminders {
            lines.push(format!("  - {item}"));
        }
    }

    lines.push("recent capture context:".to_string());
    if data.recent_captures.is_empty() {
        lines.push("  none today".to_string());
    } else {
        for capture in &data.recent_captures {
            lines.push(format!(
                "  - [{}] {}",
                capture.capture_type, capture.content_text
            ));
        }
    }

    lines.join("\n")
}

pub async fn run(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.today().await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let data = response.data.expect("today response missing data");
    println!("{}", render_today_text(&data));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::render_today_text;
    use time::OffsetDateTime;
    use vel_api_types::{ContextCapture, TodayData};

    #[test]
    fn today_text_uses_now_and_inbox_framing() {
        let data = TodayData {
            date: "2026-03-19".to_string(),
            focus_candidates: vec!["Ship phase 17".to_string()],
            reminders: vec!["Review inbox".to_string()],
            recent_captures: vec![ContextCapture {
                capture_id: "cap_123".to_string().into(),
                content_text: "Need to revisit shell copy".to_string(),
                capture_type: "note".to_string(),
                occurred_at: OffsetDateTime::UNIX_EPOCH,
                source_device: Some("web".to_string()),
            }],
        };

        let rendered = render_today_text(&data);
        assert!(rendered.contains("now: 2026-03-19"));
        assert!(rendered.contains("focus right now:"));
        assert!(rendered.contains("inbox reminders:"));
        assert!(rendered.contains("recent capture context:"));
    }
}
