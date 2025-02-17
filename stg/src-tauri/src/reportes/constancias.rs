
use calamine::{open_workbook, Reader, Xlsx};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use zip::ZipArchive;
use zip::write::FileOptions;



////    NOMBRE REPORTE     ////

#[tauri::command]
pub fn reportes_constancias_recibir_nombrereporte(nombrereporte: String) -> Result<String,String> {

    println!("📂 Nombre del reporte (Constancias): {}",nombrereporte) ;

Ok(nombrereporte)
}


////    LÓGICA DE ARCHIVOS      ////

const ARCHIVO_EXCEL: &str = "C:\\Users\\USUARIO\\Downloads\\tutorias_lee.xlsx";
const PLANTILLA_DOCX: &str = "C:\\Users\\darve\\Downloads\\Plantilla para constancias.docx";
const SALIDA_FOLDER: &str = "C:\\Users\\USUARIO\\Downloads\\Constancias_Tutores.docx";

#[tauri::command]
pub fn generar_constancias() -> Result<(), String> {
    println!("📖 Cargando archivo Excel...");
    let mut workbook: Xlsx<_> = open_workbook(ARCHIVO_EXCEL)
        .map_err(|e| format!("❌ No se pudo abrir el archivo Excel: {}", e))?;

    let range = workbook
        .worksheet_range("Sheet1")
        .map_err(|e| format!("❌ No se pudo cargar 'Sheet1': {}", e))?;

    // Asegurar que la carpeta de salida exista
    fs::create_dir_all(SALIDA_FOLDER).map_err(|e| format!("❌ Error creando carpeta de salida: {}", e))?;

    for (i, row) in range.rows().enumerate() {
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

        let salida_docx = format!(
            "{}Constancia_{}_{}.docx",
            SALIDA_FOLDER, nombre_tutor, apellido_tutor
        );

        match crear_constancia(&nombre_tutor, &apellido_tutor, &salida_docx) {
            Ok(_) => println!("✔ Constancia generada: {}", salida_docx),
            Err(e) => eprintln!("❌ Error al generar constancia para {} {}: {}", nombre_tutor, apellido_tutor, e),
        }
    }

    println!("🎉 ¡Todas las constancias han sido generadas!");
    Ok(())
}

fn crear_constancia(nombre: &str, apellido: &str, salida_path: &str) -> Result<(), String> {
    let plantilla_bytes = fs::read(PLANTILLA_DOCX)
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
