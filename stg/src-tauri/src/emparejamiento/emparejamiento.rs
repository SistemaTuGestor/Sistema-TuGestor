use calamine::{open_workbook, Reader, Xlsx, DataType};
use serde::Serialize;
use tauri::command; 

const ARCHIVO_EXCEL: &str = "C:/Users/darve/OneDrive/Documentos/GitHub/tugestor/Sistema-TuGestor/recursos/emparejamiento.xlsx";

#[derive(Debug, Serialize)]

pub struct EmparejamientoItem {  
    pub tutor: String,           
    pub materiaTutor: String,
    pub tutorado1: String,
    pub tutorado2: String,
    pub materiaTutorado1: String,
    pub materiaTutorado2: String,
    pub disponibilidad: String,
}


#[tauri::command] 
pub fn obtener_emparejamiento() -> Result<Vec<EmparejamientoItem>, String> {
    let mut workbook: Xlsx<_> = open_workbook(ARCHIVO_EXCEL)
        .map_err(|e| format!("❌ No se pudo abrir el archivo Excel: {}", e))?;

    println!("📂 Archivo Excel abierto correctamente.");

    let sheet_names = workbook.sheet_names();
    println!("📄 Hojas disponibles en el archivo: {:?}", sheet_names);

    let range = workbook
        .worksheet_range("Emparejamiento")
        .map_err(|e| format!("❌ No se pudo cargar la hoja 'Emparejamiento': {}", e))?;

    println!("📊 Datos encontrados en la hoja: {:?}", range);

    let mut emparejamientos = Vec::new();
    
    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue; // Saltar encabezado
        }

        println!("➡ Procesando fila {}: {:?}", i, row);

        let tutor = format!(
            "{} {}",
            row.get(0).and_then(|c| c.as_string()).map(|s| s.to_string()).unwrap_or_else(|| "VACÍO".to_string()),
            row.get(1).and_then(|c| c.as_string()).map(|s| s.to_string()).unwrap_or_else(|| "VACÍO".to_string())
        );

        let materiaTutor = row.get(6)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materiaTutorado1 = row.get(17)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materiaTutorado2 = row.get(36)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());

        let tutorado1 = row.get(10)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let tutorado2 = row.get(30)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());

        let disponibilidad = row.get(9)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());

        println!("👤 Tutor: {}, materiaTutor: {},materiaTutorado1: {},materiaTutorado2: {}, Tutorado1: {}, Tutorado2: {}, disponibilidad: {} ", tutor, materiaTutor,materiaTutorado1,materiaTutorado2, tutorado1, tutorado2, disponibilidad);

        emparejamientos.push(EmparejamientoItem {
            tutor,
            materiaTutor,
            tutorado1,
            tutorado2,
            materiaTutorado1,
            materiaTutorado2,
            disponibilidad,
        });
    }

    println!("✅ Emparejamiento generado con {} elementos.", emparejamientos.len());

    Ok(emparejamientos)
}
