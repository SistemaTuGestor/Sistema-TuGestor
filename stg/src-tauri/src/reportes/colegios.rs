
use chrono::NaiveDate;
use calamine::{open_workbook, Reader, Xlsx};
use serde::Serialize;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::{ZipArchive, write::FileOptions};



////    FECHA   ////

#[tauri::command]
pub fn reportes_colegios_actualizar_fecha(nueva_fecha: String) -> Result<(), String> {

    let parsed_date = NaiveDate::parse_from_str(&nueva_fecha, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse date: {}", e))?;
    let formatted_date = parsed_date.format("%d-%m-%Y").to_string();
    
    println!("Nueva fecha (Colegios): {}", formatted_date);

Ok(())
}


////    NOMBRE REPORTE     ////

#[tauri::command]
pub fn reportes_colegios_recibir_nombrereporte(nombrereporte: String) -> Result<String,String> {

    println!("📂 Nombre del reporte (Colegios): {}",nombrereporte) ;

Ok(nombrereporte)
}


////    LÓGICA DE ARCHIVOS      ////

#[derive(Serialize, Debug)]
pub struct Estudiante {
    nombre_tutor: String,
    horas_totales: f64,
}

// 🔹 Rutas de los archivos
const ARCHIVO_EXCEL: &str = "C:\\Users\\USUARIO\\Downloads\\Reporte_Tutores_LEE.xlsx";;
const PLANTILLA_DOCX: &str = "C:\\Users\\USUARIO\\Downloads\\Plantilla Reporte Final(Para Colegio y PUJ).docx";
const ARCHIVO_SALIDA: &str = "C:\\Users\\USUARIO\\Downloads\\Reporte_Colegios.docx";

#[tauri::command]
pub fn reportes_colegios_actualizar_fecha(nueva_fecha: String) -> Result<(), String> {
    let parsed_date = NaiveDate::parse_from_str(&nueva_fecha, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse date: {}", e))?;
    let formatted_date = parsed_date.format("%d-%m-%Y").to_string();
    println!("Nueva fecha: {}", formatted_date);
    Ok(())
}

#[tauri::command]
pub fn leer_estudiantes_aprobados() -> Result<Vec<String>, String> {
    let mut workbook: Xlsx<_> = open_workbook(ARCHIVO_EXCEL)
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
        let horas_totales: f64 = row.get(row.len() - 1)
            .and_then(|cell| cell.to_string().parse::<f64>().ok())
            .unwrap_or(0.0);

        if !correo.ends_with("@javeriana.edu.co") && horas_totales >= 60.0 {
            estudiantes_aprobados.push(format!("<w:p><w:r><w:t>- {}</w:t></w:r></w:p>", nombre_tutor));
        }
    }

    Ok(estudiantes_aprobados)
}

#[tauri::command]
pub fn generar_reporte_colegios(estudiantes: Vec<String>) {
    let lista_tutores = estudiantes.join("");
    let plantilla_path = Path::new(PLANTILLA_DOCX);
    let output_path = Path::new(ARCHIVO_SALIDA);

    let file = File::open(plantilla_path).expect("No se pudo abrir la plantilla");
    let mut zip = ZipArchive::new(file).expect("Error al leer el archivo ZIP");

    let mut contenido_xml = String::new();

    {
        let mut file = zip.by_name("word/document.xml").expect("No se encontró document.xml");
        file.read_to_string(&mut contenido_xml).expect("Error al leer XML");
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
