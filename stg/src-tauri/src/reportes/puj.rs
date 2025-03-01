
// VARIOS
use serde::Serialize ;
// FECHA
use chrono::Local ;
use chrono::NaiveDate ;
// PATH
use once_cell::sync::OnceCell ;
use std::sync::Mutex ;
// ARCHIVOS
use std::fs::File ;
use std::path::Path ;
use std::io::{Read,Write} ;
use calamine::{open_workbook, Reader, Xlsx} ;
use zip::{ZipArchive, write::FileOptions} ;


static FECHA : OnceCell<Mutex<String>> = OnceCell::new() ;
static PATH_PLANTILLA : OnceCell<Mutex<String>> = OnceCell::new() ;
static NOMBRE_REPORTE : OnceCell<Mutex<String>> = OnceCell::new() ;



////    FECHA   ////

#[tauri::command]
pub fn reportes_puj_actualizarfecha ( nueva_fecha:Option<String> ) -> Result<(),String> {

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
    
    // println! ( "Nueva fecha (PUJ): {}", fecha ) ;

Ok(())
}


////    PATH    ////

#[derive(Serialize)]
pub struct NombrePlantilla {
    nombre: String,
}

#[tauri::command]
pub fn reportes_puj_recibir_pathplantilla ( path:String ) -> Result<(),String> {

    let nombre = PATH_PLANTILLA.get_or_init(|| Mutex::new(String::new())) ;
    
    let mut nombre_guardado = nombre.lock().unwrap() ;
    *nombre_guardado = path ;

    // println! ( "📂 Ruta de la plantilla recibida (PUJ): {}",*nombre_guardado ) ;

Ok(())
}


////    NOMBRE REPORTE     ////

#[tauri::command]
pub fn reportes_puj_recibir_nombrereporte ( nombrereporte:String ) -> Result<(),String> {

    let nombre = NOMBRE_REPORTE.get_or_init(|| Mutex::new(String::new())) ;
    
    let mut nombre_guardado = nombre.lock().unwrap() ;
    *nombre_guardado = nombrereporte ;

    // println! ( "📂 Nombre del reporte (PUJ): {}",*nombre_guardado ) ;

Ok(())
}


////    LÓGICA DE ARCHIVOS      ////

#[derive(Serialize, Debug)]
pub struct Estudiante {
    nombre_tutor: String,
    horas_totales: f64,
}

//  --> 🔹 Rutas de los archivos.
const ARCHIVO_EXCEL: &str = "C:\\Users\\Javier\\Downloads\\LEE.xlsx" ;
// const ARCHIVO_EXCEL: &str = "C:\\Users\\USUARIO\\Downloads\\LEE.xlsx" ;
//const ARCHIVO_EXCEL: &str = "/home/user/Downloads/LEE.xlsx" ;

#[tauri::command]
pub fn reportes_puj_leer_universitarios_aprobados ( ) -> Result<Vec<String>,String> {
    
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

        if correo.ends_with("@javeriana.edu.co") && horas_totales >= 60.0 {
            estudiantes_aprobados.push(format!("<w:p><w:r><w:t>- {}</w:t></w:r></w:p>", nombre_tutor));
        }
    }

Ok ( estudiantes_aprobados )
}

#[tauri::command]
pub fn reporte_puj_generar ( estudiantes:Vec<String> ) -> Result<(),String> {

    let lista_tutores = estudiantes.join("") ;

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

    // Construir el nuevo nombre del archivo con la fecha.
    let nuevo_nombre_archivo = format!("{} ({}).docx", nombre_reporte, *fecha);

    // Construir la ruta de salida en el mismo directorio que el archivo original.
    let output_path = Path::new ( &nuevo_nombre_archivo ) ;

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

    zip_writer.finish().expect ( "Error al cerrar el ZIP" ) ;

Ok(())
}

