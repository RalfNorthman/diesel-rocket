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
use rocket_contrib::json::{Json, JsonError};
use rocket_cors::{CorsOptions, Error};
use std::fmt::Display;

type RocketResult<T> = Json<Result<T, String>>;

fn out<T>(from: QueryResult<T>) -> RocketResult<T> {
    match from {
        Ok(payload) => Json(Ok(payload)),
        Err(error) => {
            let err = format!("{}", error);
            Json(Err(err))
        }
    }
}

fn stringify(x: impl Display) -> String {
    format!("{}", x)
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
    measurement: Result<Json<NewMeasurement>, JsonError>,
) -> RocketResult<u64> {
    let result = match measurement {
        Ok(json) => {
            json.into_inner().create(&conn).map_err(stringify)
        }
        Err(error) => Err(format!("{:?}", error)),
    };
    Json(result)
}

#[post("/two-measurements", format = "json", data = "<measurements>")]
fn create_two(
    conn: MyDatabase,
    measurements: Json<[NewMeasurement; 2]>,
) -> RocketResult<(u64, u64)> {
    let v = db_create_two(&conn, measurements.into_inner());
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
