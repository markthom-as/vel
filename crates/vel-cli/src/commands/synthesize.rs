//! `vel synthesize` — run-backed synthesis (week, project). Placeholder until synthesis service is implemented.

use crate::client::ApiClient;

pub async fn run_week(_client: &ApiClient, _json: bool) -> anyhow::Result<()> {
    eprintln!("vel synthesize week: planned (run-backed weekly synthesis).");
    eprintln!("Use 'vel review week' for now.");
    Ok(())
}

pub async fn run_project(_client: &ApiClient, name: &str, _json: bool) -> anyhow::Result<()> {
    eprintln!("vel synthesize project {}: planned (run-backed project synthesis).", name);
    eprintln!("Use 'vel search {}' and 'vel review week' for now.", name);
    Ok(())
}
