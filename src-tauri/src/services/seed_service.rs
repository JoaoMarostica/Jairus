use crate::{ models::seed::Seed, repositories::seed_repository::SeedRepository};

pub struct SeedService {
    repo:SeedRepository
}

impl SeedService {
    pub fn new() -> SeedService {
        SeedService { repo: SeedRepository::new() }
    }

    pub fn add(&mut self, new:&Seed) -> Result<Seed, String> {
        match self.repo.create(new) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn get(&mut self, id:&str) -> Result<Seed, String> {
        match self.repo.read(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No seed with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn list(&mut self) -> Result<Vec<Seed>, String> {
        match self.repo.read_all() {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn save(&mut self, id:&str, changes:&Seed) -> Result<Seed, String> {
        match self.repo.update(id, changes) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No seed with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn remove(&mut self, id:&str) -> Result<Seed, String> {
        match self.repo.delete(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No seed with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }
}