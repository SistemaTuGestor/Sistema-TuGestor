
// FECHA
use chrono::Local ;
use serde::Serialize ;



#[derive(Serialize)]
pub struct Fecha {
    fecha: String,
}


#[tauri::command]
pub fn obtener_fecha ( ) -> Fecha {
    let now = Local::now() ;
    let fecha = now.format("%d/%m").to_string() ;
    Fecha { fecha }
}

