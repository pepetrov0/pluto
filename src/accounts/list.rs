//! Implements accounts list page

use askama::Template;

use crate::{auth::principal::AuthPrincipal, templates::HtmlTemplate};

#[derive(Template, Debug, Clone)]
#[template(path = "accounts/list.html")]
pub struct AccountsListPage {
}

pub async fn handler(_: AuthPrincipal) -> HtmlTemplate<AccountsListPage> {
    HtmlTemplate(AccountsListPage {})
}