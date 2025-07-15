use axum::{
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
};

#[derive(rust_embed::Embed)]
#[folder = "src/handlers/frontend"]
struct Site;

const INDEX_HTML: &str = "index.html";
pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Site::get(path) {
        Some(content) => {
            let mime = content.metadata.mimetype();

            ([(header::CONTENT_TYPE, mime)], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Site::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
