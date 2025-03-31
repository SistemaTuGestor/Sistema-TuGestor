
use calamine::{open_workbook, Reader, Xlsx, DataType};
use serde::Serialize;
use std::collections::HashSet;

const ARCHIVO_EXCEL: &str = "C:/Users/darve/OneDrive/Documentos/GitHub/tugestor/Sistema-TuGestor/recursos/emparejamiento.xlsx";

#[derive(Debug, Serialize)]
pub struct EmparejamientoItem {  
    pub tutor: String,           
    pub disponibilidadTutor: String,
    pub materiaTutor: String,
    pub tutorado1: String,
    pub tutorado1_id: String,
    pub disponibilidadTutorado1: String,
    pub materiaTutorado1: String,
    pub tutorado2: String,
    pub tutorado2_id: String,
    pub disponibilidadTutorado2: String,
    pub materiaTutorado2: String,
}

#[tauri::command] 
pub fn obtener_emparejamiento() -> Result<Vec<EmparejamientoItem>, String> {
    let mut workbook: Xlsx<_> = open_workbook(ARCHIVO_EXCEL)
        .map_err(|e| format!("❌ No se pudo abrir el archivo Excel: {}", e))?;

    println!("📂 Archivo Excel abierto correctamente.");
    let sheet_names = workbook.sheet_names();
    println!("📄 Hojas disponibles en el archivo: {:?}", sheet_names);

    // --- Procesar la hoja "Emparejamiento" ---
    let range = workbook
        .worksheet_range("Emparejamiento")
        .map_err(|e| format!("❌ No se pudo cargar la hoja 'Emparejamiento': {}", e))?;
    println!("📊 Datos encontrados en la hoja Emparejamiento: {:?}", range);

    let mut emparejamientos = Vec::new();
    let mut asignados = HashSet::new();

    for (i, row) in range.rows().enumerate() {
        if i == 0 { continue; } // Saltar encabezado

        println!("➡ Procesando fila {}: {:?}", i, row);

        // Datos del tutor
        let tutor = format!(
            "{} {}",
            row.get(0)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string()),
            row.get(1)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string())
        );
        let disponibilidadTutor = row.get(9)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materiaTutor = row.get(6)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());

        // Datos del primer tutorado
        let tutorado1 = row.get(10)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let tutorado1_id = row.get(11)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let disponibilidadTutorado1 = row.get(28)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materiaTutorado1 = row.get(16)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());

        // Datos del segundo tutorado
        let tutorado2 = row.get(30)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let tutorado2_id = row.get(31)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let disponibilidadTutorado2 = row.get(48)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materiaTutorado2 = row.get(36)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());

        // Registrar ids asignados (evitamos "VACÍO")
        if tutorado1_id != "VACÍO" {
            asignados.insert(tutorado1_id.clone());
        }
        if tutorado2_id != "VACÍO" {
            asignados.insert(tutorado2_id.clone());
        }

        println!("👤 Tutor: {} (Disponibilidad: {}), Materia: {} | Tutorado1: {} (ID: {}, Disponibilidad: {}), Materia: {} | Tutorado2: {} (ID: {}, Disponibilidad: {}), Materia: {}",
            tutor, disponibilidadTutor, materiaTutor,
            tutorado1, tutorado1_id, disponibilidadTutorado1, materiaTutorado1,
            tutorado2, tutorado2_id, disponibilidadTutorado2, materiaTutorado2
        );

        emparejamientos.push(EmparejamientoItem {
            tutor,
            disponibilidadTutor,
            materiaTutor,
            tutorado1,
            tutorado1_id,
            disponibilidadTutorado1,
            materiaTutorado1,
            tutorado2,
            tutorado2_id,
            disponibilidadTutorado2,
            materiaTutorado2,
        });
    }

    // --- Procesar la hoja "Tutorados" ---
    let range_tutorados = workbook
        .worksheet_range("Todos los tutorados ")
        .map_err(|e| format!("❌ No se pudo cargar la hoja 'Tutorados': {}", e))?;
    println!("📊 Datos encontrados en la hoja Tutorados: {:?}", range_tutorados);

    for (i, row) in range_tutorados.rows().enumerate() {
        if i == 0 { continue; } // Saltar encabezado

        let tutorado_id = row.get(1)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let tutorado_name = row.get(0)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let disponibilidadTutorado1 = row.get(18)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materia = row.get(6)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());

        if !asignados.contains(&tutorado_id) {
            emparejamientos.push(EmparejamientoItem {
                tutor: "".to_string(),
                disponibilidadTutor: "VACÍO".to_string(),
                materiaTutor: "VACÍO".to_string(),
                tutorado1: tutorado_name,
                tutorado1_id: tutorado_id,
                disponibilidadTutorado1,
                materiaTutorado1: materia,
                tutorado2: "".to_string(),
                tutorado2_id: "".to_string(),
                disponibilidadTutorado2: "VACÍO".to_string(),
                materiaTutorado2: "VACÍO".to_string(),
            });
        }
    }

    println!("✅ Emparejamiento generado con {} elementos.", emparejamientos.len());
    Ok(emparejamientos)
}
