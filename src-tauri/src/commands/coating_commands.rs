use crate::{
    models::coating::Coating,
    services::coating_service::CoatingService
};

#[tauri::command]
pub fn new_coating(new:Coating) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.create(&new)
}

#[tauri::command]
pub fn get_coating(id:String) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.read_id(&id)
}

#[tauri::command]
pub fn list_coating() -> Result<Vec<Coating>, String> {
    let mut serv = CoatingService::new();
    serv.read_all()
}

#[tauri::command]
pub fn change_coating(id:String, changes:Coating) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.update(&id, &changes)
}

#[tauri::command]
pub fn delete_coating(id:String) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.delete(&id)
}