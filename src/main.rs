#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

extern crate rocket_crud;

use self::rocket_crud::*;
use self::models::*;
use rocket::http::RawStr;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;

#[database("my_db")]
struct MyDatabase(diesel::MysqlConnection);

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello {}, from rocket!", name.as_str())
}

#[get("/fuel")]
fn fuel() -> &'static str {
    "Rusty rocket fueled by diesel!"
}

#[get("/measure")]
fn measure(conn: MyDatabase) -> Json<Vec<Measurement>> {
    let v = Measurement::all(&conn);
    Json(v)
}

fn main() {
    rocket::ignite()
        .attach(MyDatabase::fairing())
        .mount("/", routes![hello, fuel, measure])
        .launch();
}
