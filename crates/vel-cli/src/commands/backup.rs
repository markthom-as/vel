//! `vel backup` — backup guidance.

pub async fn run() -> anyhow::Result<()> {
    println!("Backup: ensure the following are copied to a safe location:");
    println!("  - Database: var/data/vel.sqlite (or the path in your config)");
    println!("  - Artifacts: var/artifacts/ (or the artifact_root in your config)");
    println!();
    println!("Example:");
    println!("  mkdir -p backup/$(date +%Y-%m-%d)");
    println!("  cp var/data/vel.sqlite backup/$(date +%Y-%m-%d)/");
    println!("  cp -r var/artifacts backup/$(date +%Y-%m-%d)/");
    Ok(())
}
