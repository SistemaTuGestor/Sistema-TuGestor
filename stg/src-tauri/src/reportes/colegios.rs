// VARIOS
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
// Nuevas importaciones necesarias
use xlsxwriter::Workbook;
use xlsxwriter::prelude::FormatColor;
use urlencoding::encode;
use std::path::PathBuf;

use std::fs;
use std::process::Command;


static FECHA: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_LEE: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_PLANTILLA: OnceCell<Mutex<String>> = OnceCell::new();
static NOMBRE_REPORTE: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_SALIDA: OnceCell<Mutex<String>> = OnceCell::new(); // Nueva variable global

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
    // Agregar nuevos campos
    #[serde(skip_serializing_if = "Option::is_none")]
    nombre_completo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telefono: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correo: Option<String>,
}

#[tauri::command]
pub fn reportes_colegios_leer_estudiantes_aprobados () -> Result<Vec<Estudiante>, String> {

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
                nombre_completo: None,
                telefono: None,
                correo: None,
            });
        }
    }

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

        // Obtener el directorio de salida del nombre del primer reporte
        let output_dir = Path::new(&nuevo_nombre_archivo).parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());
            
        // Guardar el directorio de salida en la variable global
        let _ = PATH_SALIDA.get_or_init(|| Mutex::new(String::new()))
            .lock()
            .map(|mut path| *path = output_dir.clone())
            .map_err(|e| format!("Error al guardar la ruta de salida: {}", e));
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

#[derive(Serialize)]
pub struct EnvioWhatsApp {
    nombre: String,
    telefono: String,
    institucion: String,
    mensaje: String,
    whatsapp_url: String,
    archivo_reporte: String,
}

