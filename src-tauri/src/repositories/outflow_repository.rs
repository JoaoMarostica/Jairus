use diesel::{
    prelude::*,
    SqliteConnection
};
use dotenvy::dotenv;

use core::panic;
use std::env;

use crate::schema::tb_outflow::dsl::*;
use crate::models::outflow::{
    NewOutflow,
    Outflow
};

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

    pub fn insert(&mut self, object:&NewOutflow) -> Result<Outflow, diesel::result::Error> {
        diesel::insert_into(tb_outflow)
        .values(object)
        .returning(Outflow::as_returning())
        .get_result(&mut self.connection)
    }

    pub fn select_id(&mut self, id:&i32) -> Result<Option<Outflow>, diesel::result::Error> {
        tb_outflow.select(Outflow::as_select())
        .find(id)
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn select_batch(&mut self, batch:&(i32, i32)) -> Result<Vec<Outflow>, diesel::result::Error> {
        tb_outflow.select(Outflow::as_select())
        .filter(batch_number.eq(batch.0))
        .filter(batch_year.eq(batch.1))
        .get_results(&mut self.connection)
    }

    pub fn select_all(&mut self) -> Result<Vec<Outflow>, diesel::result::Error> {
        tb_outflow.select(Outflow::as_select())
        .get_results(&mut self.connection)
    }

    pub fn update(&mut self, id:&i32, object:&NewOutflow) -> Result<Option<Outflow>, diesel::result::Error> {
        diesel::update(tb_outflow.filter(outflow_id.eq(id)))
        .set(object)
        .returning(Outflow::as_returning())
        .get_result(&mut self.connection)
        .optional()
    }

    pub fn drop(&mut self, id:&i32) -> Result<Option<Outflow>, diesel::result::Error>{
        diesel::delete(tb_outflow.find(id))
        .get_result(&mut self.connection)
        .optional()
    }
}