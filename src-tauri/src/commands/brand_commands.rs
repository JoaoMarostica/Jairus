use crate::{ models::brand::Brand, services::brand_service::BrandService};

#[tauri::command]
fn new_brand(new:Brand) -> Vec<String> {
    let mut serv = BrandService::new();
    serv.add(&new)
}

#[tauri::command]
fn get_brand(id:String) -> Result<Brand, String> {
    let mut serv = BrandService::new();
    serv.get(&id)
}

#[tauri::command]
fn list_brand() -> Result<Vec<Brand>, String> {
    let mut serv = BrandService::new();
    serv.list()
}

#[tauri::command]
fn change_brand(id:String, new_name:String) -> Result<Brand, String> {
    let mut serv = BrandService::new();
    serv.save(&id, &new_name)
}

#[tauri::command]
fn delete_brand(id:String) -> Result<Brand, String> {
    let mut serv = BrandService::new();
    serv.remove(&id)
}