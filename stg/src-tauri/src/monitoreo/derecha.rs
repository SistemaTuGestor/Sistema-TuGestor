
// ARCHIVOS
use calamine::{open_workbook, Reader, Xlsx, XlsxError} ;
use serde::Serialize ;



#[derive(Serialize)]
pub struct DatosMonitoreoDer {
    registro: String,
}

#[tauri::command]
pub fn monitoreo_derecha()  ->  Result <Vec<DatosMonitoreoDer>,String> {

    let path = "C:\\Users\\USUARIO\\Downloads\\prueba.xlsx" ;
    let mut workbook: Xlsx<_> = open_workbook(path).map_err ( |e:XlsxError| e.to_string() )? ;

    let range = workbook
        .worksheet_range ( "Hoja1" )
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

