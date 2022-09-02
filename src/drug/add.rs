use rocket::{Rocket, Build, futures};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::{sqlx, Connection};
use futures::{future::TryFutureExt};
use sqlx::Row;
use crate::Drug;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Post {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    name: Option<String>,
    age: Option<i32>,
}

#[post("/drug", data = "<post>")]
pub async fn add(mut db: Connection<Drug>, post: Json<Post>) -> Result<_>  {
    sqlx::query("INSERT INTO drug (id, name, age) VALUES (?, ?, ?)")
        .bind(post.id)
        .bind(post.name)
        .bind(post.age)
        .execute(&mut *db)
        .await?;

    Ok("OK".to_string())
}