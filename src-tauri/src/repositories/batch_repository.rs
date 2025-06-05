use diesel::{
    prelude::*,
    SqliteConnection
};
use dotenvy::dotenv;

use core::panic;
use std::env;

use crate::{
    schema::tb_batch::dsl::*,
    models::batch::*
};

pub struct BatchRepository {
    connection:SqliteConnection
}

impl BatchRepository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Self { connection: connection }
    }

    pub fn insert(&mut self, object:&Batch) -> Result<Batch, diesel::result::Error> {
        diesel::insert_into(tb_batch)
        .values(object)
        .returning(Batch::as_returning())
        .get_result(&mut self.connection)
    }

    pub fn select_id(&mut self, id:&(i32,i32)) -> Result<Option<Batch>, diesel::result::Error> {
        tb_batch.select(Batch::as_select())
        .filter(batch_number.eq(id.0))
        .filter(batch_year.eq(id.1))
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn select_year(&mut self, year:i32) -> Result<Option<Vec<Batch>>, diesel::result::Error> {
        tb_batch.select(Batch::as_select())
        .filter(batch_year.eq(year))
        .get_results(&mut self.connection)
        .optional()
    }

    pub fn select_all(&mut self) -> Result<Option<Vec<Batch>>, diesel::result::Error> {
        tb_batch.select(Batch::as_select())
        .get_results(&mut self.connection)
        .optional()
    }

    pub fn update(&mut self, id:&(i32, i32), object:&Batch) -> Result<Option<Batch>,diesel::result::Error> {
        diesel::update(tb_batch
            .filter(batch_number.eq(id.0))
            .filter(batch_year.eq(id.1)))
        .set(object)
        .returning(Batch::as_returning())
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn drop(&mut self, id:&(i32,i32)) -> Result<Option<Batch>, diesel::result::Error>{
        diesel::delete(tb_batch
            .filter(batch_number.eq(id.0))
            .filter(batch_year.eq(id.1)))
        .get_result(&mut self.connection)
        .optional()
    }
}