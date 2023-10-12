#![allow(unused)]

mod custom_json_extractor;

use axum::{
    extract::FromRef,
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use custom_json_extractor::custom_json_extractor;
use sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };
    Router::new()
        // .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .with_state(app_state)
}