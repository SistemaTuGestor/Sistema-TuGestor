
// ARCHIVOS
use calamine::{open_workbook, Reader, Xlsx, XlsxError} ;
use serde::Serialize ;



#[derive(Serialize)]
pub struct DatosNotificacionesDer {
    registro: String,
}

#[tauri::command]
pub fn notificaciones_derecha()  ->  Result <Vec<DatosNotificacionesDer>,String> {

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

