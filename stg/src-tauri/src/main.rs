// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



////  MÓDULOS  ////
mod monitoreo ;
mod notificaciones ;
mod reportes ;
mod emparejamiento ;

////  FUNCIONES DE MÓDULOS  ////
// Monitoreo.
use monitoreo::{izquierda::monitoreo_izquierda,derecha::monitoreo_derecha} ;
// Notificaciones.
use notificaciones::{izquierda::notificaciones_izquierda,derecha::notificaciones_derecha} ;
use notificaciones::{historial::guardar_historial,historial::leer_historial,historial::editar_historial,historial::actualizar_historial,historial::eliminar_historial,historial::enviar_historiales} ;
use notificaciones::{leer_archivos::notificaciones_inicio_emparejamiento,leer_archivos::notificaciones_inicio_control,leer_archivos::notificaciones_inicio_seguimiento,leer_archivos::notificaciones_inicio_links} ;
use notificaciones::{leer_archivos::leer_archivo_emparejados,leer_archivos::leer_archivo_control,leer_archivos::generar_tutores,leer_archivos::init_path_pruebas,leer_archivos::generar_tutores_enlaces} ;
// Reportes.
use reportes::fecha::obtener_fecha ;
use reportes::lee::{reportes_lee_actualizarfecha,reportes_lee_recibir_emparejamiento,reportes_lee_recibir_pathcarpeta,reportes_lee_recibir_nombrereporte,reportes_lee_leer_archivos_en_carpeta} ;
use reportes::puj::{reportes_puj_actualizarfecha,reportes_puj_recibir_lee,reportes_puj_recibir_pathplantilla,reportes_puj_recibir_nombrereporte,reportes_puj_leer_universitarios_aprobados,reporte_puj_generar} ;
use reportes::colegios::{reportes_colegios_actualizarfecha,reportes_colegios_recibir_lee,reportes_colegios_recibir_pathplantilla,reportes_colegios_recibir_nombrereporte,reportes_colegios_leer_estudiantes_aprobados,reportes_colegios_generar} ;
use reportes::tutores::{reportes_constanciastutores_actualizarfecha,reportes_tutores_recibir_lee,reportes_constanciastutores_recibir_pathplantilla,reportes_constanciastutores_recibir_nombrereporte,reportes_constanciastutores_generar} ;
use reportes::tutorados::{reportes_constanciastutorados_actualizarfecha,reportes_tutorados_recibir_emparejamiento,reportes_constanciastutorados_recibir_pathplantilla,reportes_constanciastutorados_recibir_nombrereporte,reportes_constanciastutorados_generar} ;
// Emparejamiento.
use emparejamiento::emparejamiento::obtener_emparejamiento ;


fn main ( ) {


    tauri::Builder::default ( )

        .invoke_handler ( tauri::generate_handler! [
            

            /* MONITOREO */
            monitoreo_izquierda ,
            monitoreo_derecha ,

            /* NOTIFICACIONES */
            notificaciones_izquierda ,
            notificaciones_derecha ,
            editar_historial ,
            actualizar_historial ,
            eliminar_historial ,
            enviar_historiales ,
            // ELEMENTOS
            notificaciones_inicio_emparejamiento ,
            notificaciones_inicio_control ,
            notificaciones_inicio_seguimiento ,
            notificaciones_inicio_links ,
            leer_archivo_emparejados ,
            leer_archivo_control ,
            generar_tutores ,
            generar_tutores_enlaces ,
            init_path_pruebas ,
            // HISTORIAL
            guardar_historial ,
            leer_historial ,

            /* REPORTES */
            obtener_fecha ,
            // LEE
            reportes_lee_actualizarfecha ,
            reportes_lee_recibir_emparejamiento ,
            reportes_lee_recibir_pathcarpeta ,
            reportes_lee_recibir_nombrereporte ,
            reportes_lee_leer_archivos_en_carpeta ,
            // PUJ
            reportes_puj_actualizarfecha ,
            reportes_puj_recibir_lee ,
            reportes_puj_recibir_pathplantilla ,
            reportes_puj_recibir_nombrereporte ,
            reportes_puj_leer_universitarios_aprobados ,
            reporte_puj_generar ,
            // Colegios
            reportes_colegios_actualizarfecha ,
            reportes_colegios_recibir_lee ,
            reportes_colegios_recibir_pathplantilla ,
            reportes_colegios_recibir_nombrereporte ,
            reportes_colegios_leer_estudiantes_aprobados ,
            reportes_colegios_generar ,
            // Tutores
            reportes_constanciastutores_actualizarfecha ,
            reportes_tutores_recibir_lee ,
            reportes_constanciastutores_recibir_pathplantilla ,
            reportes_constanciastutores_recibir_nombrereporte ,
            reportes_constanciastutores_generar ,
            // Tutorados
            reportes_constanciastutorados_actualizarfecha ,
            reportes_tutorados_recibir_emparejamiento ,
            reportes_constanciastutorados_recibir_pathplantilla ,
            reportes_constanciastutorados_recibir_nombrereporte ,
            reportes_constanciastutorados_generar ,
            // EMPAREJAMIENTO
            obtener_emparejamiento ,
            
        
        ] )
        .run ( tauri::generate_context!() )
        .expect ( "Error while running tauri application!!" ) ;


}

