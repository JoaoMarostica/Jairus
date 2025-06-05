use crate::{ models::coating::Coating, services::coating_service::CoatingService};

#[tauri::command]
pub fn new_coating(new:Coating) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.add(&new)
}

#[tauri::command]
pub fn get_coating(id:String) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.get(&id)
}

#[tauri::command]
pub fn list_coating() -> Result<Vec<Coating>, String> {
    let mut serv = CoatingService::new();
    serv.list()
}

#[tauri::command]
pub fn change_coating(id:String, changes:Coating) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.save(&id, &changes)
}

#[tauri::command]
pub fn delete_coating(id:String) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.remove(&id)
}