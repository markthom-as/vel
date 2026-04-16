//! vel evaluate — explicit recompute-and-persist entrypoint.

use crate::client::ApiClient;
use anyhow::Context;

fn render_evaluate_summary(inferred_states: u32, nudges_created_or_updated: u32) -> String {
    format!(
        "inferred_states: {inferred_states}  nudges_created_or_updated: {nudges_created_or_updated}\n\
         deterministic replay: cargo run -p veld-evals -- run --fixtures crates/veld-evals/fixtures/sample-day-context.json --report /tmp/vel-eval-report.json"
    )
}

pub async fn run(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.evaluate().await.context("evaluate")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!(
        "{}",
        render_evaluate_summary(d.inferred_states, d.nudges_created_or_updated)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::render_evaluate_summary;

    #[test]
    fn evaluate_summary_points_to_deterministic_replay() {
        let rendered = render_evaluate_summary(2, 3);

        assert!(rendered.contains("inferred_states: 2"));
        assert!(rendered.contains("nudges_created_or_updated: 3"));
        assert!(rendered.contains("cargo run -p veld-evals -- run"));
        assert!(rendered.contains("crates/veld-evals/fixtures/sample-day-context.json"));
        assert!(rendered.contains("vel-eval-report.json"));
    }
}