#[tauri::command]
pub fn reportes_colegios_enviar_por_whatsapp(directorio_reportes: String) -> Result<Vec<EnvioWhatsApp>, String> {
    // Si no se proporciona directorio, intentar usar el almacenado
    let directorio_final = if directorio_reportes.is_empty() {
        match reportes_colegios_obtener_directorio_salida() {
            Ok(dir) => dir,
            Err(_) => return Err("No se ha especificado un directorio y no hay uno guardado previamente".to_string())
        }
    } else {
        directorio_reportes
    };
    
    println!("📱 Preparando envíos de reportes para colegios desde: {}", directorio_final);
    
    // 1. Leer estudiantes aprobados para tener información de contacto
    let estudiantes = match reportes_colegios_leer_estudiantes_aprobados() {
        Ok(estudiantes) => estudiantes,
        Err(e) => return Err(format!("Error al leer estudiantes aprobados: {}", e)),
    };
    
    // Crear un HashMap para tener acceso rápido a las instituciones de los estudiantes
    let mut instituciones_conocidas: HashMap<String, bool> = HashMap::new();
    
    // Actualizar la estructura Estudiante para incluir los campos necesarios y recopilar instituciones
    let mut estudiantes_completos = Vec::new();
    for mut estudiante in estudiantes {
        // Crear una versión con datos de contacto
        let nombre_completo = obtener_nombre_completo_por_institucion(&estudiante.institucion)
            .unwrap_or_else(|| "Responsable".to_string());
        let telefono = obtener_telefono_por_institucion(&estudiante.institucion);
        let correo = obtener_correo_por_institucion(&estudiante.institucion)
            .unwrap_or_else(|| "sin-correo@ejemplo.com".to_string());
            
        estudiante.nombre_completo = Some(nombre_completo);
        estudiante.telefono = Some(telefono);
        estudiante.correo = Some(correo);
        
        // Registrar la institución en nuestro mapa
        instituciones_conocidas.insert(estudiante.institucion.clone(), true);
        
        estudiantes_completos.push(estudiante);
    }
    
    // 2. Buscar reportes generados en el directorio
    let path = std::path::Path::new(&directorio_final);
    let reportes = match std::fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter(|e| {
                // Crear una variable para evitar valores temporales liberados
                let path = e.path();
                
                // Obtener la extensión de manera segura
                let extension = path.extension().and_then(|ext| ext.to_str());
                
                // Realizar el filtrado
                extension.map_or(false, |ext| (ext == "docx" || ext == "pdf"))
            })
            .collect::<Vec<_>>(),
        Err(e) => return Err(format!("Error al leer directorio de reportes: {}", e)),
    };
    
    println!("📊 Encontrados {} reportes generados", reportes.len());
    
    // 3. Contacto central para todos los colegios
    let contacto_nombre = "Coordinador Servicio Social";
    let contacto_telefono = "3001234567"; // Reemplaza con el número real
    let contacto_correo = "servicio.social@javeriana.edu.co"; // Reemplaza con el correo real
    
    // 4. Generar Excel de seguimiento de envíos en el mismo directorio que los reportes
    let fecha = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let envios_file_name = format!("envios_reporte_colegios_{}.xlsx", fecha);
    
    // Usar el mismo directorio que los reportes para guardar el Excel de envíos
    let envios_path = PathBuf::from(&directorio_final).join(&envios_file_name);
    
    // Crear workbook
    let workbook = Workbook::new(envios_path.to_string_lossy().as_ref())
        .map_err(|e| format!("Error creando Excel de envíos: {}", e))?;
    let mut sheet = workbook.add_worksheet(Some("Envíos"))
        .map_err(|e| format!("Error añadiendo hoja: {}", e))?;
    
    // Formato para encabezados
    let mut header_format = xlsxwriter::Format::new();
    header_format.set_bg_color(FormatColor::Custom(0xD8E4BC));
    
    // Escribir encabezados
    sheet.write_string(0, 0, "Institución", Some(&header_format)).unwrap();
    sheet.write_string(0, 1, "Nombre Contacto", Some(&header_format)).unwrap();
    sheet.write_string(0, 2, "Teléfono Contacto", Some(&header_format)).unwrap();
    sheet.write_string(0, 3, "Correo Contacto", Some(&header_format)).unwrap();
    sheet.write_string(0, 4, "Nombre Destinatario", Some(&header_format)).unwrap();
    sheet.write_string(0, 5, "Teléfono Destinatario", Some(&header_format)).unwrap();
    sheet.write_string(0, 6, "Reporte", Some(&header_format)).unwrap();
    sheet.write_string(0, 7, "Ruta Completa", Some(&header_format)).unwrap();
    sheet.write_string(0, 8, "Enlace WhatsApp", Some(&header_format)).unwrap();
    sheet.write_string(0, 9, "Estado", Some(&header_format)).unwrap();
    
    // 5. Relacionar reportes con instituciones y crear mensajes
    let mut resultados: Vec<EnvioWhatsApp> = vec![];
    let mut row = 1;
    let mut reportes_asignados: HashMap<String, &std::fs::DirEntry> = HashMap::new();
    
    // Primero, intentar asociar cada archivo con una institución conocida
    for reporte in &reportes {
        let nombre_archivo = reporte.file_name().to_string_lossy().to_lowercase();
        
        // Buscar qué institución corresponde a este archivo
        for institucion in instituciones_conocidas.keys() {
            // Verificar si el nombre de la institución (o parte de él) está en el nombre del archivo
            if nombre_archivo.contains(&institucion.to_lowercase()) {
                reportes_asignados.insert(institucion.clone(), reporte);
                break;
            }
        }
    }
    
    // Para cada institución conocida, buscar su reporte y generar los envíos
    for institucion in instituciones_conocidas.keys() {
        // Si encontramos un reporte para esta institución
        if let Some(reporte) = reportes_asignados.get(institucion) {
            let nombre_archivo = reporte.file_name().to_string_lossy().into_owned();
            let ruta_completa = reporte.path().to_string_lossy().into_owned();
            
            // Obtener datos específicos de esta institución
            let nombre_inst_contacto = obtener_nombre_completo_por_institucion(&institucion)
                .unwrap_or_else(|| format!("Coordinador de la institución {} junto con TuTutor", institucion));
            let telefono_inst = obtener_telefono_por_institucion(&institucion);
            let correo_inst = obtener_correo_por_institucion(&institucion)
                .unwrap_or_else(|| format!("contacto@{}.edu.co", institucion.to_lowercase().replace(" ", "")));
            
            // Construir mensaje personalizado
            let mensaje = format!(
                "Hola {}, compartimos contigo el *reporte de colegio {}*.\n\
                Contacto de la institución: {} ({})\n\
                Te informamos que se ha generado el documento oficial con el informe de horas de los estudiantes.",
                contacto_nombre, institucion, nombre_inst_contacto, telefono_inst
            );
            
            // Crear enlace de WhatsApp
            let tel_limpio = contacto_telefono.replace(" ", "").replace("-", "").replace("+", "");
            let encoded_message = encode(&mensaje).to_string();
            let whatsapp_url = format!("https://api.whatsapp.com/send?phone={}&text={}",
                tel_limpio, encoded_message);
            
            // Escribir en el Excel
            sheet.write_string(row, 0, &institucion, None).unwrap();
            sheet.write_string(row, 1, &nombre_inst_contacto, None).unwrap();
            sheet.write_string(row, 2, &telefono_inst, None).unwrap();
            sheet.write_string(row, 3, &correo_inst, None).unwrap();
            sheet.write_string(row, 4, contacto_nombre, None).unwrap();
            sheet.write_string(row, 5, contacto_telefono, None).unwrap();
            sheet.write_string(row, 6, &nombre_archivo, None).unwrap();
            sheet.write_string(row, 7, &ruta_completa, None).unwrap();
            sheet.write_string(row, 8, &whatsapp_url, None).unwrap();
            sheet.write_string(row, 9, "Pendiente", None).unwrap();
            
            // Agregar a resultados
            resultados.push(EnvioWhatsApp {
                nombre: contacto_nombre.to_string(),
                telefono: tel_limpio.clone(),
                institucion: institucion.clone(),
                mensaje: mensaje.clone(),
                whatsapp_url: whatsapp_url.clone(),
                archivo_reporte: ruta_completa,  // Guardamos la ruta completa
            });
            
            row += 1;
        } else {
            println!("⚠️ No se encontró un archivo para la institución: {}", institucion);
            
            // Registrar en el Excel instituciones sin archivo
            sheet.write_string(row, 0, &institucion, None).unwrap();
            sheet.write_string(row, 1, "SIN ARCHIVO", None).unwrap();
            sheet.write_string(row, 2, "", None).unwrap();
            sheet.write_string(row, 3, "", None).unwrap();
            sheet.write_string(row, 4, "", None).unwrap();
            sheet.write_string(row, 5, "", None).unwrap();
            sheet.write_string(row, 6, "", None).unwrap();
            sheet.write_string(row, 7, "", None).unwrap();
            sheet.write_string(row, 8, "", None).unwrap();
            sheet.write_string(row, 9, "Sin reporte", None).unwrap();
            
            row += 1;
        }
    }
    
    // 6. Guardar y cerrar el Excel
    workbook.close().map_err(|e| format!("Error al guardar Excel de envíos: {}", e))?;
    
    println!("✅ Excel de envíos generado: {}", envios_path.display());
    println!("✅ Total de mensajes a enviar: {}", resultados.len());
    
    Ok(resultados)
}

