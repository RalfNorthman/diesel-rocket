pub mod models;
pub mod schema;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate dotenv;

use self::models::{Measurement, NewMeasurement};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

