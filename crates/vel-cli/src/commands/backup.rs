use anyhow::Context;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};
use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_core::ArtifactStorageKind;
use vel_storage::{ArtifactRecord, Storage};

#[derive(Debug, Clone, Serialize)]
pub struct LocalBackupManifest {
    pub generated_at: i64,
    pub db_path: String,
    pub artifact_root: String,
    pub artifacts: Vec<LocalBackupManifestArtifact>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LocalBackupManifestArtifact {
    pub artifact_id: String,
    pub artifact_type: String,
    pub storage_kind: String,
    pub storage_uri: String,
    pub content_hash: Option<String>,
    pub size_bytes: Option<i64>,
    pub local_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LocalBackupVerification {
    pub checked_at: i64,
    pub artifact_root: String,
    pub manifest_artifact_count: usize,
    pub checked_artifact_count: usize,
    pub ok: bool,
    pub mismatches: Vec<LocalBackupMismatch>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LocalBackupMismatch {
    pub kind: &'static str,
    pub artifact_id: Option<String>,
    pub storage_uri: Option<String>,
    pub local_path: Option<String>,
    pub detail: String,
}

pub async fn run_guide(config: &AppConfig) -> anyhow::Result<()> {
    println!("Backup instructions (manual; Vel does not perform automated backup):");
    println!("  - Database: {}", config.db_path);
    println!("  - Artifacts: {}", config.artifact_root);
    println!();
    println!("Example:");
    println!("  mkdir -p backup/$(date +%Y-%m-%d)");
    println!("  cp \"{}\" backup/$(date +%Y-%m-%d)/", config.db_path);
    println!(
        "  cp -r \"{}\" backup/$(date +%Y-%m-%d)/",
        config.artifact_root
    );
    println!();
    println!("Local manifest workflow:");
    println!("  vel backup manifest create --output backup-manifest.json");
    println!("  vel backup manifest verify --manifest backup-manifest.json");
    Ok(())
}

pub async fn run_manifest_create(
    config: &AppConfig,
    output: Option<&str>,
    json: bool,
) -> anyhow::Result<()> {
    let manifest = build_local_manifest(config).await?;
    let rendered = serde_json::to_string_pretty(&manifest)?;

    if let Some(output_path) = output {
        fs::write(output_path, &rendered)
            .with_context(|| format!("failed to write manifest to {output_path}"))?;
    }

    if json || output.is_none() {
        println!("{rendered}");
    } else {
        println!("wrote local backup manifest to {}", output.unwrap());
        println!("artifacts: {}", manifest.artifacts.len());
    }

    Ok(())
}

pub async fn run_manifest_verify(
    config: &AppConfig,
    manifest_path: Option<&str>,
    json: bool,
) -> anyhow::Result<()> {
    let manifest = if let Some(path) = manifest_path {
        let contents =
            fs::read_to_string(path).with_context(|| format!("failed to read manifest {path}"))?;
        serde_json::from_str::<LocalBackupManifest>(&contents)
            .with_context(|| format!("failed to parse manifest {path}"))?
    } else {
        build_local_manifest(config).await?
    };

    let verification = verify_local_manifest(&manifest)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&verification)?);
        return Ok(());
    }

    println!("artifact_root: {}", verification.artifact_root);
    println!(
        "manifest_artifacts: {}",
        verification.manifest_artifact_count
    );
    println!("checked_artifacts: {}", verification.checked_artifact_count);
    println!("ok: {}", verification.ok);
    if verification.mismatches.is_empty() {
        println!("mismatches: none");
    } else {
        println!("mismatches:");
        for mismatch in &verification.mismatches {
            println!(
                "  - {} [{}] {}",
                mismatch.kind,
                mismatch
                    .artifact_id
                    .as_deref()
                    .or(mismatch.storage_uri.as_deref())
                    .unwrap_or("-"),
                mismatch.detail
            );
        }
    }

    Ok(())
}

