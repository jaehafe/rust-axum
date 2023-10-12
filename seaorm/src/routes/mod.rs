#![allow(unused)]

mod custom_json_extractor;
mod validate_with_serde;
mod create_task;
mod get_one_task;

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
use get_one_task::get_one_task;

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
        .route("/task/:task_id", get(get_one_task))
        .layer(Extension(database))
        // .with_state(app_state)
}