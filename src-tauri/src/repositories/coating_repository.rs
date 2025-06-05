use diesel::{
    prelude::*,
    SqliteConnection
};
use dotenvy::dotenv;

use core::panic;
use std::env;

use crate::schema::tb_coating::dsl::*;
use crate::models::coating::Coating;

pub struct CoatingRepository {
    connection: SqliteConnection
}

impl CoatingRepository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

        let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        Self { connection }
    }

    pub fn insert(&mut self, object:&Coating) -> Result<Coating, diesel::result::Error> {
        diesel::insert_into(tb_coating)
        .values(object)
        .returning(Coating::as_returning())
        .get_result(&mut self.connection)
    }

    pub fn select_id(&mut self, id:&str) -> Result<Option<Coating>, diesel::result::Error> {
        tb_coating
        .find(id)
        .select(Coating::as_select())
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn select_all(&mut self) -> Result<Vec<Coating>, diesel::result::Error> {
        tb_coating
        .select(Coating::as_select())
        .get_results(&mut self.connection)
    }

    pub fn update(&mut self, id:&str, object:&Coating) -> Result<Option<Coating>, diesel::result::Error> {
        diesel::update(tb_coating.filter(coating_name.eq(id)))
        .set(object)
        .returning(Coating::as_returning())
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn drop(&mut self, id:&str) -> Result<Option<Coating>, diesel::result::Error> {
        diesel::delete(tb_coating.find(id))
        .get_result(&mut self.connection)
        .optional()
    }
}