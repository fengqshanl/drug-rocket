#[macro_use] extern crate rocket;
use rocket_db_pools::{Database};
use rocket_db_pools::sqlx::{self};
use drug::{add::read};

mod drug;

#[derive(Database)]
#[database("postgres")]
pub struct Drug(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Drug::init())
        .mount("/", routes![read])
}
