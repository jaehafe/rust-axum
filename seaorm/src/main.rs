#![allow(unused)]

use dotenvy::dotenv;
use sea_orm::{DatabaseConnection, Database};
use dotenvy_macro::dotenv;
use seaorm::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    run(database_uri).await;
}
