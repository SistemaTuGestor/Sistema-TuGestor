use urlencoding;
use open;

#[tauri::command]
pub fn monitoreo_enviar_tarea(
    nombre: String, 
    titulo: String, 
    descripcion: String,
    telefono: String
) -> Result<(), String> {
    // Construct the message text
    let message = format!(
        "Hola {}, te informamos sobre la tarea: {}.\n\nDescripción: {}\n\nSaludos",
        nombre, titulo, descripcion
    );
    
    // URL encode the message
    let encoded_message = urlencoding::encode(&message).to_string();
    
    // Construct the WhatsApp URL (usando un número de prueba)
    let whatsapp_url = format!(
        "https://api.whatsapp.com/send?phone={}&text={}",
        telefono,
        encoded_message
    );
    
    // Open the URL in the default browser
    if let Err(e) = open::that(whatsapp_url) {
        return Err(format!("Failed to open WhatsApp: {}", e));
    }
    
    Ok(())
}