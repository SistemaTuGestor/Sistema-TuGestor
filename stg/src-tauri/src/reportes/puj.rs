
// VARIOS
use serde::{Serialize, Deserialize}; // Import Deserialize
use urlencoding::encode;
use xlsxwriter::Workbook;
use xlsxwriter::prelude::FormatColor;
// JSON
use crate:: servicios::logger::log_event;
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
// ...
use std::fs;
use std::path::PathBuf;
// ...
use std::process::Command;



////    VARIABLES GLOBALES      ////

static FECHA: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_LEE: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_PLANTILLA: OnceCell<Mutex<String>> = OnceCell::new();
static NOMBRE_REPORTE: OnceCell<Mutex<String>> = OnceCell::new();
static PATH_SALIDA: OnceCell<Mutex<String>> = OnceCell::new();

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
 log_event("Iniciando lectura de estudiantes universitarios aprobados".to_string())?;
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
   log_event("lectura finalizada".to_string())?;
Ok(estudiantes_aprobados)
}

#[tauri::command]
pub fn reporte_puj_generar(estudiantes: Vec<Estudiante>) -> Result<(), String> {

    // imprimir la lista de estudiantes
    // println!("📂 Lista de estudiantes (PUJ): {:#?}", estudiantes);
     log_event("Iniciando generación de reporte PUJ".to_string())?;
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

    // Aquí usamos &output_path en lugar de output_path para evitar que se mueva
    let nuevo_docx = File::create(&output_path).expect("No se pudo crear el archivo de salida");
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

    // Después de crear el archivo de salida, guardar su directorio
    let output_dir = output_path.parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string());
        
    let _ = PATH_SALIDA.get_or_init(|| Mutex::new(String::new()))
        .lock()
        .map(|mut path| *path = output_dir.clone())
        .map_err(|e| format!("Error al guardar la ruta de salida: {}", e));

         log_event("Reporte PUJ generado exitosamente".to_string())?;
Ok(())
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

// Estructura para el resultado del envío de WhatsApp
#[derive(Serialize)]
pub struct EnvioWhatsApp {
    nombre: String,
    telefono: String,
    mensaje: String,
    whatsapp_url: String,
    archivo_reporte: String,
}

// Función para recuperar la ruta de salida guardada
#[tauri::command]
pub fn reportes_puj_obtener_directorio_salida() -> Result<String, String> {
    match PATH_SALIDA.get() {
        Some(mutex) => {
            mutex.lock()
                .map(|path| path.clone())
                .map_err(|e| format!("Error al acceder a la ruta: {}", e))
        },
        None => Err("La ruta de salida no ha sido inicializada".to_string())
    }
}

