use diesel::{prelude::*,SqliteConnection};
use dotenvy::dotenv;

use core::panic;
use std::env;

use crate::schema::tb_seed::dsl::*;
use crate::models::seed::Seed;

pub struct SeedRepository {
    connection: SqliteConnection
}

impl SeedRepository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

        let c = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        Self { connection: c }
    }

    pub fn create(&mut self, object:&Seed) -> Seed {
        diesel::insert_into(tb_seed)
        .values(object)
        .returning(Seed::as_returning())
        .get_result(&mut self.connection)
        .expect("Error creating a new record")
    }

    pub fn read(&mut self, id:&str) -> Option<Seed> {
        match tb_seed
        .find(id)
        .select(Seed::as_select())
        .get_result(&mut self.connection)
        .optional() {
            Ok(option) => option,
            Err(e) => panic!("Error trying to read coating id={}\n{}", id, e)
        }
 
    }

    pub fn read_all(&mut self, ) -> Vec<Seed> {
        match tb_seed
        .select(Seed::as_select())
        .get_results(&mut self.connection) {
            Ok(result) => result,
            Err(e) => panic!("Error trying to read all coatings\n{}", e)
        }
    }

    pub fn update(&mut self, id:&str, object:&Seed) -> Option<Seed> {
        match diesel::update(
            tb_seed
            .filter(scientific_name.eq(id)))
        .set(object)
        .returning(Seed::as_returning())
        .get_result(&mut self.connection)
        .optional() {
            Ok(option) => option,
            Err(e) => panic!("Error trying to update coating id={}\n{}", id, e)
        }
    }

    pub fn delete(&mut self, id:&str) -> usize {
        diesel::delete(tb_seed.filter(scientific_name.eq(id)))
        .execute(&mut self.connection)
        .expect("Error deleting record")
    }
}