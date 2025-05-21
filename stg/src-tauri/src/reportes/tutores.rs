// VARIOS
use serde::Serialize ;
// FECHA
use chrono::Local ;
use chrono::NaiveDate ;
// PATH
use once_cell::sync::OnceCell ;
use std::sync::Mutex ;
// ARCHIVOS
use std::fs::File;
use std::fs::{self} ;
use std::io::{Read, Write} ;
use std::path::{Path,PathBuf} ;
use zip::write::FileOptions ;
use zip::ZipArchive ;
use calamine::{open_workbook, Reader, Xlsx} ;


use docx_rs::*;
use std::io::BufWriter;
use printpdf::*;

use std::process::Command;
use xlsxwriter::Workbook;
use xlsxwriter::prelude::FormatColor;
use urlencoding::encode;
use std::collections::HashSet;



static FECHA : OnceCell<Mutex<String>> = OnceCell::new() ;
static PATH_LEE: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_PLANTILLA : OnceCell<Mutex<String>> = OnceCell::new() ;
static NOMBRE_REPORTE : OnceCell<Mutex<String>> = OnceCell::new() ;
static PATH_EMPAREJAMIENTO: OnceCell<Mutex<String>> = OnceCell::new(); // Nueva variable estática para la ruta del archivo de emparejamiento



////    FECHA   ////

#[tauri::command]
pub fn reportes_constanciastutores_actualizarfecha ( nueva_fecha:Option<String> ) -> Result<(),String> {

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
        .clone_from(&fecha) ;
    
    // println! ( "Nueva fecha (Tutores): {}", fecha ) ;

Ok(())
}


////    LEE     ////

#[tauri::command]
pub fn reportes_tutores_recibir_lee ( path:String ) -> Result<(),String> {

    let nombre = PATH_LEE.get_or_init(|| Mutex::new(String::new()));

    let mut nombre_guardado = nombre.lock().unwrap();
    *nombre_guardado = path;

    // println!("📂 Ruta de archivo LEE (Tutores): {}", *nombre_guardado);

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

#[tauri::command]
pub fn reportes_constanciastutores_generar ( ) -> Result<(),String> {

    // println!("📖 Cargando archivo Excel...") ;

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

        let nombre_tutor = row[1].to_string().trim().to_string();
        let apellido_tutor = row[0].to_string().trim().to_string();
        let modality = row[4].to_string().trim().to_string();
        // println! ( "📝 Generando constancia para: {} {}",nombre_tutor,apellido_tutor ) ;

        // Se obtiene la fecha de la variable global.
        let fecha = FECHA
            .get()
            .ok_or("❌ FECHA no ha sido inicializado")?
            .lock()
            .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;

        let salida_docx = PathBuf::from(&*directorio).join ( format! (
            "Constancia Tutor {} {} ({}).docx",
            nombre_tutor , apellido_tutor , fecha
        ) ) ;

        // Convert PathBuf to String safely
        let salida_documento = salida_docx.into_os_string()
            .into_string()
            .map_err(|e| format!("❌ El nombre del archivo no es válido UTF-8: {:?}", e))? ;
        
        match crear_constancia ( &nombre_tutor,&apellido_tutor,&modality,&salida_documento ) {
            Ok(_) => println!("✔ Constancia generada: {}", salida_documento),
            Err(e) => eprintln!("❌ Error al generar constancia para {} {}: {}" ,
            nombre_tutor , apellido_tutor , e )
        }
    }

    println!("🎉 ¡Todas las constancias han sido generadas!");

Ok(())
}

