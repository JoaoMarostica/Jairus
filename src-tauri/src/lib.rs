#[cfg_attr(mobile, tauri::mobile_entry_point)]

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

            brand_commands::new_brand,
            brand_commands::get_brand,
            brand_commands::list_brand,
            brand_commands::change_brand,
            brand_commands::delete_brand,
            brand_commands::delete_brand_weight,

            coating_commands::new_coating,
            coating_commands::get_coating,
            coating_commands::list_coating,
            coating_commands::change_coating,
            coating_commands::delete_coating,

            outflow_commands::new_outflow,
            outflow_commands::get_outflow,
            outflow_commands::change_outflow,
            outflow_commands::list_outflows,
            outflow_commands::list_all_outflows,
            outflow_commands::delete_outflow,
            outflow_commands::get_total_outflow,

            seed_commands::new_seed,
            seed_commands::get_seed,
            seed_commands::list_seed,
            seed_commands::change_seed,
            seed_commands::delete_seed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}