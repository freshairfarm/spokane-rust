use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use serde_json::json;

use crate::{AppState, models::*, schemas::*};

type OkResponse = Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)>;
type Response = (StatusCode, Json<serde_json::Value>);

pub async fn get_meetup_list(
    filter: Option<Query<FilterOptions>>,
    State(state): State<Arc<AppState>>,
) -> OkResponse{
    let Query(filter) = filter.unwrap_or_default();

    let limit = filter.limit.unwrap_or(10);
    let offset = (filter.offset.unwrap_or(1) - 1) * limit;

    let meetups = sqlx::query_as!(
        Meetup,
        r#"SELECT * FROM meetups ORDER BY meetup_id LIMIT $1 OFFSET $2"#,
        limit,
        offset,
    )
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(
                    {
                        "status": "error",
                        "message": format!("Failed to fetch meetups: {}", e.to_string())
                    }
                ))
            )
        })?;
    
    let meetups: Vec<GetMeetup> = meetups
        .iter()
        .map(|meetup| meetup.into())
        .collect();

    Ok(Json(json!(
        {
            "status": "success",
            "count": meetups.len(),
            "data": meetups
        })
    ))
}

pub async fn get_meetup(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> OkResponse {
    let meetup = sqlx::query_as!(
        Meetup,
        r#"SELECT * FROM meetups WHERE meetup_id = $1"#,
        id,
    )
        .fetch_one(&state.db)
        .await;

    match meetup {
        Ok(meetup) => {
            let meetup: GetMeetup = meetup.into();
            Ok(Json(json!(
                {
                    "status": "success",
                    "count": 1,
                    "data": meetup,
                }
            )))
        },
        Err(sqlx::Error::RowNotFound) => {
            Err((
                StatusCode::NOT_FOUND,
                Json(json!(
                    {
                        "status": "error",
                        "message": format!("Meetup with id {} not found", id)
                    }
                ))
            ))
        },
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(
                    {
                        "status": "error",
                        "message": format!("Failed to fetch meetup: {}", e.to_string())
                    }
                ))
            ))
        }
    }
}

pub async fn create_meetup(
    State(state): State<Arc<AppState>>,
    Json(create_meetup): Json<CreateMeetup>,
) -> Response {
    match sqlx::query_as!(
        Meetup,
        r#"INSERT INTO meetups (title, body_text) VALUES ($1, $2) RETURNING *"#,
        create_meetup.title,
        create_meetup.body_text,
    )
        .fetch_one(&state.db)
        .await {
            Ok(meetup) => {
                (
                    StatusCode::CREATED,
                    Json(json!(
                    {
                        "status": "success",
                        "count": 1,
                        "data": GetMeetup::from(meetup),
                    }
                )))
            },
            Err(e) => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(
                        {
                            "status": "error",
                            "message": format!("Failed to create meetup: {}", e.to_string())
                        }
                    ))
                )
            }
        }
}

pub async fn put_meetup(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
    Json(update): Json<UpdateMeetup>,
) -> OkResponse {
    let meetup = sqlx::query_as!(
        Meetup,
        r#"SELECT * FROM meetups WHERE meetup_id = $1"#,
        id,
    )
        .fetch_one(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!(
                    {
                        "status": "error",
                        "message": format!("Meetup with id {} not found", id)
                    }
                ))
            ),
            _ => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(
                        {
                            "status": "error",
                            "message": format!("Failed to fetch meetup: {}", e.to_string())
                        }
                    ))
                )
            }
        })?;
    
    let title = update.title.unwrap_or(meetup.title);
    let body_text = update.body_text.unwrap_or(meetup.body_text);

    match sqlx::query_as!(
        Meetup,
        r#"UPDATE meetups SET title = $1, body_text = $2 WHERE meetup_id = $3 RETURNING *"#,
        title,
        body_text,
        id,
    )
        .fetch_one(&state.db)
        .await {
            Ok(meetup) => {
                Ok(Json(json!(
                    {
                        "status": "success",
                        "count": 1,
                        "data": GetMeetup::from(meetup),
                    }
                )))
            },
            Err(e) => {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(
                        {
                            "status": "error",
                            "message": format!("Failed to update meetup: {}", e.to_string())
                        }
                    ))
                ))
            }
    }
}

pub async fn delete_meetup(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> OkResponse {
    match sqlx::query_as!(
        Meetup,
        r#"DELETE FROM meetups WHERE meetup_id = $1 RETURNING *"#,
        id,
    )
        .fetch_one(&state.db)
        .await {
            Ok(meetup) => {
                Ok(Json(json!({
                    "status": "success",
                    "count": 1,
                    "data": GetMeetup::from(meetup),
                })))
            },
            Err(sqlx::Error::RowNotFound) => {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(json!(
                        {
                            "status": "error",
                            "message": format!("Meetup with id {} not found", id)
                        }
                    ))
                ))
            },
            Err(e) => {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(
                        {
                            "status": "error",
                            "message": format!("Failed to delete meetup: {}", e.to_string())
                        }
                    ))
                ))
            }
        }
}