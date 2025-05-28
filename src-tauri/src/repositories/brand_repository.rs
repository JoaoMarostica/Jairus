use diesel::{prelude::*, SqliteConnection};
use dotenvy::dotenv;

use core::panic;
use std::env;

use crate::schema::{tb_brand::dsl::*};
use crate::models::brand::{Brand,VBrand};

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

    pub fn create(&mut self, object:&VBrand) -> VBrand {
        diesel::insert_into(tb_brand)
        .values(object)
        .returning(VBrand::as_returning())
        .get_result(&mut self.connection)
        .expect("Error creating a new record in brand table")
    }

    pub fn read(&mut self, id:&str) -> Option<Brand> {
        match tb_brand
        .filter(brand_name.eq(id))
        .select(VBrand::as_select())
        .get_results(&mut self.connection)
        .optional() {
            Ok(Some(result)) => {
                let mut weights:Vec<i32> = Vec::with_capacity(result.len());
                for vb in &result {
                    weights.push(vb.sack_weight);
                }

                Some(Brand::new(&result[0].brand_name, weights))
            },
            Ok(None) => None,
            Err(e) => panic!("Error trying to read brand id={}\n{}", id, e)
        }
    }

    pub fn read_all(&mut self) -> Vec<Brand> {
        match tb_brand
        .select(VBrand::as_select())
        .get_results(&mut self.connection)
        .optional() {
            Ok(Some(result)) => {
                let mut brands:Vec<Brand> = vec![];

                for vb in &result {
                    match brands.iter_mut().find(|x| x.id() == &vb.brand_name) {
                        Some(b) => b.weights().push(vb.sack_weight),
                        None => brands.push(Brand::new(&vb.brand_name, vec![])),
                    };
                }
                brands
            },
            Ok(None) => vec![],
            Err(e) => panic!("Error trying to read all brands\n{}", e)
        }
    }

    pub fn update(&mut self, id:&str, new_name:&str) -> Option<Brand> {
        match diesel::update(
            tb_brand
            .filter(brand_name.eq(id)))
        .set(brand_name.eq(new_name))
        .returning(VBrand::as_returning())
        .get_results(&mut self.connection)
        .optional() {
                Ok(Some(result)) => {
                let mut weights:Vec<i32> = Vec::with_capacity(result.len());
                for vb in &result {
                    weights.push(vb.sack_weight);
                }

                Some(Brand::new(&result[0].brand_name, weights))
            },
            Ok(None) => None,
            Err(e) => panic!("Error trying to update brand id={}\n{}", id, e)
        }
    }

    pub fn delete(&mut self, id:&str) -> usize {
        diesel::delete(tb_brand.filter(brand_name.eq(id)))
        .execute(&mut self.connection)
        .expect("Error deleting multiple records in brand table")
    }

    pub fn delete_subvalue(&mut self, id:&str, value:i32) -> usize {
        diesel::delete(
            tb_brand.filter(
                brand_name.eq(id)
                .and(sack_weight.eq(value))))
        .execute(&mut self.connection)
        .expect("Error deleting record in brand table")
    }
}