use crate::{
    models::{batch::*, stats::*},
    services::batch_service::BatchService};

#[tauri::command]
pub fn create_batch(batch: Batch) -> Result<Batch, String> {
    let mut service = BatchService::new();
    service.create_batch(&batch)
}

#[tauri::command]
pub fn get_batch(batch_number: i32, batch_year: i32) -> Result<Option<Batch>,String> {
    let mut service = BatchService::new();
    service.get_batch(&(batch_number, batch_year))
}

#[tauri::command]
pub fn list_batches() -> Result<Vec<Batch>, String> {
    let mut service = BatchService::new();
    service.list_batches()
}

#[tauri::command]
pub fn update_batch(batch_number:i32, batch_year:i32, batch: Batch) -> Result<Option<Batch>, String> {
    let mut service = BatchService::new();
    service.update_batch(&(batch_number, batch_year), &batch)
}

#[tauri::command]
pub fn delete_batch(batch_number: i32, batch_year: i32) -> Result<Option<Batch>, String> {
    let mut service = BatchService::new();
    service.delete_batch(&(batch_number, batch_year))
}

#[tauri::command]
pub fn get_batch_statistics() -> Result<BatchStatistics, String> {
    let mut service = BatchService::new();
    service.get_statistics()
}

#[tauri::command]
pub fn get_filtered_batches(query: BatchQuery) -> Result<Vec<Batch>, String> {
    let mut service = BatchService::new();
    service.get_filtered_batches(query)
}