use crate::{
    models::{outflow::{NewOutflow, Outflow}, balance::Balance},
    repositories::outflow_repository::OutflowRepository
};

pub struct OutflowService {
    repo:OutflowRepository
}

impl OutflowService {
    pub fn new() -> OutflowService {
        OutflowService { repo: OutflowRepository::new() }
    }

    pub fn add(&mut self, new:&NewOutflow) -> Result<Outflow, String> {
        match self.repo.create(new) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn get(&mut self, id:&i32) -> Result<Outflow, String> {
        match self.repo.read(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No outflow with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn list(&mut self, batch:&(i32, i32)) -> Result<Vec<Outflow>, String> {
        match self.repo.read_from(batch) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn list_all(&mut self) -> Result<Vec<Outflow>, String> {
        match self.repo.read_all() {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn save(&mut self, id:&i32, changes:&NewOutflow) -> Result<Outflow, String> {
        match self.repo.update(id, changes) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No outflow with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn remove(&mut self, id:&i32) -> Result<Outflow, String> {
        match self.repo.delete(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No outflow with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn get_total(&mut self, batch:&(i32, i32)) -> Result<Balance, String> {
        match self.list(batch) {
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
}