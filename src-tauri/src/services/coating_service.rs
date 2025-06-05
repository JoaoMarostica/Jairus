use crate::{
    models::coating::Coating,
    repositories::coating_repository::CoatingRepository
};

pub struct CoatingService {
    repo:CoatingRepository
}

impl CoatingService {
    pub fn new() -> CoatingService {
        CoatingService { repo: CoatingRepository::new() }
    }

    pub fn create(&mut self, new:&Coating) -> Result<Coating, String> {
        match self.repo.insert(new) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn read_id(&mut self, id:&str) -> Result<Coating, String> {
        match self.repo.select_id(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No coating with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn read_all(&mut self) -> Result<Vec<Coating>, String> {
        match self.repo.select_all() {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn update(&mut self, id:&str, changes:&Coating) -> Result<Coating, String> {
        match self.repo.update(id, changes) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No coating with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn delete(&mut self, id:&str) -> Result<Coating, String> {
        match self.repo.drop(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No coating with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }
}