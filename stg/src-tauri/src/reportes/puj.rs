
// FECHA
use chrono::NaiveDate ;
use serde::Serialize ;



#[derive(Serialize)]
pub struct Fecha {
    fecha: String,
}


#[tauri::command]
pub fn reportes_puj_actualizar_fecha(nueva_fecha: String) -> Result<(), String> {

    // Parse the input date (assuming the input format is "yyyy-mm-dd")
    let parsed_date = NaiveDate::parse_from_str(&nueva_fecha, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse date: {}", e))?;

    // Format the date as "dd-mm-yyyy"
    let formatted_date = parsed_date.format("%d-%m-%Y").to_string();

    println!("Nueva fecha: {}", formatted_date);

Ok(())
}

