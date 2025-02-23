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
use reportes::lee::{reportes_lee_actualizarfecha,reportes_lee_recibir_pathcarpeta,reportes_lee_recibir_nombrereporte,leer_archivos_en_carpeta} ;
use reportes::puj::{reportes_puj_actualizarfecha,reportes_puj_recibir_nombrereporte,leer_universitarios_aprobados,generar_reporte_puj} ;
use reportes::colegios::{reportes_colegios_actualizarfecha,reportes_colegios_recibir_nombrereporte,generar_reporte_colegios,leer_estudiantes_aprobados} ;
use reportes::tutores::{reportes_constanciastutores_actualizarfecha,reportes_constanciastutores_recibir_pathplantilla,reportes_constanciastutores_recibir_nombrereporte,generar_constanciastutores} ;
use reportes::tutorados::{reportes_constanciastutorados_actualizarfecha,reportes_constanciastutorados_recibir_pathplantilla,reportes_constanciastutorados_recibir_nombrereporte,generar_constanciastutorados} ;



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
            reportes_lee_actualizarfecha ,
            reportes_lee_recibir_pathcarpeta ,
            reportes_lee_recibir_nombrereporte ,
            leer_archivos_en_carpeta ,
            // PUJ
            reportes_puj_actualizarfecha ,
            reportes_puj_recibir_nombrereporte ,
            leer_universitarios_aprobados ,
            generar_reporte_puj ,
            // Colegios
            reportes_colegios_actualizarfecha ,
            reportes_colegios_recibir_nombrereporte ,
            generar_reporte_colegios ,
            leer_estudiantes_aprobados ,
            // Constancias
            reportes_constanciastutores_actualizarfecha ,
            reportes_constanciastutores_recibir_pathplantilla ,
            reportes_constanciastutores_recibir_nombrereporte ,
            generar_constanciastutores ,
            // Sponsor
            reportes_constanciastutorados_actualizarfecha ,
            reportes_constanciastutorados_recibir_pathplantilla ,
            reportes_constanciastutorados_recibir_nombrereporte ,
            generar_constanciastutorados ,
            
        
        ] )
        .run ( tauri::generate_context!() )
        .expect ( "Error while running tauri application!!" ) ;


}

