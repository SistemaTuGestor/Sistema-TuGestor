
// VARIOS
use serde::{Serialize, Deserialize};
// FECHA
use chrono::Local;
use chrono::NaiveDate;
// PATH
use once_cell::sync::OnceCell;
use std::sync::Mutex;
// ARCHIVOS
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use calamine::{open_workbook, Reader, Xlsx};
use zip::{ZipArchive, write::FileOptions};
use std::collections::HashMap;

use std::fs;
use docx_rs::*;
use std::io::BufWriter;
use printpdf::*;
use std::path::PathBuf;



static FECHA: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_LEE: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_PLANTILLA: OnceCell<Mutex<String>> = OnceCell::new();
static NOMBRE_REPORTE: OnceCell<Mutex<String>> = OnceCell::new();

////    FECHA   ////

#[tauri::command]
pub fn reportes_colegios_actualizarfecha(nueva_fecha: Option<String>) -> Result<(), String> {
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

    // println! ( "Nueva fecha (Colegios): {}", fecha ) ;

    Ok(())
}


////    LEE     ////

#[tauri::command]
pub fn reportes_colegios_recibir_lee ( path:String ) -> Result<(),String> {

    let nombre = PATH_LEE.get_or_init(|| Mutex::new(String::new()));

    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = path;

    // println!("📂 Ruta de archivo LEE (Colegios): {}", *nombre_guardado);

Ok(())
}


////    PATH    ////

#[derive(Serialize)]
pub struct NombrePlantilla {
    nombre: String,
}

#[tauri::command]
pub fn reportes_colegios_recibir_pathplantilla(path: String) -> Result<(), String> {

    let nombre = PATH_PLANTILLA.get_or_init(|| Mutex::new(String::new()));

    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = path;

    // println!("📂 Ruta de la plantilla recibida (Colegios): {}", *nombre_guardado);

Ok(())
}


////    NOMBRE REPORTE     ////

#[tauri::command]
pub fn reportes_colegios_recibir_nombrereporte(nombrereporte: String) -> Result<(), String> {

    // Initialize the global variable if it hasn't been initialized yet
    let nombre = NOMBRE_REPORTE.get_or_init(|| Mutex::new(String::new()));

    // Store the report name in the global variable
    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = nombrereporte;

    // println!("📂 Nombre del reporte (Colegios): {}", *nombre_guardado);

Ok(())
}



////    LÓGICA DE ARCHIVOS      ////

#[derive(Serialize, Deserialize, Debug)] // Derive Deserialize
pub struct Estudiante {
    nombre_tutor: String,
    institucion: String,
    horas_totales: f64,
    modalidad: f64,
}

#[tauri::command]
pub fn reportes_colegios_leer_estudiantes_aprobados ( ) -> Result<Vec<Estudiante>, String> {

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
        .map_err(|e| format!("❌ Error al cargar 'Sheet1': {}", e))?;

    let mut estudiantes_aprobados = Vec::new();

    for (i, row) in range.rows().enumerate() {
        if i == 0 || row.len() < 5 {
            continue;
        }

        let correo = row[0].to_string().trim().to_string();
        let nombre_tutor = row[1].to_string();
        let institucion = row[2].to_string();
        let modalidad: f64 = row[3].to_string().parse().unwrap_or(0.0);
        let horas_totales: f64 = row.get(row.len() - 1)
            .and_then(|cell| cell.to_string().parse::<f64>().ok())
            .unwrap_or(0.0);

        if !correo.ends_with("@javeriana.edu.co") {
            estudiantes_aprobados.push(Estudiante {
                nombre_tutor,
                institucion,
                horas_totales,
                modalidad,
            });
        }
    }

    // println!("📂 Estudiantes aprobados (Colegios): {:?}", estudiantes_aprobados);

Ok(estudiantes_aprobados)
}

