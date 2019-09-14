#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate diesel_rocket;

use self::diesel_rocket::*;
use self::models::*;
use diesel::prelude::*;
use rocket_contrib::databases::diesel::MysqlConnection;
use rocket_contrib::json::Json;
use rocket_cors::{CorsOptions, Error};

type RocketResult<T> = Json<Result<T, String>>;

fn out<T>(from: QueryResult<T>) -> RocketResult<T> {
    match from {
        Ok(payload) => Json(Ok(payload)),
        Err(error) => {
            let err = format!("{:?}", error);
            Json(Err(err))
        }
    }
}

#[database("my_db")]
struct MyDatabase(MysqlConnection);

#[get("/measurements")]
fn all(conn: MyDatabase) -> RocketResult<Vec<Measurement>> {
    let v = Measurement::all(&conn);
    out(v)
}

#[get("/measurements/<id>")]
fn id(conn: MyDatabase, id: u64) -> RocketResult<Measurement> {
    let v = Measurement::one(&conn, id);
    out(v)
}

#[post("/measurements", format = "json", data = "<measurement>")]
fn create(
    conn: MyDatabase,
    measurement: Json<NewMeasurement>,
) -> RocketResult<usize> {
    let v = measurement.create(&conn);
    out(v)
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
