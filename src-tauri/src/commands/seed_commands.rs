use crate::{ models::seed::Seed, services::seed_service::SeedService};

#[tauri::command]
fn new_seed(new:Seed) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.add(&new)
}

#[tauri::command]
fn get_seed(id:String) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.get(&id)
}

#[tauri::command]
fn list_seed() -> Result<Vec<Seed>, String> {
    let mut serv = SeedService::new();
    serv.list()
}

#[tauri::command]
fn change_seed(id:String, changes:Seed) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.save(&id, &changes)
}

#[tauri::command]
fn delete_seed(id:String) -> Result<Seed, String> {
    let mut serv = SeedService::new();
    serv.remove(&id)
}