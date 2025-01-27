// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use calamine::{open_workbook, Reader, Xlsx, XlsxError};
use serde::Serialize;



#[derive(Serialize)]
struct RowData {
    value: String,
}

#[tauri::command]
fn read_xlsx_file() -> Result<Vec<RowData>, String> {

    let path = "/home/user/Downloads/new.xlsx" ;
    let mut workbook: Xlsx<_> = open_workbook(path).map_err(|e: XlsxError| e.to_string())? ;

    let range = workbook
        .worksheet_range("Sheet1")
        .map_err(|e: XlsxError| e.to_string())? ;

    let mut data = Vec::new();

    for row in range.rows() {
        if let Some(cell) = row.get(0) {
            data.push(RowData {
                value: cell.to_string(),
            });
        }
    }

Ok(data)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_xlsx_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}