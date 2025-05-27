use diesel::{prelude::*,SqliteConnection};
use dotenvy::dotenv;

use core::panic;
use std::env;

use crate::schema::tb_coating::dsl::*;
use crate::models::coating::Coating;

pub struct CoatingRepository {
    connection: SqliteConnection
}

impl CoatingRepository {
    fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

        let c = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        Self { connection: c }
    }

    fn create(&mut self, object:&Coating) -> Coating {
        diesel::insert_into(tb_coating)
        .values(object)
        .returning(Coating::as_returning())
        .get_result(&mut self.connection)
        .expect("Error creating a new record")
    }

    fn read(&mut self, id:&str) -> Option<Coating> {
        match tb_coating
        .find(id)
        .select(Coating::as_select())
        .first(&mut self.connection)
        .optional() {
            Ok(option) => option,
            Err(e) => panic!("Error trying to read coating id={}\n{}", id, e)
        }
    }

    fn read_all(&mut self) -> Vec<Coating> {
        match tb_coating
        .select(Coating::as_select())
        .load(&mut self.connection) {
            Ok(result) => result,
            Err(e) => panic!("Error trying to read all coatings\n{}", e)
        }
    }

    fn update(&mut self, id:&str, object:&Coating) -> Option<Coating> {
        match diesel::update(
            tb_coating
            .filter(coating_name.eq(id)))
        .set(coating_name.eq(object.id()))
        .returning(Coating::as_returning())
        .get_result(&mut self.connection)
        .optional() {
            Ok(option) => option,
            Err(e) => panic!("Error trying to update coating id={}\n{}", id, e)
        }
    }

    fn delete(&mut self, id:&str) {
        diesel::delete(tb_coating.filter(coating_name.eq(id)))
        .execute(&mut self.connection)
        .expect("Error deleting record");
    }
}