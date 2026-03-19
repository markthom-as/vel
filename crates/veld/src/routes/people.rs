use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{ApiResponse, PersonAliasUpsertRequestData, PersonRecordData};
use vel_core::PersonAlias;

use crate::{errors::AppError, services, state::AppState};

/// GET /v1/people — operator-authenticated typed people registry.
pub async fn list_people(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<PersonRecordData>>>, AppError> {
    let people = services::people::list_people(&state).await?;
    Ok(Json(ApiResponse::success(
        people.into_iter().map(PersonRecordData::from).collect(),
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

/// GET /v1/people/:id — inspect one practical people record with aliases.
pub async fn get_person(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<PersonRecordData>>, AppError> {
    let person = services::people::get_person(&state, id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("person not found"))?;
    Ok(Json(ApiResponse::success(
        PersonRecordData::from(person),
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

/// POST /v1/people/:id/aliases — add or refresh alias-driven linkage for a person.
pub async fn upsert_person_alias(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<PersonAliasUpsertRequestData>,
) -> Result<Json<ApiResponse<PersonRecordData>>, AppError> {
    let person = services::people::upsert_person_alias(
        &state,
        id.trim(),
        PersonAlias {
            platform: payload.platform,
            handle: payload.handle,
            display: payload.display.unwrap_or_default(),
            source_ref: payload.source_ref.map(Into::into),
        },
    )
    .await?;

    Ok(Json(ApiResponse::success(
        PersonRecordData::from(person),
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::broadcast;
    use vel_config::AppConfig;
    use vel_core::{PersonId, PersonRecord};

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
    async fn people_routes_list_and_alias_updates() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .create_person(PersonRecord {
                id: PersonId::from("per_route_people".to_string()),
                display_name: "Annie Case".to_string(),
                given_name: Some("Annie".to_string()),
                family_name: Some("Case".to_string()),
                relationship_context: Some("accountant".to_string()),
                birthday: None,
                last_contacted_at: None,
                aliases: vec![],
                links: vec![],
            })
            .await
            .unwrap();
        let state = test_state(storage);

        let Json(list_response) = list_people(State(state.clone())).await.unwrap();
        assert_eq!(list_response.data.unwrap().len(), 1);

        let Json(person_response) = upsert_person_alias(
            State(state),
            Path("per_route_people".to_string()),
            Json(PersonAliasUpsertRequestData {
                platform: "email".to_string(),
                handle: "annie@example.com".to_string(),
                display: Some("Annie".to_string()),
                source_ref: None,
            }),
        )
        .await
        .unwrap();
        assert_eq!(person_response.data.unwrap().aliases.len(), 1);
    }
}
