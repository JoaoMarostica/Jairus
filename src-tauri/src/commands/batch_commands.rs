use crate::{ models::batch::Batch, services::batch_service::BatchService};

#[tauri::command]
pub fn exemplo() -> String {
    "funciona".to_string()
}