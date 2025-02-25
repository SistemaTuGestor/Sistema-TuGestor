
// VARIOS
use serde::Serialize ;
// FECHA
use chrono::NaiveDate ;
// PATH
use once_cell::sync::OnceCell ;
use std::sync::Mutex ;
// ARCHIVOS
use calamine::{open_workbook, Reader, Xlsx} ;
use std::fs::{self} ;
use std::io::{Read, Write} ;
use std::path::PathBuf ;
use zip::ZipArchive ;
use zip::write::FileOptions ;



static FECHA : OnceCell<Mutex<String>> = OnceCell::new() ;
static PATH_PLANTILLA : OnceCell<Mutex<String>> = OnceCell::new() ;
static NOMBRE_REPORTE : OnceCell<Mutex<String>> = OnceCell::new() ;



////    FECHA   ////

#[tauri::command]
pub fn reportes_constanciastutores_actualizarfecha ( nueva_fecha:String ) -> Result<(),String> {

    let parsed_date = NaiveDate::parse_from_str(&nueva_fecha, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse date: {}", e))? ;

    let formatted_date = parsed_date.format("%d-%m-%Y").to_string() ;

    FECHA.get_or_init(|| Mutex::new(String::new()))
        .lock()
        .map_err(|e| format!("Failed to lock mutex: {}", e))?
        .clone_from(&formatted_date) ;
    
    println!("Nueva fecha (Constancias tutores): {}", formatted_date) ;

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

// const ARCHIVO_EXCEL:&str = "C:\\Users\\USUARIO\\Downloads\\LEE.xlsx" ;
const ARCHIVO_EXCEL:&str = "/home/user/Downloads/LEE.xlsx" ;

#[tauri::command]
pub fn reportes_constanciastutores_generar ( ) -> Result<(),String> {

    println!("📖 Cargando archivo Excel...") ;

    let mut workbook: Xlsx<_> = open_workbook(ARCHIVO_EXCEL)
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

        let nombre_tutor = row[0].to_string().trim().to_string();
        let apellido_tutor = row[1].to_string().trim().to_string();

        println!("📝 Generando constancia para: {} {}", nombre_tutor, apellido_tutor);

        let salida_docx = PathBuf::from(&*directorio).join ( format! (
            "Constancia Tutor {} {}.docx",
            nombre_tutor , apellido_tutor
        ) ) ;

        // Convert PathBuf to String safely
        let salida_documento = salida_docx.into_os_string()
            .into_string()
            .map_err(|e| format!("❌ El nombre del archivo no es válido UTF-8: {:?}", e))? ;
        
        match crear_constancia ( &nombre_tutor,&apellido_tutor,&salida_documento ) {
            Ok(_) => println!("✔ Constancia generada: {}", salida_documento),
            Err(e) => eprintln!("❌ Error al generar constancia para {} {}: {}" ,
            nombre_tutor , apellido_tutor , e )
        }
    }

    println!("🎉 ¡Todas las constancias han sido generadas!");

Ok(())
}

fn crear_constancia ( nombre:&str,apellido:&str,salida_path:&str ) -> Result<(),String> {

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

