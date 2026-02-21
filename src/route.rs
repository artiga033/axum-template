use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::*;
use handlers as h;

#[derive(OpenApi)]
#[openapi()]
pub struct ApiDoc;

pub fn router() -> OpenApiRouter<crate::state::AppState> {
    let api_routes = {
        let mut api_routes = OpenApiRouter::new();
        api_routes = api_routes
            .routes(routes!(h::healthz))
            .routes(routes!(h::version));
        #[cfg(feature = "example")]
        {
            api_routes = api_routes
                .routes(routes!(h::todo::list_todos))
                .routes(routes!(h::todo::create_todo, h::todo::delete_todo))
                .routes(routes!(h::todo::mark_done, h::todo::mark_undone));
        }
        api_routes
    };

    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/", api_routes)
        .fallback(h::frontend::static_handler)
}
