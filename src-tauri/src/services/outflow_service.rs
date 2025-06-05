use crate::{
    models::{
        outflow::{
            NewOutflow,
            Outflow
        },
        balance::Balance
    },
    repositories::outflow_repository::OutflowRepository
};

pub struct OutflowService {
    repo:OutflowRepository
}

impl OutflowService {
    pub fn new() -> OutflowService {
        OutflowService { repo: OutflowRepository::new() }
    }

    pub fn create(&mut self, new:&NewOutflow) -> Result<Outflow, String> {
        match self.repo.insert(new) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn read_id(&mut self, id:&i32) -> Result<Outflow, String> {
        match self.repo.select_id(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No outflow with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn read_batch(&mut self, batch:&(i32, i32)) -> Result<Vec<Outflow>, String> {
        match self.repo.select_batch(batch) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn read_all(&mut self) -> Result<Vec<Outflow>, String> {
        match self.repo.select_all() {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn read_total(&mut self, batch:&(i32, i32)) -> Result<Balance, String> {
        match self.read_batch(batch) {
            Ok(result) => {
                let mut total = Balance::new(0, 0, 0.0);
                for o in result {
                    total.sum(o.get_balance());
                }
                Ok(total)
            },
            Err(e) => Err(e)
        }
    }

    pub fn update(&mut self, id:&i32, changes:&NewOutflow) -> Result<Outflow, String> {
        match self.repo.update(id, changes) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No outflow with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn delete(&mut self, id:&i32) -> Result<Outflow, String> {
        match self.repo.drop(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No outflow with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }
}