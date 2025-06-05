use diesel::{
    prelude::*,
    SqliteConnection
};
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

        let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        Self { connection }
    }

    pub fn insert(&mut self, object:&Seed) -> Result<Seed, diesel::result::Error> {
        diesel::insert_into(tb_seed)
        .values(object)
        .returning(Seed::as_returning())
        .get_result(&mut self.connection)
    }

    pub fn select_id(&mut self, id:&str) -> Result<Option<Seed>, diesel::result::Error> {
        tb_seed.select(Seed::as_select())
        .find(id)
        .get_result(&mut self.connection)
        .optional() 
    }

    pub fn select_all(&mut self) -> Result<Vec<Seed>, diesel::result::Error> {
        tb_seed.select(Seed::as_select())
        .get_results(&mut self.connection)
    }

    pub fn update(&mut self, id:&str, object:&Seed) -> Result<Option<Seed>, diesel::result::Error> {
        diesel::update(tb_seed.filter(scientific_name.eq(id)))
        .set(object)
        .returning(Seed::as_returning())
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn drop(&mut self, id:&str) -> Result<Option<Seed>, diesel::result::Error> {
        diesel::delete(tb_seed.find(id))
        .get_result(&mut self.connection)
        .optional()
    }
}