
// FECHA
use chrono::Local ;
use chrono::NaiveDate ;
// PATH
use once_cell::sync::OnceCell ;
use std::sync::Mutex ;
// JSON
use crate::servicios::logger::log_event ;
// ARCHIVOS
use std::fs::{self} ;
use std::path::{Path,PathBuf} ;
use std::io::{Read, Write} ;
use calamine::{open_workbook, Reader, Xlsx} ;
use zip::{ZipArchive, write::FileOptions} ;
// ...
use std::process::Command;
// ...
use serde::Serialize;
use std::collections::HashSet;
use xlsxwriter::Workbook;
use xlsxwriter::prelude::FormatColor;
use urlencoding::encode;



////    VARIABLES GLOBALES      ////

static FECHA : OnceCell<Mutex<String>> = OnceCell::new() ;
static PATH_EMPAREJAMIENTO: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_PLANTILLA : OnceCell<Mutex<String>> = OnceCell::new() ;
static NOMBRE_REPORTE : OnceCell<Mutex<String>> = OnceCell::new() ;


////    FECHA   ////

#[tauri::command]
pub fn reportes_constanciastutorados_actualizarfecha ( nueva_fecha:Option<String> ) -> Result<(),String> {

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
        let _ = log_event(format!("Generando constancia para: {}", tutorado_1))?;
        generar_constancia(&tutorado_1)?;
        let _ = log_event("generacion de constancia exitosa".to_string())?;
        let _ = log_event(format!("Generando constancia para: {}", tutorado_2))?;
        generar_constancia(&tutorado_2)?;
        let _ = log_event("generacion de constancia exitosa".to_string())?;

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
        return Err("No se encontraron archivos DOCX de constancias de tutorados para convertir".to_string());
    }

    println!("🎉 Conversión completada: {} archivos convertidos", converted_count);
    Ok(())
}

// Actualizamos la estructura ContactoSimplificado con los atributos adicionales
#[derive(Serialize, Debug, Clone)]
pub struct ContactoSimplificado {
    pub nombre: String,
    pub telefono: String,
    pub institucion: String,
    pub correo: String,
    pub archivo_reporte: Option<String>,  // Ruta completa del archivo
    pub mensaje: Option<String>,          // Mensaje personalizado
    pub whatsapp_url: Option<String>,     // URL para WhatsApp
    pub estado: Option<String>,           // Estado del envío
}

#[tauri::command]
pub fn leer_archivo_emparejamiento ( ) -> Result<Vec<ContactoSimplificado>,String> {

    let _ = log_event("iniciando lectura del archivo de emparejamiento".to_string());

    // Obtener la ruta del archivo de emparejamiento
    
    let archivo_emparejamiento = PATH_EMPAREJAMIENTO
        .get()
        .ok_or("❌ PATH_EMPAREJAMIENTO no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

    let path = Path::new(&*archivo_emparejamiento);
    
    let mut workbook: Xlsx<_> = open_workbook(path)
        .map_err(|e| format!("❌ No se pudo abrir el archivo Excel: {}", e))?;

    let range = workbook
        .worksheet_range("Emparejamiento")
        .map_err(|e| format!("❌ No se pudo cargar 'Emparejamiento': {}", e))?;

    let mut contactos: Vec<ContactoSimplificado> = Vec::new();
    
    println!("📂 Leyendo contactos simplificados del archivo de emparejamiento...");

    // Procesar cada fila del Excel, omitiendo la primera (encabezados)
    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            println!("⚠ Ignorando encabezado...");
            continue;
        }

        if row.len() < 15 { // Asegurarse que la fila tenga suficientes columnas
            continue;
        }

        // Datos del tutorado 1
        let tutorado_1 = row[10].to_string().trim().to_string();
        if !tutorado_1.is_empty() && row.len() >= 15 {
            let telefono_tutorado_1 = row[13].to_string().trim().to_string();
            let colegio_tutorado_1 = row[12].to_string().trim().to_string();
            let correo_tutorado_1 = row[15].to_string().trim().to_string();

            contactos.push(ContactoSimplificado {
                nombre: tutorado_1,
                telefono: telefono_tutorado_1,
                institucion: colegio_tutorado_1,
                correo: correo_tutorado_1,
                archivo_reporte: None,
                mensaje: None,
                whatsapp_url: None,
                estado: None,
            });
        }

        // Datos del segundo tutorado (si existe)
        if row.len() >= 35 {
            let tutorado_2 = row[30].to_string().trim().to_string();
            if !tutorado_2.is_empty() {
                let telefono_tutorado_2 = row[33].to_string().trim().to_string();
                let colegio_tutorado_2 = row[32].to_string().trim().to_string();
                let correo_tutorado_2 = row[35].to_string().trim().to_string();

                contactos.push(ContactoSimplificado {
                    nombre: tutorado_2,
                    telefono: telefono_tutorado_2,
                    institucion: colegio_tutorado_2,
                    correo: correo_tutorado_2,
                    archivo_reporte: None,
                    mensaje: None,
                    whatsapp_url: None,
                    estado: None,
                });
            }
        }
    }

    // Eliminar duplicados usando un HashSet
    let mut contactos_unicos: Vec<ContactoSimplificado> = Vec::new();
    let mut nombre_vistos = HashSet::new();
    
    for contacto in contactos {
        if !nombre_vistos.contains(&contacto.nombre) {
            nombre_vistos.insert(contacto.nombre.clone());
            contactos_unicos.push(contacto);
        }
    }

    println!("✅ Se encontraron {} contactos simplificados únicos", contactos_unicos.len());
    let _ = log_event("lectura del archivo de emparejamiento finalizada".to_string());

