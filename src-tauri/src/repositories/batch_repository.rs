use diesel::{prelude::*,SqliteConnection};
use dotenvy::dotenv;

use core::panic;
use std::env;

use crate::schema::tb_batch::dsl::*;
use crate::models::batch::Batch;

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

    pub fn create(&mut self, object:&Batch) -> Batch {
        diesel::insert_into(tb_batch)
        .values(object)
        .returning(Batch::as_returning())
        .get_result(&mut self.connection)
        .expect("Error creating a new record in batch table")
    }

    pub fn read(&mut self, id:&(i32,i32)) -> Option<Batch> {
        let (bn, by) = id;
        match tb_batch
        .filter(
            batch_number.eq(bn)
            .and(batch_year.eq(by)))
        .select(Batch::as_select())
        .get_result(&mut self.connection)
        .optional() {
            Ok(option) => option,
            Err(e) => panic!("Error trying to read batch id={:?}\n{}", id, e)
        }
 
    }

    pub fn read_all(&mut self) -> Vec<Batch> {
        match tb_batch
        .select(Batch::as_select())
        .get_results(&mut self.connection) {
            Ok(result) => result,
            Err(e) => panic!("Error trying to read all batchs\n{}", e)
        }
    }

    pub fn update(&mut self, id:&(i32, i32), object:&Batch) -> Option<Batch> {
        let (bn, by) = id;
        match diesel::update(
            tb_batch.filter(
                batch_number.eq(bn)
            .and(batch_year.eq(by))))
        .set(object)
        .returning(Batch::as_returning())
        .get_result(&mut self.connection)
        .optional() {
            Ok(option) => option,
            Err(e) => panic!("Error trying to update batch id={:?}\n{}", id, e)
        }
    }

    pub fn delete(&mut self, id:&(i32,i32)) -> usize {
        let (bn, by) = id;
        diesel::delete(
            tb_batch.filter(
                batch_number.eq(bn)
            .and(batch_year.eq(by))))
        .execute(&mut self.connection)
        .expect("Error deleting record in batch table")
    }
}