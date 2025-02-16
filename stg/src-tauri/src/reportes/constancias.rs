use calamine::{open_workbook, Reader, Xlsx, DataType};
use std::fs::File;
use docx_rs::*;

const ARCHIVO_EXCEL: &str = "C:\\Users\\USUARIO\\Downloads\\tutorias_lee.xlsx";
const ARCHIVO_SALIDA: &str = "C:\\Users\\USUARIO\\Downloads\\Constancias_Tutores.docx";

// 📥 Leer tutores aprobados desde el Excel
pub fn leer_tutores_aprobados() -> Result<Vec<String>, String> {
    let mut workbook: Xlsx<_> = open_workbook(ARCHIVO_EXCEL)
        .map_err(|e| format!("Error al abrir el archivo: {}", e))?;

    let range = workbook
        .worksheet_range("hoja1")
        .map_err(|e| format!("Error al cargar 'hoja1': {}", e))?;

    let mut tutores_aprobados = Vec::new();

    for (i, row) in range.rows().enumerate() {
        if i == 0 { continue; } // Ignorar la primera fila (encabezados)
        if row.len() < 7 { continue; } // Verificar que haya suficientes columnas

        let nombre_tutor = row.get(3).map_or("".to_string(), |cell| cell.to_string());

        let horas_cumplidas: f64 = row.get(5)
            .and_then(|cell| cell.to_string().parse::<f64>().ok())
            .unwrap_or(0.0);

        let horas_requeridas: f64 = row.get(6)
            .and_then(|cell| cell.to_string().parse::<f64>().ok())
            .unwrap_or(0.0);

        if horas_cumplidas >= horas_requeridas {
            tutores_aprobados.push(nombre_tutor);
        }
    }

    println!("✔ {} tutores han completado sus horas.", tutores_aprobados.len());
    Ok(tutores_aprobados)
}

// 📤 Generar constancias para tutores aprobados
pub fn generar_constancias(tutores: &[String]) {
    let mut doc = Docx::new();

    for tutor in tutores {
        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text("Bogotá D. C., junio de 2023")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Pontificia Universidad Javeriana")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Proyecto TuTutor")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text(" ")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("**Constancia de Tutoría**").bold()))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
                format!(
                    "Se certifica que el tutor {} ha completado satisfactoriamente sus horas de servicio en el Proyecto TuTutor.",
                    tutor
                )
            )))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
                "Agradecemos tu compromiso y dedicación en este proceso educativo."
            )))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Cordial saludo.")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Equipo TuTutor")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text(" "))); // Espacio entre constancias
    }

    let file = File::create(ARCHIVO_SALIDA).expect("No se pudo crear el archivo DOCX");
    doc.build().pack(file).expect("Error al escribir el documento DOCX");

    println!("✔ Constancias generadas correctamente en {}", ARCHIVO_SALIDA);
}

// 📌 Ejecutar el proceso
fn main() {
    match leer_tutores_aprobados() {
        Ok(tutores) => {
            if tutores.is_empty() {
                println!("✖ No hay tutores aprobados. No se generará la constancia.");
            } else {
                generar_constancias(&tutores);
            }
        }
        Err(e) => println!("✖ ERROR al procesar tutorías: {}", e),
    }
}