fn crear_constancia ( nombre:&str,apellido:&str,modality: &str,salida_path:&str ) -> Result<(),String> {

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
    document_xml = document_xml.replace("«modality»", modality);

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

#[tauri::command]
pub fn convertir_tutores_pdf(urldocs: String) -> Result<(), String> {
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
        
        if path.to_string_lossy().contains("Tutor") && path.extension().and_then(|s| s.to_str()) == Some("docx") {
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
        return Err("No se encontraron archivos DOCX de constancias de tutores para convertir".to_string());
    }

    println!("🎉 Conversión completada: {} archivos convertidos", converted_count);
    Ok(())
}

//// Modificar la estructura TutorSimplificado para incluir campos opcionales
#[derive(Serialize, Debug, Clone)]
pub struct TutorSimplificado {
    pub nombre: String,
    pub apellido: String,
    pub correo: String,
    pub telefono: String,
    pub institucion: String,
    pub archivo_reporte: Option<String>,  // Ruta completa del archivo
    pub mensaje: Option<String>,          // Mensaje personalizado
    pub whatsapp_url: Option<String>,     // URL para WhatsApp
    pub estado: Option<String>,           // Estado del envío
}

#[tauri::command]
pub fn reportes_tutores_leer_emparejamiento() -> Result<Vec<TutorSimplificado>, String> {
    // Obtener la ruta del archivo de emparejamiento
    let archivo_emparejamiento = PATH_EMPAREJAMIENTO
        .get()
        .ok_or("❌ ARCHIVO_EMPAREJAMIENTO no ha sido inicializado")?
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;
    println!("📂 Leyendo archivo de emparejamiento: {}", archivo_emparejamiento);

    let path = Path::new(&*archivo_emparejamiento);

    let mut workbook: Xlsx<_> = open_workbook(path)
        .map_err(|e| format!("❌ No se pudo abrir el archivo Excel: {}", e))?;

    let range = workbook
        .worksheet_range("Emparejamiento")
        .map_err(|e| format!("❌ No se pudo cargar 'Emparejamiento': {}", e))?;

    let mut tutores: Vec<TutorSimplificado> = Vec::new();
    
    println!("📂 Leyendo datos de tutores del archivo de emparejamiento...");

    // Procesar cada fila del Excel, omitiendo la primera (encabezados)
    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            // Ignorar la fila de encabezados
            continue;
        }

        if row.len() < 5 { // Asegurarse que la fila tenga suficientes columnas
            continue;
        }

        // Datos del tutor
        let nombre = row[0].to_string().trim().to_string();
        let apellido = row[1].to_string().trim().to_string();
        let correo = row[2].to_string().trim().to_string();
        let telefono = row[3].to_string().trim().to_string();
        let institucion = row[4].to_string().trim().to_string();

        // Solo agregar si tiene nombre y apellido
        if !nombre.is_empty() && !apellido.is_empty() {
            tutores.push(TutorSimplificado {
                nombre,
                apellido,
                correo,
                telefono,
                institucion,
                // Inicializar los campos opcionales con None
                archivo_reporte: None,
                mensaje: None,
                whatsapp_url: None,
                estado: None,
            });
        }
    }

    // Eliminar duplicados basados en combinación de nombre y apellido
    let mut tutores_unicos: Vec<TutorSimplificado> = Vec::new();
    let mut tutores_vistos = std::collections::HashSet::new();
    
    for tutor in tutores {
        let clave = format!("{}{}", tutor.nombre, tutor.apellido).to_lowercase();
        if !tutores_vistos.contains(&clave) {
            tutores_vistos.insert(clave);
            tutores_unicos.push(tutor);
        }
    }

    println!("✅ Se encontraron {} tutores únicos", tutores_unicos.len());
    
    Ok(tutores_unicos)
}

