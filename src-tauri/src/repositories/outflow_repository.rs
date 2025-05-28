use diesel::{prelude::*, SqliteConnection};
use dotenvy::dotenv;

use core::panic;
use std::env;

use crate::schema::tb_outflow::dsl::*;
use crate::models::outflow::{NewOutflow, Outflow};

pub struct OutflowRepository {
    connection: SqliteConnection
}

impl OutflowRepository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

        let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        Self { connection }
    }

    pub fn create(&mut self, object:&NewOutflow) -> Outflow {
        diesel::insert_into(tb_outflow)
        .values(object)
        .returning(Outflow::as_returning())
        .get_result(&mut self.connection)
        .expect("Error creating a new record in outflow table")
    }

    pub fn read(&mut self, id:&i32) -> Option<Outflow> {
        match tb_outflow
        .find(id)
        .select(Outflow::as_select())
        .get_result(&mut self.connection)
        .optional() {
            Ok(option) => option,
            Err(e) => panic!("Error trying to read outflow id={}\n{}", id, e)
        }
    }

    pub fn read_all(&mut self) -> Vec<Outflow> {
        match tb_outflow
        .select(Outflow::as_select())
        .get_results(&mut self.connection) {
            Ok(result) => result,
            Err(e) => panic!("Error trying to read all outflows\n{}", e)
        }
    }

    pub fn update(&mut self, id:&i32, object:&Outflow) -> Option<Outflow> {
        match diesel::update(tb_outflow.filter(outflow_id.eq(id)))
        .set(object)
        .returning(Outflow::as_returning())
        .get_result(&mut self.connection)
        .optional() {
            Ok(option) => option,
            Err(e) => panic!("Error trying to update outflow id={}\n{}", id, e)
        }
    }

    pub fn delete(&mut self, id:&i32) -> usize {
        diesel::delete(tb_outflow.find(id))
        .execute(&mut self.connection)
        .expect("Error deleting record in outflow table")
    }
}