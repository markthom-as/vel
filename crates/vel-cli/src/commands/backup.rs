//! `vel backup` — print backup instructions (guidance only; not an automated backup).

use vel_config::AppConfig;

pub async fn run(config: &AppConfig) -> anyhow::Result<()> {
    println!("Backup instructions (manual; Vel does not perform automated backup):");
    println!("  - Database: {}", config.db_path);
    println!("  - Artifacts: {}", config.artifact_root);
    println!();
    println!("Example:");
    println!("  mkdir -p backup/$(date +%Y-%m-%d)");
    println!("  cp \"{}\" backup/$(date +%Y-%m-%d)/", config.db_path);
    println!("  cp -r \"{}\" backup/$(date +%Y-%m-%d)/", config.artifact_root);
    Ok(())
}
