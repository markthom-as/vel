use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use veld_evals::{
    build_router_judge_from_models_dir, default_models_dir, load_fixture_sets, report_exit_code,
    run_fixture_sets, write_report, EvalRunOptions, JudgeExecutor,
};

#[derive(Parser)]
#[command(name = "veld-evals")]
#[command(about = "Run deterministic and judge-backed eval fixtures for Vel")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(long)]
        fixtures: PathBuf,
        #[arg(long)]
        report: Option<PathBuf>,
        #[arg(long, default_value_t = false)]
        fail_on_judge_regression: bool,
        #[arg(long)]
        models_dir: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Run {
            fixtures,
            report,
            fail_on_judge_regression,
            models_dir,
        } => {
            let fixture_sets = load_fixture_sets(&fixtures)?
                .into_iter()
                .map(|(_, set)| set)
                .collect::<Vec<_>>();
            let models_dir = models_dir.unwrap_or_else(default_models_dir);
            let judge = build_router_judge_from_models_dir(&models_dir)?;
            let options = EvalRunOptions {
                fail_on_judge_regression,
            };
            let judge_ref = judge.as_ref().map(|judge| judge as &dyn JudgeExecutor);
            let report_data =
                run_fixture_sets(fixture_sets.as_slice(), judge_ref, &options).await?;
            let json = serde_json::to_string_pretty(&report_data)?;
            println!("{json}");
            if let Some(report_path) = report {
                write_report(&report_path, &report_data)?;
            }
            std::process::exit(report_exit_code(&report_data, &options) as i32);
        }
    }
}