#[tauri::command]
pub fn reportes_tutores_enviar_por_whatsapp(directorio_reportes: String) -> Result<Vec<TutorSimplificado>, String> {
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
    
    println!("📱 Preparando envíos de constancias de tutores desde: {}", directorio_final);
    
    // 1. Leer información de contacto de los tutores
    let mut tutores = match reportes_tutores_leer_emparejamiento() {
        Ok(tutores) => tutores,
        Err(e) => return Err(format!("Error al leer tutores: {}", e)),
    };
    
    println!("📊 Encontrados {} tutores", tutores.len());
    
    // 2. Buscar archivos de constancias en el directorio
    let path = std::path::Path::new(&directorio_final);
    let reportes = match std::fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter(|e| {
                let path = e.path();
                let file_name = e.file_name().to_string_lossy().to_lowercase();
                let extension = path.extension().and_then(|ext| ext.to_str());
                
                // Filtrar por extensión (docx o pdf) y que contenga "tutor" pero NO "tutorado"
                extension.map_or(false, |ext| (ext == "docx" || ext == "pdf")) && 
                file_name.contains("tutor") && !file_name.contains("tutorado")
            })
            .collect::<Vec<_>>(),
        Err(e) => return Err(format!("Error al leer directorio de constancias: {}", e)),
    };
    
    if reportes.is_empty() {
        return Err("No se encontraron constancias de tutores en el directorio especificado".to_string());
    }
    
    println!("📊 Encontradas {} constancias de tutores", reportes.len());
    
    // 3. Generar Excel de seguimiento de envíos en el mismo directorio
    let fecha = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let envios_file_name = format!("envios_constancias_tutores_{}.xlsx", fecha);
    
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
    sheet.write_string(0, 1, "Apellido", Some(&header_format)).unwrap();
    sheet.write_string(0, 2, "Teléfono", Some(&header_format)).unwrap();
    sheet.write_string(0, 3, "Institución", Some(&header_format)).unwrap();
    sheet.write_string(0, 4, "Correo", Some(&header_format)).unwrap();
    sheet.write_string(0, 5, "Constancia", Some(&header_format)).unwrap();
    sheet.write_string(0, 6, "Ruta Completa", Some(&header_format)).unwrap();
    sheet.write_string(0, 7, "Enlace WhatsApp", Some(&header_format)).unwrap();
    sheet.write_string(0, 8, "Estado", Some(&header_format)).unwrap();
    
    // 4. Crear un mapa de archivos encontrados para búsqueda eficiente
    let mut archivos_por_nombre: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    
    // Recorrer todos los archivos y extraer información para identificar a qué tutor corresponden
    for reporte in &reportes {
        let nombre_archivo = reporte.file_name().to_string_lossy().to_string();
        let ruta_completa = reporte.path().to_string_lossy().to_string();
        
        // Extraer el nombre y apellido del tutor del nombre del archivo
        // Formato esperado: "Constancia Tutor NOMBRE APELLIDO (FECHA).extension"
        if let Some(start) = nombre_archivo.find("Tutor ") {
            if let Some(end) = nombre_archivo.find(" (") {
                let nombre_completo = nombre_archivo[start + 6..end].trim().to_string();
                // El nombre clave será en minúsculas para facilitar la comparación
                let nombre_clave = nombre_completo.to_lowercase();
                archivos_por_nombre.insert(nombre_clave, ruta_completa);
            }
        }
    }
    
    println!("🔍 Relacionando tutores con archivos...");
    
    // 5. Asociar archivos con tutores y generar URLs de WhatsApp
    let mut row = 1;
    let mut tutores_con_archivo: Vec<TutorSimplificado> = Vec::new();
    
    for tutor in &mut tutores {
        // Intentar encontrar el archivo de este tutor por nombre completo
        let mut archivo_encontrado = None;
        
        // 1. Búsqueda por nombre+apellido
        let nombre_completo = format!("{} {}", tutor.nombre, tutor.apellido).to_lowercase();
        if let Some(ruta) = archivos_por_nombre.get(&nombre_completo) {
            archivo_encontrado = Some(ruta.clone());
        } 
        // 2. Búsqueda por correo (algunos archivos pueden incluir el correo)
        else if !tutor.correo.is_empty() {
            // Buscar coincidencias en nombres de archivo
            for (_, ruta) in &archivos_por_nombre {
                if ruta.to_lowercase().contains(&tutor.correo.to_lowercase()) {
                    archivo_encontrado = Some(ruta.clone());
                    break;
                }
            }
        }
        // 3. Búsqueda por palabras clave del nombre/apellido
        else {
            // Buscar por palabras del nombre o apellido que sean distintivas
            let palabras: Vec<String> = format!("{} {}", tutor.nombre, tutor.apellido)
                .split_whitespace()
                .map(|s| s.to_lowercase())
                .filter(|s| s.len() > 3) // Solo palabras con más de 3 letras
                .collect();
                
            // Buscar coincidencias parciales
            'outer: for (nombre_archivo, ruta) in &archivos_por_nombre {
                for palabra in &palabras {
                    if nombre_archivo.contains(palabra) {
                        archivo_encontrado = Some(ruta.clone());
                        break 'outer;
                    }
                }
            }
        }
        
        // Si se encontró un archivo, actualizar los campos del tutor
        if let Some(ruta) = archivo_encontrado {
            // Obtener nombre del archivo
            let path = std::path::Path::new(&ruta);
            let nombre_archivo = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            
            // Construir mensaje personalizado
            let mensaje = format!(
                "Hola {} {},\n\nTe compartimos tu constancia de participación como tutor en el programa de Tututor.\n\
                Gracias por tu dedicación y compromiso con el programa.",
                tutor.nombre, tutor.apellido
            );
            
            // Crear enlace de WhatsApp si tenemos teléfono
            let whatsapp_url = if !tutor.telefono.is_empty() {
                // Limpiar el número de teléfono
                let tel_limpio = tutor.telefono.replace(" ", "").replace("-", "").replace("+", "");
                
                // Codificar el mensaje
                let encoded_message = encode(&mensaje).to_string();
                
                // Crear enlace
                format!("https://api.whatsapp.com/send?phone={}&text={}",
                    tel_limpio, encoded_message)
            } else {
                "No hay teléfono".to_string()
            };
            
            // Actualizar campos del tutor
            tutor.archivo_reporte = Some(ruta.clone());
            tutor.mensaje = Some(mensaje.clone());
            tutor.whatsapp_url = Some(whatsapp_url.clone());
            tutor.estado = Some("Listo para enviar".to_string());
            
            // Escribir en el Excel
            sheet.write_string(row, 0, &tutor.nombre, None).unwrap();
            sheet.write_string(row, 1, &tutor.apellido, None).unwrap();
            sheet.write_string(row, 2, &tutor.telefono, None).unwrap();
            sheet.write_string(row, 3, &tutor.institucion, None).unwrap();
            sheet.write_string(row, 4, &tutor.correo, None).unwrap();
            sheet.write_string(row, 5, &nombre_archivo, None).unwrap();
            sheet.write_string(row, 6, &ruta, None).unwrap();
            sheet.write_string(row, 7, &whatsapp_url, None).unwrap();
            sheet.write_string(row, 8, "Listo para enviar", None).unwrap();
            
            row += 1;
            tutores_con_archivo.push(tutor.clone());
        }
    }
    
    // 6. Guardar y cerrar el Excel
    workbook.close().map_err(|e| format!("Error al guardar Excel de envíos: {}", e))?;
    
    println!("✅ Excel de envíos generado: {}", envios_path.display());
    println!("✅ Total de mensajes a enviar: {}", tutores_con_archivo.len());
    
    Ok(tutores_con_archivo)
}

