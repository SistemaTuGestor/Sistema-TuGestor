
use std::fs;



#[tauri::command]
pub async fn enviar_mensaje_whatsapp() -> Result<(), String> {
    let script_path = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("scripts")
        .join("enviar.js");

    println!("Ejecutando script: {:?}", script_path); // Debugging log

    let mut command = std::process::Command::new("node");
    command.arg(script_path);

    let output = command.output().map_err(|e| e.to_string())?;

    println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
    println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Error ejecutando script: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}


#[tauri::command]
pub fn leer_qr_code() -> Result<String, String> {
  let qr_path = std::env::current_dir()
    .map_err(|e| e.to_string())?
    .join("src-tauri")
    .join("qrcode.txt");

  match fs::read_to_string(qr_path) {
    Ok(qr) => Ok(qr),
    Err(_) => Ok(String::new()),
  }
}

