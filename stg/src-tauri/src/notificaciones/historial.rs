use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use tauri::command;
use std::fs::read_dir;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Borrador {
    destinatarios: Vec<String>,
    asunto: String,
    mensaje: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BorradorEdit {
    pub destinatarios: Vec<String>,
    pub asunto: String,
    pub mensaje: String,
}

#[command]
pub fn guardar_historial(data: Borrador) -> Result<(), String> {
    let path = "C:\\Users\\Javier\\Desktop\\Proyecto Tututor\\Sistema-TuGestor\\recursos\\Qualtrics\\historiales\\historial.json";
    //let path = "C:\\Users\\USUARIO\\OneDrive\\Documents\\7 semestre\\Sistema-TuGestor\\recursos\\historial.json";

    
    let mut historial: Vec<Borrador> = if Path::new(path).exists() {
        let contenido = fs::read_to_string(path).map_err(|e| format!("Error al leer archivo: {}", e))?;// lee el JSON
        serde_json::from_str(&contenido).unwrap_or_else(|_| Vec::new()) // Aqui se guarda toda la información del JSON, la lee para persistir los datos anteriores, si no hay crea una lista vacia
    } else {
        Vec::new()// si no hay archivo crea uno con una lista vacia
    };
    
    // Agregar el nuevo dato al historial
    historial.push(data);

    // Serializar **todo el historial**
    let json_data = serde_json::to_string_pretty(&historial)
        .map_err(|e| format!("Error al serializar JSON: {}", e))?;
    
    // Escribir todo el historial en el archivo
    fs::write(Path::new(path), json_data)
        .map_err(|e| format!("Error al guardar archivo: {}", e))?;

    Ok(())
}


#[command]
pub fn leer_historial() -> Result<Vec<Borrador>, String> {
    let carpeta_path = "C:\\Users\\Javier\\Desktop\\Proyecto Tututor\\Sistema-TuGestor\\recursos\\Qualtrics\\historiales";

    if !Path::new(carpeta_path).exists() {
        return Ok(Vec::new()); // Si la carpeta no existe, devolver lista vacía
    }

    let mut historial_completo: Vec<Borrador> = Vec::new();

    // Leer los archivos dentro de la carpeta
    let archivos = read_dir(carpeta_path).map_err(|e| format!("Error al leer la carpeta: {}", e))?;

    for archivo in archivos {
        if let Ok(entrada) = archivo {
            let path = entrada.path();

            // Verificar que sea un archivo y tenga extensión .json
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                // Leer el contenido del archivo
                if let Ok(contenido) = fs::read_to_string(&path) {
                    // Intentar deserializar el JSON
                    if let Ok(historial) = serde_json::from_str::<Vec<Borrador>>(&contenido) {
                        historial_completo.extend(historial);
                    }
                }
            }
        }
    }

    Ok(historial_completo)
}


#[command]
pub fn editar_historial(asunto: String) -> Result<Vec<BorradorEdit>, String> {
    println!("entra a la func");
    println!("Buscando historial con asunto: {}", asunto);
    
    let directorio = Path::new("C:\\Users\\Javier\\Desktop\\Proyecto Tututor\\Sistema-TuGestor\\recursos\\Qualtrics\\historiales");
    
    if !directorio.exists() {
        return Err(format!("El directorio {} no existe", directorio.display()));
    }
    
    // Intentar leer todos los archivos en el directorio
    let archivos = match fs::read_dir(directorio) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("No se pudo leer el directorio: {}", e)),
    };
    
    // Iterar sobre cada archivo
    for entrada in archivos {
        let entrada = match entrada {
            Ok(entry) => entry,
            Err(e) => {
                println!("Error al leer entrada: {}", e);
                continue;
            }
        };
        
        if !entrada.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            continue;
        }
        
        if !entrada.path().extension().map(|ext| ext == "json").unwrap_or(false) {
            continue;
        }
        
        let contenido = match fs::read_to_string(entrada.path()) {
            Ok(content) => content,
            Err(e) => {
                println!("Error al leer archivo {}: {}", entrada.path().display(), e);
                continue;
            }
        };
        
        let borradores: Vec<BorradorEdit> = match serde_json::from_str(&contenido) {
            Ok(data) => data,
            Err(e) => {
                println!("Error al parsear JSON de {}: {}", entrada.path().display(), e);
                continue;
            }
        };
        
        for borrador in borradores {
            if borrador.asunto == asunto {
                // Encontramos una coincidencia, imprimir la estructura completa con formato debug
                println!("¡Coincidencia encontrada para asunto: {}!", asunto);
                println!("Estructura del borrador encontrado:");
                println!("====================================");
                println!("Destinatarios: {:?}", borrador.destinatarios);
                println!("Asunto: {}", borrador.asunto);
                println!("Mensaje: {}", borrador.mensaje);
                println!("====================================");
                println!("Estructura completa (Debug): {:#?}", borrador);
                
                return Ok(vec![borrador]);
            }
        }
    }
    
    println!("No se encontró ningún historial con asunto: {}", asunto);
    Ok(Vec::new())
}

