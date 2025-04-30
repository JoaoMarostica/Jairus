use calamine::{open_workbook, Reader, ToCellDeserializer, Xlsx};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn read_spreadsheet(path: &str) {
    let mut wb: Xlsx<_> = open_workbook(path).expect("Impossível abrir arquivo");
    if let Ok(range) = wb.worksheet_range("EST.") {
        for row in range.rows() {
            for cell in row {
                if cell.is_empty() {
                    print!("\t\t\t");
                } else {
                    print!("{}\t\t\t", cell);
                }
            }
            print!("\n");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
