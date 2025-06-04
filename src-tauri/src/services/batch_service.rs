use crate::{
    models::{batch::*,stats::*},
    repositories::batch_repository::BatchRepository};
use std::{collections::HashMap};

pub struct BatchService {
    repo: BatchRepository
}

impl BatchService {
    pub fn new() -> Self {
        Self { repo: BatchRepository::new() }
    }
    
    pub fn add(&mut self, batch: &Batch) -> Result<Batch,String> {
        match self.repo.create(batch) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }
    
    pub fn get(&mut self, id: &(i32, i32)) -> Result<Batch,String> {
        match self.repo.read(&id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No batch with id {}/{} found", id.0, id.1)),
            Err(e) => Err(e.to_string())
        }
    }
    
    pub fn list(&mut self) -> Result<Vec<Batch>,String> {
        match self.repo.read_all() {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }
    
    pub fn save(&mut self, id: &(i32, i32), changes: &Batch) -> Result<Batch,String> {
        match self.repo.update(id, changes) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No batch with id {}/{} found", id.0, id.1)),
            Err(e) => Err(e.to_string())
        }
        
    }
    
    pub fn remove(&mut self, id: &(i32, i32)) -> Result<Batch, String> {
        match self.repo.delete(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No batch with id {}/{} found", id.0, id.1)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn get_statistics(&mut self) -> Result<BatchStatistics, String> {
        if let Ok(all_batches) = self.list() {

            let total_batches = all_batches.len();
            
            let total_weight:i32 = all_batches.iter()
                .map(|batch| batch.total_weight)
                .sum();

            let mut years_map: HashMap<i32,(usize, i32)> = HashMap::new();
            for batch in &all_batches {
                let entry =
                years_map.entry(batch.batch_year).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += batch.total_weight;
            }
            
            let total_by_year: Vec<YearCount> = years_map
                .into_iter()
                .map(|(year, (count, total_weight))| YearCount { 
                    year, 
                    count, 
                    total_weight 
                })
                .collect();
            
            let mut seeds_map: HashMap<String,(usize, i32)> = HashMap::new();
            for batch in &all_batches {
                let entry = seeds_map.entry(batch.seed.clone()).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += batch.total_weight;
            }
            
            let total_by_seed: Vec<SeedCount> = seeds_map
                .into_iter()
                .map(|(seed, (count, total_weight))| SeedCount { 
                    seed, 
                    count, 
                    total_weight 
                })
                .collect();
            
            Ok(BatchStatistics {
                total_batches,
                total_weight,
                total_by_year,
                total_by_seed,
            })
        } else {
            return Err("Erro ao obter estatísticas de lotes".to_string())
        }
    }
}