use crate::{
    models::{
        batch::*,
        stats::*
    },
    repositories::batch_repository::BatchRepository
};
use std::collections::HashMap;

pub struct BatchService {
    repo: BatchRepository
}

impl BatchService {
    pub fn new() -> Self {
        Self { repo: BatchRepository::new() }
    }
    
    pub fn create(&mut self, batch: &Batch) -> Result<Batch,String> {
        if batch.batch_month < 0 || batch.batch_month > 11 {
            return Err("Month beyond valid range".to_string())
        } else if batch.sack_amount <= 0 || batch.sack_weight <= 0 || batch.pureness_score <= 0.0 {
            return Err("No input can be 0 or negative".to_string())
        } else if batch.total_weight > 10000 || batch.total_weight < batch.sack_weight {
            return Err(format!("Total weight cannot be above 10,000 or below {}", batch.sack_weight))
        } else if batch.total_pureness_score <= batch.pureness_score {
            return Err(format!("Total pureness cannot be below {}", batch.pureness_score))
        } else if batch.batch_status != 1 {
            return Err("On creation batches must be active".to_string())
        } else if batch.deleted_at != None {
            return Err("On creation batches cannot have been deleted".to_string())
        }
        
        match self.repo.insert(batch) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }
    
    pub fn read_id(&mut self, id: &(i32, i32)) -> Result<Batch,String> {
        match self.repo.select_id(&id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No batch with id {}/{} found", id.0, id.1)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn read_year(&mut self, year:i32) -> Result<Vec<Batch>, String>{
        match self.repo.select_year(year) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No batches in year {}", year)),
            Err(e) => Err(e.to_string())
        }
    }
    
    pub fn read_all(&mut self) -> Result<Vec<Batch>,String> {
        match self.repo.select_all() {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err("No batches in the database".to_string()),
            Err(e) => Err(e.to_string())
        }
    }
 
    pub fn read_statistics(&mut self) -> Result<BatchStatistics, String> {
        if let Ok(Some(all_batches)) = self.repo.select_all() {

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

    pub fn update(&mut self, id: &(i32, i32), changes: &Batch) -> Result<Batch,String> {
        match self.repo.update(id, changes) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No batch with id {}/{} found", id.0, id.1)),
            Err(e) => Err(e.to_string())
        }
        
    }
    
    pub fn delete(&mut self, id: &(i32, i32)) -> Result<Batch, String> {
        match self.repo.drop(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No batch with id {}/{} found", id.0, id.1)),
            Err(e) => Err(e.to_string())
        }
    }
}