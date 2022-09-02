#[macro_use] extern crate rocket;
use rocket_db_pools::{Database};
use rocket_db_pools::sqlx::{self};
use drug::{view::read, add::add};

mod drug;

#[derive(Database)]
#[database("postgrestest")]
pub struct Drug(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Drug::init())
        .mount("/drug", routes![read, add])
}
