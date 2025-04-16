
// ARCHIVOS
use calamine::{open_workbook, Reader, Xlsx, XlsxError} ;
use serde::Serialize ;



#[derive(Serialize)]
pub struct DatosMonitoreoIzq {
    id : String ,
    rol : String ,
    teleefono : String ,
    email : String
}

#[tauri::command]
pub fn monitoreo_izquierda()  ->  Result <Vec<DatosMonitoreoIzq>,String> {

    let path = "/home/user/Downloads/new 1.xlsx" ;
    let mut workbook: Xlsx<_> = open_workbook(path).map_err ( |e:XlsxError| e.to_string() )? ;

    let range = workbook
        .worksheet_range ( "Sheet1" )
        .map_err ( |e:XlsxError| e.to_string() )? ;

    let mut data = Vec::new() ;

    for row in range.rows ( ) {

        let id = row.get(0).map_or ( "".to_string(),|cell|cell.to_string() ) ;
        let rol = row.get(1).map_or ( "".to_string(),|cell|cell.to_string() ) ;
        let teleefono = row.get(2).map_or ( "".to_string(),|cell|cell.to_string() ) ;
        let email = row.get(3).map_or ( "".to_string(),|cell|cell.to_string() ) ;

        data.push ( DatosMonitoreoIzq { id,rol,teleefono,email } ) ;

    }

Ok(data)
}

