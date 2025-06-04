use crate::{ models::coating::Coating, repositories::coating_repository::CoatingRepository};

pub struct CoatingService {
    repo:CoatingRepository
}

impl CoatingService {
    pub fn new() -> CoatingService {
        CoatingService { repo: CoatingRepository::new() }
    }

    pub fn add(&mut self, new:&Coating) -> Result<Coating, String> {
        match self.repo.create(new) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn get(&mut self, id:&str) -> Result<Coating, String> {
        match self.repo.read(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No coating with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn list(&mut self) -> Result<Vec<Coating>, String> {
        match self.repo.read_all() {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn save(&mut self, id:&str, changes:&Coating) -> Result<Coating, String> {
        match self.repo.update(id, changes) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No coating with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn remove(&mut self, id:&str) -> Result<Coating, String> {
        match self.repo.delete(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No coating with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }
}