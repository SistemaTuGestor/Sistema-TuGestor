
use calamine::{open_workbook, Reader, Xlsx};
use serde::Serialize;
use std::path::Path;
use xlsxwriter::*;
use std::sync::Mutex;
use lazy_static::lazy_static;
use chrono::NaiveDate;

#[derive(Serialize)]
pub struct Fecha {
    fecha: String,
}


#[tauri::command]
pub fn reportes_lee_actualizar_fecha(nueva_fecha: String) -> Result<(), String> {

    // Parse the input date (assuming the input format is "yyyy-mm-dd")
    let parsed_date = NaiveDate::parse_from_str(&nueva_fecha, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse date: {}", e))?;

    // Format the date as "dd-mm-yyyy"
    let formatted_date = parsed_date.format("%d-%m-%Y").to_string();

    println!("Nueva fecha: {}", formatted_date);

Ok(())
}

#[derive(Serialize, Debug)]
pub struct DatosMonitoreo {
    nombre_completo: String,
    correo: String,
    minutos: String,
}

lazy_static! {
    static ref NOMBRE_REPORTE: Mutex<String> = Mutex::new(String::new());
    static ref RUTA_CARPETA: Mutex<String> = Mutex::new(String::new());
}

#[tauri::command]
pub fn recibir_path_carpeta(path: String) {
    println!("📂 Ruta de la carpeta recibida: {}", path);
    let mut ruta = RUTA_CARPETA.lock().unwrap();
    *ruta = path;
}

#[tauri::command]
pub fn guardar_nombre_reporte(nombrereporte: String) {
    println!("📂 Nombre del reporte recibido {}", nombrereporte);
    let mut nombre = NOMBRE_REPORTE.lock().unwrap();
    *nombre = nombrereporte;
}

#[tauri::command]
pub fn leer_excel_path_fijo_lee() -> Result<Vec<DatosMonitoreo>, String> {
    println!("➤ Entrando a la función leer_excel_path_fijo_lee");

    let path_str = "C:\\Users\\Javier\\Desktop\\Qualtrics\\Updated_Qualtrics_Seguimiento_Tutores (1).xlsx";
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
            break; // Solo leer los primeros 5 registros
        }

        if row.len() < 13 { // Asegurarse de que haya suficientes columnas
            continue;
        }

        let nombre = row.get(10).map_or("".to_string(), |cell| cell.to_string());
        let apellido = row.get(9).map_or("".to_string(), |cell| cell.to_string());
        let correo = row.get(11).map_or("".to_string(), |cell| cell.to_string());
        let minutos = row.get(22).map_or("".to_string(), |cell| cell.to_string());

        let nombre_completo = format!("{} {}", nombre, apellido);
        println!("Nombre completo: {}", nombre_completo);

        data.push(DatosMonitoreo {
            nombre_completo,
            correo,
            minutos,
        });
    }

    generar_excel(&data)?;
    Ok(data)
}

pub fn generar_excel(data: &Vec<DatosMonitoreo>) -> Result<(), String> {
    let nombre_reporte = "Alberto";//NOMBRE_REPORTE.lock().unwrap();; 
    //nombre_reporte.clone()
    let output_path = format!("C:\\Users\\Javier\\Downloads\\{}.xlsx", nombre_reporte);
    let mut workbook = Workbook::new(output_path.as_str()).map_err(|e| e.to_string())?;
    let mut sheet = workbook.add_worksheet(None).map_err(|e| e.to_string())?;

    // Escribir encabezados
    sheet.write_string(0, 0, "Correo_Tutor", None).unwrap();
    sheet.write_string(0, 1, "Tiempo (minutos)", None).unwrap();

    // Escribir datos
    for (i, dato) in data.iter().enumerate() {
        sheet.write_string(i as u32 + 1, 0, &dato.correo, None).unwrap();
        sheet.write_string(i as u32 + 1, 1, &dato.minutos, None).unwrap();
    }

    workbook.close().map_err(|e| e.to_string())?;
    println!("✔ Archivo generado en: {}", output_path);
    
    Ok(())
}

