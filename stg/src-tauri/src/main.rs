// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



// MÓDULOS
mod monitoreo ;
mod notificaciones ;

// FUNCIONES DE MÓDULOS
use monitoreo::{izquierda::monitoreo_izquierda,derecha::monitoreo_derecha} ;
use notificaciones::{izquierda::notificaciones_izquierda,derecha::notificaciones_derecha} ;



fn main ( ) {

    tauri::Builder::default ( )
        .invoke_handler ( tauri::generate_handler! [
            monitoreo_izquierda ,
            monitoreo_derecha ,
            notificaciones_izquierda ,
            notificaciones_derecha
        ])
        .run ( tauri::generate_context!() )
        .expect ( "Error while running tauri application!!" ) ;

}

