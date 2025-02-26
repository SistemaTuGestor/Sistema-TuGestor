use serde::Serialize;
use std::fs;
use calamine::{open_workbook, Reader, Xlsx};
use std::sync::Mutex;
use once_cell::sync::OnceCell;

static PATH_ARCHIVO: OnceCell<Mutex<String>> = OnceCell::new();

#[derive(Serialize, Debug)]
pub struct TutoresPUJ {
    nombre: String,
    apellido: String,
    correo: String,
    institucion: String,
    telefono: Vec<String>,
    horas: String,
    tutorados: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct TutoresColegio {
    nombre: String,
    apellido: String,
    correo: String,
    institucion: String,
    telefono: Vec<String>,
    horas: String,
    tutorados: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct FuncionariosColegio {
    nombre: String,
    correo: String,
}

#[derive(Serialize, Debug)]
pub struct TutoradosEmparejados {
    nombre: String,
    correo: String,
}

#[tauri::command]
pub fn leer_archivo_emparejados() -> Result<(Vec<TutoresPUJ>, Vec<TutoresColegio>, Vec<FuncionariosColegio>, Vec<TutoradosEmparejados>), String> {
    println!("Iniciando la función leer_archivo_emparejados...");

    // let archivo_path = PATH_ARCHIVO.get().expect("Global variable not initialized");
    // let archivo_path_guard = archivo_path.lock().unwrap();
    // let path = archivo_path_guard.as_str();
    let path = "C:\\Users\\USUARIO\\Downloads\\ejemplo.xlsx"; // Hardcoded path for now
    println!("Ruta del archivo: {}", path);

    // Intentar abrir el archivo
    let mut workbook: Xlsx<_> = match open_workbook(path) {
        Ok(wb) => {
            println!("Archivo abierto correctamente.");
            wb
        }
        Err(e) => {
            println!("Error al abrir el archivo: {}", e);
            return Err(format!("Error al abrir el archivo: {}", e));
        }
    };

    // Intentar acceder a la hoja "Sheet1"
    let range = match workbook.worksheet_range("Emparejamiento") {
        Ok(r) => {
            println!("Hoja de cálculo 'Emparejamiento' cargada correctamente.");
            r
        }
        Err(e) => {
            println!("No se pudo cargar la hoja 'Emparejamiento': {}", e);
            return Err(format!("No se pudo cargar la hoja 'Emparejamiento': {}", e));
        }
    };

    let mut tutores_puj: Vec<TutoresPUJ> = Vec::new();
    let mut tutores_colegio: Vec<TutoresColegio> = Vec::new();
    let mut funcionarios_colegio: Vec<FuncionariosColegio> = Vec::new();
    let mut tutorados_emparejados: Vec<TutoradosEmparejados> = Vec::new();

    println!("Comenzando a leer las filas de la hoja de cálculo...");
    let mut fila_actual = 1; // Contador de filas para debug

    for row in range.rows().skip(1) { // Omitir encabezados
        println!("Leyendo fila {}", fila_actual);
        fila_actual += 1;

        if row.len() < 9 {
            println!("Fila con menos de 9 columnas, se omite.");
            continue;
        }

        let nombre = row.get(0).map_or("".to_string(), |cell| cell.to_string());
        let apellido = row.get(1).map_or("".to_string(), |cell| cell.to_string());
        let correo = row.get(2).map_or("".to_string(), |cell| cell.to_string());
        let telefono = row.get(3).map_or("".to_string(), |cell| cell.to_string());
        let institucion = row.get(4).map_or("".to_string(), |cell| cell.to_string());
        let horas = row.get(8).map_or("".to_string(), |cell| cell.to_string());

        if institucion.is_empty() {
            println!("Institución vacía, se omite la fila.");
            continue;
        }

        println!(
            "Nombre: {}, Apellido: {}, Correo: {}, Teléfono: {}, Institución: {}, Horas: {}",
            nombre, apellido, correo, telefono, institucion, horas
        );

        if institucion == "Pontificia Universidad Javeriana" {
            println!("Agregando a tutores PUJ");
            tutores_puj.push(TutoresPUJ { 
                nombre: nombre.clone(), 
                apellido: apellido.clone(),
                correo: correo.clone(),
                institucion: institucion.clone(),
                telefono: vec![telefono.clone()],
                horas: horas.clone(),
                tutorados: vec![],
            });
        } else {
            println!("Agregando a tutores Colegio");
            tutores_colegio.push(TutoresColegio { 
                nombre: nombre.clone(), 
                apellido: apellido.clone(),
                correo: correo.clone(),
                institucion: institucion.clone(),
                telefono: vec![telefono.clone()],
                horas: horas.clone(),
                tutorados: vec![],
            });
        }
    }

    println!("Lectura finalizada.");
    println!("Total Tutores PUJ: {}", tutores_puj.len());
    println!("Total Tutores Colegio: {}", tutores_colegio.len());

    println!("Tutores PUJ: {:?}", tutores_puj);
    println!("Tutores Colegio: {:?}", tutores_colegio);

    Ok((tutores_puj, tutores_colegio, funcionarios_colegio, tutorados_emparejados))
}
