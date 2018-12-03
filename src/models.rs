use super::schema::measurements;
use super::schema::measurements::dsl::*;
use diesel::prelude::*;

#[derive(Queryable, Debug, Serialize)]
pub struct Measurement {
    pub id: u64,
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub comment: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "measurements"]
pub struct NewMeasurement {
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub comment: Option<String>,
}

impl Measurement {
    pub fn all(conn: &MysqlConnection) -> Vec<Measurement> {
        measurements.load::<Measurement>(conn).unwrap()
    }
}

impl NewMeasurement {
    pub fn create(&self, conn: &MysqlConnection) {

        diesel::insert_into(measurements)
            .values(self)
            .execute(conn)
            .expect("insert failed");
    }
}
