use crate::{
    models::{batch::*,stats::*},
    repositories::batch_repository::BatchRepository};
use std::collections::HashMap;

pub struct BatchService {
    repo: BatchRepository
}

impl BatchService {
    pub fn new() -> Self {
        Self { repo: BatchRepository::new() }
    }
    
    pub fn create_batch(&mut self, batch: &Batch) -> Result<Batch,String> {
        println!("Criando lote: {:?}", batch);
        match self.repo.create(batch) {
            Ok(result) => Ok({
                println!("resultado: {:?}", result);
                result
            }),
            Err(e) => {
                println!("Erro ao criar lote: {:?}", e);
                Err(format!("Erro ao criar lote\n{}", e))
            }
        }
    }
    
    pub fn get_batch(&mut self, id: &(i32, i32)) -> Result<Option<Batch>,String> {
        match self.repo.read(&id) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Erro ao buscar lote {:?}\n{}", id, e))
        }
    }
    
    pub fn list_batches(&mut self) -> Result<Vec<Batch>,String> {
        match self.repo.read_all() {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Erro ao listar lotes\n{}",e))
        }
    }
    
    pub fn update_batch(&mut self, id: &(i32, i32), batch: &Batch) -> Result<Option<Batch>,String> {
        match self.repo.update(id, batch) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Erro ao atualizar lote {}-{}\n{}",
            batch.batch_number, batch.batch_year, e)
            )
        }
        
    }
    
    pub fn delete_batch(&mut self, id: &(i32, i32)) -> Result<Option<Batch>, String> {
        match self.repo.delete(id) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Erro ao excluir lote {:?}\n{}", id, e))
        }
    }

    pub fn get_filtered_batches(
        &mut self,
        query: BatchQuery,
    ) -> Result<Vec<Batch>, String> {
        self.repo
            .filter_batches(&query)
            .map_err(|e| format!("Erro ao filtrar lotes: {}", e))
    }

/*
    /// Filtra lotes por ano
    pub fn filter_by_year(&mut self, year_value: i32) -> Vec<Batch> {
        let all_batches = self.list_batches();
        all_batches.into_iter()
            .filter(|batch| batch.batch_year == year_value)
            .collect()
    }
    
    /// Filtra lotes por mês
    pub fn filter_by_month(&mut self, month_value: i32) -> Vec<Batch> {
        let all_batches = self.list_batches();
        all_batches.into_iter()
            .filter(|batch| batch.batch_month == month_value)
            .collect()
    }
    
    /// Filtra lotes por semente
    pub fn filter_by_seed(&mut self, seed_name: &str) -> Vec<Batch> {
        let all_batches = self.list_batches();
        all_batches.into_iter()
            .filter(|batch| batch.seed == seed_name)
            .collect()
    }
*/

    pub fn get_statistics(&mut self) -> Result<BatchStatistics, String> {
        if let Ok(all_batches) = self.list_batches() {

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

// Valida se o nome da coluna é permitido (para evitar injeção de SQL)
fn is_valid_column(col: &str) -> bool {
    matches!(
        col,
        "id" | "batch_number" | "batch_year" | "batch_month" |
        "seed" | "coating" | "brand" | "sack_weight" | "sack_amount" |
        "total_weight" | "pureness_score" | "total_pureness_score" |
        "batch_status" | "deleted_at" | "origin"
    )
}
