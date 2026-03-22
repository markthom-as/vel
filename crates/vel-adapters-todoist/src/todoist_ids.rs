use sha2::{Digest, Sha256};
use vel_core::{IntegrationAccountId, SyncLinkId};

pub const TODOIST_PROVIDER: &str = "todoist";
pub const TODOIST_TASK_REMOTE_TYPE: &str = "task";
pub const TODOIST_MODULE_ID: &str = "module.integration.todoist";

pub fn todoist_integration_account_id(external_account_ref: &str) -> IntegrationAccountId {
    IntegrationAccountId::from(prefixed_hash_id(
        "integration_account",
        &format!("{TODOIST_PROVIDER}:account:{external_account_ref}"),
    ))
}

pub fn todoist_sync_link_id(
    integration_account_id: &str,
    remote_type: &str,
    remote_id: &str,
) -> SyncLinkId {
    SyncLinkId::from(prefixed_hash_id(
        "sync_link",
        &format!("{TODOIST_PROVIDER}:{integration_account_id}:{remote_type}:{remote_id}"),
    ))
}

pub fn todoist_provider_object_ref(remote_type: &str, remote_id: &str) -> String {
    format!("{TODOIST_PROVIDER}:{remote_type}:{remote_id}")
}

fn prefixed_hash_id(prefix: &str, source: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(source.as_bytes());
    let digest = hasher.finalize();
    format!("{prefix}_{}", hex::encode(&digest[..12]))
}

#[cfg(test)]
mod tests {
    use super::{
        todoist_integration_account_id, todoist_provider_object_ref, todoist_sync_link_id,
    };

    #[test]
    fn todoist_ids_are_stable_and_prefixed() {
        let account_id = todoist_integration_account_id("acct_primary");
        let link_id = todoist_sync_link_id(account_id.as_ref(), "task", "todo_123");

        assert!(account_id.as_ref().starts_with("integration_account_"));
        assert!(link_id.as_ref().starts_with("sync_link_"));
        assert_eq!(
            todoist_provider_object_ref("task", "todo_123"),
            "todoist:task:todo_123"
        );
        assert_eq!(
            account_id,
            todoist_integration_account_id("acct_primary"),
            "account ids should be deterministic for the same external ref"
        );
    }
}
