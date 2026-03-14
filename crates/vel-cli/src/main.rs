mod client;
mod commands;

use anyhow::Context;
use clap::{Parser, Subcommand};
use vel_config::AppConfig;

#[derive(Debug, Parser)]
#[command(name = "vel", about = "Vel operator shell")]
struct Cli {
    #[arg(long)]
    base_url: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Health {
        #[arg(long)]
        json: bool,
    },
    Capture {
        text: String,
    },
    Today {
        #[arg(long)]
        json: bool,
    },
    Morning {
        #[arg(long)]
        json: bool,
    },
    Search {
        query: String,
        #[arg(long)]
        capture_type: Option<String>,
        #[arg(long)]
        source_device: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        json: bool,
    },
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    Show {
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = AppConfig::load().context("loading config")?;
    let base_url = cli.base_url.unwrap_or_else(|| config.base_url.clone());
    let client = client::ApiClient::new(base_url);

    match cli.command {
        Command::Health { json } => commands::health::run(&client, json).await,
        Command::Capture { text } => commands::capture::run(&client, text).await,
        Command::Today { json } => commands::today::run(&client, json).await,
        Command::Morning { json } => commands::morning::run(&client, json).await,
        Command::Search {
            query,
            capture_type,
            source_device,
            limit,
            json,
        } => commands::search::run(&client, query, capture_type, source_device, limit, json).await,
        Command::Config { command } => match command {
            ConfigCommand::Show { json } => commands::config::run(&config, json),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_parses_capture() {
        let cli = Cli::try_parse_from(["vel", "capture", "remember lidar"]).unwrap();
        match cli.command {
            Command::Capture { text } => assert_eq!(text, "remember lidar"),
            _ => panic!("expected capture command"),
        }
    }

    #[test]
    fn cli_parses_search() {
        let cli = Cli::try_parse_from(["vel", "search", "--limit", "5", "lidar"]).unwrap();
        match cli.command {
            Command::Search { query, limit, .. } => {
                assert_eq!(query, "lidar");
                assert_eq!(limit, Some(5));
            }
            _ => panic!("expected search command"),
        }
    }

    #[test]
    fn cli_parses_today() {
        let cli = Cli::try_parse_from(["vel", "today", "--json"]).unwrap();
        match cli.command {
            Command::Today { json } => assert!(json),
            _ => panic!("expected today command"),
        }
    }

    #[test]
    fn cli_command_tree_builds() {
        Cli::command().debug_assert();
    }
}
