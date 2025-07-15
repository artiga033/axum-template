use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::*;
use handlers as h;

#[derive(OpenApi)]
#[openapi()]
pub struct ApiDoc;

pub fn router<S: Send + Sync + Clone + 'static>() -> OpenApiRouter<S> {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/", OpenApiRouter::new().routes(routes!(h::healthz)))
        .fallback(h::frontend::static_handler)
}
