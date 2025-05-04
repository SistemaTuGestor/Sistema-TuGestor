
// FECHA
use chrono::Local ;
use chrono::NaiveDate ;
// PATH
use once_cell::sync::OnceCell ;
use std::sync::Mutex ;
// ARCHIVOS
use std::fs::{self} ;
use std::path::{Path,PathBuf} ;
use std::io::{Read, Write} ;
use calamine::{open_workbook, Reader, Xlsx} ;
use zip::{ZipArchive, write::FileOptions} ;



static FECHA : OnceCell<Mutex<String>> = OnceCell::new() ;
static PATH_EMPAREJAMIENTO: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_PLANTILLA : OnceCell<Mutex<String>> = OnceCell::new() ;
static NOMBRE_REPORTE : OnceCell<Mutex<String>> = OnceCell::new() ;


////    FECHA   ////

#[tauri::command]
pub fn reportes_constanciastutorados_actualizarfecha(nueva_fecha: Option<String>) -> Result<(), String> {

    let fecha = match nueva_fecha {
        Some(fecha) => {
            let parsed_date = NaiveDate::parse_from_str(&fecha, "%Y-%m-%d")
                .map_err(|e| format!("Failed to parse date: {}", e))?;
            parsed_date.format("%d-%m-%Y").to_string()
        }
        None => {
            Local::now().format("%d-%m-%Y").to_string()
        }
    };

    FECHA.get_or_init(|| Mutex::new(String::new()))
        .lock()
        .map_err(|e| format!("Failed to lock mutex: {}", e))?
        .clone_from(&fecha);

Ok(())
}


////    EMPAREJAMIENTO     ////

#[tauri::command]
pub fn reportes_tutorados_recibir_emparejamiento ( path:String ) -> Result<(),String> {

    let nombre = PATH_EMPAREJAMIENTO.get_or_init(|| Mutex::new(String::new()));

    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = path;

    // println!("📂 Ruta de archivo Emparejamiento (Tutorados): {}", *nombre_guardado);

Ok(())
}


////    PATH    ////

#[tauri::command]
pub fn reportes_constanciastutorados_recibir_pathplantilla ( path:String) -> Result<(),String> {

    let nombre = PATH_PLANTILLA.get_or_init(|| Mutex::new(String::new())) ;
    let mut nombre_guardado = nombre.lock().unwrap() ;
    *nombre_guardado = path ;

Ok(())
}


////    NOMBRE REPORTE     ////

#[tauri::command]
pub fn reportes_constanciastutorados_recibir_nombrereporte(nombrereporte: String) -> Result<(), String> {

    let nombre = NOMBRE_REPORTE.get_or_init(|| Mutex::new(String::new()));
    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = nombrereporte;

Ok(())
}



////    LÓGICA DE ARCHIVOS      ////

