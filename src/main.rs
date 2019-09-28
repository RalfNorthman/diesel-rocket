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
use diesel::result::Error as DieselError;
use rocket_contrib::databases::diesel::MysqlConnection;
use rocket_contrib::json::{Json, JsonError};
use rocket_cors::{CorsOptions, Error};

type JsonResult<'a, T> = Result<Json<T>, JsonError<'a>>;
type CustomResult<T> = Result<Json<T>, CustomError>;

#[derive(Debug, Responder)]
enum CustomError {
    #[response(status = 500, content_type = "plain")]
    DbError(String),
    #[response(status = 422, content_type = "plain")]
    ApiError(String),
}

impl<'a> From<JsonError<'a>> for CustomError {
    fn from(error: JsonError<'a>) -> Self {
        let s = format!("{:?}", error).replace("\\", "");
        CustomError::ApiError(s)
    }
}

impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> Self {
        let s = format!("{}", error);
        CustomError::DbError(s)
    }
}

trait Returnable<T, E, F> {
    fn out(self) -> Result<Json<T>, F>;
}

use serde::Serialize;

impl<T, E, F> Returnable<T, E, F> for Result<T, E>
where
    T: Serialize,
    F: From<E>,
{
    fn out(self) -> Result<Json<T>, F> {
        Ok(Json(self?))
    }
}

#[database("my_db")]
struct MyDatabase(MysqlConnection);

#[get("/measurements")]
fn all(conn: MyDatabase) -> CustomResult<Vec<Measurement>> {
    Measurement::all(&conn).out()
}

#[get("/measurements/<id>")]
fn id(conn: MyDatabase, id: u64) -> CustomResult<Measurement> {
    let v = Measurement::one(&conn, id)?;
    Ok(Json(v))
}

#[post("/measurements", format = "json", data = "<measurement>")]
fn create(
    conn: MyDatabase,
    measurement: JsonResult<NewMeasurement>,
) -> CustomResult<u64> {
    let insert = measurement?;
    let last_id = insert.create(&conn)?;
    Ok(Json(last_id))
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
