use crate::{
    models::{outflow::{Outflow, NewOutflow}, balance::Balance},
    services::outflow_service::OutflowService
};

#[tauri::command]
pub fn new_outflow(new:NewOutflow) -> Result<Outflow, String> {
    let mut serv = OutflowService::new();
    serv.add(&new)
}

#[tauri::command]
pub fn get_outflow(id:i32) -> Result<Outflow, String> {
    let mut serv = OutflowService::new();
    serv.get(&id)
}

#[tauri::command]
pub fn change_outflow(id:i32, changes:NewOutflow) -> Result<Outflow, String> {
    let mut  serv = OutflowService::new();
    serv.save(&id, &changes)
}

#[tauri::command]
pub fn list_outflows(batch:(i32, i32)) -> Result<Vec<Outflow>, String> {
    let mut serv = OutflowService::new();
    serv.list(&batch)
}

#[tauri::command]
pub fn list_all_outflows() -> Result<Vec<Outflow>, String> {
    let mut serv = OutflowService::new();
    serv.list_all()
}

#[tauri::command]
pub fn delete_outflow(id:i32) -> Result<Outflow, String> {
    let mut serv = OutflowService::new();
    serv.remove(&id)
}

#[tauri::command]
pub fn get_total_outflow(batch:(i32, i32)) -> Result<Balance, String> {
    let mut serv = OutflowService::new();
    serv.get_total(&batch)
}