use crate::{ models::batch::Batch, repositories::batch_repository::BatchRepository};
use crate::commands::batch_commands::{BatchStatistics, YearCount, SeedCount};
use std::collections::HashMap;

pub struct BatchService {
    repo: BatchRepository
}

impl BatchService {
    /// Cria uma nova instância do serviço de lotes
    pub fn new() -> Self {
        Self {
            repo: BatchRepository::new()
        }
    }
    
    /// Cria um novo lote no banco de dados
    pub fn create_batch(&mut self, batch: &Batch) -> Batch {
        self.repo.create(batch)
    }
    
    /// Obtém um lote específico pelo ID composto (batch_number, batch_year)
    pub fn get_batch(&mut self, id: &(i32, i32)) -> Option<Batch> {
        self.repo.read(id)
    }
    
    /// Lista todos os lotes
    pub fn list_batches(&mut self) -> Vec<Batch> {
        self.repo.read_all()
    }
    
    /// Atualiza um lote existente
    pub fn update_batch(&mut self, id: &(i32, i32), batch: &Batch) -> Option<Batch> {
        self.repo.update(id, batch)
    }
    
    /// Exclui um lote
    pub fn delete_batch(&mut self, id: &(i32, i32)) -> usize {
        self.repo.delete(id)
    }
    
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
    
    /// Obtém estatísticas de lotes
    pub fn get_statistics(&mut self) -> BatchStatistics {
        let all_batches = self.list_batches();
        
        // Total de lotes
        let total_batches = all_batches.len();
        
        // Peso total
        let total_weight: i32 = all_batches.iter()
            .map(|batch| batch.total_weight)
            .sum();
        
        // Agrupar por ano
        let mut years_map: HashMap<i32, (usize, i32)> = HashMap::new();
        for batch in &all_batches {
            let entry = years_map.entry(batch.batch_year).or_insert((0, 0));
            entry.0 += 1; // Incrementa contagem
            entry.1 += batch.total_weight; // Adiciona peso
        }
        
        let total_by_year: Vec<YearCount> = years_map
            .into_iter()
            .map(|(year, (count, total_weight))| YearCount { 
                year, 
                count, 
                total_weight 
            })
            .collect();
        
        // Agrupar por semente
        let mut seeds_map: HashMap<String, (usize, i32)> = HashMap::new();
        for batch in &all_batches {
            let entry = seeds_map.entry(batch.seed.clone()).or_insert((0, 0));
            entry.0 += 1; // Incrementa contagem
            entry.1 += batch.total_weight; // Adiciona peso
        }
        
        let total_by_seed: Vec<SeedCount> = seeds_map
            .into_iter()
            .map(|(seed, (count, total_weight))| SeedCount { 
                seed, 
                count, 
                total_weight 
            })
            .collect();
        
        BatchStatistics {
            total_batches,
            total_weight,
            total_by_year,
            total_by_seed,
        }
    }
}
