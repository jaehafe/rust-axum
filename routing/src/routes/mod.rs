mod hello_world;

use axum::{Router, routing::{get, post, patch}, body::Body};
use hello_world::hello_world;

pub fn create_routes() -> Router {
  Router::new().route("/", get(hello_world))
}