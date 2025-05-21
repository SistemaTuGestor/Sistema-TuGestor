// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



////  MÓDULOS  ////
mod monitoreo;
mod notificaciones;
mod reportes;
mod emparejamiento;
mod servicios;


////  FUNCIONES DE MÓDULOS  ////
// Servicios.
use servicios::tarea::monitoreo_enviar_tarea;
use servicios::{whatsapp::procesar_datos_para_whatsapp,whatsapp::procesar_mensajes_desde_json,whatsapp::exportar_mensajes_a_excel};
// Monitoreo.
use monitoreo::{persistencia::leer_excel_emparejamiento, persistencia::actualizar_json_monitoreo,  persistencia::obtener_roles_unicos, persistencia::obtener_instituciones_unicas};
use monitoreo::persistencia::{cargar_datos_json,agregar_tarea_y_guardar,agregar_imagen_y_guardar,eliminar_item_monitoreo,editar_item_monitoreo,toggle_hecho_monitoreo};
// Notificaciones.
use notificaciones::{historial::guardar_historial,historial::leer_historial,historial::editar_historial, historial::actualizar_historial, historial::eliminar_historial, historial::enviar_historiales};
use notificaciones::leer_archivos::{notificaciones_inicio_emparejamiento,notificaciones_inicio_control,notificaciones_inicio_seguimiento,notificaciones_inicio_links,leer_archivo_emparejados,generar_tutores,generar_tutores_enlaces,init_path_pruebas,leer_archivo_control};
// Reportes.
use reportes::fecha::obtener_fecha;
use reportes::lee::{reportes_lee_actualizarfecha, reportes_lee_recibir_emparejamiento, reportes_lee_recibir_pathcarpeta, reportes_lee_recibir_nombrereporte, reportes_lee_leer_archivos_en_carpeta};
use reportes::puj::{reportes_puj_actualizarfecha, reportes_puj_recibir_lee, reportes_puj_recibir_pathplantilla, reportes_puj_recibir_nombrereporte, reportes_puj_leer_universitarios_aprobados, reporte_puj_generar, convertir_puj_pdf,reportes_puj_enviar_por_whatsapp,verificar_pdfs_existentes_puj};
use reportes::colegios::{reportes_colegios_actualizarfecha, reportes_colegios_recibir_lee, reportes_colegios_recibir_pathplantilla, reportes_colegios_recibir_nombrereporte, reportes_colegios_leer_estudiantes_aprobados, reportes_colegios_generar, convertir_colegios_pdf,reportes_colegios_enviar_por_whatsapp,verificar_pdfs_existentes_colegios};
use reportes::tutores::{reportes_constanciastutores_actualizarfecha, reportes_tutores_recibir_lee, reportes_constanciastutores_recibir_pathplantilla, reportes_constanciastutores_recibir_nombrereporte, reportes_constanciastutores_generar, convertir_tutores_pdf,reportes_tutores_enviar_por_whatsapp,reportes_tutores_recibir_emparejamiento,verificar_pdfs_existentes_tutores};
use reportes::tutorados::{reportes_constanciastutorados_actualizarfecha, reportes_tutorados_recibir_emparejamiento, reportes_constanciastutorados_recibir_pathplantilla, reportes_constanciastutorados_recibir_nombrereporte, reportes_constanciastutorados_generar, convertir_tutorados_pdf,reportes_tutorados_enviar_por_whatsapp,verificar_pdfs_existentes_tutorados};
// Emparejamiento.
use emparejamiento::emparejamiento::obtener_emparejamiento;
use emparejamiento::emparejamiento::filtrar_emparejamientos;
use emparejamiento::emparejamiento::emparejamiento_automatico;
use emparejamiento::emparejamiento::actualizar_campo_tutor;


fn main() {


    tauri::Builder::default()

        .invoke_handler(tauri::generate_handler![

            /*      SERVICIOS   */
            monitoreo_enviar_tarea,
            procesar_datos_para_whatsapp,

            /*      MONITOREO    */
            leer_excel_emparejamiento,
            cargar_datos_json,
            actualizar_json_monitoreo,
            obtener_roles_unicos,
            obtener_instituciones_unicas,
            agregar_tarea_y_guardar,
            agregar_imagen_y_guardar,
            eliminar_item_monitoreo,
            editar_item_monitoreo,
            toggle_hecho_monitoreo,

            /*      NOTIFICACIONES  */
            editar_historial,
            actualizar_historial,
            eliminar_historial,
            enviar_historiales,
            procesar_mensajes_desde_json,
            exportar_mensajes_a_excel,
            // ELEMENTOS
            notificaciones_inicio_emparejamiento,
            notificaciones_inicio_control,
            notificaciones_inicio_seguimiento,
            notificaciones_inicio_links,
            leer_archivo_emparejados,
            leer_archivo_control,
            generar_tutores,
            generar_tutores_enlaces,
            init_path_pruebas,
            // HISTORIAL
            guardar_historial,
            leer_historial,

            /*      REPORTES    */
            obtener_fecha,
            // LEE
            reportes_lee_actualizarfecha,
            reportes_lee_recibir_emparejamiento,
            reportes_lee_recibir_pathcarpeta,
            reportes_lee_recibir_nombrereporte,
            reportes_lee_leer_archivos_en_carpeta,
            // PUJ
            reportes_puj_actualizarfecha,
            reportes_puj_recibir_lee,
            reportes_puj_recibir_pathplantilla,
            reportes_puj_recibir_nombrereporte,
            reportes_puj_leer_universitarios_aprobados,
            reporte_puj_generar,
            convertir_puj_pdf,
            reportes_puj_enviar_por_whatsapp,
            verificar_pdfs_existentes_puj,
            // Colegios
            reportes_colegios_actualizarfecha,
            reportes_colegios_recibir_lee,
            reportes_colegios_recibir_pathplantilla,
            reportes_colegios_recibir_nombrereporte,
            reportes_colegios_leer_estudiantes_aprobados,
            reportes_colegios_generar,
            convertir_colegios_pdf,
            reportes_colegios_enviar_por_whatsapp,
            verificar_pdfs_existentes_colegios,
            // Tutores
            reportes_constanciastutores_actualizarfecha,
            reportes_tutores_recibir_lee,
            reportes_constanciastutores_recibir_pathplantilla,
            reportes_constanciastutores_recibir_nombrereporte,
            reportes_constanciastutores_generar,
            convertir_tutores_pdf,
            reportes_tutores_enviar_por_whatsapp,
            reportes_tutores_recibir_emparejamiento,
            verificar_pdfs_existentes_tutores,
            // Tutorados
            reportes_constanciastutorados_actualizarfecha,
            reportes_tutorados_recibir_emparejamiento,
            reportes_constanciastutorados_recibir_pathplantilla,
            reportes_constanciastutorados_recibir_nombrereporte,
            reportes_constanciastutorados_generar,
            convertir_tutorados_pdf,
            reportes_tutorados_enviar_por_whatsapp,
            verificar_pdfs_existentes_tutorados,

            /*      EMPAREJAMIENTO   */
            obtener_emparejamiento,
            filtrar_emparejamientos,
            emparejamiento_automatico,
            actualizar_campo_tutor,

        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application!!");

    
}

