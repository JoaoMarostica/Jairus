use crate::{ models::coating::Coating, services::coating_service::CoatingService};

#[tauri::command]
fn new_coating(new:Coating) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.add(&new)
}

#[tauri::command]
fn get_coating(id:String) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.get(&id)
}

#[tauri::command]
fn list_coating() -> Result<Vec<Coating>, String> {
    let mut serv = CoatingService::new();
    serv.list()
}

#[tauri::command]
fn change_coating(id:String, changes:Coating) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.save(&id, &changes)
}

#[tauri::command]
fn delete_coating(id:String) -> Result<Coating, String> {
    let mut serv = CoatingService::new();
    serv.remove(&id)
}