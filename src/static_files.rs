//! Implements serving of static files

use axum::{
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
    routing, Router,
};
use axum_extra::{
    headers::{CacheControl, ContentType},
    TypedHeader,
};
use rust_embed::RustEmbed;

use crate::AppState;

#[derive(RustEmbed)]
#[folder = "static"]
struct StaticFiles;

struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match StaticFiles::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();

                (
                    TypedHeader(ContentType::from(mime)),
                    #[cfg(debug_assertions)]
                    TypedHeader(CacheControl::new().with_no_cache().with_no_store()),
                    #[cfg(not(debug_assertions))]
                    TypedHeader(
                        CacheControl::new().with_max_age(std::time::Duration::from_secs(3600)),
                    ),
                    content.data,
                )
                    .into_response()
            }
            None => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

async fn handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();
    StaticFile(path)
}

pub fn router() -> Router<AppState> {
    Router::new().route("/*file", routing::get(handler))
}
