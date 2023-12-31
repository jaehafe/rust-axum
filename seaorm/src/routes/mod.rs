#![allow(unused)]

mod custom_json_extractor;
mod validate_with_serde;
mod create_task;
mod get_tasks;

use axum::{
    extract::FromRef,
    middleware,
    routing::{delete, get, patch, post, put},
    Router, Extension,
};
use custom_json_extractor::custom_json_extractor;
use sea_orm::DatabaseConnection;
use validate_with_serde::validate_with_serde;
use create_task::create_task;
use get_tasks::{get_tasks, get_all_tasks};
#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub async fn create_routes(database: DatabaseConnection) -> Router {
    // let app_state = AppState { database };
    Router::new()
        // .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/task/:task_id", get(get_tasks))
        .layer(Extension(database))
        // .with_state(app_state)
}