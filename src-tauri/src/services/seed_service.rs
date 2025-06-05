use crate::{
    models::seed::Seed,
    repositories::seed_repository::SeedRepository
};

pub struct SeedService {
    repo:SeedRepository
}

impl SeedService {
    pub fn new() -> SeedService {
        SeedService { repo: SeedRepository::new() }
    }

    pub fn create(&mut self, new:&Seed) -> Result<Seed, String> {
        match self.repo.insert(new) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn read_id(&mut self, id:&str) -> Result<Seed, String> {
        match self.repo.select_id(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No seed with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn read_all(&mut self) -> Result<Vec<Seed>, String> {
        match self.repo.select_all() {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn update(&mut self, id:&str, changes:&Seed) -> Result<Seed, String> {
        match self.repo.update(id, changes) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No seed with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn delete(&mut self, id:&str) -> Result<Seed, String> {
        match self.repo.drop(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No seed with id {} found",id)),
            Err(e) => Err(e.to_string())
        }
    }
}