Ok(contactos_unicos)
}

#[tauri::command]
pub fn reportes_tutorados_enviar_por_whatsapp ( directorio_reportes:String ) -> Result<Vec<ContactoSimplificado>,String> {
    
    // Si no se proporciona directorio, usar el directorio de salida
    let directorio_final = if directorio_reportes.is_empty() {
        match NOMBRE_REPORTE.get() {
            Some(mutex) => {
                let dir = mutex.lock()
                    .map_err(|e| format!("Error al acceder al directorio: {}", e))?;
                dir.clone()
            },
            None => return Err("No se ha especificado un directorio y no hay uno guardado previamente".to_string())
        }
    } else {
        directorio_reportes
    };
    
    println!("📱 Preparando envíos de constancias de tutorados desde: {}", directorio_final);
    
    // 1. Leer información de contacto de los tutorados
    let mut contactos = match leer_archivo_emparejamiento() {
        Ok(contactos) => contactos,
        Err(e) => return Err(format!("Error al leer contactos: {}", e)),
    };
    
    println!("📊 Encontrados {} contactos de tutorados", contactos.len());
    
    // 2. Buscar archivos de constancias en el directorio
    let path = std::path::Path::new(&directorio_final);
    let reportes = match std::fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter(|e| {
                let path = e.path();
                let file_name = e.file_name().to_string_lossy().to_lowercase();
                let extension = path.extension().and_then(|ext| ext.to_str());
                
                // Filtrar por extensión (docx o pdf) y que contenga "tutorado" específicamente
                extension.map_or(false, |ext| (ext == "docx" || ext == "pdf")) && 
                file_name.contains("tutorado")
            })
            .collect::<Vec<_>>(),
        Err(e) => return Err(format!("Error al leer directorio de constancias: {}", e)),
    };
    
    if reportes.is_empty() {
        return Err("No se encontraron constancias de tutorados en el directorio especificado".to_string());
    }
    
    println!("📊 Encontradas {} constancias de tutorados", reportes.len());
    
    // 3. Generar Excel de seguimiento de envíos en el mismo directorio
    let fecha = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let envios_file_name = format!("envios_constancias_tutorados_{}.xlsx", fecha);
    
    // Usar el mismo directorio para guardar el Excel de envíos
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
    sheet.write_string(0, 0, "Nombre", Some(&header_format)).unwrap();
    sheet.write_string(0, 1, "Teléfono", Some(&header_format)).unwrap();
    sheet.write_string(0, 2, "Institución", Some(&header_format)).unwrap();
    sheet.write_string(0, 3, "Correo", Some(&header_format)).unwrap();
    sheet.write_string(0, 4, "Constancia", Some(&header_format)).unwrap();
    sheet.write_string(0, 5, "Ruta Completa", Some(&header_format)).unwrap();
    sheet.write_string(0, 6, "Enlace WhatsApp", Some(&header_format)).unwrap();
    sheet.write_string(0, 7, "Estado", Some(&header_format)).unwrap();
    
    // 4. Crear un mapa de archivos encontrados por nombre de tutorado para búsqueda eficiente
    let mut archivos_por_nombre: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    
    // Recorrer todos los archivos y extraer el nombre del tutorado del nombre del archivo
    for reporte in &reportes {
        let nombre_archivo = reporte.file_name().to_string_lossy().to_string();
        let ruta_completa = reporte.path().to_string_lossy().to_string();
        
        // Extraer el nombre del tutorado del nombre del archivo
        // Formato esperado: "Constancia Tutorado NOMBRE (FECHA).extension"
        if let Some(start) = nombre_archivo.find("Tutorado ") {
            if let Some(end) = nombre_archivo.find(" (") {
                let nombre_tutorado = nombre_archivo[start + 9..end].trim().to_string();
                archivos_por_nombre.insert(nombre_tutorado, ruta_completa);
            }
        }
    }
    
    println!("🔍 Relacionando contactos con archivos...");
    
    // 5. Asociar archivos con contactos y generar URLs de WhatsApp
    let mut row = 1;
    let mut contactos_con_archivo: Vec<ContactoSimplificado> = Vec::new();
    
    for contacto in &mut contactos {
        // Intentar encontrar el archivo de este tutorado por nombre
        let mut archivo_encontrado = None;
        
        // 1. Búsqueda directa por nombre completo
        if let Some(ruta) = archivos_por_nombre.get(&contacto.nombre) {
            archivo_encontrado = Some(ruta.clone());
        } 
        // 2. Búsqueda por palabras clave del nombre
        else {
            // Dividir el nombre en palabras para búsqueda parcial
            let palabras: Vec<String> = contacto.nombre
                .split_whitespace()
                .map(|s| s.to_lowercase())
                .collect();
                
            // Filtrar palabras muy cortas
            let palabras_clave: Vec<String> = palabras
                .iter()
                .filter(|p| p.len() > 3)  // Solo palabras con más de 3 letras
                .cloned()
                .collect();
                
            // Buscar coincidencias parciales
            for (nombre_archivo, ruta) in &archivos_por_nombre {
                let nombre_lower = nombre_archivo.to_lowercase();
                
                // Si alguna palabra clave está en el nombre del archivo
                if palabras_clave.iter().any(|palabra| nombre_lower.contains(palabra)) {
                    archivo_encontrado = Some(ruta.clone());
                    break;
                }
            }
        }
        
        // Si se encontró un archivo, actualizar los campos del contacto
        if let Some(ruta) = archivo_encontrado {
            // Obtener nombre del archivo
            let path = std::path::Path::new(&ruta);
            let nombre_archivo = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            
            // Construir mensaje personalizado
            let mensaje = format!(
                "Hola {},\n\nTe compartimos tu constancia de participación en el programa de Tututor.\n\
                Gracias por tu dedicación y compromiso con el programa.",
                contacto.nombre
            );
            
            // Crear enlace de WhatsApp si tenemos teléfono
            let whatsapp_url = if !contacto.telefono.is_empty() {
                // Limpiar el número de teléfono
                let tel_limpio = contacto.telefono.replace(" ", "").replace("-", "").replace("+", "");
                
                // Codificar el mensaje
                let encoded_message = encode(&mensaje).to_string();
                
                // Crear enlace
                format!("https://api.whatsapp.com/send?phone={}&text={}",
                    tel_limpio, encoded_message)
            } else {
                "No hay teléfono".to_string()
            };
            
            // Actualizar campos del contacto
            contacto.archivo_reporte = Some(ruta.clone());
            contacto.mensaje = Some(mensaje.clone());
            contacto.whatsapp_url = Some(whatsapp_url.clone());
            contacto.estado = Some("Listo para enviar".to_string());
            
            // Escribir en el Excel
            sheet.write_string(row, 0, &contacto.nombre, None).unwrap();
            sheet.write_string(row, 1, &contacto.telefono, None).unwrap();
            sheet.write_string(row, 2, &contacto.institucion, None).unwrap();
            sheet.write_string(row, 3, &contacto.correo, None).unwrap();
            sheet.write_string(row, 4, &nombre_archivo, None).unwrap();
            sheet.write_string(row, 5, &ruta, None).unwrap();
            sheet.write_string(row, 6, &whatsapp_url, None).unwrap();
            sheet.write_string(row, 7, "Listo para enviar", None).unwrap();
            
            row += 1;
            contactos_con_archivo.push(contacto.clone());
        }
    }
    
    // 6. Guardar y cerrar el Excel
    workbook.close().map_err(|e| format!("Error al guardar Excel de envíos: {}", e))?;
    
    println!("✅ Excel de envíos generado: {}", envios_path.display());
    println!("✅ Total de mensajes a enviar: {}", contactos_con_archivo.len());
    
    Ok(contactos_con_archivo)
}