async fn build_local_manifest(config: &AppConfig) -> anyhow::Result<LocalBackupManifest> {
    let storage = Storage::connect(&config.db_path).await?;
    let artifacts = storage.list_artifacts(10_000).await?;
    let artifact_root = PathBuf::from(&config.artifact_root);

    let artifacts = artifacts
        .into_iter()
        .map(|record| build_manifest_artifact(record, &artifact_root))
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(LocalBackupManifest {
        generated_at: OffsetDateTime::now_utc().unix_timestamp(),
        db_path: config.db_path.clone(),
        artifact_root: config.artifact_root.clone(),
        artifacts,
    })
}

fn build_manifest_artifact(
    record: ArtifactRecord,
    artifact_root: &Path,
) -> anyhow::Result<LocalBackupManifestArtifact> {
    let local_path = managed_artifact_path(artifact_root, &record);
    let content_hash = if let Some(existing) = record.content_hash.clone() {
        Some(existing)
    } else if let Some(path) = local_path.as_deref().filter(|path| path.exists()) {
        Some(hash_file(path)?)
    } else {
        None
    };

    let size_bytes = if let Some(size) = record.size_bytes {
        Some(size)
    } else if let Some(path) = local_path.as_deref().filter(|path| path.exists()) {
        Some(
            fs::metadata(path)
                .with_context(|| format!("failed to stat {}", path.display()))?
                .len() as i64,
        )
    } else {
        None
    };

    Ok(LocalBackupManifestArtifact {
        artifact_id: record.artifact_id.to_string(),
        artifact_type: record.artifact_type,
        storage_kind: record.storage_kind.to_string(),
        storage_uri: record.storage_uri,
        content_hash,
        size_bytes,
        local_path: local_path.map(|path| path.to_string_lossy().to_string()),
    })
}

fn verify_local_manifest(
    manifest: &LocalBackupManifest,
) -> anyhow::Result<LocalBackupVerification> {
    let artifact_root = PathBuf::from(&manifest.artifact_root);
    let mut mismatches = Vec::new();
    let mut checked_artifact_count = 0usize;
    let mut expected_files = BTreeSet::new();

    for artifact in &manifest.artifacts {
        let Some(local_path_str) = artifact.local_path.as_deref() else {
            continue;
        };

        checked_artifact_count += 1;
        let local_path = PathBuf::from(local_path_str);
        if let Ok(relative) = local_path.strip_prefix(&artifact_root) {
            expected_files.insert(relative.to_string_lossy().to_string());
        }

        match fs::metadata(&local_path) {
            Ok(metadata) => {
                if !metadata.is_file() {
                    mismatches.push(LocalBackupMismatch {
                        kind: "metadata_without_blob",
                        artifact_id: Some(artifact.artifact_id.clone()),
                        storage_uri: Some(artifact.storage_uri.clone()),
                        local_path: Some(local_path.display().to_string()),
                        detail: "managed artifact path is not a readable file".to_string(),
                    });
                    continue;
                }

                let observed_size = metadata.len() as i64;
                if let Some(expected_size) = artifact.size_bytes {
                    if observed_size != expected_size {
                        mismatches.push(LocalBackupMismatch {
                            kind: "size_mismatch",
                            artifact_id: Some(artifact.artifact_id.clone()),
                            storage_uri: Some(artifact.storage_uri.clone()),
                            local_path: Some(local_path.display().to_string()),
                            detail: format!(
                                "expected {expected_size} bytes but found {observed_size}"
                            ),
                        });
                    }
                }

                let observed_hash = hash_file(&local_path)?;
                if let Some(expected_hash) = artifact.content_hash.as_deref() {
                    if observed_hash != expected_hash {
                        mismatches.push(LocalBackupMismatch {
                            kind: "hash_mismatch",
                            artifact_id: Some(artifact.artifact_id.clone()),
                            storage_uri: Some(artifact.storage_uri.clone()),
                            local_path: Some(local_path.display().to_string()),
                            detail: format!("expected {expected_hash} but found {observed_hash}"),
                        });
                    }
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                mismatches.push(LocalBackupMismatch {
                    kind: "missing_file",
                    artifact_id: Some(artifact.artifact_id.clone()),
                    storage_uri: Some(artifact.storage_uri.clone()),
                    local_path: Some(local_path.display().to_string()),
                    detail: "manifest entry points to a missing local file".to_string(),
                });
            }
            Err(error) => {
                mismatches.push(LocalBackupMismatch {
                    kind: "unreadable_file",
                    artifact_id: Some(artifact.artifact_id.clone()),
                    storage_uri: Some(artifact.storage_uri.clone()),
                    local_path: Some(local_path.display().to_string()),
                    detail: error.to_string(),
                });
            }
        }
    }

    if artifact_root.exists() {
        for extra_file in list_relative_files(&artifact_root)? {
            if !expected_files.contains(&extra_file) {
                mismatches.push(LocalBackupMismatch {
                    kind: "blob_without_manifest_entry",
                    artifact_id: None,
                    storage_uri: Some(extra_file.clone()),
                    local_path: Some(artifact_root.join(&extra_file).display().to_string()),
                    detail: "local file exists under artifact_root but is not in the manifest"
                        .to_string(),
                });
            }
        }
    }

    Ok(LocalBackupVerification {
        checked_at: OffsetDateTime::now_utc().unix_timestamp(),
        artifact_root: manifest.artifact_root.clone(),
        manifest_artifact_count: manifest.artifacts.len(),
        checked_artifact_count,
        ok: mismatches.is_empty(),
        mismatches,
    })
}

fn managed_artifact_path(artifact_root: &Path, record: &ArtifactRecord) -> Option<PathBuf> {
    if record.storage_kind != ArtifactStorageKind::Managed {
        return None;
    }

    Some(artifact_root.join(&record.storage_uri))
}

fn hash_file(path: &Path) -> anyhow::Result<String> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let digest = Sha256::digest(bytes);
    Ok(format!("sha256:{}", hex::encode(digest)))
}