#[command]
pub fn actualizar_historial(asunto_original: String, data: BorradorEdit) -> Result<(), String> {
    let path = "C:\\Users\\Javier\\Desktop\\Proyecto Tututor\\Sistema-TuGestor\\recursos\\Qualtrics\\historiales\\historial.json";
    
    // Verificar si el archivo existe
    if !Path::new(path).exists() {
        return Err("El archivo de historial no existe".to_string());
    }
    
    // Leer el contenido actual del archivo
    let contenido = fs::read_to_string(path)
        .map_err(|e| format!("Error al leer archivo: {}", e))?;
    
    // Deserializar el contenido
    let mut historial: Vec<BorradorEdit> = serde_json::from_str(&contenido)
        .map_err(|e| format!("Error al deserializar JSON: {}", e))?;
    
    // Buscar la entrada con el asunto original y actualizarla
    let mut encontrado = false;
    for borrador in &mut historial {
        if borrador.asunto == asunto_original {
            *borrador = data.clone();
            encontrado = true;
            break;
        }
    }
    
    // Si no se encontró la entrada, devolver error
    if !encontrado {
        return Err(format!("No se encontró ninguna entrada con el asunto: {}", asunto_original));
    }
    
    // Serializar todo el historial
    let json_data = serde_json::to_string_pretty(&historial)
        .map_err(|e| format!("Error al serializar JSON: {}", e))?;
    
    // Escribir todo el historial en el archivo
    fs::write(Path::new(path), json_data)
        .map_err(|e| format!("Error al guardar archivo: {}", e))?;
    
    println!("Historial actualizado con éxito para el asunto: {}", asunto_original);
    Ok(())
}


#[command]
pub fn eliminar_historial(asunto: String) -> Result<(), String> {
    let directorio = "C:\\Users\\Javier\\Desktop\\Proyecto Tututor\\Sistema-TuGestor\\recursos\\Qualtrics\\historiales";
    
    // Verificar si el directorio existe
    if !Path::new(directorio).exists() {
        return Err(format!("El directorio {} no existe", directorio));
    }
    
    // Intentar leer todos los archivos en el directorio
    let archivos = match fs::read_dir(directorio) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("No se pudo leer el directorio: {}", e)),
    };
    
    let mut entrada_eliminada = false;
    
    // Iterar sobre cada archivo
    for entrada in archivos {
        let entrada = match entrada {
            Ok(entry) => entry,
            Err(e) => {
                println!("Error al leer entrada: {}", e);
                continue;
            }
        };
        
        // Verificar que sea un archivo
        if !entrada.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            continue;
        }
        
        // Verificar que sea un archivo JSON
        if !entrada.path().extension().map(|ext| ext == "json").unwrap_or(false) {
            continue;
        }
        
        let ruta_archivo = entrada.path();
        println!("Buscando en archivo: {}", ruta_archivo.display());
        
        // Leer el contenido del archivo
        let contenido = match fs::read_to_string(&ruta_archivo) {
            Ok(content) => content,
            Err(e) => {
                println!("Error al leer archivo {}: {}", ruta_archivo.display(), e);
                continue;
            }
        };
        
        // Parsear el JSON
        let mut borradores: Vec<Borrador> = match serde_json::from_str(&contenido) {
            Ok(data) => data,
            Err(e) => {
                println!("Error al parsear JSON de {}: {}", ruta_archivo.display(), e);
                continue;
            }
        };
        
        // Guardar la longitud original para verificar si se eliminó algo
        let longitud_original = borradores.len();
        
        // Filtrar para eliminar la entrada con el asunto especificado
        borradores.retain(|borrador| borrador.asunto != asunto);
        
        // Si se eliminó alguna entrada, actualizar el archivo
        if borradores.len() < longitud_original {
            // Serializar el historial actualizado
            let json_data = match serde_json::to_string_pretty(&borradores) {
                Ok(data) => data,
                Err(e) => return Err(format!("Error al serializar JSON: {}", e)),
            };
            
            // Escribir el historial actualizado en el archivo
            match fs::write(&ruta_archivo, json_data) {
                Ok(_) => {
                    println!("Entrada eliminada con éxito del archivo: {}", ruta_archivo.display());
                    entrada_eliminada = true;
                },
                Err(e) => return Err(format!("Error al escribir archivo {}: {}", ruta_archivo.display(), e)),
            }
        }
    }
    
    // Verificar si se eliminó alguna entrada en algún archivo
    if !entrada_eliminada {
        return Err(format!("No se encontró ninguna entrada con el asunto: {}", asunto));
    }
    
    Ok(())
}