#[tauri::command]
pub fn reportes_constanciastutorados_generar() -> Result<(), String> {

    // println!("📖 Cargando archivo Excel...");

    let archivo_emparejamiento = PATH_EMPAREJAMIENTO
        .get()
        .ok_or("❌ ARCHIVO_EMPAREJAMIENTO no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

    let path = Path::new(&*archivo_emparejamiento);
    
    let mut workbook: Xlsx<_> = open_workbook(path)
        .map_err(|e| format!("❌ No se pudo abrir el archivo Excel: {}", e))?;

    let range = workbook
        .worksheet_range("Emparejamiento")
        .map_err(|e| format!("❌ No se pudo cargar 'Emparejamiento': {}", e))?;

    // Asegurar que la carpeta de salida exista.
    let directorio = NOMBRE_REPORTE
        .get()
        .ok_or("❌ NOMBRE_REPORTE no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

    fs::create_dir_all(&*directorio).map_err(|e| format!("❌ Error creando carpeta de salida: {}", e))?;

    for (i, row) in range.rows().enumerate() {
        
        if i == 0 {
            println!("⚠ Ignorando encabezado...");
            continue;
        }

        if row.len() < 28 {  // La columna ab es la número 28 (índice 28 en Rust)
            eprintln!("⚠ ERROR: Fila {} tiene menos de 35 columnas, se omite.", i + 1);
            continue;
        }

        let tutorado_1 = row[10].to_string().trim().to_string();
        let tutorado_2 = row[30].to_string().trim().to_string();
        
        let fecha = FECHA
            .get()
            .ok_or("❌ FECHA no ha sido inicializado")?
            .lock()
            .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

        let generar_constancia = |nombre_tutorado: &str| -> Result<(), String> {
            
            if nombre_tutorado.is_empty() {
                return Ok(());
            }

            let salida_docx = PathBuf::from(&*directorio).join(format!(
                "Constancia Tutorado {} ({}).docx",
                nombre_tutorado, fecha
            ));

            let salida_documento = salida_docx.into_os_string()
                .into_string()
                .map_err(|e| format!("❌ Nombre de archivo no válido UTF-8: {:?}", e))?;

            match crear_constancia(&nombre_tutorado, &salida_documento) {
                Ok(_) => println!("✔ Constancia generada: {}", salida_documento),
                Err(e) => eprintln!("❌ Error al generar constancia para {}: {}", nombre_tutorado, e),
            }

        Ok(())
        };

        generar_constancia(&tutorado_1)?;
        generar_constancia(&tutorado_2)?;

    }

    // println!("🎉 ¡Todas las constancias han sido generadas!");

Ok(())
}


fn crear_constancia ( nombre:&str , salida_path:&str ) -> Result<(),String> {

    let path_plantilla = PATH_PLANTILLA
        .get()
        .ok_or("❌ PATH_PLANTILLA no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

    let plantilla_bytes = fs::read(&*path_plantilla)
        .map_err(|e| format!("❌ No se pudo leer la plantilla DOCX: {}", e))?;

    let cursor = std::io::Cursor::new(plantilla_bytes);
    let mut zip = ZipArchive::new(cursor)
        .map_err(|e| format!("❌ No se pudo abrir el archivo DOCX como ZIP: {}", e))?;

    let mut document_xml = String::new();
    {
        let mut file = zip.by_name("word/document.xml")
            .map_err(|e| format!("❌ No se encontró 'word/document.xml' en la plantilla: {}", e))?;
        file.read_to_string(&mut document_xml)
            .map_err(|e| format!("❌ Error al leer el contenido XML: {}", e))?;
    }

    document_xml = document_xml.replace("«nom_tutor»", nombre);

    let mut buffer = std::io::Cursor::new(Vec::new());
    {
        let mut zip_writer = zip::ZipWriter::new(&mut buffer);
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)
                .map_err(|e| format!("❌ Error al leer archivo del ZIP: {}", e))?;
            let options = FileOptions::default().compression_method(file.compression());

            let mut content = Vec::new();
            file.read_to_end(&mut content)
                .map_err(|e| format!("❌ Error al leer contenido de '{}': {}", file.name(), e))?;

            if file.name() == "word/document.xml" {
                zip_writer.start_file(file.name(), options)
                    .map_err(|e| format!("❌ Error al escribir archivo ZIP: {}", e))?;
                zip_writer.write_all(document_xml.as_bytes())
                    .map_err(|e| format!("❌ Error al escribir el documento XML: {}", e))?;
            } else {
                zip_writer.start_file(file.name(), options)
                    .map_err(|e| format!("❌ Error al escribir archivo ZIP: {}", e))?;
                zip_writer.write_all(&content)
                    .map_err(|e| format!("❌ Error al escribir archivo ZIP: {}", e))?;
            }
        }
    }

    fs::write(salida_path, buffer.into_inner())
        .map_err(|e| format!("❌ Error al guardar el archivo DOCX modificado: {}", e))?;

Ok(())
}



// TESTING

#[cfg(test)]
mod tests {
    
    use super::*;
    use std::path::PathBuf;
    use std::fs;

    fn get_test_data_path(filename: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../../recursos/test_data");
        path.push(filename);
        path
    }

    fn setup_test_environment() {
        // Configurar rutas de prueba
        let emparejamiento_path = get_test_data_path("emparejamiento_final.xlsx");
        let plantilla_path = get_test_data_path("plantilla_tutorados.docx");
        
        reportes_tutorados_recibir_emparejamiento(emparejamiento_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_EMPAREJAMIENTO");
            
        reportes_constanciastutorados_recibir_pathplantilla(plantilla_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_PLANTILLA");
            
        reportes_constanciastutorados_recibir_nombrereporte("output_test".to_string())
            .expect("Error al configurar NOMBRE_REPORTE");
            
        reportes_constanciastutorados_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Error al configurar FECHA");
    }

    #[test]
    fn test_actualizar_fecha() {
        let result = reportes_constanciastutorados_actualizarfecha(Some("2023-05-15".to_string()));
        assert!(result.is_ok());
        
        let fecha_guardada = FECHA.get().unwrap().lock().unwrap();
        assert_eq!(*fecha_guardada, "15-05-2023");
    }

    #[test]
    fn test_recibir_paths() {
        assert!(reportes_tutorados_recibir_emparejamiento("test_emparejamiento.xlsx".to_string()).is_ok());
        assert!(reportes_constanciastutorados_recibir_pathplantilla("test_plantilla.docx".to_string()).is_ok());
        assert!(reportes_constanciastutorados_recibir_nombrereporte("Test Report".to_string()).is_ok());
    }

    #[test]
    #[ignore = "Requiere archivos de prueba"]
    fn test_generar_constancias() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let output_dir = temp_dir.path().to_str().unwrap().to_string();
        
        // Initialize test data
        let emparejamiento_path = get_test_data_path("emparejamiento_final.xlsx");
        let plantilla_path = get_test_data_path("plantilla_tutorados.docx");
        
        reportes_tutorados_recibir_emparejamiento(emparejamiento_path.to_str().unwrap().to_string())
            .expect("Failed to set emparejamiento path");
        reportes_constanciastutorados_recibir_pathplantilla(plantilla_path.to_str().unwrap().to_string())
            .expect("Failed to set plantilla path");
        reportes_constanciastutorados_recibir_nombrereporte(output_dir.clone())
            .expect("Failed to set output dir");
        reportes_constanciastutorados_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Failed to set date");

        // Run test
        let result = reportes_constanciastutorados_generar();
        assert!(result.is_ok(), "Failed to generate constancias: {:?}", result);
        
        // Verify files were created
        let entries = fs::read_dir(&output_dir)
            .expect("Failed to read output dir")
            .count();
        assert!(entries > 0, "No files were generated");
    }

    #[test]
    #[ignore = "Requiere plantilla DOCX"]
    fn test_crear_constancia_individual() {
        setup_test_environment();
        
        let test_name = "Tutorado de Prueba";
        let output_path = get_test_data_path("output/constancia_test.docx");
        
        // Asegurar que el directorio de output existe
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).expect("Error al crear directorio de output");
        }

        let result = crear_constancia(test_name, output_path.to_str().unwrap());
        assert!(result.is_ok(), "Error al crear constancia: {:?}", result.err());
        assert!(output_path.exists(), "No se generó el archivo de constancia");

        // Limpieza
        if output_path.exists() {
            fs::remove_file(output_path).ok();
        }
    }

}

