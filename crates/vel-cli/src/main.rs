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
    Doctor {
        #[arg(long)]
        json: bool,
    },
    Health {
        #[arg(long)]
        json: bool,
    },
    Capture {
        text: Option<String>,
        #[arg(long)]
        stdin: bool,
        #[arg(long)]
        r#type: Option<String>,
        #[arg(long)]
        source: Option<String>,
    },
    Recent {
        #[arg(long, default_value = "20")]
        limit: u32,
        #[arg(long)]
        today: bool,
        #[arg(long)]
        json: bool,
    },
    Today {
        #[arg(long)]
        json: bool,
    },
    Morning {
        #[arg(long)]
        json: bool,
    },
    EndOfDay {
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
    Inspect {
        #[command(subcommand)]
        command: InspectCommand,
    },
    Run {
        #[command(subcommand)]
        command: RunCommand,
    },
    Review {
        #[command(subcommand)]
        command: ReviewCommand,
    },
    Artifact {
        #[command(subcommand)]
        command: ArtifactCommand,
    },
    Import {
        #[command(subcommand)]
        command: ImportCommand,
    },
    Export {
        #[arg(long)]
        captures: bool,
        #[arg(long)]
        runs: bool,
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long)]
        json: bool,
    },
    Backup {},
    Synthesize {
        #[command(subcommand)]
        command: SynthesizeCommand,
    },
}

#[derive(Debug, Subcommand)]
enum InspectCommand {
    Capture { id: String },
    Artifact { id: String },
}

#[derive(Debug, Subcommand)]
enum RunCommand {
    List {
        #[arg(long)]
        json: bool,
    },
    Inspect {
        id: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    Show {
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum ReviewCommand {
    Today { #[arg(long)] json: bool },
    Week { #[arg(long)] json: bool },
}

#[derive(Debug, Subcommand)]
enum ArtifactCommand {
    Latest {
        #[arg(long)]
        r#type: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum ImportCommand {
    File {
        path: String,
        #[arg(long, default_value = "note")]
        r#type: String,
    },
    Lines {
        #[arg(long, default_value = "note")]
        r#type: String,
    },
    #[command(name = "url")]
    CaptureUrl { url: String },
}

#[derive(Debug, Subcommand)]
enum SynthesizeCommand {
    Week { #[arg(long)] json: bool },
    Project { name: String, #[arg(long)] json: bool },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = AppConfig::load().context("loading config")?;
    let base_url = cli.base_url.unwrap_or_else(|| config.base_url.clone());
    let client = client::ApiClient::new(base_url);

    match cli.command {
        Command::Doctor { json } => commands::doctor::run(&client, json).await,
        Command::Health { json } => commands::health::run(&client, json).await,
        Command::Capture {
            text,
            stdin,
            r#type: capture_type,
            source,
        } => commands::capture::run(&client, text, *stdin, capture_type.clone(), source.clone()).await,
        Command::Recent { limit, today, json } => commands::recent::run(&client, *limit, *today, *json).await,
        Command::Today { json } => commands::today::run(&client, json).await,
        Command::Morning { json } => commands::morning::run(&client, json).await,
        Command::EndOfDay { json } => commands::end_of_day::run(&client, json).await,
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
        Command::Inspect { command } => match command {
            InspectCommand::Capture { id } => commands::inspect::run_capture(&client, &id).await,
            InspectCommand::Artifact { id } => commands::inspect::run_artifact(&client, &id).await,
        },
        Command::Run { command } => match command {
            RunCommand::List { json } => commands::runs::run_list(&client, json).await,
            RunCommand::Inspect { id, json } => commands::runs::run_inspect(&client, &id, json).await,
        },
        Command::Review { command } => match command {
            ReviewCommand::Today { json } => commands::review::run_today(&client, json).await,
            ReviewCommand::Week { json } => commands::review::run_week(&client, json).await,
        },
        Command::Artifact { command } => match command {
            ArtifactCommand::Latest { r#type: t, json } => commands::artifact::run_latest(&client, &t, json).await,
        },
        Command::Import { command } => match command {
            ImportCommand::File { path, r#type: t } => commands::import_::run_file(&client, &path, &t).await,
            ImportCommand::Lines { r#type: t } => commands::import_::run_lines(&client, &t).await,
            ImportCommand::CaptureUrl { url } => commands::import_::run_capture_url(&client, &url).await,
        },
        Command::Export { captures, runs, format, json } => commands::export_::run(&client, *captures, *runs, format, *json).await,
        Command::Backup {} => commands::backup::run().await,
        Command::Synthesize { command } => match command {
            SynthesizeCommand::Week { json } => commands::synthesize::run_week(&client, *json).await,
            SynthesizeCommand::Project { name, json } => commands::synthesize::run_project(&client, &name, *json).await,
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
            Command::Capture { text, .. } => assert_eq!(text, Some("remember lidar".to_string())),
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
