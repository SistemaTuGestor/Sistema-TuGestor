// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



////  MÓDULOS  ////
mod monitoreo ;
mod notificaciones ;
mod reportes ;

////  FUNCIONES DE MÓDULOS  ////
// Monitoreo.
use monitoreo::{izquierda::monitoreo_izquierda,derecha::monitoreo_derecha} ;
// Notificaciones.
use notificaciones::{izquierda::notificaciones_izquierda,derecha::notificaciones_derecha} ;
// Reportes.
use reportes::fecha::obtener_fecha ;
use reportes::lee::{reportes_lee_actualizar_fecha,reportes_lee_recibir_pathcarpeta,reportes_lee_recibir_nombrereporte,leer_archivos_en_carpeta};
use reportes::puj::{reportes_puj_actualizar_fecha,leer_universitarios_aprobados,generar_reporte_puj} ;
use reportes::colegios::{reportes_colegios_actualizar_fecha, generar_reporte_colegios, leer_estudiantes_aprobados} ;
use reportes::constancias::{generar_constancias} ;



fn main ( ) {


    tauri::Builder::default ( )

        .invoke_handler ( tauri::generate_handler! [
            

            /* MONITOREO */
            monitoreo_izquierda ,
            monitoreo_derecha ,

            /* NOTIFICACIONES */
            notificaciones_izquierda ,
            notificaciones_derecha ,

            /* REPORTES */
            obtener_fecha ,
            // LEE
            reportes_lee_actualizar_fecha ,
            reportes_lee_recibir_pathcarpeta ,
            reportes_lee_recibir_nombrereporte ,
            leer_archivos_en_carpeta ,
            // PUJ
            reportes_puj_actualizar_fecha ,
            leer_universitarios_aprobados ,
            generar_reporte_puj ,
            // Colegios
            reportes_colegios_actualizar_fecha ,
            generar_reporte_colegios ,
            leer_estudiantes_aprobados ,
            // Constancias
            generar_constancias ,
            
            // Sponsor
            
        
        ] )
        .run ( tauri::generate_context!() )
        .expect ( "Error while running tauri application!!" ) ;


}