#[tauri::command]
pub fn verificar_pdfs_existentes_tutorados ( directorio_reportes:String ) -> Result<bool, String> {
    println!("🔍 Verificando PDFs existentes en: {}", directorio_reportes);
    let path = std::path::Path::new(&directorio_reportes);
    
    if !path.exists() {
        return Err(format!("El directorio {} no existe", directorio_reportes));
    }
    
    let entries = match std::fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("Error al leer el directorio: {}", e)),
    };
    
    let mut found_pdfs = false;
    
    // Obtener la fecha de la variable global para buscar archivos con esa fecha
    let fecha = match FECHA.get() {
        Some(mutex) => {
            match mutex.lock() {
                Ok(guard) => guard.clone(),
                Err(_) => Local::now().format("%d-%m-%Y").to_string() // Fecha actual como valor por defecto
            }
        },
        None => Local::now().format("%d-%m-%Y").to_string() // Fecha actual como valor por defecto
    };
    
    println!("🔍 Buscando PDFs de constancias de tutorados con fecha: {}", fecha);
    
    // Buscar constancias de tutorados en formato PDF
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            
            if let Some(extension) = path.extension() {
                if extension == "pdf" {
                    let nombre_archivo = entry.file_name().to_string_lossy().to_lowercase();
                    
                    // Verificar formatos específicos para constancias de tutorados
                    // Formato: "Constancia Tutorado NOMBRE APELLIDO (FECHA).pdf"
                    if nombre_archivo.contains("constancia") && 
                       nombre_archivo.contains("tutorado") && 
                       nombre_archivo.contains("(") && 
                       nombre_archivo.contains(")") {
                        
                        println!("✅ Encontrado archivo PDF de constancia de tutorado: {}", nombre_archivo);
                        found_pdfs = true;
                        break;
                    }
                }
            }
        }
    }
    
    println!("✅ Verificación de PDFs para tutorados: {}", if found_pdfs { "Encontrados" } else { "No encontrados" });

    Ok(found_pdfs)
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

    fn setup_test_environment() {
        // Configurar rutas de prueba
        let emparejamiento_path = get_test_data_path("emparejamiento_final.xlsx");
        let plantilla_path = get_test_data_path("plantilla_tutorados.docx");
        
        reportes_tutorados_recibir_emparejamiento(emparejamiento_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_EMPAREJAMIENTO");
            
        reportes_constanciastutorados_recibir_pathplantilla(plantilla_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_PLANTILLA");
            
        reportes_constanciastutorados_recibir_nombrereporte("output_test".to_string())
            .expect("Error al configurar NOMBRE_REPORTE");
            
        reportes_constanciastutorados_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Error al configurar FECHA");
    }

    #[test]
    fn test_actualizar_fecha() {
        let result = reportes_constanciastutorados_actualizarfecha(Some("2023-05-15".to_string()));
        assert!(result.is_ok());
        
        let fecha_guardada = FECHA.get().unwrap().lock().unwrap();
        assert_eq!(*fecha_guardada, "15-05-2023");
    }

    #[test]
    fn test_recibir_paths() {
        assert!(reportes_tutorados_recibir_emparejamiento("test_emparejamiento.xlsx".to_string()).is_ok());
        assert!(reportes_constanciastutorados_recibir_pathplantilla("test_plantilla.docx".to_string()).is_ok());
        assert!(reportes_constanciastutorados_recibir_nombrereporte("Test Report".to_string()).is_ok());
    }

    #[test]
    #[ignore = "Requiere archivos de prueba"]
    fn test_generar_constancias() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let output_dir = temp_dir.path().to_str().unwrap().to_string();
        
        // Initialize test data
        let emparejamiento_path = get_test_data_path("emparejamiento_final.xlsx");
        let plantilla_path = get_test_data_path("plantilla_tutorados.docx");
        
        reportes_tutorados_recibir_emparejamiento(emparejamiento_path.to_str().unwrap().to_string())
            .expect("Failed to set emparejamiento path");
        reportes_constanciastutorados_recibir_pathplantilla(plantilla_path.to_str().unwrap().to_string())
            .expect("Failed to set plantilla path");
        reportes_constanciastutorados_recibir_nombrereporte(output_dir.clone())
            .expect("Failed to set output dir");
        reportes_constanciastutorados_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Failed to set date");

        // Run test
        let result = reportes_constanciastutorados_generar();
        assert!(result.is_ok(), "Failed to generate constancias: {:?}", result);
        
        // Verify files were created
        let entries = fs::read_dir(&output_dir)
            .expect("Failed to read output dir")
            .count();
        assert!(entries > 0, "No files were generated");
    }

    #[test]
    #[ignore = "Requiere plantilla DOCX"]
    fn test_crear_constancia_individual() {
        setup_test_environment();
        
        let test_name = "Tutorado de Prueba";
        let output_path = get_test_data_path("output/constancia_test.docx");
        
        // Asegurar que el directorio de output existe
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).expect("Error al crear directorio de output");
        }

        let result = crear_constancia(test_name, output_path.to_str().unwrap());
        assert!(result.is_ok(), "Error al crear constancia: {:?}", result.err());
        assert!(output_path.exists(), "No se generó el archivo de constancia");

        // Limpieza
        if output_path.exists() {
            fs::remove_file(output_path).ok();
        }
    }

    #[test]
    #[ignore = "Requiere archivos de prueba"]
    fn test_rendimiento_tutorados ( ) {

        use std::time::Instant;

        let temp_dir = tempfile::tempdir().expect("No se pudo crear carpeta temporal");
        let output_dir = temp_dir.path().to_str().unwrap().to_string();

        // Rutas de prueba
        let emparejamiento_path = get_test_data_path("emparejamiento_final.xlsx");
        let plantilla_path = get_test_data_path("plantilla_tutorados.docx");

        // Configuración de entorno
        reportes_tutorados_recibir_emparejamiento(emparejamiento_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_EMPAREJAMIENTO");
        reportes_constanciastutorados_recibir_pathplantilla(plantilla_path.to_str().unwrap().to_string())
            .expect("Error al configurar PATH_PLANTILLA");
        reportes_constanciastutorados_recibir_nombrereporte(output_dir.clone())
            .expect("Error al configurar NOMBRE_REPORTE");
        reportes_constanciastutorados_actualizarfecha(Some("2023-01-01".to_string()))
            .expect("Error al configurar FECHA");

        let start_time = Instant::now();

        // Contar filas en la hoja de emparejamiento (ignorando encabezado)
        let mut workbook: Xlsx<_> = open_workbook(emparejamiento_path).expect("No se pudo abrir el archivo Excel");
        let range = workbook
            .worksheet_range("Emparejamiento")
            .expect("No se pudo encontrar la hoja 'Emparejamiento'");
        let total_registros = range.rows().skip(1).count(); // omitir encabezado

        println!("📊 Registros en emparejamiento: {}", total_registros);

        // Ejecutar generación
        let result = reportes_constanciastutorados_generar();
        assert!(result.is_ok(), "Falló la generación de constancias: {:?}", result.err());

        // Verificar archivos generados
        let archivos_generados: Vec<_> = fs::read_dir(&output_dir)
            .expect("No se pudo leer el directorio de salida")
            .filter_map(|e| {
                let entry = e.ok()?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("docx") &&
                   path.file_name().and_then(|s| s.to_str()).map(|n| n.contains("Tutorado")).unwrap_or(false) {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        let duracion = start_time.elapsed();
        println!("\n⏱ Tiempo total: {:.2?}", duracion);

        println!("📄 Constancias generadas: {}\n", archivos_generados.len());

        assert!(
            archivos_generados.len() > 0,
            "No se generaron constancias DOCX"
        );
    }

}

