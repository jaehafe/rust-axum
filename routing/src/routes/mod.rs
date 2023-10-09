mod hello_world;
mod mirror_body_string;

use axum::{
  Router,
  routing::{get, post, patch},
  body::Body
};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;

pub fn create_routes() -> Router {
  Router::new().route("/", get(hello_world))
    .route("/mirror_body_string", post(mirror_body_string))
}