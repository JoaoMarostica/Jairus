use crate::{models::seed::Seed, services::seed_service::SeedService};

#[tauri::command]
pub fn new_seed(new:Seed) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.add(&new)
}

#[tauri::command]
pub fn get_seed(id:String) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.get(&id)
}

#[tauri::command]
pub fn list_seed() -> Result<Vec<Seed>, String> {
    let mut serv = SeedService::new();
    serv.list()
}

#[tauri::command]
pub fn change_seed(id:String, changes:Seed) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.save(&id, &changes)
}

#[tauri::command]
pub fn delete_seed(id:String) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.remove(&id)
}