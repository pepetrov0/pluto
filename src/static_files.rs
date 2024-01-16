use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

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
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND).into_response(),
        }
    }
}

pub async fn handler(uri: Uri) -> impl IntoResponse {
    let path = uri
        .path()
        .trim_start_matches('/')
        .trim_start_matches("static/")
        .to_string();
    StaticFile(path)
}
