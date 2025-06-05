use diesel::{prelude::*, SqliteConnection};
use dotenvy::dotenv;

use core::panic;
use std::env;

use crate::schema::{tb_brand::dsl::*};
use crate::models::brand::VBrand;

pub struct BrandRepository {
    connection: SqliteConnection
}

impl BrandRepository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

        let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        Self { connection }
    }

    pub fn create(&mut self, object:&VBrand) -> Result<VBrand, diesel::result::Error> {
        diesel::insert_into(tb_brand)
        .values(object)
        .get_result(&mut self.connection)
    }

    pub fn read(&mut self, id:&str) -> Result<Option<Vec<VBrand>>, diesel::result::Error> {
        tb_brand
        .filter(brand_name.eq(id))
        .get_results(&mut self.connection)
        .optional()
    }

    pub fn read_all(&mut self) -> Result<Option<Vec<VBrand>>, diesel::result::Error> {
        tb_brand
        .select(VBrand::as_select())
        .get_results(&mut self.connection)
        .optional()
    }

    pub fn update(&mut self, id:&str, new_name:&str) -> Result<Option<Vec<VBrand>>, diesel::result::Error> {
        diesel::update(
            tb_brand
            .filter(brand_name.eq(id)))
        .set(brand_name.eq(new_name))
        .returning(VBrand::as_returning())
        .get_results(&mut self.connection)
        .optional()
    }

    pub fn delete(&mut self, id:&str) -> Result<Option<Vec<VBrand>>, diesel::result::Error> {
        diesel::delete(tb_brand.filter(brand_name.eq(id)))
        .get_results(&mut self.connection)
        .optional()
    }

    pub fn delete_subvalue(&mut self, id:&str, value:i32) -> Result<Option<VBrand>, diesel::result::Error> {
        diesel::delete(tb_brand
            .filter(brand_name.eq(id))
            .filter(sack_weight.eq(value)))
        .get_result(&mut self.connection)
        .optional()
    }
}