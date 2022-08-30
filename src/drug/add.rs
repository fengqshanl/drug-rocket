use rocket_db_pools::{Connection};
use rocket_db_pools::sqlx::{self, Row};
use crate::Drug;

#[get("/")]
pub async fn read(mut db: Connection<Drug>) -> Option<String> {
   sqlx::query("\\l")
       .fetch_one(&mut *db).await
       .and_then(|r| {
            // println!("{:?}; {:?}",r.try_get(0)?, r.columns());
            Ok(r.try_get(0)?)
        })
       .ok()
}