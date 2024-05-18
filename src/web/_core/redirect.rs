//! This module implements redirection utilities.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect as AxumRedirect},
};

use super::Hx;

const HTMX_LOCATION_HEADER: &str = "HX-Location";

enum Type {
    SeeOther,
    // Temporary,
    // Permanent,
}

/// A redirect
pub struct Redirect {
    is_htmx_request: bool,
    ty: Type,
    to: String,
}

impl Redirect {
    /// Constructs a new 'see other' redirect.
    pub fn see_other(hx: Hx, to: &str) -> Self {
        Self {
            is_htmx_request: hx.request,
            ty: Type::SeeOther,
            to: to.to_owned(),
        }
    }

    // Constructs a new 'temporary' redirect.
    // pub fn temporary(HxRequest(is_htmx_request): HxRequest, to: &str) -> Self {
    //     Self {
    //         is_htmx_request,
    //         ty: Type::Temporary,
    //         to: to.to_owned(),
    //     }
    // }

    // Constructs a new 'permanent' redirect.
    // pub fn permanent(HxRequest(is_htmx_request): HxRequest, to: &str) -> Self {
    //     Self {
    //         is_htmx_request,
    //         ty: Type::Permanent,
    //         to: to.to_owned(),
    //     }
    // }
}

impl IntoResponse for Redirect {
    fn into_response(self) -> axum::response::Response {
        match self.is_htmx_request {
            // If the request is a HTMX request, return OK with HX-Location
            true => (StatusCode::OK, [(HTMX_LOCATION_HEADER, self.to)], ()).into_response(),
            // Otherwise, return a standard redirect
            false => {
                match self.ty {
                    Type::SeeOther => AxumRedirect::to(&self.to),
                    // Type::Temporary => StatusCode::TEMPORARY_REDIRECT,
                    // Type::Permanent => StatusCode::PERMANENT_REDIRECT,
                }
                .into_response()
            }
        }
    }
}