#[tauri::command]
pub fn reportes_colegios_generar(estudiantes: Vec<Estudiante>) -> Result<(), String> {

    // Agrupar estudiantes por institución
    let mut estudiantes_por_institucion: HashMap<String, Vec<Estudiante>> = HashMap::new();
    for estudiante in estudiantes {
        estudiantes_por_institucion
            .entry(estudiante.institucion.clone())
            .or_insert_with(Vec::new)
            .push(estudiante);
    }

    // Generar un archivo por cada institución
    for (institucion, estudiantes) in estudiantes_por_institucion {
        let lista_tutores = estudiantes.iter()
            .map(|e| {
                let check_mark = if e.horas_totales >= e.modalidad { "✔" } else { "❌" };
                format!("<w:p><w:r><w:t>- {} ({}) {}</w:t></w:r></w:p>", e.nombre_tutor, e.modalidad, check_mark)
            })
            .collect::<Vec<String>>()
            .join("");

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

        // Construir el nuevo nombre del archivo con la fecha y la institución.
        let nuevo_nombre_archivo = format!("{} - {} ({}).docx", nombre_reporte, institucion, *fecha);

        // Construir la ruta de salida en el mismo directorio que el archivo original.
        let output_path = Path::new(&nuevo_nombre_archivo);

        // Se obtiene del PATH de la variable global.
        let path_plantilla = PATH_PLANTILLA
            .get()
            .ok_or("❌ PATH_PLANTILLA no ha sido inicializado")?
            .lock()
            .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

        // Abrir la plantilla DOCX.
        let file = File::open(&*path_plantilla)
            .map_err(|e| format!("❌ No se pudo abrir la plantilla: {}", e))?;

        // Se lee el DOCX como un ZIP.
        let mut zip = ZipArchive::new(file)
            .map_err(|e| format!("❌ Error al leer el archivo ZIP: {}", e))?;

        // Leer el contenido XML dentro del DOCX.
        let mut contenido_xml = String::new();
        {
            let mut file = zip.by_name("word/document.xml")
                .map_err(|e| format!("❌ No se encontró document.xml: {}", e))?;
            file.read_to_string(&mut contenido_xml)
                .map_err(|e| format!("❌ Error al leer XML: {}", e))?;
        }

        contenido_xml = contenido_xml.replace("&lt;&lt;lista&gt;&gt;", &lista_tutores);
        contenido_xml = contenido_xml.replace("<<lista>>", &lista_tutores);

        let nuevo_docx = File::create(output_path).expect("No se pudo crear el archivo de salida");
        let mut zip_writer = zip::ZipWriter::new(nuevo_docx);

        for i in 0..zip.len() {
            let mut file = zip.by_index(i).expect("Error al acceder al ZIP");
            let name = file.name().to_string();

            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("Error al leer archivo del ZIP");

            let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
            zip_writer.start_file(name.clone(), options).expect("Error al escribir ZIP");

            if name == "word/document.xml" {
                zip_writer.write_all(contenido_xml.as_bytes()).expect("Error al escribir document.xml");
            } else {
                zip_writer.write_all(&buffer).expect("Error al copiar archivo en el ZIP");
            }
        }

        zip_writer.finish().expect("Error al cerrar el ZIP");
    }

Ok(())
}

#[tauri::command]
pub fn convertir_colegios_pdf(urldocs: String) -> Result<(), String> {
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
        
        // Verificar que el archivo sea un .docx y contenga "Colegio" en su nombre
        if path.to_string_lossy().contains("Colegio") && path.extension().and_then(|s| s.to_str()) == Some("docx") {
            let pdf_name = path.with_extension("pdf");
            
            println!("📄 Convirtiendo: {} -> {}", path.display(), pdf_name.display());

            let (doc, page1, layer1) = PdfDocument::new(
                "Reporte Colegios", 
                Mm(210.0),  // A4 width
                Mm(297.0),  // A4 height
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
        return Err("No se encontraron archivos DOCX de colegios para convertir".to_string());
    }

    println!("🎉 Conversión completada: {} archivos convertidos", converted_count);
    Ok(())
}