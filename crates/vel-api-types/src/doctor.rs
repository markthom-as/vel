use serde::{Deserialize, Serialize};

use crate::BackupTrustData;

/// Status of a single diagnostic check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticStatus {
    Ok,
    Warn,
    Fail,
}

/// A single diagnostic check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCheck {
    pub name: String,
    pub status: DiagnosticStatus,
    pub message: String,
}

/// Results of diagnostic checks for `vel doctor`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorData {
    pub checks: Vec<DiagnosticCheck>,
    pub backup: BackupTrustData,
    pub schema_version: u32,
    pub version: String,
}
