use vel_core::{PersonAlias, PersonRecord};
use vel_storage::StorageError;

use crate::{errors::AppError, state::AppState};

pub async fn list_people(state: &AppState) -> Result<Vec<PersonRecord>, AppError> {
    Ok(state.storage.list_people().await?)
}

pub async fn get_person(state: &AppState, id: &str) -> Result<Option<PersonRecord>, AppError> {
    Ok(state.storage.get_person(id).await?)
}

pub async fn upsert_person_alias(
    state: &AppState,
    person_id: &str,
    alias: PersonAlias,
) -> Result<PersonRecord, AppError> {
    if alias.platform.trim().is_empty() {
        return Err(AppError::bad_request(
            "person alias platform must not be empty",
        ));
    }
    if alias.handle.trim().is_empty() {
        return Err(AppError::bad_request(
            "person alias handle must not be empty",
        ));
    }

    let person = state
        .storage
        .get_person(person_id)
        .await?
        .ok_or_else(|| AppError::not_found("person not found"))?;
    state
        .storage
        .upsert_person_alias(&person.id, &alias)
        .await
        .map_err(map_people_storage_error)?;
    state
        .storage
        .get_person(person_id)
        .await?
        .ok_or_else(|| AppError::not_found("person not found after alias upsert"))
}

fn map_people_storage_error(error: StorageError) -> AppError {
    match error {
        StorageError::Validation(message) => AppError::bad_request(message),
        other => other.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;
    use tokio::sync::broadcast;
    use vel_config::AppConfig;
    use vel_core::{IntegrationSourceRef, PersonId};

    fn test_state(storage: vel_storage::Storage) -> AppState {
        let (broadcast_tx, _) = broadcast::channel(8);
        AppState::new(
            storage,
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        )
    }

    #[tokio::test]
    async fn people_service_lists_and_updates_aliases() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let person = storage
            .create_person(PersonRecord {
                id: PersonId::from("per_people_service".to_string()),
                display_name: "Annie Case".to_string(),
                given_name: Some("Annie".to_string()),
                family_name: Some("Case".to_string()),
                relationship_context: Some("accountant".to_string()),
                birthday: None,
                last_contacted_at: Some(OffsetDateTime::now_utc()),
                aliases: vec![],
                links: vec![],
            })
            .await
            .unwrap();
        let state = test_state(storage);

        let updated = upsert_person_alias(
            &state,
            person.id.as_ref(),
            PersonAlias {
                platform: "email".to_string(),
                handle: "annie@example.com".to_string(),
                display: "Annie".to_string(),
                source_ref: Some(IntegrationSourceRef {
                    family: vel_core::IntegrationFamily::Messaging,
                    provider_key: "gmail".to_string(),
                    connection_id: "icn_people".to_string().into(),
                    external_id: "msg_annie".to_string(),
                }),
            },
        )
        .await
        .unwrap();

        assert_eq!(list_people(&state).await.unwrap().len(), 1);
        assert_eq!(updated.aliases.len(), 1);
        assert_eq!(updated.aliases[0].platform, "email");
        assert_eq!(
            updated.aliases[0]
                .source_ref
                .as_ref()
                .map(|value| value.external_id.as_str()),
            Some("msg_annie")
        );
    }
}
