
// VARIOS
use serde::Serialize ;
// FECHA
use chrono::Local ;
use chrono::NaiveDate ;
// PATH
use once_cell::sync::OnceCell ;
use std::sync::Mutex ;
// ARCHIVOS
use std::fs::{self} ;
use std::io::{Read, Write} ;
use std::path::{Path,PathBuf} ;
use zip::write::FileOptions ;
use zip::ZipArchive ;
use calamine::{open_workbook, Reader, Xlsx} ;



static FECHA : OnceCell<Mutex<String>> = OnceCell::new() ;
static PATH_LEE: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_PLANTILLA : OnceCell<Mutex<String>> = OnceCell::new() ;
static NOMBRE_REPORTE : OnceCell<Mutex<String>> = OnceCell::new() ;



////    FECHA   ////

#[tauri::command]
pub fn reportes_constanciastutores_actualizarfecha ( nueva_fecha:Option<String> ) -> Result<(),String> {

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
        .clone_from(&fecha) ;
    
    // println! ( "Nueva fecha (Tutores): {}", fecha ) ;

Ok(())
}


////    LEE     ////

#[tauri::command]
pub fn reportes_tutores_recibir_lee ( path:String ) -> Result<(),String> {

    let nombre = PATH_LEE.get_or_init(|| Mutex::new(String::new()));

    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = path;

    // println!("📂 Ruta de archivo LEE (Tutores): {}", *nombre_guardado);

Ok(())
}


////    PATH    ////

#[derive(Serialize)]
pub struct NombrePlantilla {
    nombre: String,
}

#[tauri::command]
pub fn reportes_constanciastutores_recibir_pathplantilla ( path:String ) -> Result<(),String> {

    // Initialize the global variable if it hasn't been initialized yet
    let nombre = PATH_PLANTILLA.get_or_init(|| Mutex::new(String::new())) ;
    
    // Store the report name in the global variable
    let mut nombre_guardado = nombre.lock().unwrap() ;
    *nombre_guardado = path ;

    // println!("📂 Ruta de la carpeta recibida (Constancias tutores): {}",*nombre_guardado) ;

Ok(())
}


////    NOMBRE REPORTE     ////

#[tauri::command]
pub fn reportes_constanciastutores_recibir_nombrereporte ( nombrereporte:String ) -> Result<(),String> {

    // Initialize the global variable if it hasn't been initialized yet
    let nombre = NOMBRE_REPORTE.get_or_init(|| Mutex::new(String::new())) ;
    
    // Store the report name in the global variable
    let mut nombre_guardado = nombre.lock().unwrap() ;
    *nombre_guardado = nombrereporte ;
    
    // println!("📂 Nombre del reporte (Constancias tutores): {}",*nombre_guardado) ;

Ok(())
}



////    LÓGICA DE ARCHIVOS      ////

