// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



// MÓDULOS
mod monitoreo ;
mod notificaciones ;
mod reportes ;

// FUNCIONES DE MÓDULOS
use monitoreo::{izquierda::monitoreo_izquierda,derecha::monitoreo_derecha} ;
use notificaciones::{izquierda::notificaciones_izquierda,derecha::notificaciones_derecha} ;
use reportes::fecha::{obtener_fecha,actualizar_fecha} ;
use reportes::lee::leer_excel_path_fijo_lee ;
use reportes::lee::recibir_path_carpeta ;


fn main ( ) {


    tauri::Builder::default ( )

        .invoke_handler(tauri::generate_handler![

            monitoreo_izquierda,
            monitoreo_derecha,
            notificaciones_izquierda,
            notificaciones_derecha,
            obtener_fecha,
            actualizar_fecha,
            leer_excel_path_fijo_lee,
            recibir_path_carpeta
            
        
        ] )
        .run ( tauri::generate_context!() )
        .expect ( "Error while running tauri application!!" ) ;


}

