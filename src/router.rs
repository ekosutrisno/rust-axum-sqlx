use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

use crate::{
    handler::{
        create_note_handler, delete_note_handler, edit_note_handler, get_note_handler,
        note_list_handler,
    },
    model::ModelController,
};

#[allow(dead_code)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub fn todo_router(pool: &Pool<Postgres>) -> Router {
    let app_state = Arc::new(AppState { db: pool.clone() });

    Router::new()
        .route(
            "/api/notes",
            get(note_list_handler).post(create_note_handler),
        )
        .route(
            "/api/notes/:id",
            get(get_note_handler)
                .put(edit_note_handler)
                .delete(delete_note_handler),
        )
        .with_state(app_state)
}

pub fn new_todo_router(pool: &Pool<Postgres>) -> Router {
    let app_state = Arc::new(AppState { db: pool.clone() });

    Router::new()
        .route(
            "/api/notes",
            get(ModelController::note_list_handler).post(ModelController::create_note_handler),
        )
        .route(
            "/api/notes/:id",
            get(ModelController::get_note_handler)
                .put(ModelController::edit_note_handler)
                .delete(ModelController::delete_note_handler),
        )
        .with_state(app_state)
}
