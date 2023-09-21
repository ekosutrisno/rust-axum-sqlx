use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

use crate::handler::{create_note_handler, health_checker_handler, note_list_handler};

#[allow(dead_code)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub fn create_router(pool: Pool<Postgres>) -> Router {
    let app_state = Arc::new(AppState { db: pool.clone() });

    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route(
            "/api/notes",
            get(note_list_handler).post(create_note_handler),
        )
        .with_state(app_state)
}
