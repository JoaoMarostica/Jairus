use crate::{
    models::brand::Brand,
    services::brand_service::BrandService
};

#[tauri::command]
pub fn new_brand(new:Brand) -> Vec<String> {
    let mut serv = BrandService::new();
    serv.create(&new)
}

#[tauri::command]
pub fn get_brand(id:String) -> Result<Brand, String> {
    let mut serv = BrandService::new();
    serv.read_id(&id)
}

#[tauri::command]
pub fn list_brand() -> Result<Vec<Brand>, String> {
    let mut serv = BrandService::new();
    serv.read_all()
}

#[tauri::command]
pub fn change_brand(id:String, new_name:String) -> Result<Brand, String> {
    let mut serv = BrandService::new();
    serv.update(&id, &new_name)
}

#[tauri::command]
pub fn delete_brand(id:String) -> Result<Brand, String> {
    let mut serv = BrandService::new();
    serv.delete(&id)
}

#[tauri::command]
pub fn delete_brand_weight(id:String, value:i32) -> Result<i32, String> {
    let mut serv = BrandService::new();
    serv.delete_value(&id, value)
}