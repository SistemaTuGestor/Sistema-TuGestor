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
    telefono: Vec<String>,
    institucion: String,
    
}

#[derive(Serialize, Debug)]
pub struct TutoradosEmparejados {
    nombre: String,
    correo: String,
    telefono: Vec<String>,
    id: String,
    colegio: String,
    vocabulario: String,
    gramatica: String,
    escucha: String,
    lectura: String,
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
    g: String,
}

#[derive(Serialize, Debug)]
pub struct TutoradosControl {
    nombre: String,
    correo: String,
    telefono: Vec<String>,
    id: String,
    colegio: String,
    vocabulario: String,
    gramatica: String,
    escucha: String,
    lectura: String,
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
    g: String,
}

#[tauri::command]
pub fn leer_archivo_emparejados() -> Result<(Vec<TutoresPUJ>, Vec<TutoresColegio>, Vec<FuncionariosColegio>, Vec<TutoradosEmparejados>), String> {
    //println!("Iniciando la función leer_archivo_emparejados...");

    // let archivo_path = PATH_ARCHIVO.get().expect("Global variable not initialized");
    // let archivo_path_guard = archivo_path.lock().unwrap();
    // let path = archivo_path_guard.as_str();
    // let path = "/home/user/Downloads/ejemplo.xlsx"; // Hardcoded path for now
    let path = "C:\\Users\\USUARIO\\Downloads\\ejemplo.xlsx"; // Hardcoded path for now
    //println!("Ruta del archivo: {}", path);

    // Intentar abrir el archivo
    let mut workbook: Xlsx<_> = match open_workbook(path) {
        Ok(wb) => {
           // println!("Archivo abierto correctamente.");
            wb
        }
        Err(e) => {
           // println!("Error al abrir el archivo: {}", e);
            return Err(format!("Error al abrir el archivo: {}", e));
        }
    };

    // Intentar acceder a la hoja "Emparejamiento"
    let range = match workbook.worksheet_range("Emparejamiento") {
        Ok(r) => {
            //println!("Hoja de cálculo 'Emparejamiento' cargada correctamente.");
            r
        }
        Err(e) => {
          //  println!("No se pudo cargar la hoja 'Emparejamiento': {}", e);
            return Err(format!("No se pudo cargar la hoja 'Emparejamiento': {}", e));
        }
    };

    let mut tutores_puj: Vec<TutoresPUJ> = Vec::new();
    let mut tutores_colegio: Vec<TutoresColegio> = Vec::new();
    let mut funcionarios_colegio: Vec<FuncionariosColegio> = Vec::new();
    let mut tutorados_emparejados: Vec<TutoradosEmparejados> = Vec::new();

   // println!("Comenzando a leer las filas de la hoja de cálculo...");
    let mut fila_actual = 1; // Contador de filas para debug

    for row in range.rows().skip(1) { // Omitir encabezados
        //println!("Leyendo fila {}", fila_actual);
        fila_actual += 1;

        if row.len() < 9 {
           // println!("Fila con menos de 9 columnas, se omite.");
            continue;
        }

        let nombre = row.get(0).map_or("".to_string(), |cell| cell.to_string());
        let apellido = row.get(1).map_or("".to_string(), |cell| cell.to_string());
        let correo = row.get(2).map_or("".to_string(), |cell| cell.to_string());
        let telefono = row.get(3).map_or("".to_string(), |cell| cell.to_string());
        let institucion = row.get(4).map_or("".to_string(), |cell| cell.to_string());
        let horas = row.get(8).map_or("".to_string(), |cell| cell.to_string());
        let tutoradonombre = row.get(9).map_or("".to_string(), |cell| cell.to_string());
        let tutorado2 = row.get(27).map_or("".to_string(), |cell| cell.to_string());

        if institucion.is_empty() {
            //println!("Institución vacía, se omite la fila.");
            continue;
        }

       // println!(
          //  "Nombre: {}, Apellido: {}, Correo: {}, Teléfono: {}, Institución: {}, Horas: {}",
           // nombre, apellido, correo, telefono, institucion, horas
       // );

        if institucion == "Pontificia Universidad Javeriana" {
            //println!("Agregando a tutores PUJ");
            let mut tutor = TutoresPUJ { 
                nombre: nombre.clone(), 
                apellido: apellido.clone(),
                correo: correo.clone(),
                institucion: institucion.clone(),
                telefono: vec![telefono.clone()],
                horas: horas.clone(),
                tutorados: vec![],
            };
            if !tutoradonombre.is_empty() {
                tutor.tutorados.push(tutoradonombre.clone());
            }
            if !tutorado2.is_empty() {
                tutor.tutorados.push(tutorado2.clone());
            }
            tutores_puj.push(tutor);
        } else {
            //println!("Agregando a tutores Colegio");
            let mut tutor = TutoresColegio { 
                nombre: nombre.clone(), 
                apellido: apellido.clone(),
                correo: correo.clone(),
                institucion: institucion.clone(),
                telefono: vec![telefono.clone()],
                horas: horas.clone(),
                tutorados: vec![],
            };
            if !tutoradonombre.is_empty() {
                tutor.tutorados.push(tutoradonombre.clone());
            }
            if !tutorado2.is_empty() {
                tutor.tutorados.push(tutorado2.clone());
            }
            tutores_colegio.push(tutor);
        }

        if !tutoradonombre.is_empty() {
            tutorados_emparejados.push(TutoradosEmparejados {
                nombre: tutoradonombre.clone(),
                correo: row.get(14).map_or("".to_string(), |cell| cell.to_string()),
                telefono: vec![
                    row.get(12).map_or("".to_string(), |cell| cell.to_string()),
                    row.get(13).map_or("".to_string(), |cell| cell.to_string()),
                ],
                id: row.get(10).map_or("".to_string(), |cell| cell.to_string()),
                colegio: row.get(11).map_or("".to_string(), |cell| cell.to_string()),
                vocabulario: row.get(16).map_or("".to_string(), |cell| cell.to_string()),
                gramatica: row.get(17).map_or("".to_string(), |cell| cell.to_string()),
                escucha: row.get(18).map_or("".to_string(), |cell| cell.to_string()),
                lectura: row.get(19).map_or("".to_string(), |cell| cell.to_string()),
                a: row.get(20).map_or("".to_string(), |cell| cell.to_string()),
                b: row.get(21).map_or("".to_string(), |cell| cell.to_string()),
                c: row.get(22).map_or("".to_string(), |cell| cell.to_string()),
                d: row.get(23).map_or("".to_string(), |cell| cell.to_string()),
                e: row.get(24).map_or("".to_string(), |cell| cell.to_string()),
                f: row.get(25).map_or("".to_string(), |cell| cell.to_string()),
                g: row.get(26).map_or("".to_string(), |cell| cell.to_string()),
            });
        }

        if !tutorado2.is_empty() {
            tutorados_emparejados.push(TutoradosEmparejados {
                nombre: tutorado2.clone(),
                correo: row.get(32).map_or("".to_string(), |cell| cell.to_string()),
                telefono: vec![
                    row.get(30).map_or("".to_string(), |cell| cell.to_string()),
                    row.get(31).map_or("".to_string(), |cell| cell.to_string()),
                ],
                id: row.get(28).map_or("".to_string(), |cell| cell.to_string()),
                colegio: row.get(29).map_or("".to_string(), |cell| cell.to_string()),
                vocabulario: row.get(34).map_or("".to_string(), |cell| cell.to_string()),
                gramatica: row.get(35).map_or("".to_string(), |cell| cell.to_string()),
                escucha: row.get(36).map_or("".to_string(), |cell| cell.to_string()),
                lectura: row.get(37).map_or("".to_string(), |cell| cell.to_string()),
                a: row.get(38).map_or("".to_string(), |cell| cell.to_string()),
                b: row.get(39).map_or("".to_string(), |cell| cell.to_string()),
                c: row.get(40).map_or("".to_string(), |cell| cell.to_string()),
                d: row.get(41).map_or("".to_string(), |cell| cell.to_string()),
                e: row.get(42).map_or("".to_string(), |cell| cell.to_string()),
                f: row.get(43).map_or("".to_string(), |cell| cell.to_string()),
                g: row.get(44).map_or("".to_string(), |cell| cell.to_string()),
            });
        }
    }

   // println!("Lectura finalizada.");
    println!("Total Tutores PUJ: {}", tutores_puj.len());
    println!("Total Tutores Colegio: {}", tutores_colegio.len());
    //println!("Total Tutorados Emparejados: {}", tutorados_emparejados.len());

    //println!("Tutores PUJ: {:?}", tutores_puj);
    //println!("Tutores Colegio: {:?}", tutores_colegio);
    //println!("Tutorados Emparejados: {:?}", tutorados_emparejados);

    Ok((tutores_puj, tutores_colegio, funcionarios_colegio, tutorados_emparejados))
}

