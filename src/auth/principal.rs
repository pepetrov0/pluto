use axum::{
    async_trait, extract::FromRequestParts, http::request::Parts, response::Redirect,
    RequestPartsExt,
};

use crate::{
    core::database::ReadonlyRepository,
    domain::{self, sessions::Session, users::User},
    AppState,
};

pub struct NoAuthPrincipal;

#[derive(Debug)]
pub struct AuthPrincipal(pub User);

#[async_trait]
impl FromRequestParts<AppState> for NoAuthPrincipal {
    type Rejection = Redirect;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        AuthPrincipal::from_request_parts(parts, state)
            .await
            .err()
            .map(|_| Self)
            .ok_or_else(|| Redirect::to("/"))
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthPrincipal {
    type Rejection = Redirect;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = parts
            .extract_with_state::<Session, _>(state)
            .await
            .map(|v| v.usr)
            .map_err(|_| Redirect::to("/login"))?;

        let mut repository = ReadonlyRepository::from_pool(&state.database)
            .await
            .ok_or_else(|| Redirect::to("/login"))?;

        domain::users::find(&mut repository, &user)
            .await
            .ok()
            .flatten()
            .map(AuthPrincipal)
            .ok_or_else(|| Redirect::to("/login"))
    }
}
