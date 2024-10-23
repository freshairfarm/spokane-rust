use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use serde_json::json;

use crate::{repositories, schemas::*, AppState};

type ObjectResponse = Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)>;
type Response = (StatusCode, Json<serde_json::Value>);

fn not_found(id: i64) -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "status": "error",
            "message": format!("Object of id {} not found", id)
        }))
    )
}

pub async fn get_meetup_list(
    filter: Option<Query<PaginationOptions>>,
    State(state): State<Arc<AppState>>,
) -> ObjectResponse{
    let Query(filter) = filter.unwrap_or_default();

    let offset = (filter.offset.unwrap_or(1) - 1) * filter.limit.unwrap_or(1);

    let meetups: Vec<GetMeetup> = match repositories::get_meetups(
        &state.db,
        offset,
        filter.limit,
    )
        .await {
            Ok(meetups) => meetups.iter().map(|meetup| Into::<GetMeetup>::into(meetup)).collect(),
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Failed to fetch meetups: {}", e.to_string())
                    }))
                ))
            }
        };

    Ok(Json(json!({
        "status": "success",
        "count": meetups.len(),
        "data": meetups,
    })))
}

pub async fn get_meetup(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> ObjectResponse {
    let meetup = match repositories::get_meetup_by_id(&state.db, id)
        .await {
            Ok(meetup) => meetup,
            Err(sqlx::Error::RowNotFound) => return Err(not_found(id)),
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Failed to fetch meetup: {}", e.to_string())
                    }))
                ))
            }
        };

    Ok(Json(json!({
        "status": "success",
        "count": 1,
        "data": Into::<GetMeetup>::into(meetup),
    })))
}

pub async fn create_meetup(
    State(state): State<Arc<AppState>>,
    Json(create_meetup): Json<CreateMeetup>,
) -> ObjectResponse {
    match repositories::create_meetup(&state.db, create_meetup.title, create_meetup.body_text)
        .await {
            Ok(id) => {
                Ok(Json(json!({
                    "status": "success",
                    "count": 1,
                    "data": id,
                })))
            },
            Err(e) => {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Failed to create meetup: {}", e.to_string())
                    }))
                ))
            }
        }
}

pub async fn put_meetup(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
    Json(update): Json<UpdateMeetup>,
) -> ObjectResponse {
    let meetup = match repositories::get_meetup_by_id(&state.db, id)
        .await {
            Ok(meetup) => meetup,
            Err(sqlx::Error::RowNotFound) => {
                return Err(not_found(id));
            },
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Failed to fetch meetup: {}", e.to_string())
                    }))
                ))
            }
        };
    
    let title = update.title.unwrap_or(meetup.title);
    let body_text = update.body_text.unwrap_or(meetup.body_text);

    match repositories::update_meetup(&state.db, id, title, body_text)
        .await {
            Ok(_) => {
                Ok(Json(json!({
                    "status": "success",
                    "count": 1,
                    "data": id,
                })))
            },
            Err(e) => {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Failed to update meetup: {}", e.to_string())
                    }))
                ))
            }
        }
}

pub async fn delete_meetup(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> ObjectResponse {
    match repositories::delete_meetup(&state.db, id).await {
        Ok(_) => {
            Ok(Json(json!({
                "status": "success",
                "count": 1,
                "data": id,
            })))
        },
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to delete meetup: {}", e.to_string())
                }))
            ))
        }
    }
}