#[tauri::command]
pub fn reportes_constanciastutores_generar ( ) -> Result<(),String> {

    // println!("📖 Cargando archivo Excel...") ;

    let archivo_lee = PATH_LEE
        .get()
        .ok_or("❌ ARCHIVO_LEE no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

    let path = Path::new(&*archivo_lee);
    
    let mut workbook: Xlsx<_> = open_workbook(path)
        .map_err(|e| format!("❌ No se pudo abrir el archivo Excel: {}", e))?;

    let range = workbook
        .worksheet_range("Sheet1")
        .map_err(|e| format!("❌ No se pudo cargar 'Sheet1': {}", e))?;

    // Asegurar que la carpeta de salida exista.
    let directorio = NOMBRE_REPORTE
        .get()
        .ok_or("❌ NOMBRE_REPORTE no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))? ;

    fs::create_dir_all(&*directorio).map_err(|e| format!("❌ Error creando carpeta de salida: {}", e))? ;

    for (i,row) in range.rows().enumerate() {
        if i == 0 {
            println!("⚠ Ignorando encabezado...");
            continue;
        }

        if row.len() < 2 {
            eprintln!("⚠ ERROR: Fila {} tiene menos de 2 columnas, se omite.", i + 1);
            continue;
        }

        let nombre_tutor = row[1].to_string().trim().to_string();
        let apellido_tutor = row[0].to_string().trim().to_string();
        let modality = row[4].to_string().trim().to_string();
        // println! ( "📝 Generando constancia para: {} {}",nombre_tutor,apellido_tutor ) ;

        // Se obtiene la fecha de la variable global.
        let fecha = FECHA
            .get()
            .ok_or("❌ FECHA no ha sido inicializado")?
            .lock()
            .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

        let salida_docx = PathBuf::from(&*directorio).join ( format! (
            "Constancia Tutor {} {} ({}).docx",
            nombre_tutor , apellido_tutor , fecha
        ) ) ;

        // Convert PathBuf to String safely
        let salida_documento = salida_docx.into_os_string()
            .into_string()
            .map_err(|e| format!("❌ El nombre del archivo no es válido UTF-8: {:?}", e))? ;
        
        match crear_constancia ( &nombre_tutor,&apellido_tutor,&modality,&salida_documento ) {
            Ok(_) => println!("✔ Constancia generada: {}", salida_documento),
            Err(e) => eprintln!("❌ Error al generar constancia para {} {}: {}" ,
            nombre_tutor , apellido_tutor , e )
        }
    }

    println!("🎉 ¡Todas las constancias han sido generadas!");

Ok(())
}

fn crear_constancia ( nombre:&str,apellido:&str,modality: &str,salida_path:&str ) -> Result<(),String> {

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
    document_xml = document_xml.replace("«Apellido_tutor»", apellido);
    document_xml = document_xml.replace("«modality»", modality);

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

    println!("✔ Constancia guardada: {}", salida_path);

Ok(())
}



// TESTING

#[cfg(test)]
mod tests {
    
    use super::*;
    use std::path::PathBuf;
    use std::fs;
    use tempfile;
    use std::io::Read;

    fn get_test_data_path(filename: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../../recursos/test_data");
        path.push(filename);
        path
    }

    #[test]
    fn test_actualizar_fecha() {
        let result = reportes_constanciastutores_actualizarfecha(Some("2023-05-15".to_string()));
        assert!(result.is_ok());
        
        let fecha_guardada = FECHA.get().unwrap().lock().unwrap();
        assert_eq!(*fecha_guardada, "15-05-2023");
    }

    #[test]
    fn test_recibir_paths() {
        assert!(reportes_tutores_recibir_lee("test_lee.xlsx".to_string()).is_ok());
        assert!(reportes_constanciastutores_recibir_pathplantilla("test_plantilla.docx".to_string()).is_ok());
        assert!(reportes_constanciastutores_recibir_nombrereporte("Test Report".to_string()).is_ok());
    }

    #[test]
    #[ignore = "Requires specific test files"]
    fn test_generar_constancias_with_real_files() {
        // Setup
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let output_dir = temp_dir.path().to_str().unwrap().to_string();
        
        // Initialize with actual test files
        let lee_path = get_test_data_path("test_data.xlsx");
        let plantilla_path = get_test_data_path("plantilla_tutores.docx");
        
        if !lee_path.exists() || !plantilla_path.exists() {
            panic!("Test files not found at:\n- {}\n- {}", lee_path.display(), plantilla_path.display());
        }

        // Configure paths
        reportes_tutores_recibir_lee(lee_path.to_str().unwrap().to_string())
            .expect("Failed to set LEE path");
        reportes_constanciastutores_recibir_pathplantilla(plantilla_path.to_str().unwrap().to_string())
            .expect("Failed to set plantilla path");
        reportes_constanciastutores_recibir_nombrereporte(output_dir.clone())
            .expect("Failed to set output dir");
        reportes_constanciastutores_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Failed to set date");

        // Run generation
        let result = reportes_constanciastutores_generar();
        assert!(result.is_ok(), "Failed to generate constancias: {:?}", result);
        
        // Verify output files were created
        let entries: Vec<_> = fs::read_dir(&output_dir)
            .expect("Failed to read output dir")
            .collect();
        
        // Should create 7 files (one per tutor in test_data.xlsx)
        assert_eq!(entries.len(), 7, "Incorrect number of files generated");
    }

    #[test]
    #[ignore = "Requires DOCX template"]
    fn test_crear_constancia_content() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("constancia_test.docx");
        
        // Set up test data with actual template
        let plantilla_path = get_test_data_path("plantilla_tutores.docx");
        if !plantilla_path.exists() {
            panic!("Plantilla not found at {}", plantilla_path.display());
        }

        reportes_constanciastutores_recibir_pathplantilla(plantilla_path.to_str().unwrap().to_string())
            .expect("Failed to set plantilla path");
        reportes_constanciastutores_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Failed to set date");

        // Test with sample data
        let result = crear_constancia(
            "Test", 
            "Tutor",
            "Virtual",
            output_path.to_str().unwrap()
        );
        
        assert!(result.is_ok(), "Error creating constancia: {:?}", result);
        assert!(output_path.exists(), "Output file not created");
        
        // Verify content was replaced - read file as raw bytes
        let content = fs::read(&output_path).expect("Failed to read output file");
        
        // Convert to string lossy to search for our text
        let content_str = String::from_utf8_lossy(&content);
        assert!(content_str.contains("Test"), "Name not found in output");
        assert!(content_str.contains("Tutor"), "Last name not found in output");
        assert!(content_str.contains("Virtual"), "Modality not found in output");
    }

    #[test]
    fn test_skip_invalid_rows() {
        // Setup with mock data that includes invalid rows
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let output_dir = temp_dir.path().to_str().unwrap().to_string();
        
        // Create a test Excel file with some invalid rows
        let test_excel = temp_dir.path().join("test.xlsx");
        fs::write(&test_excel, include_bytes!("../../../../recursos/test_data/test_data.xlsx"))
            .expect("Failed to create test Excel file");
        
        // Configure paths - don't set plantilla path to test skipping
        reportes_tutores_recibir_lee(test_excel.to_str().unwrap().to_string())
            .expect("Failed to set LEE path");
        reportes_constanciastutores_recibir_nombrereporte(output_dir.clone())
            .expect("Failed to set output dir");
        
        // Should skip processing since the Excel file is empty/invalid
        let result = reportes_constanciastutores_generar();
        assert!(result.is_ok(), "Should handle invalid Excel file gracefully");
    }

}

