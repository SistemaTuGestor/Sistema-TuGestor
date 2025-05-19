
// VARIOS
use serde::{Serialize, Deserialize}; // Import Deserialize
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

use std::fs;
use docx_rs::*;
use std::io::BufWriter;
use printpdf::*;
use std::path::PathBuf;

use std::process::Command;



static FECHA: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_LEE: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_PLANTILLA: OnceCell<Mutex<String>> = OnceCell::new();
static NOMBRE_REPORTE: OnceCell<Mutex<String>> = OnceCell::new();

////    FECHA   ////

#[tauri::command]
pub fn reportes_puj_actualizarfecha(nueva_fecha: Option<String>) -> Result<(), String> {
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

// println! ( "Nueva fecha (PUJ): {}", fecha ) ;

Ok(())
}

////    LEE     ////

#[tauri::command]
pub fn reportes_puj_recibir_lee ( path:String ) -> Result<(),String> {

    let nombre = PATH_LEE.get_or_init(|| Mutex::new(String::new()));

    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = path;

    // println!("📂 Ruta de archivo LEE (PUJ): {}", *nombre_guardado);

Ok(())
}

////    PATH    ////

#[derive(Serialize)]
pub struct NombrePlantilla {
    nombre: String,
}

#[tauri::command]
pub fn reportes_puj_recibir_pathplantilla(path: String) -> Result<(), String> {
    let nombre = PATH_PLANTILLA.get_or_init(|| Mutex::new(String::new()));

    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = path;

    // println!("📂 Ruta de la plantilla recibida (PUJ): {}", *nombre_guardado);

    Ok(())
}

////    NOMBRE REPORTE     ////

#[tauri::command]
pub fn reportes_puj_recibir_nombrereporte(nombrereporte: String) -> Result<(), String> {
    
    let nombre = NOMBRE_REPORTE.get_or_init(|| Mutex::new(String::new()));

    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = nombrereporte;

    // println!("📂 Nombre del reporte (PUJ): {}", *nombre_guardado);

Ok(())
}

////    LÓGICA DE ARCHIVOS      ////

#[derive(Serialize, Deserialize, Debug)] // Derive Deserialize
pub struct Estudiante {
    nombre_tutor: String,
    horas_totales: f64,
    modalidad: f64,
}



#[tauri::command]
pub fn reportes_puj_leer_universitarios_aprobados() -> Result<Vec<Estudiante>, String> {

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
        let modalidad: f64 = row[3].to_string().parse().unwrap_or(0.0);
        let horas_totales: f64 = row.get(row.len() - 1)
            .and_then(|cell| cell.to_string().parse::<f64>().ok())
            .unwrap_or(0.0);
        // println!("📂 Correo: {}", correo);

        if correo.ends_with("@javeriana.edu.co")  {
            estudiantes_aprobados.push(Estudiante {
                nombre_tutor,
                horas_totales,
                modalidad,
            });
        }
    }

    // println!("📂 Lista de estudiantes (PUJ): {:#?}", estudiantes_aprobados);

Ok(estudiantes_aprobados)
}