// Funciones auxiliares para obtener datos de contacto
// (Estas deberás implementarlas según cómo obtengas los datos en tu aplicación)

fn obtener_nombre_completo_por_institucion(institucion: &str) -> Option<String> {
    // Implementa la lógica para obtener el nombre del contacto
    // Puedes leer de un JSON, base de datos, etc.
    Some(format!("Coordinador {}", institucion))
}

fn obtener_telefono_por_institucion(_institucion: &str) -> String {
    // Prefijo con _ para evitar la advertencia de variable sin usar
    "3001234567".to_string()
}

fn obtener_correo_por_institucion(institucion: &str) -> Option<String> {
    // Implementa la lógica para obtener el correo del contacto
    Some(format!("contacto.{}@educacion.co", institucion.to_lowercase().replace(" ", ".")))
}

// Función auxiliar para extraer nombre de institución
fn extraer_institucion_de_nombre_archivo(nombre_archivo: &str) -> String {
    // Eliminar extensión
    let sin_extension = nombre_archivo.rsplit_once('.').map_or(nombre_archivo, |(n, _)| n);
    
    // Asumimos que el nombre del archivo sigue un patrón como:
    // "Reporte_Servicio_Social_[NOMBRE_COLEGIO]_[FECHA].docx"
    let partes: Vec<&str> = sin_extension.split('_').collect();
    
    if partes.len() >= 4 {
        // Intentar encontrar la parte que corresponde al colegio
        // Esta lógica puede necesitar ajustes según el formato exacto de tus nombres de archivo
        let posible_institucion = partes[3..partes.len()-1].join(" ");
        if !posible_institucion.is_empty() {
            return posible_institucion;
        }
    }
    
    // Si no podemos extraer correctamente, devolvemos el nombre sin extensión
    sin_extension.to_string()
}

#[tauri::command]
pub fn reportes_colegios_obtener_directorio_salida() -> Result<String, String> {
    match PATH_SALIDA.get() {
        Some(mutex) => {
            mutex.lock()
                .map(|path| path.clone())
                .map_err(|e| format!("Error al acceder a la ruta: {}", e))
        },
        None => Err("La ruta de salida no ha sido inicializada".to_string())
    }
}

// Función para obtener la ruta de los recursos
fn get_resource_path() -> PathBuf {
    // Obtener la ruta de ejecución (directorio del ejecutable)
    let exe_path = std::env::current_exe().expect("No se pudo obtener la ruta del ejecutable");
    let exe_dir = exe_path.parent().expect("No se pudo obtener el directorio del ejecutable");
    
    // En modo desarrollo, la carpeta de recursos estará en el directorio raíz del proyecto
    // En producción, podría estar en un subdirectorio de recursos
    #[cfg(debug_assertions)]
    {
        // En desarrollo, subir varios niveles hasta la raíz del proyecto
        let mut path = exe_dir.to_path_buf();
        // Ajustar según la estructura de tu proyecto
        path.pop(); // subir un nivel
        path.pop(); // subir otro nivel si es necesario
        path.join("resources")
    }
    
    #[cfg(not(debug_assertions))]
    {
        // En producción, los recursos podrían estar en una carpeta específica relativa al ejecutable
        exe_dir.join("resources")
    }
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

