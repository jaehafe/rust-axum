use axum::{extract::Path, Extension, Json, response::IntoResponse, http::{StatusCode}};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;
use crate::database::tasks::Entity as Tasks;

#[derive(Serialize)]
pub struct ResponseTask {
  id: i32,
  title: String,
  priority: Option<String>,
  description: Option<String>,
}

pub async fn get_one_task(
  Path(task_id): Path<i32>, 
  Extension(database): Extension<DatabaseConnection>
) -> Result<Json<ResponseTask>, StatusCode> {
  let task = Tasks::find_by_id(task_id).one(&database)
    .await.unwrap();
  if let Some(task) = task {
    Ok(Json(ResponseTask {
      id: task.id,
      title: task.title,
      priority: task.priority,
      description: task.description,
    }))
  } else {
    Err(StatusCode::NOT_FOUND)
  }
}