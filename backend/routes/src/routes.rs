use axum::Router;
use axum::routing::{get, post};
use crate::handlers::{add_food, get_food, AppState};

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/add_food", post(add_food))
        .route("/get_food/:id", get(get_food))
        .with_state(state)
}
