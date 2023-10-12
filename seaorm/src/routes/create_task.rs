use axum::Extension;
use sea_orm::{
  DatabaseConnection,
  ActiveValue::Set, ActiveModelTrait
};

use crate::database::tasks;

pub async fn create_task(Extension(database): Extension<DatabaseConnection>) {
  let new_task = tasks::ActiveModel {
    priority: Set(Some("A".to_owned())),
    title: Set("new title".to_owned()),
    description: Set(Some("new task description".to_owned())),
    ..Default::default()
  };

  let result = new_task.save(&database).await.unwrap();
  dbg!(result);
}