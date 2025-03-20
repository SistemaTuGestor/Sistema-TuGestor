// VARIOS
use serde::Serialize;
// FECHA
use chrono::Local;
use chrono::NaiveDate;
// PATH
use std::sync::Mutex;
use once_cell::sync::OnceCell;
// ARCHIVOS
use std::fs;
use std::collections::HashMap;
use calamine::{open_workbook, Reader, Xlsx};
use xlsxwriter::*;
use std::path::Path;

static FECHA: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_CARPETA: OnceCell<Mutex<String>> = OnceCell::new();
static NOMBRE_REPORTE: OnceCell<Mutex<String>> = OnceCell::new();

////    FECHA   ////

#[derive(Serialize)]
pub struct Fecha {
    fecha: String,
}

#[tauri::command]
pub fn reportes_lee_actualizarfecha(nueva_fecha: Option<String>) -> Result<(), String> {
    let fecha = match nueva_fecha {
        Some(fecha) => {
            let parsed_date = NaiveDate::parse_from_str(&fecha, "%Y-%m-%d")
                .map_err(|e| format!("Failed to parse date: {}", e))?;
            parsed_date.format("%d-%m-%Y").to_string()
        }
        None => Local::now().format("%d-%m-%Y").to_string(),
    };

    FECHA.get_or_init(|| Mutex::new(String::new()))
        .lock()
        .map_err(|e| format!("Failed to lock mutex: {}", e))?
        .clone_from(&fecha);

    // println! ( "Nueva fecha (LEE): {}", fecha ) ;

    Ok(())
}

////    PATH    ////

#[derive(Serialize)]
pub struct NombreCarpeta {
    nombre: String,
}

#[tauri::command]
pub fn reportes_lee_recibir_pathcarpeta(path: String) -> Result<(), String> {
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
pub fn reportes_lee_recibir_nombrereporte(nombrereporte: String) -> Result<(), String> {
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
    institucion: String,
    horas: String,
    modalidad: String,
    minutos_por_semana: Vec<u32>,
    minutos_totales: u32,
    horas_totales: f32,
}

#[derive(Serialize, Debug)]
pub struct Emparejamiento {
    nombre_completo: String,
    correo: String,
    modalidad: String,
    horas: String,
}

#[tauri::command]
pub fn reportes_lee_leer_archivos_en_carpeta() -> Result<Vec<DatosMonitoreo>, String> {
    let mut registros: HashMap<String, (String, String, String, Vec<u32>, u32)> = HashMap::new();

    let carpeta_path = PATH_CARPETA.get().expect("Global variable not initialized");
    // println!("📂 Ruta de la carpeta recibida (LEE): {}",PATH_CARPETA.get().unwrap().lock().unwrap()) ;
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
            let institucion = row.get(19).map_or("".to_string(), |cell| cell.to_string());
            let horas = row.get(20).map_or("".to_string(), |cell| cell.to_string());
            let minutos = row.get(22).map_or("0".to_string(), |cell| cell.to_string()).parse::<u32>().unwrap_or(0);

            let nombre_completo = format!("{} {}", nombre, apellido);

            registros.entry(correo.clone()).and_modify(|(_, _, _, semanas, total_minutos)| {
                semanas.push(minutos);
                *total_minutos += minutos;
            }).or_insert((nombre_completo, institucion, horas, vec![minutos], minutos));
        }
    }

    let data: Vec<DatosMonitoreo> = registros.into_iter().map(|(correo, (nombre_completo, institucion, horas, minutos_por_semana, minutos_totales))| {
       // println!("Correo: {} | Nombre: {} | Institucion: {} | Horas: {} | Minutos por semana: {:?} | Minutos totales: {} | Horas totales: {:.2}", correo, nombre_completo, institucion, horas, minutos_por_semana, minutos_totales, minutos_totales as f32 / 60.0);
        DatosMonitoreo {
            nombre_completo,
            correo,
            institucion,
            horas,
            modalidad: "".to_string(),
            minutos_por_semana,
            minutos_totales,
            horas_totales: minutos_totales as f32 / 60.0,
        }
    }).collect();

    let emparejamientos = reportes_lee_leer_archivo_emparejamiento()?;
    let data_actualizada = actualizar_horas(data, emparejamientos);
    generar_excel(&data_actualizada)?;
    Ok(data_actualizada)
}

