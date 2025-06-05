use crate::{
    models::seed::Seed,
    services::seed_service::SeedService
};

#[tauri::command]
pub fn new_seed(new:Seed) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.create(&new)
}

#[tauri::command]
pub fn get_seed(id:String) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.read_id(&id)
}

#[tauri::command]
pub fn list_seed() -> Result<Vec<Seed>, String> {
    let mut serv = SeedService::new();
    serv.read_all()
}

#[tauri::command]
pub fn change_seed(id:String, changes:Seed) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.update(&id, &changes)
}

#[tauri::command]
pub fn delete_seed(id:String) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.delete(&id)
}