use calamine::{open_workbook, Reader, Xlsx};
use std::fs::write;
use std::path::PathBuf;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn process_file(file_name: String, file_bytes: Vec<u8>) -> Result<String, String> {
    // Cria diretório e salva arquivo
    let mut path = PathBuf::from("./uploads");
    std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    path.push(&file_name);
    write(&path, file_bytes).map_err(|e| e.to_string())?;

    // Lê a planilha e imprime no terminal
    let path_str = path.to_string_lossy().to_string();
    read_spreadsheet(&path_str)?;

    Ok(format!("Arquivo salvo e lido com sucesso: {}", path_str))
}

pub fn read_spreadsheet(path: &str) -> Result<(), String> {
    let mut wb: Xlsx<_> = open_workbook(path).expect("Impossível abrir arquivo");

    if let Ok(range) = wb.worksheet_range("EST.") {
        for row in range.rows() {
            for cell in row {
                print!("{}\t", cell);
            }
            println!();
        }
    } else {
        return Err("A planilha 'EST.' não foi encontrada.".to_string());
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            process_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
