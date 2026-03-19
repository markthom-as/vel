use std::process::Stdio;

use serde::Deserialize;
use tokio::{
    process::Command,
    time::{timeout, Duration},
};
use vel_config::AppConfig;

#[derive(Debug, Deserialize)]
struct TailscaleStatus {
    #[serde(rename = "Self")]
    self_node: Option<TailscaleSelfNode>,
}

#[derive(Debug, Deserialize)]
struct TailscaleSelfNode {
    #[serde(rename = "DNSName")]
    dns_name: Option<String>,
}

pub(crate) async fn discover_base_url(config: &AppConfig) -> Option<String> {
    let output = timeout(
        Duration::from_secs(2),
        Command::new("tailscale")
            .arg("status")
            .arg("--json")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output(),
    )
    .await
    .ok()?
    .ok()?;

    if !output.status.success() {
        return None;
    }

    let status = serde_json::from_slice::<TailscaleStatus>(&output.stdout).ok()?;
    let dns_name = status.self_node?.dns_name?;
    build_base_url(config, &dns_name)
}

fn build_base_url(config: &AppConfig, dns_name: &str) -> Option<String> {
    let host = dns_name.trim().trim_end_matches('.');
    if host.is_empty() {
        return None;
    }

    let scheme = reqwest::Url::parse(&config.base_url)
        .ok()
        .map(|url| url.scheme().to_string())
        .unwrap_or_else(|| "http".to_string());
    let port = reqwest::Url::parse(&config.base_url)
        .ok()
        .and_then(|url| url.port_or_known_default())
        .or_else(|| port_from_bind_addr(&config.bind_addr))
        .unwrap_or(4130);

    Some(format!("{scheme}://{host}:{port}"))
}

fn port_from_bind_addr(bind_addr: &str) -> Option<u16> {
    bind_addr.rsplit(':').next()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_base_url_uses_tailscale_dns_name() {
        let config = AppConfig::default();
        assert_eq!(
            build_base_url(&config, "vel-desktop.tailnet.ts.net."),
            Some("http://vel-desktop.tailnet.ts.net:4130".to_string())
        );
    }

    #[test]
    fn build_base_url_keeps_scheme_and_port_from_base_url() {
        let mut config = AppConfig::default();
        config.base_url = "https://127.0.0.1:8443".to_string();
        assert_eq!(
            build_base_url(&config, "vel-desktop.tailnet.ts.net."),
            Some("https://vel-desktop.tailnet.ts.net:8443".to_string())
        );
    }

    #[test]
    fn build_base_url_rejects_empty_dns_name() {
        let config = AppConfig::default();
        assert_eq!(build_base_url(&config, "."), None);
    }
}
