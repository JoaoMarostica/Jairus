use calamine::{open_workbook, Reader, Xlsx};

#[tauri::command]
fn process_file(path: &str) -> Option<String> {
    let mut wb: Xlsx<_> = open_workbook(path).expect("Impossível abrir arquivo");
    let range = wb.worksheet_range_at(0);
    match range {
        Some(res) => {
            if let Ok(_range) = res {
                print!("funciona\n");
                None
            } else {
                Some("não foi possível abrir a planilha 1\n".to_string())
            }
        },
        None => Some("não foi possível abrir a planilha 2\n".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            process_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
