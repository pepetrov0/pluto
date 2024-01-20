//! Implements rendering of askama templates

use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::{headers::ContentType, TypedHeader};

/// Represents a HTML template
pub struct HtmlTemplate<T: Template>(pub T);

impl<T: Template> IntoResponse for HtmlTemplate<T> {
    fn into_response(self) -> axum::response::Response {
        let mut html = match self.0.render().ok() {
            Some(v) => v.into_bytes(),
            None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if minify_html_onepass::truncate(
            &mut html,
            &minify_html_onepass::Cfg {
                minify_js: true,
                minify_css: true,
            },
        )
        .is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        (TypedHeader(ContentType::html()), html).into_response()
    }
}

/// A struct for configuration navigation
#[derive(Debug, Clone, Default)]
pub struct Navigation {
    pub no_logout: bool
}