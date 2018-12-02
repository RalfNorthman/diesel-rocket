#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

extern crate rocket_crud;

use self::diesel::prelude::*;
use self::rocket_crud::*;
use self::models::*;
use rocket::http::RawStr;
use rocket::Data;
use rocket_contrib::databases::diesel;

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
fn measure(conn: MyDatabase) -> String {
    let v = load_from_db(&conn);
    format!("{:?}", v)
}

fn load_from_db(conn: &diesel::MysqlConnection) -> Vec<Measurement> {
    Measurement::all(&conn)
}

fn main() {
    rocket::ignite()
        .attach(MyDatabase::fairing())
        .mount("/", routes![hello, fuel, measure])
        .launch();
}
