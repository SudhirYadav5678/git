use axum::routing::{get, Router};

use crate::handlres::root::{base, heath_check_route};

pub fn root() -> Router {
    Router::new()
        .route("/", get(base))
        .route("/heath", get(heath_check_route))
}
