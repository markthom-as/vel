//! Background job processing. Claims pending jobs from storage and completes them.
//! For v0, capture_ingest jobs are no-op: we just mark them succeeded so the pipeline exists.

use std::time::Duration;
use tracing::{debug, warn};
use vel_storage::Storage;

const POLL_INTERVAL: Duration = Duration::from_secs(5);
const JOB_TYPE_CAPTURE_INGEST: &str = "capture_ingest";

pub async fn run_ingestion_worker(storage: Storage) {
    loop {
        if let Err(e) = poll_once(&storage).await {
            warn!(error = %e, "ingestion worker poll failed");
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

async fn poll_once(storage: &Storage) -> Result<(), vel_storage::StorageError> {
    let Some(job) = storage.claim_next_pending_job(JOB_TYPE_CAPTURE_INGEST).await? else {
        return Ok(());
    };

    debug!(job_id = %job.job_id, "processing capture_ingest job");

    // v0: no actual processing (e.g. transcription, extraction). Just mark succeeded.
    // Later: parse payload_json for capture_id, run pipeline, then mark succeeded or failed.
    storage.mark_job_succeeded(&job.job_id.to_string()).await?;
    Ok(())
}
