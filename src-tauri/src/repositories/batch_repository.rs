use diesel::{prelude::*,SqliteConnection};
use diesel::sql_types::Text;
use dotenvy::dotenv;

use core::panic;
use std::env;

use crate::{schema::tb_batch::dsl::*,models::batch::*};

pub struct BatchRepository {
    connection:SqliteConnection
}

define_sql_function! {
    fn lower(x: Text) -> Text;
}

impl BatchRepository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Self { connection: connection }
    }

    pub fn create(&mut self, object:&Batch) -> Result<Batch, diesel::result::Error> {
        diesel::insert_into(tb_batch)
        .values(object)
        .returning(Batch::as_returning())
        .get_result(&mut self.connection)
    }

    pub fn read(&mut self, id:&(i32,i32)) -> Result<Option<Batch>, diesel::result::Error> {
        let (bn, by) = id;
        tb_batch.filter(
            batch_number.eq(bn)
            .and(batch_year.eq(by)))
        .select(Batch::as_select())
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn read_all(&mut self) -> Result<Vec<Batch>, diesel::result::Error> {
        tb_batch.select(Batch::as_select())
        .get_results(&mut self.connection)
    }

    pub fn update(&mut self, id:&(i32, i32), object:&Batch) -> Result<Option<Batch>,diesel::result::Error> {
        let (bn, by) = id;
        diesel::update(tb_batch.filter(
            batch_number.eq(bn)
            .and(batch_year.eq(by))))
        .set(object)
        .returning(Batch::as_returning())
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn delete(&mut self, id:&(i32,i32)) -> Result<Option<Batch>, diesel::result::Error>{
        let (bn, by) = id;
        diesel::delete(tb_batch.filter(
            batch_number.eq(bn)
            .and(batch_year.eq(by))))
        .get_result(&mut self.connection)
        .optional()
    }
}