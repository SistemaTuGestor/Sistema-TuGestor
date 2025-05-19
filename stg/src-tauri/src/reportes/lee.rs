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
static PATH_EMPAREJAMIENTO : OnceCell<Mutex<String>> = OnceCell::new() ;
static PATH_CARPETA: OnceCell<Mutex<String>> = OnceCell::new();
static NOMBRE_REPORTE: OnceCell<Mutex<String>> = OnceCell::new();

////    FECHA   ////

#[derive(Serialize)]
pub struct Fecha {
    fecha: String,
}

#[tauri::command]
pub fn reportes_lee_actualizarfecha ( nueva_fecha:Option<String> ) -> Result<(),String> {
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

////    ARCHIVO EMPAREJAMIENTO  ////

#[derive(Serialize)]
pub struct NombreArchivo {
    nombre:String ,
}

#[tauri::command]
pub fn reportes_lee_recibir_emparejamiento ( path:String ) -> Result<(),String> {

    let nombre = PATH_EMPAREJAMIENTO.get_or_init(|| Mutex::new(String::new())) ;
    
    let mut nombre_guardado = nombre.lock().unwrap() ;
    *nombre_guardado = path ;

    // println! ( "📂 Ruta archivo recibido (Emparejamiento): {}",*nombre_guardado ) ;

Ok(())
}

////    PATH    ////

#[derive(Serialize)]
pub struct NombreCarpeta {
    nombre: String,
}

#[tauri::command]
pub fn reportes_lee_recibir_pathcarpeta ( path:String ) -> Result<(),String> {
    
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
pub fn reportes_lee_recibir_nombrereporte ( nombrereporte:String ) -> Result<(),String> {
    
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
    institucion: String,
    horas: String,
}

#[tauri::command]
pub fn reportes_lee_leer_archivos_en_carpeta() -> Result<Vec<DatosMonitoreo>, String> {
    // Leer los emparejamientos primero
    let emparejamientos = reportes_lee_leer_archivo_emparejamiento()?;

    // Crear un mapa de emparejamientos para búsqueda rápida por correo
    let emparejamientos_map: HashMap<String, (String, String, String, String)> = emparejamientos
        .into_iter()
        .map(|e| (e.correo.clone(), (e.nombre_completo, e.institucion, e.horas, e.modalidad)))
        .collect();

    // Leer los archivos de la carpeta
    let carpeta_path = PATH_CARPETA.get().expect("Global variable not initialized");
    let carpeta_path_guard = carpeta_path.lock().unwrap();
    let archivos = fs::read_dir(carpeta_path_guard.as_str())
        .map_err(|e| format!("Error al leer la carpeta: {}", e))?;

    let mut registros: HashMap<String, (String, String, String, String, Vec<u32>, u32)> = HashMap::new();

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

        for row in range.rows().skip(1) {
            if row.len() < 13 {
                continue;
            }

            let correo = row.get(11).map_or("".to_string(), |cell| cell.to_string());
            let minutos = row.get(22).map_or("0".to_string(), |cell| cell.to_string()).parse::<u32>().unwrap_or(0);

            // Verificar si el correo está en los emparejamientos
            if let Some((nombre_completo, institucion, horas, modalidad)) = emparejamientos_map.get(&correo) {
                registros.entry(correo.clone()).and_modify(|(_, _, _, _, semanas, total_minutos)| {
                    semanas.push(minutos);
                    *total_minutos += minutos;
                }).or_insert((
                    nombre_completo.clone(),
                    institucion.clone(),
                    horas.clone(),
                    modalidad.clone(),
                    vec![minutos],
                    minutos,
                ));
            }
        }
    }

    // Convertir los registros en el formato final
    let data: Vec<DatosMonitoreo> = registros
        .into_iter()
        .map(|(correo, (nombre_completo, institucion, horas, modalidad, minutos_por_semana, minutos_totales))| {
            DatosMonitoreo {
                nombre_completo,
                correo,
                institucion,
                horas,
                modalidad,
                minutos_por_semana,
                minutos_totales,
                horas_totales: minutos_totales as f32 / 60.0,
            }
        })
        .collect();

    let emparejamientos = reportes_lee_leer_archivo_emparejamiento()?;
    let data_actualizada = actualizar_horas(data, emparejamientos);
    generar_excel(&data_actualizada)?;

Ok(data_actualizada)
}

#[tauri::command]
pub fn reportes_lee_leer_archivo_emparejamiento() -> Result<Vec<Emparejamiento>, String> {

    // println! ( "📂 Leyendo archivo de emparejamiento..." ) ;
    let mut registros : Vec<Emparejamiento> = Vec::new() ;

    let ubicacioon = PATH_EMPAREJAMIENTO
        .get()
        .ok_or("❌ PATH_EMPAREJAMIENTO no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;
    let path = Path::new(&*ubicacioon);
    // println! ( "Ruta del archivo: {}",path.display() ) ;

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
        let institucion = row.get(4).map_or("".to_string(), |cell| cell.to_string());
        let modalidad = row.get(7).map_or("".to_string(), |cell| cell.to_string());
        let horas = row.get(8).map_or("".to_string(), |cell| cell.to_string());

        let nombre_completo = format!("{} {}", nombre, apellido);

        registros.push(Emparejamiento {
            nombre_completo: nombre_completo.clone(),
            correo: correo.clone(),
            institucion: institucion.clone(),
            modalidad: modalidad.clone(),
            horas: horas.clone(),
        });

        //println!("Nombre: {} | Correo: {} | Horas: {} | Modalidad: {}", nombre_completo, correo, horas, modalidad);
    }

Ok(registros)
}

pub fn actualizar_horas(mut datos_monitoreo: Vec<DatosMonitoreo>, emparejamientos: Vec<Emparejamiento>) -> Vec<DatosMonitoreo> {
    let emparejamientos_map: HashMap<String, (String, String, String)> = emparejamientos.into_iter()
        .map(|e| (e.correo, (e.nombre_completo, e.horas, e.modalidad)))
        .collect();

    for dato in &mut datos_monitoreo {
        if let Some((nombre_completo,horas, modalidad)) = emparejamientos_map.get(&dato.correo) {
            dato.nombre_completo = nombre_completo.clone();
            dato.horas = horas.clone();
            dato.modalidad = modalidad.clone();
            
        }
    }

    //println!("✔ Horas actualizadas");
    //println!("📂 Datos actualizados: {:#?}", datos_monitoreo);

    datos_monitoreo
  
}

pub fn generar_excel(data: &Vec<DatosMonitoreo>) -> Result<(), String> {

    // Se obtiene el nombre del reporte de la variable global.
    let nombre_reporte = NOMBRE_REPORTE
        .get()
        .ok_or("❌ NOMBRE_REPORTE no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

    // Se obtiene la fecha de la variable global.
    let fecha = FECHA
        .get()
        .ok_or("❌ FECHA no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

    // Construir el nuevo nombre del archivo con la fecha.
    let output_path = format!("{} ({}).xlsx", nombre_reporte, *fecha);
 
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
   //println!("✔ Encabezados escritos");

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



// TESTING

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::PathBuf;
    use std::fs;

    // Función helper para obtener rutas de prueba
    fn get_test_data_path(filename: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../../recursos/test_data");
        path.push(filename);
        path
    }

    #[test]
    fn test_actualizar_fecha() {
        let result = reportes_lee_actualizarfecha(Some("2023-05-15".to_string()));
        assert!(result.is_ok());
        
        let fecha_guardada = FECHA.get().unwrap().lock().unwrap();
        assert_eq!(*fecha_guardada, "15-05-2023");
    }

    #[test]
    fn test_recibir_paths() {
        assert!(reportes_lee_recibir_emparejamiento("test_emparejamiento.xlsx".to_string()).is_ok());
        assert!(reportes_lee_recibir_pathcarpeta("test_carpeta".to_string()).is_ok());
        assert!(reportes_lee_recibir_nombrereporte("Test Report".to_string()).is_ok());
    }

    #[test]
    #[ignore = "Requiere archivo de emparejamiento"]
    fn test_leer_archivo_emparejamiento() {
        let test_file = get_test_data_path("emparejamiento_final.xlsx");
        assert!(test_file.exists(), "Archivo de emparejamiento no encontrado: {:?}", test_file);
        
        reportes_lee_recibir_emparejamiento(test_file.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_EMPAREJAMIENTO");
        
        let result = reportes_lee_leer_archivo_emparejamiento();
        assert!(result.is_ok(), "Error: {:?}", result.err());
        
        let emparejamientos = result.unwrap();
        assert!(!emparejamientos.is_empty(), "El archivo debería contener emparejamientos");
    }

    #[test]
    #[ignore = "Requiere carpeta con archivos de monitoreo"]
    fn test_leer_archivos_en_carpeta() {
        // Configurar paths de prueba
        let emparejamiento_path = get_test_data_path("emparejamiento_final.xlsx");
        let carpeta_path = get_test_data_path("test_carpeta");
        
        assert!(emparejamiento_path.exists(), "Archivo de emparejamiento no encontrado");
        assert!(carpeta_path.exists(), "Carpeta de monitoreo no encontrada");
        
        reportes_lee_recibir_emparejamiento(emparejamiento_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_EMPAREJAMIENTO");
            
        reportes_lee_recibir_pathcarpeta(carpeta_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_CARPETA");
        
        let result = reportes_lee_leer_archivos_en_carpeta();
        assert!(result.is_ok(), "Error: {:?}", result.err());
        
        let datos = result.unwrap();
        assert!(!datos.is_empty(), "Debería haber datos procesados");
    }

    #[test]
    #[ignore = "Requiere archivos de prueba completos"]
    fn test_generar_excel() {
        // Configuración inicial
        let emparejamiento_path = get_test_data_path("emparejamiento_final.xlsx");
        let carpeta_path = get_test_data_path("test_carpeta");
        
        reportes_lee_recibir_emparejamiento(emparejamiento_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_EMPAREJAMIENTO");
            
        reportes_lee_recibir_pathcarpeta(carpeta_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_CARPETA");
            
        reportes_lee_recibir_nombrereporte("Test Report".to_string())
            .expect("Error al configurar NOMBRE_REPORTE");
            
        reportes_lee_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Error al configurar FECHA");
        
        // Obtener datos de prueba
        let datos = reportes_lee_leer_archivos_en_carpeta()
            .expect("Error al obtener datos de prueba");
        
        // Ejecutar prueba
        let result = generar_excel(&datos);
        assert!(result.is_ok(), "Error al generar Excel: {:?}", result.err());
        
        // Verificar archivo generado
        let output_file = Path::new("Test Report (01-01-2023).xlsx");
        assert!(output_file.exists(), "No se generó el archivo Excel");
        
        // Limpieza
        fs::remove_file(output_file).ok();
    }

    #[test]
    #[ignore = "Prueba de rendimiento - requiere archivos reales"]
    fn test_rendimiento_procesamiento_lee ( ) {

        use std::time::Instant;
        
        // Configurar paths de prueba
        let emparejamiento_path = get_test_data_path("emparejamiento_final.xlsx");
        let carpeta_path = get_test_data_path("test_carpeta");
        
        println!("📊 Iniciando prueba de rendimiento para procesamiento LEE...");
        println!("📂 Archivo de emparejamiento: {}", emparejamiento_path.display());
        println!("📂 Carpeta de monitoreo: {}", carpeta_path.display());
        
        // Configuración inicial
        reportes_lee_recibir_emparejamiento(emparejamiento_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_EMPAREJAMIENTO");
        reportes_lee_recibir_pathcarpeta(carpeta_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_CARPETA");
        reportes_lee_recibir_nombrereporte("Test Rendimiento LEE".to_string())
            .expect("Error al configurar NOMBRE_REPORTE");
        reportes_lee_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Error al configurar FECHA");

        let mut total_rows = 0 ;

        // 🔽 LÍNEAS AGREGADAS: contar registros por archivo XLSX
        let archivos = fs::read_dir(carpeta_path).expect("No se pudo leer la carpeta de monitoreo");
        for entrada in archivos {
            let entrada = entrada.expect("No se pudo leer una entrada");
            let path = entrada.path();
            if path.extension().and_then(|s| s.to_str()) == Some("xlsx") {
                let mut workbook: Xlsx<_> = open_workbook(&path).expect("No se pudo abrir el archivo XLSX");
                if let Ok(range) = workbook.worksheet_range("Sheet1") {
                    let row_count = range.rows().skip(1).count();
                    total_rows += row_count ;
                    println!("📊 Archivo: {:?} - Registros: {}", path.file_name().unwrap(), row_count);
                }
            }
        }

        println!("\n📊 Total Registros: {}\n", total_rows);
        
        // Medir tiempo de ejecución
        let start_time = Instant::now();
        
        // Ejecutar funciones a medir
        let emparejamientos = reportes_lee_leer_archivo_emparejamiento()
            .expect("Error al leer emparejamientos");
        println!("📈 Emparejamientos cargados: {}", emparejamientos.len());
        
        let datos = reportes_lee_leer_archivos_en_carpeta()
            .expect("Error al procesar archivos de monitoreo");
        println!("📈 Registros procesados: {}", datos.len());
        
        // Mostrar distribución por institución
        let mut por_institucion: HashMap<String, usize> = HashMap::new();
        for dato in &datos {
            *por_institucion.entry(dato.institucion.clone()).or_default() += 1;
        }
        
        println!("📊 Distribución por institución:");
        for (institucion, count) in por_institucion {
            println!("   • {}: {} registros", institucion, count);
        }
        
        // Mostrar estadísticas de tiempo
        let duration = start_time.elapsed();
        println!("⏱️ Tiempo total de procesamiento: {:.2?}", duration);
        println!("📊 Prueba de rendimiento completada");
        
        // Limpieza (opcional)
        let output_file = Path::new("Test Rendimiento LEE (01-01-2023).xlsx");
        if output_file.exists() {
            fs::remove_file(output_file).expect("Error al limpiar archivo generado");
            println!("🧹 Archivo temporal eliminado");
        }
    }

}

