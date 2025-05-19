
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

use std::process::Command;



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
pub fn reportes_colegios_generar ( estudiantes: Vec<Estudiante> ) -> Result<(), String> {

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
        
        if path.to_string_lossy().contains("Colegio") && path.extension().and_then(|s| s.to_str()) == Some("docx") {
            let docx_path = path.to_string_lossy().to_string();
            let pdf_path = path.with_extension("pdf").to_string_lossy().to_string();
            
            println!("📄 Convirtiendo: {} -> {}", docx_path, pdf_path);

            // Script de PowerShell para convertir DOCX a PDF usando Word
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
        return Err("No se encontraron archivos DOCX de colegios para convertir".to_string());
    }

    println!("🎉 Conversión completada: {} archivos convertidos", converted_count);
    Ok(())
}




// TESTING

#[cfg(test)]
mod tests {
    
    use super::*;
    use std::path::PathBuf;
    
    fn get_test_data_path(filename: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../../recursos/test_data"); // Cambiado a la nueva ubicación
        path.push(filename);
        path
    }

    #[test]
    fn test_actualizar_fecha() {
        // Test con fecha proporcionada
        let result = reportes_colegios_actualizarfecha(Some("2023-05-15".to_string()));
        assert!(result.is_ok());
        
        // Verificar que la fecha se actualizó correctamente
        let fecha_guardada = FECHA.get().unwrap().lock().unwrap();
        assert_eq!(*fecha_guardada, "15-05-2023");
    }

    #[test]
    fn test_recibir_paths() {
        assert!(reportes_colegios_recibir_lee("test_path.xlsx".to_string()).is_ok());
        assert!(reportes_colegios_recibir_pathplantilla("test_plantilla.docx".to_string()).is_ok());
        assert!(reportes_colegios_recibir_nombrereporte("Test Report".to_string()).is_ok());
    }

    #[test]
    #[ignore = "Requiere archivo Excel de prueba"]
    fn test_leer_estudiantes_aprobados() {
        let test_file = get_test_data_path("test_data.xlsx");
        println!("Buscando archivo en: {:?}", test_file);
        assert!(test_file.exists(), "El archivo de prueba no existe en {:?}", test_file);
        
        reportes_colegios_recibir_lee(test_file.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_LEE");
        
        let result = reportes_colegios_leer_estudiantes_aprobados();
        assert!(result.is_ok(), "Error al leer estudiantes: {:?}", result.err());
        
        let estudiantes = result.unwrap();
        assert!(!estudiantes.is_empty(), "No se encontraron estudiantes aprobados");
    }

    #[test]
    #[ignore = "Requiere plantilla DOCX"]
    fn test_generar_documentos() {
        let plantilla_path = get_test_data_path("plantilla.docx");
        println!("Buscando plantilla en: {:?}", plantilla_path);
        assert!(plantilla_path.exists(), "La plantilla no existe en {:?}", plantilla_path);
        
        // Configuración de prueba
        reportes_colegios_recibir_pathplantilla(plantilla_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_PLANTILLA");
        reportes_colegios_recibir_nombrereporte("Test Report".to_string())
            .expect("Error al configurar NOMBRE_REPORTE");
        reportes_colegios_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Error al configurar FECHA");
        
        let estudiantes = vec![
            Estudiante {
                nombre_tutor: "Tutor Test".to_string(),
                institucion: "Colegio Test".to_string(),
                horas_totales: 10.0,
                modalidad: 8.0,
            }
        ];
        
        let result = reportes_colegios_generar(estudiantes);
        assert!(result.is_ok(), "Error al generar documentos: {:?}", result.err());
        
        // Verificar que se creó el archivo
        let output_file = Path::new("Test Report - Colegio Test (01-01-2023).docx");
        assert!(output_file.exists(), "No se generó el archivo de salida");
        
        // Limpieza (opcional)
        std::fs::remove_file(output_file).ok();
    }

    #[test]
    #[ignore = "Prueba de rendimiento - no ejecutar en pruebas normales"]
    fn test_rendimiento_lectura_y_generacion ( ) {
        
        use std::time::Instant;
        
        // Configurar paths de prueba
        let test_file = get_test_data_path("test_data.xlsx");
        let plantilla_path = get_test_data_path("plantilla.docx");
        
        println!("📊 Iniciando prueba de rendimiento...");
        println!("📂 Archivo de datos: {}", test_file.display());
        println!("📄 Plantilla DOCX: {}", plantilla_path.display());
        
        // Configuración inicial
        reportes_colegios_recibir_lee(test_file.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_LEE");
        reportes_colegios_recibir_pathplantilla(plantilla_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_PLANTILLA");
        reportes_colegios_recibir_nombrereporte("Test Rendimiento".to_string())
            .expect("Error al configurar NOMBRE_REPORTE");
        reportes_colegios_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Error al configurar FECHA");
        
        // Medir tiempo de ejecución
        let start_time = Instant::now();
        
        // Ejecutar funciones a medir
        let estudiantes = reportes_colegios_leer_estudiantes_aprobados()
            .expect("Error al leer estudiantes");
        
        // Mostrar estadísticas de los datos
        println!("📈 Registros procesados:");
        println!("   - Total estudiantes: {}", estudiantes.len());
        
        // Contar estudiantes por institución
        let mut por_institucion: HashMap<String, usize> = HashMap::new();
        for e in &estudiantes {
            *por_institucion.entry(e.institucion.clone()).or_default() += 1;
        }
        
        println!("   - Distribución por institución:");
        for (institucion, count) in por_institucion {
            println!("     • {}: {} estudiantes", institucion, count);
        }
        
        // Generar los documentos
        reportes_colegios_generar(estudiantes)
            .expect("Error al generar reportes");
        
        let duration = start_time.elapsed();
        
        println!("⏱️ Tiempo total de ejecución: {:.2?}", duration);
        println!("📊 Prueba de rendimiento completada");
        
        // Limpieza (opcional - eliminar archivos generados)
        let output_pattern = "Test Rendimiento - *.docx";
        let mut files_cleaned = 0;
        for entry in glob::glob(output_pattern).expect("Error al buscar archivos para limpiar") {
            if let Ok(path) = entry {
                std::fs::remove_file(&path).ok();
                println!("🧹 Archivo eliminado: {}", path.display());
                files_cleaned += 1;
            }
        }
        println!("🧽 Total archivos limpiados: {}", files_cleaned);
    }

}