#[tauri::command]
pub fn leer_archivo_control() -> Result<Vec<TutoradosControl>, String> {
    println!("Iniciando la función leer_archivo_control...");

    // let archivo_path = PATH_ARCHIVO.get().expect("Global variable not initialized");
    // let archivo_path_guard = archivo_path.lock().unwrap();
    // let path = archivo_path_guard.as_str();
    let path = "C:\\Users\\USUARIO\\Downloads\\EjemploControl.xlsx"; // Hardcoded path for now
   // println!("Ruta del archivo: {}", path);

    // Intentar abrir el archivo
    let mut workbook: Xlsx<_> = match open_workbook(path) {
        Ok(wb) => {
            //println!("Archivo abierto correctamente.");
            wb
        }
        Err(e) => {
            //println!("Error al abrir el archivo: {}", e);
            return Err(format!("Error al abrir el archivo: {}", e));
        }
    };

    // Intentar acceder a la hoja "Control"
    let range = match workbook.worksheet_range("Inscritos en lista de espera") {
        Ok(r) => {
           // println!("Hoja de cálculo 'Inscritos en lista de espera' cargada correctamente.");
            r
        }
        Err(e) => {
           // println!("No se pudo cargar la hoja 'Inscritos en lista de espera': {}", e);
            return Err(format!("No se pudo cargar la hoja 'Inscritos en lista de espera': {}", e));
        }
    };

    let mut tutorados_control: Vec<TutoradosControl> = Vec::new();

    //println!("Comenzando a leer las filas de la hoja de cálculo...");
    let mut fila_actual = 1; // Contador de filas para debug

    for row in range.rows().skip(1) { // Omitir encabezados
       // println!("Leyendo fila {}", fila_actual);
        fila_actual += 1;

        if row.len() < 17 {
            //println!("Fila con menos de 17 columnas, se omite.");
            continue;
        }

        let nombre = row.get(0).map_or("".to_string(), |cell| cell.to_string());
        let id = row.get(1).map_or("".to_string(), |cell| cell.to_string());
        let institucion = row.get(2).map_or("".to_string(), |cell| cell.to_string());
        let telefono = vec![
            row.get(3).map_or("".to_string(), |cell| cell.to_string()),
            row.get(4).map_or("".to_string(), |cell| cell.to_string()),
        ];
        let correo = row.get(5).map_or("".to_string(), |cell| cell.to_string());
        let vocabulario = row.get(6).map_or("".to_string(), |cell| cell.to_string());
        let gramatica = row.get(7).map_or("".to_string(), |cell| cell.to_string());
        let escucha = row.get(8).map_or("".to_string(), |cell| cell.to_string());
        let lectura = row.get(9).map_or("".to_string(), |cell| cell.to_string());
        let a = row.get(10).map_or("".to_string(), |cell| cell.to_string());
        let b = row.get(11).map_or("".to_string(), |cell| cell.to_string());
        let c = row.get(12).map_or("".to_string(), |cell| cell.to_string());
        let d = row.get(13).map_or("".to_string(), |cell| cell.to_string());
        let e = row.get(14).map_or("".to_string(), |cell| cell.to_string());
        let f = row.get(15).map_or("".to_string(), |cell| cell.to_string());
        let g = row.get(16).map_or("".to_string(), |cell| cell.to_string());

        if nombre.is_empty() {
           // println!("Nombre vacío, se omite la fila.");
            continue;
        }

        tutorados_control.push(TutoradosControl {
            nombre,
            correo,
            telefono,
            id,
            colegio: institucion,
            vocabulario,
            gramatica,
            escucha,
            lectura,
            a,
            b,
            c,
            d,
            e,
            f,
            g,
        });
    }

    //println!("Lectura finalizada.");
    println!("Total Tutorados Control: {}", tutorados_control.len());
    //println!("Tutorados Control: {:?}", tutorados_control);

    Ok(tutorados_control)
}




