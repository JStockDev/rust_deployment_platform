use std::sync::Arc;

use axum::{Router, routing::get};

use crate::RouterState;

pub fn build_router(router_state: RouterState) -> Router {
    Router::new()
        .route("/", get("Hello!"))
        .with_state(Arc::new(router_state))
}
