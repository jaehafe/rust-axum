use axum::{response::{Response, IntoResponse}, http::StatusCode};

pub async fn returns_201() -> Response {
  (StatusCode::CREATED, "this is 201".to_owned()).into_response()
}