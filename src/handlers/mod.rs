//! Handlers for the API endpoints.
//!
//! Public functions in this module and its submodules shuuld all be valid axum handlers, and should be documented with `#[utoipa::path]` to be included in the OpenAPI spec.
//!
//! Submodules are used to group related handlers together.
//!
//! They are logically related api endpoints, but are not enforced to match any particular rules, neither RESTful resources nor path prefix groups. Bussiness logic should go first.
//!
//! However, for the easy of maintenance and code readability, it's recommended to group them based on a clear rule, such as:
//! - RESTful resources: e.g. `users`, `posts`, `comments` shuld be in seperate modules.
//! - path prefix groups: e.g. `/foo/bar/baz` is mapped to `crate::handlers::foo::bar::baz`.
//! - bussiness logic: e.g. go with OpenAPI tags, so that handlers under module `foo` should all have `#[utoipa::path(tags("foo"))]`.
//!
//! The rules are not strict, after all the routes are explicitly registered in `src/routes.rs`.

use serde::Serialize;
use utoipa::ToSchema;

pub mod frontend;

/// Health check
///
/// Check if the server is healthy
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

#[derive(Serialize, ToSchema)]
struct VersionResponse {
    pub name: String,
    pub version: String,
}
impl Default for VersionResponse {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}
/// Get API backend version
///
/// Get the current version of the API backend program
#[utoipa::path(
    get,
    path = "/version", 
    params(),
    responses(
        (status=200,description="API version",content_type="application/json", body=VersionResponse),
        (status="default",description="error")
    )
)]
pub async fn version() -> impl axum::response::IntoResponse {
    axum::Json(VersionResponse::default())
}
