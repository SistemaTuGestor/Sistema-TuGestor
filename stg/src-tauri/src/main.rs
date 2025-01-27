// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use calamine::{open_workbook, Reader, Xlsx, XlsxError};
use serde::Serialize;



#[derive(Serialize)]
struct DatosMonitoreoIzq {
    id : String ,
    rol : String ,
    contacto : String
}

#[tauri::command]
fn monitoreo_izquierdo()  ->  Result <Vec<DatosMonitoreoIzq>,String> {

    let path = "/home/user/Downloads/new 1.xlsx" ;
    let mut workbook: Xlsx<_> = open_workbook(path).map_err ( |e:XlsxError| e.to_string() )? ;

    let range = workbook
        .worksheet_range ( "Sheet1" )
        .map_err ( |e:XlsxError| e.to_string() )? ;

    let mut data = Vec::new() ;

    for row in range.rows ( ) {

        let id = row.get(0).map_or ( "".to_string(),|cell|cell.to_string() ) ;
        let rol = row.get(1).map_or ( "".to_string(),|cell|cell.to_string() ) ;
        let contacto = row.get(2).map_or ( "".to_string(),|cell|cell.to_string() ) ;

        data.push ( DatosMonitoreoIzq { id,rol,contacto } ) ;

    }

Ok(data)
}


#[derive(Serialize)]
struct DatosMonitoreoDer {
    registro: String,
}

#[tauri::command]
fn monitoreo_derecho()  ->  Result <Vec<DatosMonitoreoDer>,String> {

    let path = "/home/user/Downloads/new 2.xlsx" ;
    let mut workbook: Xlsx<_> = open_workbook(path).map_err ( |e:XlsxError| e.to_string() )? ;

    let range = workbook
        .worksheet_range ( "Sheet1" )
        .map_err ( |e:XlsxError| e.to_string() )? ;

    let mut data = Vec::new() ;
    
    for row in range.rows ( ) {
        if let Some(cell) = row.get(0) {
            data.push ( DatosMonitoreoDer {
                registro:cell.to_string() ,
            }) ;
        }
    }

Ok(data)
}

#[derive(Serialize)]
struct DatosNotificacionesIzq {
    asunto: String,
    contactos: String,
}

#[tauri::command]
fn notificaciones_izquierda()  ->  Result <Vec<DatosNotificacionesIzq>,String> {

    let path = "/home/user/Downloads/new 3.xlsx" ;
    let mut workbook: Xlsx<_> = open_workbook(path).map_err ( |e:XlsxError| e.to_string() )? ;

    let range = workbook
        .worksheet_range ( "Sheet1" )
        .map_err ( |e:XlsxError| e.to_string() )? ;

    let mut data = Vec::new() ;

    for row in range.rows() {

        let asunto = row.get(0).map_or("".to_string(), |cell| cell.to_string());
        let contactos = row.get(1).map_or("".to_string(), |cell| cell.to_string());

        data.push ( DatosNotificacionesIzq { asunto,contactos } ) ;

    }

Ok(data)
}


#[derive(Serialize)]
struct DatosNotificacionesDer {
    registro: String,
}

#[tauri::command]
fn notificaciones_derecha()  ->  Result <Vec<DatosNotificacionesDer>,String> {

    let path = "/home/user/Downloads/new 4.xlsx" ;
    let mut workbook: Xlsx<_> = open_workbook(path).map_err ( |e:XlsxError| e.to_string() )? ;

    let range = workbook
        .worksheet_range ( "Sheet1" )
        .map_err ( |e:XlsxError| e.to_string() )? ;

    let mut data = Vec::new() ;
    
    for row in range.rows ( ) {
        if let Some(cell) = row.get(0) {
            data.push ( DatosNotificacionesDer {
                registro:cell.to_string() ,
            }) ;
        }
    }

Ok(data)
}


fn main ( ) {
    tauri::Builder::default ( )
        .invoke_handler ( tauri::generate_handler! [
            monitoreo_izquierdo ,
            monitoreo_derecho ,
            notificaciones_izquierda ,
            notificaciones_derecha
        ])
        .run ( tauri::generate_context!() )
        .expect ( "error while running tauri application" ) ;
}

