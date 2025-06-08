#[cfg_attr(mobile, tauri::mobile_entry_point)]

mod commands;
mod services;
mod repositories;
mod models;
mod schema;

use crate::commands::*;
use dotenvy::dotenv;
use std::env;
use std::path::PathBuf;

fn get_database_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get current exe path");
    let exe_dir = exe_path.parent().expect("Failed to get exe directory");
    exe_dir.join("database").join("Jairus.db")
}

pub fn run() {
    let db_path = get_database_path();
    std::env::set_var("DATABASE_URL", &db_path);

    // Opcional: só usa .env se ele existir
    let _ = dotenv();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            batch_commands::add_batch,
            batch_commands::get_batch,
            batch_commands::list_batches_by_year,
            batch_commands::list_batches,
            batch_commands::get_batch_statistics,
            batch_commands::change_batch,
            batch_commands::remove_batch,
            batch_commands::get_stock_report_command,
            batch_commands::generate_selected_batches_pdf,

            brand_commands::add_brand,
            brand_commands::add_brand_weight,
            brand_commands::get_brand,
            brand_commands::list_brands,
            brand_commands::change_brand,
            brand_commands::remove_brand,
            brand_commands::remove_brand_weight,

            coating_commands::add_coating,
            coating_commands::get_coating,
            coating_commands::list_coatings,
            coating_commands::change_coating,
            coating_commands::remove_coating,

            outflow_commands::add_outflow,
            outflow_commands::get_outflow,
            outflow_commands::change_outflow,
            outflow_commands::list_outflows_by_batch,
            outflow_commands::list_outflows,
            outflow_commands::get_total_outflow,
            outflow_commands::remove_outflow,

            seed_commands::add_seed,
            seed_commands::get_seed,
            seed_commands::list_seeds,
            seed_commands::change_seed,
            seed_commands::remove_seed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}