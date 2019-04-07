#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

extern crate diesel_rocket;

use self::diesel_rocket::*;
use self::models::*;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use rocket_cors::{Error, CorsOptions};

#[database("my_db")]
struct MyDatabase(diesel::MysqlConnection);

#[get("/measurements")]
fn all(conn: MyDatabase) -> Json<Vec<Measurement>> {
    let v = Measurement::all(&conn);
    Json(v)
}

#[get("/measurements/<id>")]
fn id(conn: MyDatabase, id: u64) -> Json<Vec<Measurement>> {
    let m = Measurement::one(&conn, id);
    Json(m)
}

#[post("/measurements", format = "json", data = "<measurement>")]
fn create(conn: MyDatabase, measurement: Json<NewMeasurement>) {
    measurement.create(&conn);
}

fn main() -> Result<(), Error> {

    let cors = CorsOptions::default().to_cors()?; 

    rocket::ignite()
        .attach(MyDatabase::fairing())
        .mount("/", routes![all, create, id])
        .attach(cors)
        .launch();

    Ok(())
}
