use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    handlers::auth_handler::{me, sign_in, sign_up},
};

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/sign-up", post(sign_up))
        .route("/sign-in", post(sign_in))
        .route("/me", get(me));
}
