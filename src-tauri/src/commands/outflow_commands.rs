use crate::{
    models::{
        outflow::{
            Outflow,
            NewOutflow
        },
        balance::Balance
    },
    services::outflow_service::OutflowService
};

#[tauri::command]
pub fn new_outflow(new:NewOutflow) -> Result<Outflow, String> {
    let mut serv = OutflowService::new();
    serv.create(&new)
}

#[tauri::command]
pub fn get_outflow(id:i32) -> Result<Outflow, String> {
    let mut serv = OutflowService::new();
    serv.read_id(&id)
}
#[tauri::command]
pub fn list_outflows(batch:(i32, i32)) -> Result<Vec<Outflow>, String> {
    let mut serv = OutflowService::new();
    serv.read_batch(&batch)
}

#[tauri::command]
pub fn list_all_outflows() -> Result<Vec<Outflow>, String> {
    let mut serv = OutflowService::new();
    serv.read_all()
}

#[tauri::command]
pub fn get_total_outflow(batch:(i32, i32)) -> Result<Balance, String> {
    let mut serv = OutflowService::new();
    serv.read_total(&batch)
}

#[tauri::command]
pub fn change_outflow(id:i32, changes:NewOutflow) -> Result<Outflow, String> {
    let mut  serv = OutflowService::new();
    serv.update(&id, &changes)
}

#[tauri::command]
pub fn delete_outflow(id:i32) -> Result<Outflow, String> {
    let mut serv = OutflowService::new();
    serv.delete(&id)
}