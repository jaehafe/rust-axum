use axum::{extract::{Path, Query}, Extension, Json, response::IntoResponse, http::StatusCode};
use chrono::{DateTime, FixedOffset};
use sea_orm::{DatabaseConnection, EntityTrait, Condition};
use serde::{Serialize, Deserialize};
use crate::database::tasks::{Entity as Tasks, self};

#[derive(Serialize)]
pub struct ResponseTask {
  id: i32,
  title: String,
  priority: Option<String>,
  description: Option<String>,
}

pub async fn get_tasks(
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

#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    priority: Option<String>,
}

pub async fn get_all_tasks(
  Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
  let tasks = Tasks::find()
    .all(&database)
    .await
    .map_err(|_error | StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(|db_task | ResponseTask { 
      id: db_task.id,
      title: db_task.title,
      priority: db_task.priority, 
      description: db_task.description,
    })
    .collect();
  
  Ok(Json(tasks))
}


