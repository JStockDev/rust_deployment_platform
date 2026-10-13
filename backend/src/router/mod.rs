pub mod login;
pub mod oauth2_redirect;

use std::sync::Arc;
use axum::{Router, routing::get};

use crate::RouterState;

pub fn build_router(router_state: RouterState) -> Router {
    Router::new()
        .route("/api", get("Hello!"))
        .route("/api/auth/login", get(login::route))
        .route("/api/auth/redirect", get(oauth2_redirect::route))
        .with_state(Arc::new(router_state))
}
