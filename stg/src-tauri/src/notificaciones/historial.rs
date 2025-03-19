use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use tauri::command;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Borrador {
    destinatarios: Vec<String>,
    asunto: String,
    mensaje: String,
}

#[command]
pub fn guardar_historial(data: Borrador) -> Result<(), String> {
    //let path = "C:\\Users\\Javier\\Desktop\\Proyecto Tututor\\Sistema-TuGestor\\recursos\\historial.json";
    let path = "C:\\Users\\USUARIO\\OneDrive\\Documents\\7 semestre\\Sistema-TuGestor\\recursos\\historial.json";

    
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
    //let path = "C:\\Users\\Javier\\Desktop\\Proyecto Tututor\\Sistema-TuGestor\\recursos\\historial.json";
    let path = "C:\\Users\\USUARIO\\OneDrive\\Documents\\7 semestre\\Sistema-TuGestor\\recursos\\historial.json";

    if !Path::new(path).exists() {
        return Ok(Vec::new()); // Si el archivo no existe, devolver lista vacía
    }

    let contenido = fs::read_to_string(path).map_err(|e| format!("Error al leer archivo: {}", e))?;
    let historial: Vec<Borrador> = serde_json::from_str(&contenido)
        .map_err(|e| format!("Error al deserializar JSON: {}", e))?;

    Ok(historial)
}



