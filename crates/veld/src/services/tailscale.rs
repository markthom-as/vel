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
    #[serde(rename = "Peer", default)]
    peers: std::collections::BTreeMap<String, TailscalePeerNode>,
}

#[derive(Debug, Deserialize)]
struct TailscaleSelfNode {
    #[serde(rename = "DNSName")]
    dns_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TailscalePeerNode {
    #[serde(rename = "DNSName")]
    dns_name: Option<String>,
    #[serde(rename = "HostName")]
    host_name: Option<String>,
    #[serde(rename = "OS")]
    os: Option<String>,
    #[serde(rename = "Online")]
    online: Option<bool>,
    #[serde(rename = "TailscaleIPs", default)]
    tailscale_ips: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TailscalePeer {
    pub dns_name: String,
    pub host_name: Option<String>,
    pub os: Option<String>,
    pub online: bool,
    pub tailscale_ips: Vec<String>,
    pub base_url: String,
}

pub(crate) async fn discover_base_url(config: &AppConfig) -> Option<String> {
    let status = status().await?;
    let dns_name = status.self_node?.dns_name?;
    build_base_url(config, &dns_name)
}

pub(crate) async fn discover_peers(config: &AppConfig) -> Vec<TailscalePeer> {
    let Some(status) = status().await else {
        return Vec::new();
    };
    let Some(self_dns_name) = status
        .self_node
        .as_ref()
        .and_then(|node| node.dns_name.as_deref())
        .map(normalize_dns_name)
    else {
        return Vec::new();
    };

    status
        .peers
        .into_values()
        .filter_map(|peer| {
            let dns_name = normalize_dns_name(peer.dns_name.as_deref()?);
            if dns_name.is_empty() || dns_name == self_dns_name {
                return None;
            }
            let base_url = build_base_url(config, &dns_name)?;
            Some(TailscalePeer {
                dns_name,
                host_name: peer
                    .host_name
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToString::to_string),
                os: peer
                    .os
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToString::to_string),
                online: peer.online.unwrap_or(false),
                tailscale_ips: peer.tailscale_ips,
                base_url,
            })
        })
        .collect()
}

async fn status() -> Option<TailscaleStatus> {
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

    serde_json::from_slice::<TailscaleStatus>(&output.stdout).ok()
}

fn build_base_url(config: &AppConfig, dns_name: &str) -> Option<String> {
    let host = normalize_dns_name(dns_name);
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

fn normalize_dns_name(dns_name: &str) -> String {
    dns_name.trim().trim_end_matches('.').to_string()
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

    #[test]
    fn normalize_dns_name_strips_trailing_dot() {
        assert_eq!(
            normalize_dns_name("joves-macbook-pro.tailnet.ts.net."),
            "joves-macbook-pro.tailnet.ts.net"
        );
    }

    #[test]
    fn discovers_peer_base_urls_from_status_payload() {
        let config = AppConfig::default();
        let status = serde_json::from_str::<TailscaleStatus>(
            r#"{
              "Self": { "DNSName": "corvid.tailnet.ts.net." },
              "Peer": {
                "peer_1": {
                  "DNSName": "joves-macbook-pro.tailnet.ts.net.",
                  "HostName": "joves-macbook-pro",
                  "OS": "macOS",
                  "Online": true,
                  "TailscaleIPs": ["100.106.75.48"]
                }
              }
            }"#,
        )
        .unwrap();

        let peers = status
            .peers
            .into_values()
            .filter_map(|peer| {
                let dns_name = normalize_dns_name(peer.dns_name.as_deref()?);
                let base_url = build_base_url(&config, &dns_name)?;
                Some(TailscalePeer {
                    dns_name,
                    host_name: peer.host_name,
                    os: peer.os,
                    online: peer.online.unwrap_or(false),
                    tailscale_ips: peer.tailscale_ips,
                    base_url,
                })
            })
            .collect::<Vec<_>>();

        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].dns_name, "joves-macbook-pro.tailnet.ts.net");
        assert_eq!(
            peers[0].base_url,
            "http://joves-macbook-pro.tailnet.ts.net:4130"
        );
        assert!(peers[0].online);
    }
}
