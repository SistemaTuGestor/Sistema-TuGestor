use calamine::{open_workbook, DataType, Reader, Xlsx};
use serde::Serialize;
use std::path::Path;

#[derive(Serialize, Debug)]
pub struct DatosMonitoreo {
    direccion_ip: String,
    progreso: String,  // Cambiado de i32 a String
    segundos: String,  // Cambiado de i32 a String
    correo: String,
}

#[tauri::command] // Para poder llamarla desde TypeScript en Tauri
pub fn leer_excel_path_fijo_lee() -> Result<Vec<DatosMonitoreo>, String> {
    println!("➤ Entrando a la función `leer_excel_path_fijo_lee`");

    let path_str = "C:\\Users\\USUARIO\\Downloads\\Updated_Qualtrics_Seguimiento_Tutores.xlsx";
    let path = Path::new(path_str);
    println!("➤ Intentando abrir el archivo en la ruta: {}", path_str);

    if !path.exists() {
        println!("✖ ERROR: El archivo no existe en la ruta especificada.");
        return Err(format!("Archivo no encontrado: {}", path_str));
    }

    let mut workbook: Xlsx<_> = match open_workbook(path) {
        Ok(wb) => {
            println!("✔ Archivo abierto correctamente.");
            wb
        },
        Err(e) => {
            println!("✖ ERROR al abrir el archivo: {}", e);
            return Err(format!("Error al abrir el archivo: {}", e));
        }
    };

    let sheet_names = workbook.sheet_names();
    println!("➤ Hojas disponibles en el archivo: {:?}", sheet_names);

    let range = match workbook.worksheet_range("Sheet1") {
        Ok(r) => {
            println!("✔ Hoja 'Sheet1' encontrada y cargada.");
            r
        },
        Err(e) => {
            println!("✖ ERROR: No se pudo cargar la hoja 'Sheet1'. {}", e);
            return Err(format!("Error al cargar 'Sheet1': {}", e));
        }
    };

    let mut data = Vec::new();

    println!("➤ Comenzando la lectura de las filas...");

    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue; // Ignorar la primera fila (encabezados)
        }

        if i >= 6 {
            println!("✔ Se han leído los primeros 5 registros.");
            break;
        }

        if row.len() < 12 {
            println!("✖ ERROR: La fila tiene menos de 12 columnas.");
            continue;
        }

        let direccion_ip = row.get(3).map_or("".to_string(), |cell| cell.to_string());
        let progreso = row.get(4).map_or("".to_string(), |cell| cell.to_string());
        let segundos = row.get(5).map_or("".to_string(), |cell| cell.to_string());
        let correo = row.get(11).map_or("".to_string(), |cell| cell.to_string());

        println!(
            "  ➝ Datos extraídos: Dirección IP: {}, Progreso: {}, Segundos: {}, Correo: {}",
            direccion_ip, progreso, segundos, correo
        );

        data.push(DatosMonitoreo {
            direccion_ip,
            progreso,
            segundos,
            correo,
        });
    }

    println!("✔ Finalizada la lectura del archivo.");
    Ok(data)
}
