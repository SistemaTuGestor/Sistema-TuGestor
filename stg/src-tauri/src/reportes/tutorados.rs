
// FECHA
use chrono::Local ;
use chrono::NaiveDate ;
// PATH
use once_cell::sync::OnceCell ;
use std::sync::Mutex ;
// ARCHIVOS
use std::fs::File;
use std::fs::{self} ;
use std::path::{Path,PathBuf} ;
use std::io::{Read, Write} ;
use calamine::{open_workbook, Reader, Xlsx} ;
use zip::{ZipArchive, write::FileOptions} ;

use docx_rs::*;
use std::io::BufWriter;
use printpdf::*;


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

#[tauri::command]
pub fn convertir_tutorados_pdf(urldocs: String) -> Result<(), String> {
    let path = Path::new(&urldocs);
    let dir_path = if path.is_file() {
        path.parent()
            .ok_or_else(|| format!("No se pudo obtener el directorio padre de: {}", urldocs))?
    } else {
        path
    };
    
    if !dir_path.exists() {
        return Err(format!("El directorio {} no existe", dir_path.display()));
    }

    println!("🔍 Buscando archivos DOCX en: {}", dir_path.display());

    let entries = fs::read_dir(dir_path)
        .map_err(|e| format!("Error al leer el directorio: {}", e))?;

    let mut converted_count = 0;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Error al leer entrada: {}", e))?;
        let path = entry.path();
        
        if path.to_string_lossy().contains("Tutorado") && path.extension().and_then(|s| s.to_str()) == Some("docx") {
            let pdf_name = path.with_extension("pdf");
            
            println!("📄 Convirtiendo: {} -> {}", path.display(), pdf_name.display());

            let (doc, page1, layer1) = PdfDocument::new(
                "Constancia Tutorado", 
                Mm(210.0),
                Mm(297.0),
                "Layer 1"
            );

            let file = File::create(&pdf_name)
                .map_err(|e| format!("Error al crear PDF {}: {}", pdf_name.display(), e))?;
            let mut writer = BufWriter::new(file);

            doc.save(&mut writer)
                .map_err(|e| format!("Error al guardar PDF {}: {}", pdf_name.display(), e))?;

            println!("✅ Convertido exitosamente: {}", pdf_name.display());
            converted_count += 1;
        }
    }

    if converted_count == 0 {
        return Err("No se encontraron archivos DOCX de constancias de tutorados para convertir".to_string());
    }

    println!("🎉 Conversión completada: {} archivos convertidos", converted_count);
    Ok(())
}

