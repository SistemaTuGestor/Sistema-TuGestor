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
use notificaciones::{izquierda::notificaciones_izquierda,derecha::notificaciones_derecha,historial::guardar_historial,historial::leer_historial,leer_archivos::leer_archivo_emparejados,leer_archivos::leer_archivo_control} ;
use notificaciones::{enviar::enviar_mensaje_whatsapp,enviar::leer_qr_code} ;
// Reportes.
use reportes::fecha::obtener_fecha ;
use reportes::lee::{reportes_lee_actualizarfecha,reportes_lee_recibir_pathcarpeta,reportes_lee_recibir_nombrereporte,reportes_lee_leer_archivos_en_carpeta} ;
use reportes::puj::{reportes_puj_actualizarfecha,reportes_puj_recibir_pathplantilla,reportes_puj_recibir_nombrereporte,reportes_puj_leer_universitarios_aprobados,reporte_puj_generar} ;
use reportes::colegios::{reportes_colegios_actualizarfecha,reportes_colegios_recibir_pathplantilla,reportes_colegios_recibir_nombrereporte,reportes_colegios_leer_estudiantes_aprobados,reportes_colegios_generar} ;
use reportes::tutores::{reportes_constanciastutores_actualizarfecha,reportes_constanciastutores_recibir_pathplantilla,reportes_constanciastutores_recibir_nombrereporte,reportes_constanciastutores_generar} ;
use reportes::tutorados::{reportes_constanciastutorados_actualizarfecha,reportes_constanciastutorados_recibir_pathplantilla,reportes_constanciastutorados_recibir_nombrereporte,reportes_constanciastutorados_generar} ;



fn main ( ) {


    tauri::Builder::default ( )

        .invoke_handler ( tauri::generate_handler! [
            

            /* MONITOREO */
            monitoreo_izquierda ,
            monitoreo_derecha ,

            /* NOTIFICACIONES */
            notificaciones_izquierda ,
            notificaciones_derecha ,
            leer_archivo_emparejados ,
            leer_archivo_control ,
            guardar_historial ,
            leer_historial ,
            enviar_mensaje_whatsapp ,
            leer_qr_code ,

            /* REPORTES */
            obtener_fecha ,
            // LEE
            reportes_lee_actualizarfecha ,
            reportes_lee_recibir_pathcarpeta ,
            reportes_lee_recibir_nombrereporte ,
            reportes_lee_leer_archivos_en_carpeta ,
            // PUJ
            reportes_puj_actualizarfecha ,
            reportes_puj_recibir_pathplantilla ,
            reportes_puj_recibir_nombrereporte ,
            reportes_puj_leer_universitarios_aprobados ,
            reporte_puj_generar ,
            // Colegios
            reportes_colegios_actualizarfecha ,
            reportes_colegios_recibir_pathplantilla ,
            reportes_colegios_recibir_nombrereporte ,
            reportes_colegios_leer_estudiantes_aprobados ,
            reportes_colegios_generar ,
            // Tutores
            reportes_constanciastutores_actualizarfecha ,
            reportes_constanciastutores_recibir_pathplantilla ,
            reportes_constanciastutores_recibir_nombrereporte ,
            reportes_constanciastutores_generar ,
            // Tutorados
            reportes_constanciastutorados_actualizarfecha ,
            reportes_constanciastutorados_recibir_pathplantilla ,
            reportes_constanciastutorados_recibir_nombrereporte ,
            reportes_constanciastutorados_generar ,
            
        
        ] )
        .run ( tauri::generate_context!() )
        .expect ( "Error while running tauri application!!" ) ;


}