#[tauri::command]
pub fn reporte_puj_generar(estudiantes: Vec<Estudiante>) -> Result<(), String> {

    // imprimir la lista de estudiantes
    // println!("📂 Lista de estudiantes (PUJ): {:#?}", estudiantes);

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

    // Extraer el nombre del archivo (sin la ruta) y su extensión.
    let path = Path::new(&*nombre_reporte);
    let file_name = path.file_stem()
        .and_then(|name| name.to_str())
        .ok_or("❌ No se pudo extraer el nombre del archivo de NOMBRE_REPORTE")?;
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .ok_or("❌ No se pudo extraer la extensión del archivo de NOMBRE_REPORTE")?;

    // Se obtiene la fecha de la variable global.
    let fecha = FECHA
        .get()
        .ok_or("❌ FECHA no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

    // Construir el nuevo nombre del archivo con la fecha.
    let nuevo_nombre_archivo = format!("{} ({}).{}", file_name, *fecha, extension);

    // Construir la ruta de salida en el mismo directorio que el archivo original.
    let output_path = path.parent()
        .ok_or("❌ No se pudo obtener el directorio padre de NOMBRE_REPORTE")?
        .join(nuevo_nombre_archivo);

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

    #[test]
    fn test_actualizar_fecha() {
        let result = reportes_puj_actualizarfecha(Some("2023-05-15".to_string()));
        assert!(result.is_ok());
        
        let fecha_guardada = FECHA.get().unwrap().lock().unwrap();
        assert_eq!(*fecha_guardada, "15-05-2023");
    }

    #[test]
    fn test_recibir_paths() {
        assert!(reportes_puj_recibir_lee("test_path.xlsx".to_string()).is_ok());
        assert!(reportes_puj_recibir_pathplantilla("test_plantilla.docx".to_string()).is_ok());
        assert!(reportes_puj_recibir_nombrereporte("Test Report.docx".to_string()).is_ok());
    }

    #[test]
    #[ignore = "Requiere archivo Excel de prueba"]
    fn test_leer_universitarios_aprobados() {
        let test_file = get_test_data_path("test_data.xlsx");
        println!("Buscando archivo en: {:?}", test_file);
        assert!(test_file.exists(), "El archivo de prueba no existe en {:?}", test_file);
        
        reportes_puj_recibir_lee(test_file.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_LEE");
        
        let result = reportes_puj_leer_universitarios_aprobados();
        assert!(result.is_ok(), "Error al leer estudiantes: {:?}", result.err());
        
        let estudiantes = result.unwrap();
        assert!(!estudiantes.is_empty(), "No se encontraron estudiantes aprobados");
        assert!(estudiantes.iter().all(|e| e.nombre_tutor.contains("@javeriana.edu.co")));
    }

    #[test]
    #[ignore = "Requiere plantilla DOCX"]
    fn test_generar_documento() {
        let test_file = get_test_data_path("test_data.xlsx");
        let plantilla_path = get_test_data_path("plantilla.docx");
        
        assert!(test_file.exists(), "Archivo de datos no encontrado");
        assert!(plantilla_path.exists(), "Plantilla no encontrada");
        
        // Configurar entorno de prueba
        reportes_puj_recibir_lee(test_file.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_LEE");
            
        reportes_puj_recibir_pathplantilla(plantilla_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_PLANTILLA");
            
        reportes_puj_recibir_nombrereporte("Reporte PUJ.docx".to_string())
            .expect("Error al configurar NOMBRE_REPORTE");
            
        reportes_puj_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Error al configurar FECHA");
        
        // Obtener datos de prueba
        let estudiantes = reportes_puj_leer_universitarios_aprobados()
            .expect("Error al obtener estudiantes");
        
        // Generar documento
        let result = reporte_puj_generar(estudiantes);
        assert!(result.is_ok(), "Error al generar documento: {:?}", result.err());
        
        // Verificar archivo generado
        let output_file = Path::new("Reporte PUJ (01-01-2023).docx");
        assert!(output_file.exists(), "No se generó el archivo de salida");
        
        // Limpieza
        fs::remove_file(output_file).ok();
    }

    #[test]
    #[ignore = "Prueba de rendimiento - no ejecutar en pruebas normales"]
    fn test_rendimiento_puj_lectura_y_generacion ( ) {

        use std::time::Instant;
        
        // Configurar paths de prueba
        let test_file = get_test_data_path("test_data.xlsx");
        let plantilla_path = get_test_data_path("plantilla.docx");
        
        println!("📊 Iniciando prueba de rendimiento para PUJ...");
        println!("📂 Archivo de datos: {}", test_file.display());
        println!("📄 Plantilla DOCX: {}", plantilla_path.display());
        
        // Configuración inicial
        reportes_puj_recibir_lee(test_file.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_LEE");
        reportes_puj_recibir_pathplantilla(plantilla_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_PLANTILLA");
        reportes_puj_recibir_nombrereporte("Test Rendimiento PUJ.docx".to_string())
            .expect("Error al configurar NOMBRE_REPORTE");
        reportes_puj_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Error al configurar FECHA");
        
        // Medir tiempo de ejecución
        let start_time = Instant::now();
        
        // Ejecutar funciones a medir
        let estudiantes = reportes_puj_leer_universitarios_aprobados()
            .expect("Error al leer estudiantes");
        
        // Mostrar estadísticas de los datos
        println!("📈 Registros procesados:");
        println!("   - Total estudiantes PUJ: {}", estudiantes.len());
        
        // Contar estudiantes aprobados vs no aprobados
        let (aprobados, no_aprobados) = estudiantes.iter().fold((0, 0), |(aprob, no_aprob), e| {
            if e.horas_totales >= e.modalidad {
                (aprob + 1, no_aprob)
            } else {
                (aprob, no_aprob + 1)
            }
        });
        
        println!("   - Aprobados: {} ({}%)", aprobados, (aprobados as f32 / estudiantes.len() as f32 * 100.0).round());
        println!("   - No aprobados: {} ({}%)", no_aprobados, (no_aprobados as f32 / estudiantes.len() as f32 * 100.0).round());
        
        // Generar el documento
        reporte_puj_generar(estudiantes)
            .expect("Error al generar reporte");
        
        let duration = start_time.elapsed();
        
        println!("⏱️ Tiempo total de ejecución: {:.2?}", duration);
        println!("📊 Prueba de rendimiento completada");
        
        // Limpieza (opcional - eliminar archivos generados)
        let output_file = Path::new("Test Rendimiento PUJ (01-01-2023).docx");
        if output_file.exists() {
            fs::remove_file(output_file).expect("Error al limpiar archivo generado");
            println!("🧹 Archivo temporal eliminado");
        }
    }

}


#[tauri::command]
pub fn convertir_puj_pdf(urldocs: String) -> Result<(), String> {
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
        
        if path.to_string_lossy().contains("PUJ") && path.extension().and_then(|s| s.to_str()) == Some("docx") {
            let docx_path = path.to_string_lossy().to_string();
            let pdf_path = path.with_extension("pdf").to_string_lossy().to_string();
            
            println!("📄 Convirtiendo: {} -> {}", docx_path, pdf_path);

            let ps_script = format!(r#"
                $word = New-Object -ComObject Word.Application
                $word.Visible = $false
                $doc = $word.Documents.Open("{}")
                $doc.SaveAs([ref] "{}", [ref] 17)
                $doc.Close()
                $word.Quit()
                [System.Runtime.Interopservices.Marshal]::ReleaseComObject($word)
            "#, docx_path.replace("\\", "\\\\"), pdf_path.replace("\\", "\\\\"));

            let output = Command::new("powershell")
                .args(["-Command", &ps_script])
                .output()
                .map_err(|e| format!("Error al ejecutar PowerShell: {}", e))?;

            if !output.status.success() {
                return Err(format!(
                    "Error al convertir archivo: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }

            println!("✅ Convertido exitosamente: {}", pdf_path);
            converted_count += 1;
        }
    }

    if converted_count == 0 {
        return Err("No se encontraron archivos DOCX de PUJ para convertir".to_string());
    }

    println!("🎉 Conversión completada: {} archivos convertidos", converted_count);
    Ok(())
}
