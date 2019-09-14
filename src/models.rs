use super::schema::measurements;
use super::schema::measurements::dsl::*;
use diesel::prelude::*;

#[derive(Queryable, Debug, Serialize)]
pub struct Measurement {
    pub id: u64,
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub comment: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "measurements"]
pub struct NewMeasurement<'a> {
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub comment: Option<&'a str>,
}

impl Measurement {
    pub fn all(
        conn: &MysqlConnection,
    ) -> QueryResult<Vec<Measurement>> {
        measurements.load::<Measurement>(conn)
    }

    pub fn one(
        conn: &MysqlConnection,
        id_arg: u64,
    ) -> QueryResult<Measurement> {
        measurements.find(id_arg).first(conn)
    }
}

impl NewMeasurement<'_> {
    pub fn create(
        &self,
        conn: &MysqlConnection,
    ) -> QueryResult<usize> {
        diesel::insert_into(measurements).values(self).execute(conn)
    }
}
