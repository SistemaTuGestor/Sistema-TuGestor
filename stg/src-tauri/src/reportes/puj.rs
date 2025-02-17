
use chrono::NaiveDate;
use calamine::{open_workbook, Reader, Xlsx};
use serde::Serialize;
use std::fs::File;
use docx_rs::*;



////    FECHA   ////

#[tauri::command]
pub fn reportes_puj_actualizar_fecha(nueva_fecha: String) -> Result<(), String> {

    let parsed_date = NaiveDate::parse_from_str(&nueva_fecha, "%Y-%m-%d")
        .map_err(|e| format!("❌ Error al parsear la fecha: {}", e))?;
    let formatted_date = parsed_date.format("%d-%m-%Y").to_string();

    println!("Nueva fecha (PUJ): {}", formatted_date);

Ok(())
}


////    NOMBRE REPORTE     ////

#[tauri::command]
pub fn reportes_puj_recibir_nombrereporte(nombrereporte: String) -> Result<String,String> {

    println!("📂 Nombre del reporte (PUJ): {}",nombrereporte) ;

Ok ( nombrereporte )
}


////    LÓGICA DE ARCHIVOS      ////

#[derive(Serialize, Debug)]
pub struct Estudiante {
    nombre_tutor: String,
    horas_totales: f64,
}

// 🔹 Rutas de los archivos
const ARCHIVO_EXCEL: &str = "C:\\Users\\USUARIO\\Downloads\\Reporte_Tutores_LEE.xlsx";
const ARCHIVO_SALIDA: &str = "C:\\Users\\USUARIO\\Downloads\\Reporte_Colegios.docx";

#[tauri::command]
pub fn leer_universitarios_aprobados() -> Result<Vec<String>, String> {
    println!("🔍 Iniciando lectura del archivo: {}", ARCHIVO_EXCEL);

    let mut workbook: Xlsx<_> = open_workbook(ARCHIVO_EXCEL)
        .map_err(|e| format!("❌ No se pudo abrir el archivo Excel: {}", e))?;

    let range = workbook
        .worksheet_range("Sheet1")
        .map_err(|e| format!("❌ Error al cargar 'Sheet1': {}", e))?;

    let mut estudiantes_aprobados = Vec::new();

    for (i, row) in range.rows().enumerate() {
        println!("📄 Procesando fila {}: {:?}", i + 1, row);
        if i == 0 {
            println!("⚠ Ignorando encabezado...");
            continue;
        }

        if row.len() < 5 {
            eprintln!("⚠ ERROR: Fila {} tiene menos de 5 columnas, se omite.", i + 1);
            continue;
        }

        let correo = row[0].to_string().trim().to_string();
        let nombre_tutor = row[1].to_string().trim().to_string();
        let horas_totales: f64 = row
            .get(row.len() - 1)
            .and_then(|cell| cell.to_string().parse::<f64>().ok())
            .unwrap_or(0.0);

        if correo.ends_with("@javeriana.edu.co") {
            println!("👨‍🎓 Universitario: {}, Horas: {}", nombre_tutor, horas_totales);
            if horas_totales >= 60.0 {
                println!("✅ {} ha completado las horas requeridas.", nombre_tutor);
                estudiantes_aprobados.push(nombre_tutor);
            }
        }
    }

    println!("✔ {} universitarios han completado sus horas.", estudiantes_aprobados.len());
    Ok(estudiantes_aprobados)
}

#[tauri::command]
pub fn generar_reporte_puj(estudiantes: Vec<String>) {
    println!("📄 Iniciando generación del reporte...");
    println!("📌 Número de tutores aprobados: {}", estudiantes.len());

    if estudiantes.is_empty() {
        eprintln!("❌ No hay tutores aprobados, cancelando reporte.");
        return;
    }

    let mut doc = Docx::new();

    doc = doc
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Bogotá D. C., junio de 2023")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Coordinador")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Pontificia Universidad Javeriana")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Cr.7 No.40B-36, Bogotá, Colombia")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(" ")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("**Reporte Horas Servicio**").bold()));

    doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text(
        "Desde el laboratorio de Economía de la Educación, certificamos que los siguientes tutores han completado satisfactoriamente sus horas de servicio en el Proyecto TuTutor."
    )));

    for estudiante in &estudiantes {
        println!("📝 Agregando al reporte: {}", estudiante);
        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text(format!("- {}", estudiante))));
    }

    let file = match File::create(ARCHIVO_SALIDA) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("❌ ERROR: No se pudo crear el archivo DOCX -> {}", e);
            return;
        }
    };

    match doc.build().pack(file) {
        Ok(_) => println!("✔ Reporte generado correctamente en: {}", ARCHIVO_SALIDA),
        Err(e) => eprintln!("❌ ERROR al escribir el documento DOCX: {}", e),
    }
}

