use crate::{ models::brand::Brand, services::brand_service::BrandService};

#[tauri::command]
pub fn new_brand(new:Brand) -> Vec<String> {
    let mut serv = BrandService::new();
    serv.add(&new)
}

#[tauri::command]
pub fn get_brand(id:String) -> Result<Brand, String> {
    let mut serv = BrandService::new();
    serv.get(&id)
}

#[tauri::command]
pub fn list_brand() -> Result<Vec<Brand>, String> {
    let mut serv = BrandService::new();
    serv.list()
}

#[tauri::command]
pub fn change_brand(id:String, new_name:String) -> Result<Brand, String> {
    let mut serv = BrandService::new();
    serv.save(&id, &new_name)
}

#[tauri::command]
pub fn delete_brand(id:String) -> Result<Brand, String> {
    let mut serv = BrandService::new();
    serv.remove(&id)
}

#[tauri::command]
pub fn delete_brand_weight(id:String, value:i32) -> Result<i32, String> {
    let mut serv = BrandService::new();
    serv.remove_part(&id, value)
}