#[tauri::command]
pub fn reportes_lee_leer_archivo_emparejamiento() -> Result<Vec<Emparejamiento>, String> {
    println!("📂 Leyendo archivo de emparejamiento...");
    let mut registros: Vec<Emparejamiento> = Vec::new();

    // 🔥 Ruta quemada del archivo Excel
    let archivo_excel = "C:\\Users\\USUARIO\\Downloads\\ejemplo.xlsx";  // Cambia la ruta por la correcta

    let path = Path::new(&archivo_excel);
   // println!("Ruta del archivo: {}", path.display());

    // Intentar abrir el archivo
    let mut workbook: Xlsx<_> = match open_workbook(path) {
        Ok(wb) => wb,
        Err(e) => return Err(format!("Error al abrir el archivo: {}", e)),
    };

    // Intentar acceder a la hoja "Emparejamiento"
    let range = match workbook.worksheet_range("Emparejamiento") {
        Ok(r) => r,
        Err(e) => return Err(format!("No se pudo cargar la hoja 'Emparejamiento': {}", e)),
    };

    for row in range.rows().skip(1) { // Omitir encabezados
        if row.len() < 8 {
            continue;
        }

        let nombre = row.get(0).map_or("".to_string(), |cell| cell.to_string());
        let apellido = row.get(1).map_or("".to_string(), |cell| cell.to_string());
        let correo = row.get(2).map_or("".to_string(), |cell| cell.to_string());
        let modalidad = row.get(7).map_or("".to_string(), |cell| cell.to_string());
        let horas = row.get(8).map_or("".to_string(), |cell| cell.to_string());

        let nombre_completo = format!("{} {}", nombre, apellido);

        registros.push(Emparejamiento {
            nombre_completo: nombre_completo.clone(),
            correo: correo.clone(),
            modalidad: modalidad.clone(),
            horas: horas.clone(),
        });

        //println!("Nombre: {} | Correo: {} | Horas: {} | Modalidad: {}", nombre_completo, correo, horas, modalidad);
    }

    Ok(registros)
}

pub fn actualizar_horas(mut datos_monitoreo: Vec<DatosMonitoreo>, emparejamientos: Vec<Emparejamiento>) -> Vec<DatosMonitoreo> {
    let emparejamientos_map: HashMap<String, (String, String)> = emparejamientos.into_iter()
        .map(|e| (e.correo, (e.horas, e.modalidad)))
        .collect();

    for dato in &mut datos_monitoreo {
        if let Some((horas, modalidad)) = emparejamientos_map.get(&dato.correo) {
            dato.horas = horas.clone();
            dato.modalidad = modalidad.clone();
            
        }
    }
   // println!("✔ Horas actualizadas");
   // println!("📂 Datos actualizados: {:#?}", datos_monitoreo);
    datos_monitoreo
  
}

pub fn generar_excel(data: &Vec<DatosMonitoreo>) -> Result<(), String> {
    // Se obtiene el nombre del reporte de la variable global.
    let output_path = NOMBRE_REPORTE
        .get()
        .ok_or("❌ NOMBRE_REPORTE no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

    //println!("📂 Generando archivo Excel en: {}", output_path);

    // Check if the file already exists and try to delete it
    if Path::new(&*output_path).exists() {
        println!("⚠ Archivo ya existe, intentando eliminarlo...");
        fs::remove_file(&*output_path).map_err(|e| format!("Error al eliminar el archivo existente: {}", e))?;
        println!("✔ Archivo existente eliminado");
    }

    // Crear el archivo de Excel.
    let workbook = Workbook::new(&output_path)
        .map_err(|e| format!("Error creating workbook: {}", e))?;
    //println!("✔ Workbook creado");

    let mut sheet = workbook.add_worksheet(None).map_err(|e| format!("Error adding worksheet: {}", e))?;
    //println!("✔ Worksheet agregado");

    // Encabezados con formato de semanas dinámicas
    sheet.write_string(0, 0, "Correo", None).unwrap();
    sheet.write_string(0, 1, "Nombre_tutorado", None).unwrap();
    sheet.write_string(0, 2, "Institucion", None).unwrap();
    sheet.write_string(0, 3, "Horas", None).unwrap();
    sheet.write_string(0, 4, "Modalidad", None).unwrap();
   // println!("✔ Encabezados escritos");

    // Agregar encabezados para cada semana
    let max_semanas = data.iter().map(|d| d.minutos_por_semana.len()).max().unwrap_or(0);
    for i in 0..max_semanas {
        sheet.write_string(0, (i + 5) as u16, &format!("Semana {}", i + 1), None).unwrap();
    }
    //println!("✔ Encabezados de semanas escritos");

    // Agregar columnas de total y horas
    sheet.write_string(0, (max_semanas + 5) as u16, "Minutos totales", None).unwrap();
    sheet.write_string(0, (max_semanas + 6) as u16, "Horas totales", None).unwrap();
   // println!("✔ Columnas de total y horas escritos");

    for (i, dato) in data.iter().enumerate() {
        sheet.write_string((i + 1) as u32, 0, &dato.correo, None).unwrap();
        sheet.write_string((i + 1) as u32, 1, &dato.nombre_completo, None).unwrap();
        sheet.write_string((i + 1) as u32, 2, &dato.institucion, None).unwrap();
        sheet.write_string((i + 1) as u32, 3, &dato.horas, None).unwrap();
        sheet.write_string((i + 1) as u32, 4, &dato.modalidad, None).unwrap();

        // Escribir minutos por semana
        for (j, min_semana) in dato.minutos_por_semana.iter().enumerate() {
            sheet.write_number((i + 1) as u32, (j + 5) as u16, *min_semana as f64, None).unwrap();
        }

        // Escribir totales
        sheet.write_number((i + 1) as u32, (max_semanas + 5) as u16, dato.minutos_totales as f64, None).unwrap();
        sheet.write_number((i + 1) as u32, (max_semanas + 6) as u16, dato.horas_totales as f64, None).unwrap();
    }
    //println!("✔ Datos escritos");

    workbook.close().map_err(|e| format!("Error closing workbook: {}", e))?;
    //println!("✔ Workbook cerrado");

    Ok(())
}

