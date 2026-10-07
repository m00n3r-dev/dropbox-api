use axum::{Router, routing::post};

use crate::{AppState, handlers::auth_handler::sign_up};

pub fn router() -> Router<AppState> {
    return Router::new().route("/sign-up", post(sign_up));
}
