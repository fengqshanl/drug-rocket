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

#[get("/drug")]
pub async fn read(mut db: Connection<Drug>) -> Json<Vec<Post>> {
    let mut arr: Vec<Post> = Vec::new();
    sqlx::query("SELECT * FROM drug")
    .fetch_all(&mut *db)
    .map_ok(|r| {
        for item in r {
            arr.push(Post { 
                id: item.try_get::<i32, &str>("id").ok(),
                name: item.try_get::<String, &str>("name").ok(), 
                age: item.try_get::<i32, &str>("age").ok()
            });
        };
    })
    .await
    .ok();
    Json(arr)
}