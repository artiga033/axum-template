mod conv;
mod schema;

use axum::{Json, extract::State, response::IntoResponse};

/// List todo items
///
/// Lists all items at once. In this simple example, no pagination, ordering or filtering is implemented.
#[utoipa::path(
    get,
    path = "/todos",
    params(),
    responses(
        (status=200,description="list of todo items",content_type="application/json", body=Vec<schema::Todo>),
        (status="default",description="error")
    )
)]
pub async fn list_todos(State(repo): State<crate::repo::TodoRepo>) -> impl IntoResponse {
    let todos: Vec<_> = repo
        .list_todos()
        .await
        .unwrap_or_default()
        .into_iter()
        .map(schema::Todo::from_entity)
        .collect();
    Json(todos)
}

/// Create a todo item
///
/// Create a new todo item, `done` defaults to false.
#[utoipa::path(
    post,
    path = "/todos",
    request_body = schema::CreateTodo,
    responses(
        (status=201,description="created todo item",content_type="application/json", body=schema::Todo),
        (status="default",description="error")
    )
)]
pub async fn create_todo(
    State(repo): State<crate::repo::TodoRepo>,
    Json(payload): Json<schema::CreateTodo>,
) -> impl IntoResponse {
    let entity = payload.into_entity();
    match repo.create_todo(entity).await {
        Ok(todo) => (
            axum::http::StatusCode::CREATED,
            Json(Some(schema::Todo::from_entity(todo))),
        ),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

/// Delete a todo item
///
/// Delete a todo item by its ID.
#[utoipa::path(
    delete,
    path = "/todos/{id}",
    params(
        ("id" = i64, Path, description = "ID of the todo item to delete")
    ),
    responses(
        (status=204,description="todo item deleted"),
        (status="default",description="error")
    )
)]
pub async fn delete_todo(
    State(repo): State<crate::repo::TodoRepo>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> impl IntoResponse {
    match repo.delete_todo(id).await {
        Ok(_) => axum::http::StatusCode::NO_CONTENT,
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Mark a todo item as done
///
/// This operation is idempotent, marking an already done item as done will not cause error.
#[utoipa::path(
    put,
    path = "/todos/{id}/done",
    params(
        ("id" = i64, Path, description = "ID of the todo item to mark as done")
    ),
    responses(
        (status=204,description="todo item marked as done"),
        (status="default",description="error")
    )
)]
pub async fn mark_done(
    State(repo): State<crate::repo::TodoRepo>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> impl IntoResponse {
    match repo.mark_done(id).await {
        Ok(_) => axum::http::StatusCode::NO_CONTENT,
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Mark a todo item as undone
///
/// This operation is idempotent, marking an already undone item as undone will not cause error.
#[utoipa::path(
    delete,
    path = "/todos/{id}/done",
    params(
        ("id" = i64, Path, description = "ID of the todo item to mark as undone")
    ),
    responses(
        (status=204,description="todo item marked as undone"),
        (status="default",description="error")
    )
)]
pub async fn mark_undone(
    State(repo): State<crate::repo::TodoRepo>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> impl IntoResponse {
    match repo.mark_undone(id).await {
        Ok(_) => axum::http::StatusCode::NO_CONTENT,
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}
