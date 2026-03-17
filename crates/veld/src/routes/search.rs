use axum::{
    extract::{Query, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{ApiResponse, SearchQuery, SearchResults};
use vel_storage::SearchFilters;

use crate::{errors::AppError, state::AppState};

pub async fn search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<ApiResponse<SearchResults>>, AppError> {
    let search_text = query.q.trim();
    if search_text.is_empty() {
        return Err(AppError::bad_request("search query must not be empty"));
    }

    let results: Vec<vel_storage::SearchResult> = state
        .storage
        .search_captures(
            search_text,
            SearchFilters {
                capture_type: query.capture_type,
                source_device: query.source_device,
                limit: query.limit,
            },
        )
        .await?;

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SearchResults {
            results: results
                .into_iter()
                .map(vel_api_types::SearchResult::from)
                .collect(),
        },
        request_id,
    )))
}
