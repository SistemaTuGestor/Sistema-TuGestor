
#[allow(unused_imports)]
// VARIOS
use serde::Serialize ;
// FECHA
use chrono::NaiveDate ;
// PATH
use once_cell::sync::OnceCell;
use std::sync::Mutex;
// ARCHIVOS
use std::fs;
use std::collections::HashMap;
use calamine::{open_workbook, Reader, Xlsx} ;
use std::path::Path ;
use xlsxwriter::* ;



static FECHA : OnceCell<Mutex<String>> = OnceCell::new() ;
static PATH_CARPETA : OnceCell<Mutex<String>> = OnceCell::new() ;
static NOMBRE_REPORTE : OnceCell<Mutex<String>> = OnceCell::new() ;



////    FECHA   ////

#[derive(Serialize)]
pub struct Fecha {
    fecha: String,
}


#[tauri::command]
pub fn reportes_lee_actualizar_fecha(nueva_fecha: String) -> Result<(),String> {

    let parsed_date = NaiveDate::parse_from_str(&nueva_fecha, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse date: {}", e))?;

    let formatted_date = parsed_date.format("%d-%m-%Y").to_string();

    // println!("Nueva fecha (LEE): {}", formatted_date);

Ok(())
}


////    PATH    ////

#[derive(Serialize)]
pub struct NombreCarpeta {
    nombre: String,
}

#[tauri::command]
pub fn reportes_lee_recibir_pathcarpeta(path: String) -> Result<(),String> {

    // Initialize the global variable if it hasn't been initialized yet
    let nombre = PATH_CARPETA.get_or_init(|| Mutex::new(String::new()));
    
    // Store the report name in the global variable
    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = path;

    // println!("📂 Ruta de la carpeta recibida (LEE): {}",path) ;

Ok(())
}

#[derive(Serialize)]
pub struct NombreReporte {
    nombre: String,
}


////    NOMBRE REPORTE     ////

#[tauri::command]
pub fn reportes_lee_recibir_nombrereporte (nombrereporte: String) -> Result<(),String> {
    
    // Initialize the global variable if it hasn't been initialized yet
    let nombre = NOMBRE_REPORTE.get_or_init(|| Mutex::new(String::new()));
    
    // Store the report name in the global variable
    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = nombrereporte;
    
    // println!("📂 Nombre del reporte (LEE): {}",nombrereporte) ;

Ok(())
}


////    LÓGICA DE GENERAR REPORTE     ////

#[derive(Serialize, Debug)]
pub struct DatosMonitoreo {
    nombre_completo: String,
    correo: String,
    minutos_por_semana: Vec<u32>,
    minutos_totales: u32,
    horas_totales: f32,
}

#[tauri::command]
pub fn leer_archivos_en_carpeta() -> Result<Vec<DatosMonitoreo>, String> {

    let mut registros: HashMap<String, (String, Vec<u32>, u32)> = HashMap::new();
    
    let carpeta_path = PATH_CARPETA.get().expect("Global variable not initialized");
    println!("📂 Ruta de la carpeta recibida (LEE): {}",PATH_CARPETA.get().unwrap().lock().unwrap()) ;
    let carpeta_path_guard = carpeta_path.lock().unwrap(); 
    let archivos = fs::read_dir(carpeta_path_guard.as_str()).map_err(|e| format!("Error al leer la carpeta: {}", e))?;
    
    for entrada in archivos {
        let entrada = entrada.map_err(|e| format!("Error al leer un archivo en la carpeta: {}", e))?;
        let path = entrada.path();
        if path.extension().and_then(|s| s.to_str()) != Some("xlsx") {
            continue;
        }
        
        let mut workbook: Xlsx<_> = match open_workbook(&path) {
            Ok(wb) => wb,
            Err(e) => {
                println!("✖ ERROR al abrir el archivo: {}", e);
                continue;
            }
        };
        
        let range = match workbook.worksheet_range("Sheet1") {
            Ok(r) => r,
            Err(e) => {
                println!("✖ ERROR: No se pudo cargar la hoja 'Sheet1'. {}", e);
                continue;
            }
        };
        
        for row in range.rows().skip(1) { // Omitir encabezados
            if row.len() < 13 {
                continue;
            }
            
            let nombre = row.get(10).map_or("".to_string(), |cell| cell.to_string());
            let apellido = row.get(9).map_or("".to_string(), |cell| cell.to_string());
            let correo = row.get(11).map_or("".to_string(), |cell| cell.to_string());
            let minutos = row.get(22).map_or("0".to_string(), |cell| cell.to_string()).parse::<u32>().unwrap_or(0);
            
            let nombre_completo = format!("{} {}", nombre, apellido);
            
            registros.entry(correo.clone()).and_modify(|(_, semanas, total_minutos)| {
                semanas.push(minutos);
                *total_minutos += minutos;
            }).or_insert((nombre_completo, vec![minutos], minutos));
        }
    }
    
    let data: Vec<DatosMonitoreo> = registros.into_iter().map(|(correo, (nombre_completo, minutos_por_semana, minutos_totales))| {
        println!("Correo: {} | Nombre: {} | Minutos por semana: {:?} | Minutos totales: {} | Horas totales: {:.2}", correo, nombre_completo, minutos_por_semana, minutos_totales, minutos_totales as f32 / 60.0);
        DatosMonitoreo {
            nombre_completo,
            correo,
            minutos_por_semana,
            minutos_totales,
            horas_totales: minutos_totales as f32 / 60.0,
        }
    }).collect();
    
    generar_excel(&data)?;
    Ok(data)
}

pub fn generar_excel(data: &Vec<DatosMonitoreo>) -> Result<(), String> {

    let output_path = NOMBRE_REPORTE.get().unwrap().lock().unwrap() ;
    println!("📂 Nombre del reporte (LEE): {}",output_path) ;
    
    let workbook = Workbook::new(&output_path).map_err(|e| e.to_string())?;
    let mut sheet = workbook.add_worksheet(None).map_err(|e| e.to_string())?;
    
    // Encabezados con formato de semanas dinámicas
    sheet.write_string(0, 0, "Correo", None).unwrap();
    sheet.write_string(0, 1, "Nombre_tutorado", None).unwrap();
    
    // Agregar encabezados para cada semana
    let max_semanas = data.iter().map(|d| d.minutos_por_semana.len()).max().unwrap_or(0);
    for i in 0..max_semanas {
        sheet.write_string(0, (i + 2) as u16, &format!("Semana {}", i + 1), None).unwrap();
    }
    
    // Agregar columnas de total y horas
    sheet.write_string(0, (max_semanas + 2) as u16, "Minutos totales", None).unwrap();
    sheet.write_string(0, (max_semanas + 3) as u16, "Horas totales", None).unwrap();
    
    for (i, dato) in data.iter().enumerate() {
        sheet.write_string((i + 1) as u32, 0, &dato.correo, None).unwrap();
        sheet.write_string((i + 1) as u32, 1, &dato.nombre_completo, None).unwrap();
        
        // Escribir minutos por semana
        for (j, min_semana) in dato.minutos_por_semana.iter().enumerate() {
            sheet.write_number((i + 1) as u32, (j + 2) as u16, *min_semana as f64, None).unwrap();
        }
        
        // Escribir totales
        sheet.write_number((i + 1) as u32, (max_semanas + 2) as u16, dato.minutos_totales as f64, None).unwrap();
        sheet.write_number((i + 1) as u32, (max_semanas + 3) as u16, dato.horas_totales as f64, None).unwrap();
    }
    
    workbook.close().map_err(|e| e.to_string())?;
    println!("✔ Archivo generado en: {}", output_path);
    
Ok(())
}


////    NUEVO NOMBRE DEL REPORTE    ////

/*
pub fn reportes_lee_guardar_nombrereporte ( ) {
    
    let nuevafecha = FECHA ;
    let nuevonombre = NOMBRE_REPORTE ;

    let combined_result = format!("{}+{}", nueva_fecha, nuevo_nombre);

Ok ( combined_result )
}
*/

