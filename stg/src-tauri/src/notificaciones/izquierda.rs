
// ARCHIVOS
use calamine::{open_workbook, Reader, Xlsx, XlsxError} ;
use serde::Serialize ;



#[derive(Serialize)]
pub struct DatosNotificacionesIzq {
    asunto: String,
    contactos: String,
}

#[tauri::command]
pub fn notificaciones_izquierda()  ->  Result <Vec<DatosNotificacionesIzq>,String> {

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

