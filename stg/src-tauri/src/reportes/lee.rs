
// VARIOS
use serde::Serialize ;
// FECHA
use chrono::NaiveDate ;
// ARCHIVOS
use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;



#[derive(Serialize)]
pub struct Fecha {
    fecha: String,
}


#[tauri::command]
pub fn reportes_lee_actualizar_fecha(nueva_fecha: String) -> Result<(), String> {

    // Parse the input date (assuming the input format is "yyyy-mm-dd")
    let parsed_date = NaiveDate::parse_from_str(&nueva_fecha, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse date: {}", e))?;

    // Format the date as "dd-mm-yyyy"
    let formatted_date = parsed_date.format("%d-%m-%Y").to_string();

    println!("Nueva fecha: {}", formatted_date);

Ok(())
}



#[derive(Serialize, Debug)]
pub struct DatosMonitoreo {
    nombre_completo: String,
    correo: String,
    minutos: String,
}


#[tauri::command]
pub fn reportes_lee_recibir_pathcarpeta(path: String) {
    println!("📂 Ruta de la carpeta recibissda: {}", path);
}

#[tauri::command]
pub fn leer_excel_path_fijo_lee() -> Result<Vec<DatosMonitoreo>, String> {
    println!("➤ Entrando a la función `leer_excel_path_fijo_lee`");

    let path_str = "C:\\Users\\Javier\\Desktop\\Qualtrics\\Updated_Qualtrics_Seguimiento_Tutores.xlsx";
    let path = Path::new(path_str);
    println!("➤ Intentando abrir el archivo en la ruta: {}", path_str);

    if !path.exists() {
        println!("✖ ERROR: El archivo no existe en la ruta especificada.");
        return Err(format!("Archivo no encontrado: {}", path_str));
    }

    let mut workbook: Xlsx<_> = match open_workbook(path) {
        Ok(wb) => {
            println!("✔ Archivo abierto correctamente.");
            wb
        },
        Err(e) => {
            println!("✖ ERROR al abrir el archivo: {}", e);
            return Err(format!("Error al abrir el archivo: {}", e));
        }
    };

    let sheet_names = workbook.sheet_names();
    println!("➤ Hojas disponibles en el archivo: {:?}", sheet_names);

    let range = match workbook.worksheet_range("Sheet1") {
        Ok(r) => {
            println!("✔ Hoja 'Sheet1' encontrada y cargada.");
            r
        },
        Err(e) => {
            println!("✖ ERROR: No se pudo cargar la hoja 'Sheet1'. {}", e);
            return Err(format!("Error al cargar 'Sheet1': {}", e));
        }
    };

    let mut data = Vec::new();

    println!("➤ Comenzando la lectura de las filas...");

    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue; // Ignorar la primera fila (encabezados)
        }

        if i >= 6 {
            break; // Solo leer los primeros 5 registros
        }

        if row.len() < 13 { // Asegurarse de que haya suficientes columnas
            continue;
        }

        let nombre = row.get(10).map_or("".to_string(), |cell| cell.to_string());
        let apellido = row.get(9).map_or("".to_string(), |cell| cell.to_string());
        let correo = row.get(11).map_or("".to_string(), |cell| cell.to_string());
        let minutos = row.get(22).map_or("".to_string(), |cell| cell.to_string());

        let nombre_completo = format!("{} {}", nombre, apellido);

        data.push(DatosMonitoreo {
            nombre_completo,
            correo,
            minutos,
        });
    }

    for dato in &data {
        println!(
            "Nombre Completo: {}, Correo: {}, Minutos: {}",
            dato.nombre_completo, dato.correo, dato.minutos
        );
    }

    println!("✔ Finalizada la lectura del archivo y datos impresos correctamente.");
    Ok(data)
}

