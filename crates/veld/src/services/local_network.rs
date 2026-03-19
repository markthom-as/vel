use std::net::Ipv4Addr;

use if_addrs::{get_if_addrs, IfAddr};
use vel_config::AppConfig;

pub(crate) fn discover_lan_base_url(config: &AppConfig) -> Option<String> {
    let host = select_private_ipv4(
        &get_if_addrs()
            .ok()?
            .into_iter()
            .map(|interface| interface.addr)
            .collect::<Vec<_>>(),
    )?;
    Some(build_base_url(config, host))
}

pub(crate) fn lan_broadcast_targets() -> Vec<Ipv4Addr> {
    let interfaces = match get_if_addrs() {
        Ok(interfaces) => interfaces,
        Err(error) => {
            tracing::debug!(error = %error, "failed to enumerate local interfaces for LAN discovery");
            return vec![Ipv4Addr::new(255, 255, 255, 255)];
        }
    };

    let mut targets = interfaces
        .into_iter()
        .filter_map(|interface| match interface.addr {
            IfAddr::V4(addr) if !addr.ip.is_loopback() && is_private_or_link_local(addr.ip) => addr
                .broadcast
                .or_else(|| Some(compute_broadcast(addr.ip, addr.netmask))),
            _ => None,
        })
        .collect::<Vec<_>>();
    targets.push(Ipv4Addr::new(255, 255, 255, 255));
    targets.sort();
    targets.dedup();
    targets
}

fn build_base_url(config: &AppConfig, host: Ipv4Addr) -> String {
    let scheme = reqwest::Url::parse(&config.base_url)
        .ok()
        .map(|url| url.scheme().to_string())
        .unwrap_or_else(|| "http".to_string());
    let port = reqwest::Url::parse(&config.base_url)
        .ok()
        .and_then(|url| url.port_or_known_default())
        .or_else(|| config.bind_addr.rsplit(':').next()?.parse().ok())
        .unwrap_or(4130);

    format!("{scheme}://{host}:{port}")
}

fn select_private_ipv4(addrs: &[IfAddr]) -> Option<Ipv4Addr> {
    addrs.iter().find_map(|addr| match addr {
        IfAddr::V4(addr) if !addr.ip.is_loopback() && is_private_or_link_local(addr.ip) => {
            Some(addr.ip)
        }
        _ => None,
    })
}

fn compute_broadcast(ip: Ipv4Addr, netmask: Ipv4Addr) -> Ipv4Addr {
    Ipv4Addr::from(u32::from(ip) | !u32::from(netmask))
}

fn is_private_or_link_local(ip: Ipv4Addr) -> bool {
    ip.is_private() || (ip.octets()[0] == 169 && ip.octets()[1] == 254)
}

#[cfg(test)]
mod tests {
    use super::*;
    use if_addrs::{IfAddr, Ifv4Addr};

    #[test]
    fn select_private_ipv4_prefers_non_loopback_private_addresses() {
        let addrs = vec![
            IfAddr::V4(Ifv4Addr {
                ip: Ipv4Addr::LOCALHOST,
                netmask: Ipv4Addr::new(255, 0, 0, 0),
                broadcast: Some(Ipv4Addr::new(127, 255, 255, 255)),
                prefixlen: 8,
            }),
            IfAddr::V4(Ifv4Addr {
                ip: Ipv4Addr::new(192, 168, 1, 22),
                netmask: Ipv4Addr::new(255, 255, 255, 0),
                broadcast: Some(Ipv4Addr::new(192, 168, 1, 255)),
                prefixlen: 24,
            }),
        ];

        assert_eq!(
            select_private_ipv4(&addrs),
            Some(Ipv4Addr::new(192, 168, 1, 22))
        );
    }

    #[test]
    fn compute_broadcast_uses_interface_netmask() {
        assert_eq!(
            compute_broadcast(
                Ipv4Addr::new(192, 168, 50, 21),
                Ipv4Addr::new(255, 255, 255, 0),
            ),
            Ipv4Addr::new(192, 168, 50, 255)
        );
    }

    #[test]
    fn build_base_url_uses_configured_scheme_and_port() {
        let mut config = AppConfig::default();
        config.base_url = "https://127.0.0.1:8443".to_string();

        assert_eq!(
            build_base_url(&config, Ipv4Addr::new(192, 168, 1, 22)),
            "https://192.168.1.22:8443"
        );
    }
}
