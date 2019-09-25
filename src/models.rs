use super::schema::measurements;
use super::schema::measurements::dsl::*;
use diesel::mysql::types::Unsigned;
use diesel::prelude::*;
use diesel::sql_types::BigInt;

#[derive(Queryable, Debug, Serialize)]
pub struct Measurement {
    pub id: u64,
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub comment: String,
    pub my_ref: Option<u64>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "measurements"]
pub struct NewMeasurement<'a> {
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub comment: Option<&'a str>,
    pub my_ref: Option<u64>,
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

no_arg_sql_function!(last_insert_id, Unsigned<BigInt>);

fn last_id(conn: &MysqlConnection) -> u64 {
    diesel::select(last_insert_id).first(conn).unwrap()
}

impl NewMeasurement<'_> {
    pub fn create(&self, conn: &MysqlConnection) -> QueryResult<u64> {
        conn.transaction(|| {
            diesel::insert_into(measurements)
                .values(self)
                .execute(conn)?;
            Ok(last_id(conn))
        })
    }
}
