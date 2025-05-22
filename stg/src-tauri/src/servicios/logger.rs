
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub fecha: String,
    pub hora: String,
    pub accion: String,
}

/// Función de logging interna (sin Tauri command)
pub fn log_event_internal(accion: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Intentando escribir log: {}", accion); // Debug
    
    let ahora = Local::now();
    let log = LogEntry {
        fecha: ahora.format("%Y-%m-%d").to_string(),
        hora: ahora.format("%H:%M:%S").to_string(),
        accion: accion.to_string(),
    };

    let log_path = "log.json";
    
    // Mostrar el directorio actual
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let full_path = current_dir.join(log_path);
    println!("Intentando escribir en: {:?}", full_path); // Debug
    
    // Read existing logs with better error handling
    let mut logs: Vec<LogEntry> = if full_path.exists() {
        let file = File::open(&full_path)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|e| {
            eprintln!("Warning: Could not parse existing log file ({}), starting fresh", e);
            Vec::new()
        })
    } else {
        Vec::new()
    };

    logs.push(log);

    // Write with buffered writer for better performance
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&full_path)?;
    
    println!("Archivo abierto exitosamente"); // Debug
    
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &logs)?;
    
    println!("Log escrito exitosamente"); // Debug
    Ok(())
}

/// Comando de Tauri para logging
#[tauri::command]
pub fn log_event(accion: String) -> Result<String, String> {
    match log_event_internal(&accion) {
        Ok(()) => Ok("Log guardado exitosamente".to_string()),
        Err(e) => Err(format!("Error al guardar log: {}", e)),
    }
}

/// Alternative version that appends to file (more efficient for large logs)
pub fn log_event_append_internal(accion: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ahora = Local::now();
    let log = LogEntry {
        fecha: ahora.format("%Y-%m-%d").to_string(),
        hora: ahora.format("%H:%M:%S").to_string(),
        accion: accion.to_string(),
    };

    let log_path = "log_append.jsonl";
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;
    
    let json_line = serde_json::to_string(&log)?;
    writeln!(file, "{}", json_line)?;
    
    Ok(())
}

/// Comando de Tauri para logging con append
#[tauri::command]
pub fn log_event_append(accion: String) -> Result<String, String> {
    match log_event_append_internal(&accion) {
        Ok(()) => Ok("Log guardado exitosamente (append)".to_string()),
        Err(e) => Err(format!("Error al guardar log: {}", e)),
    }
}

/// Read all log entries from the JSON file
pub fn read_logs_internal() -> Result<Vec<LogEntry>, Box<dyn std::error::Error>> {
    let log_path = "log.json";
    
    if !Path::new(log_path).exists() {
        return Ok(Vec::new());
    }
    
    let file = File::open(log_path)?;
    let reader = BufReader::new(file);
    let logs: Vec<LogEntry> = serde_json::from_reader(reader)?;
    
    Ok(logs)
}

/// Comando de Tauri para leer logs
#[tauri::command]
pub fn read_logs() -> Result<Vec<LogEntry>, String> {
    match read_logs_internal() {
        Ok(logs) => Ok(logs),
        Err(e) => Err(format!("Error al leer logs: {}", e)),
    }
}

/// Read all log entries from JSONL format
pub fn read_logs_jsonl_internal() -> Result<Vec<LogEntry>, Box<dyn std::error::Error>> {
    let log_path = "log_append.jsonl";
    
    if !Path::new(log_path).exists() {
        return Ok(Vec::new());
    }
    
    let mut file = File::open(log_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let mut logs = Vec::new();
    for line in contents.lines() {
        if !line.trim().is_empty() {
            match serde_json::from_str::<LogEntry>(line) {
                Ok(log) => logs.push(log),
                Err(e) => eprintln!("Warning: Could not parse log line '{}': {}", line, e),
            }
        }
    }
    
    Ok(logs)
}

/// Comando de Tauri para leer logs JSONL
#[tauri::command]
pub fn read_logs_jsonl() -> Result<Vec<LogEntry>, String> {
    match read_logs_jsonl_internal() {
        Ok(logs) => Ok(logs),
        Err(e) => Err(format!("Error al leer logs JSONL: {}", e)),
    }
}

/// Convenience function that logs and handles errors internally (para uso interno)
pub fn log_event_safe(accion: &str) {
    println!("log_event_safe llamado con: {}", accion); // Debug
    if let Err(e) = log_event_internal(accion) {
        eprintln!("Error al escribir log: {}", e);
        println!("Error al escribir log: {}", e); // También con println para asegurar que se vea
    } else {
        println!("Log guardado exitosamente"); // Debug
    }
}

/// Función pública para usar dentro de Rust (acepta &str)
pub fn log(accion: &str) {
    log_event_safe(accion);
}

/// Get logs from today
pub fn get_today_logs_internal() -> Result<Vec<LogEntry>, Box<dyn std::error::Error>> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let all_logs = read_logs_internal()?;
    
    let today_logs: Vec<LogEntry> = all_logs
        .into_iter()
        .filter(|log| log.fecha == today)
        .collect();
    
    Ok(today_logs)
}

/// Comando de Tauri para obtener logs de hoy
#[tauri::command]
pub fn get_today_logs() -> Result<Vec<LogEntry>, String> {
    match get_today_logs_internal() {
        Ok(logs) => Ok(logs),
        Err(e) => Err(format!("Error al obtener logs de hoy: {}", e)),
    }
}

/// Clear all logs
pub fn clear_logs_internal() -> Result<(), Box<dyn std::error::Error>> {
    let log_path = "log.json";
    if Path::new(log_path).exists() {
        std::fs::remove_file(log_path)?;
    }
    
    let log_path_jsonl = "log_append.jsonl";
    if Path::new(log_path_jsonl).exists() {
        std::fs::remove_file(log_path_jsonl)?;
    }
    
    Ok(())
}

/// Comando de Tauri para limpiar logs
#[tauri::command]
pub fn clear_logs() -> Result<String, String> {
    match clear_logs_internal() {
        Ok(()) => Ok("Logs eliminados exitosamente".to_string()),
        Err(e) => Err(format!("Error al eliminar logs: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_log_event_internal() {
        let _ = fs::remove_file("log.json");
        
        assert!(log_event_internal("Test action").is_ok());
        
        let logs = read_logs_internal().unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].accion, "Test action");
        
        let _ = fs::remove_file("log.json");
    }

    #[test]
    fn test_log_event_append_internal() {
        let _ = fs::remove_file("log_append.jsonl");
        
        assert!(log_event_append_internal("Test action 1").is_ok());
        assert!(log_event_append_internal("Test action 2").is_ok());
        
        let logs = read_logs_jsonl_internal().unwrap();
        assert_eq!(logs.len(), 2);
        
        let _ = fs::remove_file("log_append.jsonl");
    }

    #[test]
    fn test_today_logs() {
        let _ = fs::remove_file("log.json");
        
        log_event_internal("Today's action").unwrap();
        let today_logs = get_today_logs_internal().unwrap();
        assert_eq!(today_logs.len(), 1);
        
        let _ = fs::remove_file("log.json");
    }
}