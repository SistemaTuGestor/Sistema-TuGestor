
use calamine::{open_workbook, Reader, Xlsx};
use xlsxwriter::*;

const ARCHIVO_ENTRADA: &str = "C:\\Users\\USUARIO\\Downloads\\metrics_sponsor.xlsx";
const ARCHIVO_SALIDA: &str = "C:\\Users\\USUARIO\\Downloads\\Reporte_Sponsor.xlsx";

// 📥 Leer datos del archivo de métricas
pub fn leer_métricas() -> Result<Vec<Vec<String>>, String> {
    let mut workbook: Xlsx<_> = open_workbook(ARCHIVO_ENTRADA)
        .map_err(|e| format!("Error al abrir el archivo: {}", e))?;

    let range = workbook
        .worksheet_range("Métricas")
        .map_err(|e| format!("Error al cargar 'Métricas': {}", e))?;

    let mut datos: Vec<Vec<String>> = Vec::new();

    for row in range.rows() {
        let fila: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
        datos.push(fila);
    }

    println!("✔ {} filas de métricas leídas correctamente.", datos.len());
    Ok(datos)
}

// 📊 Calcular métricas actualizadas
pub fn calcular_métricas(datos: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut datos_actualizados = datos.to_vec();

    for fila in &mut datos_actualizados {
        if let Some(valor) = fila.get(1) {
            if let Ok(numero) = valor.parse::<f64>() {
                let nuevo_valor = numero * 1.1; // Simulación de cálculo (ejemplo: aumento del 10%)
                fila.push(format!("{:.2}", nuevo_valor));
            }
        }
    }

    datos_actualizados
}

// 📤 Generar nuevo reporte en Excel
pub fn generar_reporte(datos: &[Vec<String>]) {

    let  workbook = Workbook::new(ARCHIVO_SALIDA).unwrap();
    let mut sheet = workbook.add_worksheet(Some("Métricas_Actualizadas")).unwrap();

    for (i, fila) in datos.iter().enumerate() {
        for (j, valor) in fila.iter().enumerate() {
            sheet.write_string(i as u32, j as u16, valor, None).unwrap();
        }
    }

    workbook.close().expect("Error al cerrar el archivo Excel");
    println!("✔ Reporte Sponsor generado correctamente en {}", ARCHIVO_SALIDA);
}

// 📌 Ejecutar el proceso
fn main() {
    match leer_métricas() {
        Ok(datos) => {
            let datos_actualizados = calcular_métricas(&datos);
            generar_reporte(&datos_actualizados);
        }
        Err(e) => println!("✖ ERROR al procesar métricas: {}", e),
    }
}

