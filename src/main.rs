#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

use rocket_db_pools::{Database};
use rocket_db_pools::sqlx::{self};
use drug::{add::read};

mod drug;

#[derive(Database)]
#[database("postgrestest")]
pub struct Drug(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Drug::init())
        .mount("/", routes![read])
}
