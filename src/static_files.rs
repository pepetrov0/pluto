//! Implements serving of static files

use std::time::Duration;

use axum::{
    http::Uri,
    response::{IntoResponse, Response},
    routing, Router,
};
use axum_extra::{
    headers::{CacheControl, ContentType},
    TypedHeader,
};
use rust_embed::RustEmbed;

use crate::{compression, errors::AppError, AppState};

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
                    TypedHeader(CacheControl::new().with_max_age(Duration::from_secs(3600))),
                    content.data,
                )
                    .into_response()
            }
            None => AppError::NotFound.into_response(),
        }
    }
}

async fn handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();
    StaticFile(path)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/*file", routing::get(handler))
        .layer(compression::default())
}