fn list_relative_files(root: &Path) -> anyhow::Result<Vec<String>> {
    let mut out = Vec::new();
    walk_relative_files(root, root, &mut out)?;
    out.sort();
    Ok(out)
}

fn walk_relative_files(root: &Path, current: &Path, out: &mut Vec<String>) -> anyhow::Result<()> {
    for entry in fs::read_dir(current)
        .with_context(|| format!("failed to read directory {}", current.display()))?
    {
        let entry =
            entry.with_context(|| format!("failed to read entry under {}", current.display()))?;
        let path = entry.path();
        if path.is_dir() {
            walk_relative_files(root, &path, out)?;
            continue;
        }
        if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .with_context(|| format!("failed to strip prefix {}", root.display()))?;
            out.push(relative.to_string_lossy().to_string());
        }
    }
    Ok(())
}

impl<'de> serde::Deserialize<'de> for LocalBackupManifest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawManifest {
            generated_at: i64,
            db_path: String,
            artifact_root: String,
            artifacts: Vec<LocalBackupManifestArtifact>,
        }

        let raw = RawManifest::deserialize(deserializer)?;
        Ok(Self {
            generated_at: raw.generated_at,
            db_path: raw.db_path,
            artifact_root: raw.artifact_root,
            artifacts: raw.artifacts,
        })
    }
}

impl<'de> serde::Deserialize<'de> for LocalBackupManifestArtifact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawArtifact {
            artifact_id: String,
            artifact_type: String,
            storage_kind: String,
            storage_uri: String,
            content_hash: Option<String>,
            size_bytes: Option<i64>,
            local_path: Option<String>,
        }

        let raw = RawArtifact::deserialize(deserializer)?;
        Ok(Self {
            artifact_id: raw.artifact_id,
            artifact_type: raw.artifact_type,
            storage_kind: raw.storage_kind,
            storage_uri: raw.storage_uri,
            content_hash: raw.content_hash,
            size_bytes: raw.size_bytes,
            local_path: raw.local_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_relative_files_walks_nested_tree() {
        let root = std::env::temp_dir().join(format!("vel-backup-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("nested")).unwrap();
        fs::write(root.join("a.txt"), "a").unwrap();
        fs::write(root.join("nested").join("b.txt"), "b").unwrap();

        let files = list_relative_files(&root).unwrap();
        assert_eq!(files, vec!["a.txt".to_string(), "nested/b.txt".to_string()]);

        fs::remove_dir_all(root).unwrap();
    }
}
