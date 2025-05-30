#[cfg_attr(mobile, tauri::mobile_entry_point)]
// use crate::commands::batch_commands;
use crate::commands::*;

mod commands;
mod services;
mod repositories;
mod models;
mod schema;


pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            batch_commands::create_batch,
            batch_commands::get_batch,
            batch_commands::list_batches,
            batch_commands::update_batch,
            batch_commands::delete_batch,
            batch_commands::get_batch_statistics,
            batch_commands::get_filtered_batches,
//            batch_commands::filter_batches_by_year,
//            batch_commands::filter_batches_by_month,
//            batch_commands::filter_batches_by_seed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}