// Función para enviar reportes PUJ por WhatsApp
#[tauri::command]
pub fn reportes_puj_enviar_por_whatsapp(directorio_reportes: String) -> Result<Vec<EnvioWhatsApp>, String> {
    // Si no se proporciona directorio, intentar usar el almacenado
    let directorio_final = if directorio_reportes.is_empty() {
        match reportes_puj_obtener_directorio_salida() {
            Ok(dir) => dir,
            Err(_) => return Err("No se ha especificado un directorio y no hay uno guardado previamente".to_string())
        }
    } else {
        directorio_reportes
    };
    
    println!("📱 Preparando envíos de reportes para PUJ desde: {}", directorio_final);
    
    // 1. Leer estudiantes universitarios para tener información
    let estudiantes = match reportes_puj_leer_universitarios_aprobados() {
        Ok(estudiantes) => estudiantes,
        Err(e) => return Err(format!("Error al leer estudiantes universitarios: {}", e)),
    };
    
    println!("📊 Encontrados {} estudiantes universitarios", estudiantes.len());
    
    // 2. Buscar reportes generados en el directorio
    let path = std::path::Path::new(&directorio_final);
    let reportes = match std::fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter(|e| {
                // Crear una variable para evitar valores temporales liberados
                let path = e.path();
                
                // Obtener el nombre como string para búsqueda de "puj"
                let file_name = e.file_name().to_string_lossy().to_lowercase();
                
                // Obtener la extensión de manera segura
                let extension = path.extension().and_then(|ext| ext.to_str());
                
                // Realizar el filtrado
                extension.map_or(false, |ext| (ext == "docx" || ext == "pdf")) && 
                file_name.contains("puj")
            })
            .collect::<Vec<_>>(),
        Err(e) => return Err(format!("Error al leer directorio de reportes: {}", e)),
    };
    
    if reportes.is_empty() {
        return Err("No se encontraron reportes de PUJ en el directorio especificado".to_string());
    }
    
    println!("📊 Encontrados {} reportes PUJ", reportes.len());
    
    // 3. Generar Excel de seguimiento de envíos en el mismo directorio que los reportes
    let fecha = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let envios_file_name = format!("envios_reporte_puj_{}.xlsx", fecha);
    
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
    sheet.write_string(0, 0, "Reporte", Some(&header_format)).unwrap();
    sheet.write_string(0, 1, "Ubicación completa", Some(&header_format)).unwrap();
    sheet.write_string(0, 2, "Destinatario", Some(&header_format)).unwrap();
    sheet.write_string(0, 3, "Teléfono", Some(&header_format)).unwrap();
    sheet.write_string(0, 4, "Enlace WhatsApp", Some(&header_format)).unwrap();
    sheet.write_string(0, 5, "Estado", Some(&header_format)).unwrap();
    
    // Información de contacto del coordinador PUJ (número fijo para todos los reportes)
    let coordinador_nombre = "Coordinador Tututor"; // Reemplaza con el nombre real
    let coordinador_telefono = "3053902328"; // Reemplaza con el número real
    
    // 4. Crear un mensaje para cada reporte encontrado, todos dirigidos al mismo número
    let mut resultados: Vec<EnvioWhatsApp> = vec![];
    let mut row = 1;
    
    // Para cada reporte, crear un mensaje
    for reporte in &reportes {
        let nombre_archivo = reporte.file_name().to_string_lossy().into_owned();
        let ruta_completa = reporte.path().to_string_lossy().into_owned();
        
        // Construir mensaje personalizado
        let mensaje = format!(
            "Hola {}, compartimos contigo el *reporte de PUJ: {}*.\n\n\
            Te informamos que se ha generado el documento oficial con el informe completo.\n\n\
            Resumen: {} tutores universitarios, {} reportes generados.",
            coordinador_nombre, nombre_archivo, estudiantes.len(), reportes.len()
        );
        
        // Crear enlace de WhatsApp
        let tel_limpio = coordinador_telefono.replace(" ", "").replace("-", "").replace("+", "");
        let encoded_message = encode(&mensaje).to_string();
        let whatsapp_url = format!("https://api.whatsapp.com/send?phone={}&text={}",
            tel_limpio, encoded_message);
        
        // Escribir en el Excel
        sheet.write_string(row, 0, &nombre_archivo, None).unwrap();
        sheet.write_string(row, 1, &ruta_completa, None).unwrap();
        sheet.write_string(row, 2, coordinador_nombre, None).unwrap();
        sheet.write_string(row, 3, coordinador_telefono, None).unwrap();
        sheet.write_string(row, 4, &whatsapp_url, None).unwrap();
        sheet.write_string(row, 5, "Pendiente", None).unwrap();
        
        // Agregar a resultados
        resultados.push(EnvioWhatsApp {
            nombre: coordinador_nombre.to_string(),
            telefono: tel_limpio.clone(),
            mensaje,
            whatsapp_url,
            archivo_reporte: ruta_completa,  // Guardamos la ruta completa, no solo el nombre
        });
        
        row += 1;
    }
    
    // 5. Guardar y cerrar el Excel
    workbook.close().map_err(|e| format!("Error al guardar Excel de envíos: {}", e))?;
    
    // Guardar también la ubicación del Excel de envíos
    let _ = PATH_SALIDA.get_or_init(|| Mutex::new(String::new()))
        .lock()
        .map(|mut path| *path = envios_path.parent().unwrap().to_string_lossy().to_string())
        .map_err(|e| format!("Error al guardar la ruta del Excel de envíos: {}", e));
    
    println!("✅ Excel de envíos generado: {}", envios_path.display());
    println!("✅ Total de mensajes a enviar: {}", resultados.len());
    
    Ok(resultados)
}

#[tauri::command]
pub fn verificar_pdfs_existentes_puj ( directorio_reportes:String ) -> Result<bool, String> {
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
    
    println!("🔍 Buscando PDFs de PUJ con fecha: {}", fecha);
    
    // Patrón para buscar: archivos que empiecen con "PUJ" y tengan la fecha entre paréntesis
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            
            if let Some(extension) = path.extension() {
                if extension == "pdf" {
                    let nombre_archivo = entry.file_name().to_string_lossy().to_lowercase();
                    
                    // Verificar si el nombre comienza con "PUJ" y contiene la fecha
                    if nombre_archivo.starts_with("puj") || 
                       (nombre_archivo.contains("puj") && nombre_archivo.contains("(") && 
                        nombre_archivo.contains(")")) {
                        
                        println!("✅ Encontrado archivo PDF: {}", nombre_archivo);
                        found_pdfs = true;
                        break;
                    }
                }
            }
        }
    }
    
    println!("✅ Verificación de PDFs para PUJ: {}", if found_pdfs { "Encontrados" } else { "No encontrados" });

Ok(found_pdfs)
}

