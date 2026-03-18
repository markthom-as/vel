use crate::client::ApiClient;

/// Note: agent launch protocol (ticket 006) is not yet active.
/// The /v1/connect/* routes currently return 404 via deny_undefined_route (SP2 Lane B).
/// These commands are stubs that will be wired to active backend routes in SP2.
///
/// For current worker/node registration information, use:
///   vel sync status   — shows registered workers via /v1/sync/cluster
pub async fn run_list_instances(_client: &ApiClient, _json: bool) -> anyhow::Result<()> {
    eprintln!(
        "Note: agent launch protocol is not yet active (ticket 006 — SP2 Lane B).\n\
         This command will list active connect instances once the protocol is implemented.\n\
         \n\
         To see currently registered workers, run:\n\
         \n\
           vel sync status\n\
         \n\
         This shows registered worker nodes via GET /v1/sync/cluster."
    );
    Ok(())
}

pub async fn run_inspect_instance(
    _client: &ApiClient,
    _id: &str,
    _json: bool,
) -> anyhow::Result<()> {
    eprintln!(
        "Note: agent launch protocol is not yet active (ticket 006 — SP2 Lane B).\n\
         Instance inspection will be available once the connect lifecycle protocol is implemented.\n\
         \n\
         To inspect a specific worker node, run:\n\
         \n\
           vel sync status\n\
         \n\
         This shows registered worker nodes and their capabilities via GET /v1/sync/cluster."
    );
    Ok(())
}
