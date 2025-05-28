use crate::models::batch::Batch;
use crate::services::batch_service::BatchService;
use serde::{Deserialize, Serialize};

// Estrutura para receber dados de criação/atualização de um lote
#[derive(Deserialize)]
pub struct BatchPayload {
    pub batch_number: i32,
    pub batch_year: i32,
    pub batch_month: i32,
    pub seed: String,
    pub coating: String,
    pub brand: String,
    pub sack_weight: i32,
    pub sack_amount: i32,
    pub total_weight: i32,
    pub pureness_score: f32,
    pub total_pureness_score: f32,
    pub origin: String,
}

// Estrutura para resposta de erro
#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

// Comando para criar um novo lote
#[tauri::command]
pub fn create_batch(payload: BatchPayload) -> Result<Batch, ErrorResponse> {
    let mut service = BatchService::new();
    
    let batch = Batch::new(
        payload.batch_number,
        payload.batch_year,
        payload.batch_month,
        payload.seed,
        payload.coating,
        payload.brand,
        payload.sack_weight,
        payload.sack_amount,
        payload.total_weight,
        payload.pureness_score,
        payload.total_pureness_score,
        payload.origin,
    );
    
    match std::panic::catch_unwind(move || service.create_batch(&batch)) {
        Ok(result) => Ok(result),
        Err(_) => Err(ErrorResponse {
            message: "Erro ao criar lote".to_string(),
        }),
    }
}

// Comando para obter um lote específico pelo ID composto (batch_number, batch_year)
#[tauri::command]
pub fn get_batch(batch_number: i32, batch_year: i32) -> Result<Option<Batch>, ErrorResponse> {
    let mut service = BatchService::new();
    let id = (batch_number, batch_year);
    
    match std::panic::catch_unwind(move || service.get_batch(&id)) {
        Ok(result) => Ok(result),
        Err(_) => Err(ErrorResponse {
            message: format!("Erro ao buscar lote {}-{}", batch_number, batch_year),
        }),
    }
}

// Comando para listar todos os lotes
#[tauri::command]
pub fn list_batches() -> Result<Vec<Batch>, ErrorResponse> {
    let mut service = BatchService::new();
    
    match std::panic::catch_unwind(move || service.list_batches()) {
        Ok(result) => Ok(result),
        Err(_) => Err(ErrorResponse {
            message: "Erro ao listar lotes".to_string(),
        }),
    }
}

// Comando para atualizar um lote existente
#[tauri::command]
pub fn update_batch(payload: BatchPayload) -> Result<Option<Batch>, ErrorResponse> {
    let mut service = BatchService::new();
    
    let id = (payload.batch_number, payload.batch_year);
    let batch = Batch::new(
        payload.batch_number,
        payload.batch_year,
        payload.batch_month,
        payload.seed,
        payload.coating,
        payload.brand,
        payload.sack_weight,
        payload.sack_amount,
        payload.total_weight,
        payload.pureness_score,
        payload.total_pureness_score,
        payload.origin,
    );
    
    match std::panic::catch_unwind(move || service.update_batch(&id, &batch)) {
        Ok(result) => Ok(result),
        Err(_) => Err(ErrorResponse {
            message: format!("Erro ao atualizar lote {}-{}", payload.batch_number, payload.batch_year),
        }),
    }
}

// Comando para excluir um lote
#[tauri::command]
pub fn delete_batch(batch_number: i32, batch_year: i32) -> Result<usize, ErrorResponse> {
    let mut service = BatchService::new();
    let id = (batch_number, batch_year);
    
    match std::panic::catch_unwind(move || service.delete_batch(&id)) {
        Ok(result) => Ok(result),
        Err(_) => Err(ErrorResponse {
            message: format!("Erro ao excluir lote {}-{}", batch_number, batch_year),
        }),
    }
}

// Comando para filtrar lotes por ano
#[tauri::command]
pub fn filter_batches_by_year(year: i32) -> Result<Vec<Batch>, ErrorResponse> {
    let mut service = BatchService::new();
    
    match std::panic::catch_unwind(move || service.filter_by_year(year)) {
        Ok(result) => Ok(result),
        Err(_) => Err(ErrorResponse {
            message: format!("Erro ao filtrar lotes por ano {}", year),
        }),
    }
}

// Comando para filtrar lotes por mês
#[tauri::command]
pub fn filter_batches_by_month(month: i32) -> Result<Vec<Batch>, ErrorResponse> {
    let mut service = BatchService::new();
    
    match std::panic::catch_unwind(move || service.filter_by_month(month)) {
        Ok(result) => Ok(result),
        Err(_) => Err(ErrorResponse {
            message: format!("Erro ao filtrar lotes por mês {}", month),
        }),
    }
}

// Comando para filtrar lotes por semente
#[tauri::command]
pub fn filter_batches_by_seed(seed_name: String) -> Result<Vec<Batch>, ErrorResponse> {
    let mut service = BatchService::new();
    
    match std::panic::catch_unwind(move || service.filter_by_seed(&seed_name)) {
        Ok(result) => Ok(result),
        Err(_) => Err(ErrorResponse {
            message: format!("Erro ao filtrar lotes por semente {}", seed_name),
        }),
    }
}

// Comando para calcular estatísticas de lotes
#[tauri::command]
pub fn get_batch_statistics() -> Result<BatchStatistics, ErrorResponse> {
    let mut service = BatchService::new();
    
    match std::panic::catch_unwind(move || service.get_statistics()) {
        Ok(result) => Ok(result),
        Err(_) => Err(ErrorResponse {
            message: "Erro ao obter estatísticas de lotes".to_string(),
        }),
    }
}

// Estrutura para estatísticas de lotes
#[derive(Serialize)]
pub struct BatchStatistics {
    pub total_batches: usize,
    pub total_weight: i32,
    pub total_by_year: Vec<YearCount>,
    pub total_by_seed: Vec<SeedCount>,
}

#[derive(Serialize)]
pub struct YearCount {
    pub year: i32,
    pub count: usize,
    pub total_weight: i32,
}

#[derive(Serialize)]
pub struct SeedCount {
    pub seed: String,
    pub count: usize,
    pub total_weight: i32,
}