#[tauri::command]
pub fn reportes_tutores_recibir_emparejamiento(archivoPathEmparejamiento: String) -> Result<(), String> {
    let mut path_emparejamiento = PATH_EMPAREJAMIENTO.get_or_init(|| Mutex::new(String::new()))
        .lock()
        .map_err(|e| format!("❌ No se pudo bloquear el Mutex: {}", e))?;
    
    *path_emparejamiento = archivoPathEmparejamiento;
    println!("✅ Ruta de archivo de emparejamiento para tutores actualizada: {}", path_emparejamiento);
    
    Ok(())
}

#[tauri::command]
pub fn verificar_pdfs_existentes_tutores(directorio_reportes: String, tipo: String) -> Result<bool, String> {
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
    
    println!("🔍 Buscando PDFs de constancias de tutores con fecha: {}", fecha);
    
    // Buscar constancias de tutores en formato PDF
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            
            if let Some(extension) = path.extension() {
                if extension == "pdf" {
                    let nombre_archivo = entry.file_name().to_string_lossy().to_lowercase();
                    
                    // Verificar formatos específicos para constancias de tutores
                    // Formato: "Constancia Tutor NOMBRE APELLIDO (FECHA).pdf"
                    // o formatos similares que contengan email, etc.
                    if nombre_archivo.contains("constancia") && 
                       nombre_archivo.contains("tutor") && 
                       !nombre_archivo.contains("tutorado") && // Excluir tutorados
                       nombre_archivo.contains("(") && 
                       nombre_archivo.contains(")") {
                        
                        println!("✅ Encontrado archivo PDF de constancia de tutor: {}", nombre_archivo);
                        found_pdfs = true;
                        break;
                    }
                }
            }
        }
    }
    
    println!("✅ Verificación de PDFs para tutores: {}", if found_pdfs { "Encontrados" } else { "No encontrados" });
    Ok(found_pdfs)
}