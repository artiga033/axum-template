pub mod frontend;

#[utoipa::path(
    get,
    path = "/healthz", 
    params(),
    responses(
        (status=204,description="healthy"),
        (status="default",description="error")
    )
)]
pub async fn healthz() -> impl axum::response::IntoResponse {
    axum::response::NoContent
}