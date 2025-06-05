use crate::{
    models::{
        balance::Balance, outflow::{
            NewOutflow,
            Outflow
        }
    },
    services::outflow_service::OutflowService
};

#[tauri::command]
pub fn add_outflow(new:NewOutflow) -> Result<Outflow, String> {
    let mut serv = OutflowService::new();
    serv.create(&new)
}

#[tauri::command]
pub fn get_outflow(id:i32) -> Result<Outflow, String> {
    let mut serv = OutflowService::new();
    serv.read_id(&id)
}
#[tauri::command]
pub fn list_outflows_by_batch(batch_number:i32, batch_year:i32) -> Result<Vec<Outflow>, String> {
    let mut serv = OutflowService::new();
    serv.read_batch(&(batch_number, batch_year))
}

#[tauri::command]
pub fn list_outflows() -> Result<Vec<Outflow>, String> {
    let mut serv = OutflowService::new();
    serv.read_all()
}

#[tauri::command]
pub fn get_total_outflow(batch_number:i32, batch_year:i32) -> Result<Balance, String> {
    let mut serv = OutflowService::new();
    serv.read_total(&(batch_number, batch_year))
}

#[tauri::command]
pub fn change_outflow(id:i32, changes:NewOutflow) -> Result<Outflow, String> {
    let mut  serv = OutflowService::new();
    serv.update(&id, &changes)
}

#[tauri::command]
pub fn remove_outflow(id:i32) -> Result<Outflow, String> {
    let mut serv = OutflowService::new();
    serv.delete(&id)
}