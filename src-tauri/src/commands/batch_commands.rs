use crate::{
    models::{
        batch::*,
        stats::*
    },
    services::batch_service::BatchService
};

#[tauri::command]
pub fn create_batch(batch: Batch) -> Result<Batch, String> {
    let mut service = BatchService::new();
    service.create(&batch)
}

#[tauri::command]
pub fn get_batch(batch_number: i32, batch_year: i32) -> Result<Batch, String> {
    let mut service = BatchService::new();
    service.read_id(&(batch_number, batch_year))
}

#[tauri::command]
pub fn get_batch_by_year(year:i32) -> Result<Vec<Batch>, String> {
    let mut service = BatchService::new();
    service.read_year(year)
}

#[tauri::command]
pub fn list_batches() -> Result<Vec<Batch>, String> {
    let mut service = BatchService::new();
    service.read_all()
}

#[tauri::command]
pub fn get_batch_statistics() -> Result<BatchStatistics, String> {
    let mut service = BatchService::new();
    service.read_statistics()
}

#[tauri::command]
pub fn update_batch(batch_number:i32, batch_year:i32, batch: Batch) -> Result<Batch, String> {
    let mut service = BatchService::new();
    service.update(&(batch_number, batch_year), &batch)
}

#[tauri::command]
pub fn delete_batch(batch_number: i32, batch_year: i32) -> Result<Batch, String> {
    let mut service = BatchService::new();
    service.delete(&(batch_number, batch_year))
}