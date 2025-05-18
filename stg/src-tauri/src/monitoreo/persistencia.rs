use calamine::{open_workbook, Reader, Xlsx, XlsxError} ;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
use std::path::PathBuf;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tarea {
    nombre: String,
    descripcion: String,
    hecho: bool, // Nuevo campo
}

impl Tarea {
    pub fn new(nombre: &str, descripcion: &str) -> Self {
        Tarea {
            nombre: nombre.to_string(),
            descripcion: descripcion.to_string(),
            hecho: false, // Por defecto, la tarea no está hecha
        }
    }
}

impl std::fmt::Display for Tarea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.nombre, self.descripcion)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Imagen {
    url: String,
}

impl Imagen {
    pub fn new(url: &str) -> Self {
        Imagen {
            url: url.to_string(),
        }
    }
}

impl std::fmt::Display for Imagen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tutor {
    id : String,
    nombre: String,
    apellido: String,
    rol: String,
    telefono: String,
    correo: String,
    institucion: String,
    tareas: Vec<Tarea>,
    imagenes: Vec<Imagen>,
    progreso: f32, // Nuevo campo
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tutorado {
    id : String,
    nombre: String,
    cedula: String,
    rol: String,
    telefono: Vec<String>,
    correo: String,
    institucion: String, 
    tareas: Vec<Tarea>,
    imagenes: Vec<Imagen>,
    progreso: f32, // Nuevo campo
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MonitoreoData {
    tutores: Vec<Tutor>,
    tutorado1: Vec<Tutorado>,
    tutorado2: Vec<Tutorado>,
}

//Conseguir ruta auxiliar para los archivos de recursos
fn get_resource_path() -> PathBuf {
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let mut path = current_exe.parent().unwrap().to_path_buf();
    
    // Navegar hacia arriba hasta encontrar la carpeta "Sistema-TuGestor"
    while !path.ends_with("Sistema-TuGestor") && path.parent().is_some() {
        path = path.parent().unwrap().to_path_buf();
    }
    
    // Añadir la carpeta recursos
    path.push("recursos");
    path
}

#[tauri::command]
pub fn leer_excel_emparejamiento() -> Result<(Vec<Tutor>, Vec<Tutorado>, Vec<Tutorado>), String>{

    let base_path = get_resource_path();

    let json_path = base_path.join("monitoreo").join("monitoreo.json");
    let excel_path = base_path.join("EmparejamientoFINAL.xlsx");
/*
    if json_path.exists() {
        println!("El archivo JSON ya existe, no es necesario regenerarlo.");
        return Err("Ya existe el archivo JSON".to_string());
    }
*/

    //let ubicacion = "C:\\Users\\Javier\\Desktop\\Proyecto TuGestor\\Sistema-TuGestor\\recursos\\EmparejamientoFINAL.xlsx";
    let mut workbook: Xlsx<_> = match open_workbook(&excel_path) {
        Ok(wb) => wb,
        Err(e) => return Err(format!("Error al abrir el archivo: {}", e))
    };

    let range = match workbook.worksheet_range("Emparejamiento") {
        Ok(r) => {
            r
        }
        Err(e) => {
            return Err(format!("No se pudo cargar la hoja 'Emparejamiento': {}", e));
        }
    };

    let mut tutores: Vec<Tutor> = Vec::new();
    let mut tutorados1: Vec<Tutorado> = Vec::new();
    let mut tutorados2: Vec<Tutorado> = Vec::new();
    let mut contador_tutores = 0;
    let mut contador_tutorados = 0;
    let mut _fila_actual = 1;

    for row in range.rows().skip(1) {
        _fila_actual += 1;

       let nombretutor = row.get(0).map_or("".to_string(), |cell| cell.to_string());
        let apellidotutor = row.get(1).map_or("".to_string(), |cell| cell.to_string());
        let correo = row.get(2).map_or("".to_string(), |cell| cell.to_string());
        let telefono = row.get(3).map_or("".to_string(), |cell| cell.to_string());
        let institucion = row.get(4).map_or("".to_string(), |cell| cell.to_string());
        let nombretutorados1 = row.get(10).map_or("".to_string(), |cell| cell.to_string());
        let cedulatutorados1 = row.get(11).map_or("".to_string(), |cell| cell.to_string());
        let instituciontut1 = row.get(12).map_or("".to_string(), |cell| cell.to_string());
        let telefonotut1 = row.get(13).map_or("".to_string(), |cell| cell.to_string());
        let telefono2tut1 = row.get(14).map_or("".to_string(), |cell| cell.to_string());
        let correotut1 = row.get(15).map_or("".to_string(), |cell| cell.to_string());
        let nombretutorados2 = row.get(30).map_or("".to_string(), |cell| cell.to_string());
        let cedulatutorados2 = row.get(31).map_or("".to_string(), |cell| cell.to_string());
        let instituciontut2 = row.get(32).map_or("".to_string(), |cell| cell.to_string());
        let telefonotut2 = row.get(33).map_or("".to_string(), |cell| cell.to_string());
        let telefono2tut2 = row.get(34).map_or("".to_string(), |cell| cell.to_string());
        let correotut2 = row.get(35).map_or("".to_string(), |cell| cell.to_string());

        //Toda esta parte es para probar que se creen los usuarios con tareas y mostrarlas correctamente.
        let mut tarea = Tarea{
          nombre:format!("tarea{}", _fila_actual), 
          descripcion: "Tarea importante !!!".to_string(), 
          hecho: false, // Por defecto, la tarea no está hecha
        };

        let mut tarea2 = Tarea{
            nombre:"Otra tarea mas".to_string(), 
            descripcion: "Tarea igual de importante que la otra :3!!!".to_string(), 
            hecho: false, // Por defecto, la tarea no está hecha
        };

        let mut imagen = Imagen{
            url: "C:\\Users\\Javier\\Desktop\\Proyecto Tututor\\Sistema-TuGestor\\recursos\\imagenes\\prueba.jpg".to_string(),
        };

        let mut imagen2 = Imagen{
            url: "C:\\Users\\Javier\\Diferente".to_string(),
        };

        let mut lista_tareas = Vec::new();
        lista_tareas.push(tarea);
        lista_tareas.push(tarea2);
        
        let mut lista_imagenes = Vec::new();
        lista_imagenes.push(imagen);
        lista_imagenes.push(imagen2);

        let mut lista_tutoradoNumeros = Vec::new();
        lista_tutoradoNumeros.push(telefonotut1);
        lista_tutoradoNumeros.push(telefono2tut1);

        let mut lista_tutoradoNumeros2 = Vec::new();
        lista_tutoradoNumeros2.push(telefonotut2);
        lista_tutoradoNumeros2.push(telefono2tut2);
    
        contador_tutores += 1;
        let mut tutor = Tutor{
            id: contador_tutores.to_string(),
            nombre: nombretutor.clone(),
            apellido: apellidotutor.clone(),
            rol: "Tutor".to_string(),
            correo: correo.clone(),
            telefono: telefono.clone(),
            institucion: institucion.clone(),
            tareas: lista_tareas.clone(),
            imagenes: lista_imagenes.clone(),
            progreso: 0.0, // Por defecto, el progreso es 0
        };
        contador_tutorados += 1;
        let mut tutorado1 = Tutorado{
            id: contador_tutorados.to_string(),
            nombre: nombretutorados1.clone(),
            cedula: cedulatutorados1.clone(),
            rol: "Tutorado".to_string(),
            institucion: instituciontut1.clone(),
            telefono: lista_tutoradoNumeros,
            correo: correo.clone(),
            tareas: lista_tareas.clone(),
            imagenes: lista_imagenes.clone(),
            progreso: 0.0, // Por defecto, el progreso es 0
        };
        contador_tutorados += 1;
        let mut tutorado2 = Tutorado{
            id: contador_tutorados.to_string(),
            nombre: nombretutorados2.clone(),
            cedula: cedulatutorados2.clone(),
            rol: "Tutorado".to_string(),
            institucion: instituciontut2.clone(),
            telefono: lista_tutoradoNumeros2,
            correo: correotut2.clone(),
            tareas: lista_tareas.clone(),
            imagenes: lista_imagenes.clone(),
            progreso: 0.0, // Por defecto, el progreso es 0
        };

        tutores.push(tutor);
        tutorados1.push(tutorado1);
        tutorados2.push(tutorado2);

    }
    
    println!("Se generaron {} tutores:", tutores.len());
    for (i, tutores) in tutores.iter().enumerate() {
        println!("Tutor #{}: {} {}", i+1, tutores.nombre, tutores.apellido);
        println!("Rol: {}", tutores.rol);
        println!("  Correo: {}", tutores.correo);
        println!("  Teléfono: {}", tutores.telefono);
        println!("  Institución: {}", tutores.institucion);
        println!("  Tareas: {}", tutores.tareas.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "));
        println!("  imagen: {}", tutores.imagenes.iter().map(|i| i.url.clone()).collect::<Vec<_>>().join(", "));
        println!("-----------------------------------");
    }

    println!("Se generaron {} tutorados1:", tutorados1.len());
    for (i, tutorados1) in tutorados1.iter().enumerate() {
        println!("Tutor #{}: {}", i+1, tutorados1.nombre);
        println!("Cedula #{}:", tutorados1.cedula);
        println!("Rol: {}", tutorados1.rol);
        println!("  Institución: {}", tutorados1.institucion);
        println!("  Teléfonos: {}", tutorados1.telefono.join(", "));
        println!("  Correo: {}", tutorados1.correo);
        println!("  Tareas: {}", tutorados1.tareas.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "));
        println!("  imagen: {}", tutorados1.imagenes.iter().map(|i| i.url.clone()).collect::<Vec<_>>().join(", "));
        println!("-----------------------------------");
    }

    println!("Se generaron {} tutorados2:", tutorados2.len());
    for (i, tutorados2) in tutorados2.iter().enumerate() {
        println!("Tutor #{}: {}", i+1, tutorados2.nombre);
        println!("Cedula #{}:", tutorados2.cedula);
        println!("Rol: {}", tutorados2.rol);
        println!("  Institución: {}", tutorados2.institucion);
        println!("  Teléfonos: {}", tutorados2.telefono.join(", "));
        println!("  Correo: {}", tutorados2.correo);
        println!("  Tareas: {}", tutorados2.tareas.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "));
        println!("  imagen: {}", tutorados2.imagenes.iter().map(|i| i.url.clone()).collect::<Vec<_>>().join(", "));
        println!("-----------------------------------");
    }

    
    // Antes de guardar, actualiza tareas y progreso
    actualizar_tareas_y_progreso(&mut tutores, &mut tutorados1, &mut tutorados2);

    if let Err(e) = guardar_monitoreo_json(tutores.clone(), tutorados1.clone(), tutorados2.clone()) {
        println!("Error al guardar monitoreo: {}", e);
    } else {
        println!("Monitoreo guardado exitosamente en JSON.");
    }

    Ok((tutores, tutorados1, tutorados2))

}


fn guardar_monitoreo_json(
    tutores: Vec<Tutor>,
    tutorado1: Vec<Tutorado>,
    tutorado2: Vec<Tutorado>,
) -> Result<(), String> {
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    let data = MonitoreoData {
        tutores,
        tutorado1,
        tutorado2,
    };

    let json_string = match serde_json::to_string_pretty(&data) {
        Ok(json) => json,
        Err(e) => return Err(format!("Error serializando JSON: {}", e)),
    };

    //let path = "C:\\Users\\Javier\\Desktop\\Proyecto TuGestor\\Sistema-TuGestor\\recursos\\monitoreo\\monitoreo.json";
    if let Some(parent) = json_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return Err(format!("Error creando directorios: {}", e));
        }
    }

    match File::create(json_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(json_string.as_bytes()) {
                return Err(format!("Error escribiendo el archivo JSON: {}", e));
            }
        }
        Err(e) => return Err(format!("No se pudo crear el archivo JSON: {}", e)),
    }

    Ok(())
}


#[tauri::command]
pub fn cargar_datos_json() -> Result<String, String> {
    //let ruta = "C:\\Users\\Javier\\Desktop\\Proyecto TuGestor\\Sistema-TuGestor\\recursos\\monitoreo\\monitoreo.json";
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    std::fs::read_to_string(json_path).map_err(|e| format!("No se pudo leer el JSON: {}", e))
}

#[tauri::command] //Función para eliminación
pub fn actualizar_json_monitoreo(json_data: String) -> Result<String, String> {
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    // Validar y parsear el JSON recibido
    let mut data: MonitoreoData = serde_json::from_str(&json_data)
        .map_err(|e| format!("JSON inválido: {}", e))?;

    // Asegurar que todas las tareas tengan el campo 'hecho'
    for tutor in data.tutores.iter_mut() {
        for tarea in tutor.tareas.iter_mut() {
            // Si por alguna razón falta el campo, lo ponemos en false
            // (esto solo es relevante si el JSON fue manipulado mal)
            // En Rust, esto no suele pasar, pero es seguro.
            if tarea.hecho != true && tarea.hecho != false {
                tarea.hecho = false;
            }
        }
    }
    // Este bloque asegura que todas las tareas de tutorado1 y tutorado2 tengan el campo 'hecho'.
// Normalmente no es necesario si siempre serializas/deserializas bien, pero es una protección extra.
/*

    */
    for tutorado in data.tutorado1.iter_mut().chain(data.tutorado2.iter_mut()) {
        for tarea in tutorado.tareas.iter_mut() {
            if tarea.hecho != true && tarea.hecho != false {
                tarea.hecho = false;
            }
        }
    }

    // Actualizar progreso de todos los usuarios
    actualizar_tareas_y_progreso(&mut data.tutores, &mut data.tutorado1, &mut data.tutorado2);

    // Serializar y guardar el JSON actualizado
    let json_string = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Error serializando JSON: {}", e))?;

    if let Some(parent) = json_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Error creando directorios: {}", e))?;
    }

    std::fs::write(&json_path, json_string)
        .map_err(|e| format!("Error al escribir el JSON: {}", e))?;

    Ok("JSON actualizado correctamente".to_string())
}

#[tauri::command]
pub fn guardar_datos_json(datos: String) -> Result<String, String> {
    //let ruta = "C:\\Users\\Javier\\Desktop\\Proyecto TuGestor\\Sistema-TuGestor\\recursos\\monitoreo\\monitoreo.json";
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    // Verificar que los datos sean un JSON válido antes de escribir
    match serde_json::from_str::<serde_json::Value>(&datos) {
        Ok(_) => {
            // JSON válido, proceder a escribir
            match std::fs::write(&json_path, datos) {
                Ok(_) => Ok("Datos guardados correctamente".to_string()),
                Err(e) => Err(format!("Error al escribir el archivo JSON: {}", e))
            }
        },
        Err(e) => Err(format!("JSON inválido: {}", e))
    }
}

#[tauri::command]
pub fn get_image(path: String) -> Result<Vec<u8>, String> {
    match std::fs::read(&path) {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Error al leer la imagen: {}", e)),
    }
}

// Función para actualizar tareas y progreso
/// Esta función actualiza el campo 'progreso' de todos los tutores y tutorados.
/// Además, (por ahora) asigna aleatoriamente el campo 'hecho' de cada tarea a true o false.
/// Si quieres que todas las tareas estén sin hacer, cambia `tarea.hecho = rng.gen_bool(0.5)` por `tarea.hecho = false`
pub fn actualizar_tareas_y_progreso(tutores: &mut Vec<Tutor>, tutorados1: &mut Vec<Tutorado>, tutorados2: &mut Vec<Tutorado>) {
    let mut rng = rand::thread_rng();

    for tutor in tutores.iter_mut() {
        let total = tutor.tareas.len();
        let mut hechas = 0;
        for tarea in tutor.tareas.iter_mut() {
            // Puedes cambiar esto a false si quieres que todas estén sin hacer
            tarea.hecho = rng.gen_bool(0.5); // Aleatorio true/false
            if tarea.hecho { hechas += 1; }
        }
        tutor.progreso = if total > 0 { hechas as f32 / total as f32 } else { 0.0 };
    }

    for tutorado in tutorados1.iter_mut().chain(tutorados2.iter_mut()) {
        let total = tutorado.tareas.len();
        let mut hechas = 0;
        for tarea in tutorado.tareas.iter_mut() {
           // tarea.hecho = rng.gen_bool(0.5);
            if tarea.hecho { hechas += 1; }
        }
        tutorado.progreso = if total > 0 { hechas as f32 / total as f32 } else { 0.0 };
    }
}

#[tauri::command]
pub fn obtener_roles_unicos() -> Result<Vec<String>, String> {
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    match std::fs::read_to_string(json_path) {
        Ok(json_str) => {
            match serde_json::from_str::<serde_json::Value>(&json_str) {
                Ok(json_data) => {
                    let mut roles = std::collections::HashSet::new();
                    
                    // Extraer roles de tutores
                    if let Some(tutores) = json_data["tutores"].as_array() {
                        for tutor in tutores {
                            if let Some(rol) = tutor["rol"].as_str() {
                                roles.insert(rol.to_string());
                            }
                        }
                    }
                    
                    // Extraer roles de tutorados1
                    if let Some(tutorados) = json_data["tutorado1"].as_array() {
                        for tutorado in tutorados {
                            if let Some(rol) = tutorado["rol"].as_str() {
                                roles.insert(rol.to_string());
                            }
                        }
                    }
                    
                    // Extraer roles de tutorados2
                    if let Some(tutorados) = json_data["tutorado2"].as_array() {
                        for tutorado in tutorados {
                            if let Some(rol) = tutorado["rol"].as_str() {
                                roles.insert(rol.to_string());
                            }
                        }
                    }
                    
                    Ok(roles.into_iter().collect())
                },
                Err(e) => Err(format!("Error al parsear el JSON: {}", e))
            }
        },
        Err(e) => Err(format!("Error al leer el archivo JSON: {}", e))
    }
}

#[tauri::command]
pub fn obtener_instituciones_unicas() -> Result<Vec<String>, String> {
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    match std::fs::read_to_string(json_path) {
        Ok(json_str) => {
            match serde_json::from_str::<serde_json::Value>(&json_str) {
                Ok(json_data) => {
                    let mut instituciones = std::collections::HashSet::new();
                    
                    // Extraer instituciones de tutores
                    if let Some(tutores) = json_data["tutores"].as_array() {
                        for tutor in tutores {
                            if let Some(institucion) = tutor["institucion"].as_str() {
                                if !institucion.is_empty() {
                                    instituciones.insert(institucion.to_string());
                                }
                            }
                        }
                    }
                    
                    // Extraer instituciones de tutorados1
                    if let Some(tutorados) = json_data["tutorado1"].as_array() {
                        for tutorado in tutorados {
                            if let Some(institucion) = tutorado["institucion"].as_str() {
                                if !institucion.is_empty() {
                                    instituciones.insert(institucion.to_string());
                                }
                            }
                        }
                    }
                    
                    // Extraer instituciones de tutorados2
                    if let Some(tutorados) = json_data["tutorado2"].as_array() {
                        for tutorado in tutorados {
                            if let Some(institucion) = tutorado["institucion"].as_str() {
                                if !institucion.is_empty() {
                                    instituciones.insert(institucion.to_string());
                                }
                            }
                        }
                    }
                    
                    Ok(instituciones.into_iter().collect())
                },
                Err(e) => Err(format!("Error al parsear el JSON: {}", e))
            }
        },
        Err(e) => Err(format!("Error al leer el archivo JSON: {}", e))
    }
}

#[tauri::command]
pub fn agregar_tarea_y_guardar(
    correo: String,
    nombre_tarea: String,
    descripcion: String,
    hecho: Option<bool>,
    es_tutor: bool,
    es_tutorado1: bool,
) -> Result<String, String> {
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    // Leer el JSON actual
    let json_str = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("No se pudo leer el JSON: {}", e))?;
    let mut data: MonitoreoData = serde_json::from_str(&json_str)
        .map_err(|e| format!("JSON inválido: {}", e))?;

    let tarea = Tarea {
        nombre: nombre_tarea,
        descripcion,
        hecho: hecho.unwrap_or(false),
    };

    let mut encontrada = false;

    if es_tutor {
        for tutor in data.tutores.iter_mut() {
            if tutor.correo == correo {
                tutor.tareas.push(tarea.clone());
                encontrada = true;
                break;
            }
        }
    } else if es_tutorado1 {
        for tutorado in data.tutorado1.iter_mut() {
            if tutorado.correo == correo {
                tutorado.tareas.push(tarea.clone());
                encontrada = true;
                break;
            }
        }
    } else {
        for tutorado in data.tutorado2.iter_mut() {
            if tutorado.correo == correo {
                tutorado.tareas.push(tarea.clone());
                encontrada = true;
                break;
            }
        }
    }

    if !encontrada {
        return Err("No se encontró el usuario".to_string());
    }

    // Actualizar progreso después de agregar la tarea
    actualizar_tareas_y_progreso(&mut data.tutores, &mut data.tutorado1, &mut data.tutorado2);

    // Guardar el JSON actualizado
    let json_string = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Error serializando JSON: {}", e))?;
    std::fs::write(&json_path, json_string)
        .map_err(|e| format!("Error al escribir el JSON: {}", e))?;

    Ok("Tarea agregada y progreso actualizado".to_string())
}

#[tauri::command]
pub fn agregar_imagen_y_guardar(
    correo: String,
    url: String,
    es_tutor: bool,
    es_tutorado1: bool,
) -> Result<String, String> {
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    let json_str = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("No se pudo leer el JSON: {}", e))?;
    let mut data: MonitoreoData = serde_json::from_str(&json_str)
        .map_err(|e| format!("JSON inválido: {}", e))?;

    let imagen = Imagen { url };

    let mut encontrada = false;

    if es_tutor {
        for tutor in data.tutores.iter_mut() {
            if tutor.correo == correo {
                tutor.imagenes.push(imagen.clone());
                encontrada = true;
                break;
            }
        }
    } else if es_tutorado1 {
        for tutorado in data.tutorado1.iter_mut() {
            if tutorado.correo == correo {
                tutorado.imagenes.push(imagen.clone());
                encontrada = true;
                break;
            }
        }
    } else {
        for tutorado in data.tutorado2.iter_mut() {
            if tutorado.correo == correo {
                tutorado.imagenes.push(imagen.clone());
                encontrada = true;
                break;
            }
        }
    }

    if !encontrada {
        return Err("No se encontró el usuario".to_string());
    }

    // Guardar el JSON actualizado
    let json_string = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Error serializando JSON: {}", e))?;
    std::fs::write(&json_path, json_string)
        .map_err(|e| format!("Error al escribir el JSON: {}", e))?;

    Ok("Imagen agregada correctamente".to_string())
}

#[tauri::command]
pub fn eliminar_item_monitoreo(
    correo: String,
    registro: String,
    es_tutor: bool,
    es_tutorado1: bool,
) -> Result<String, String> {
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    let json_str = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("No se pudo leer el JSON: {}", e))?;
    let mut data: MonitoreoData = serde_json::from_str(&json_str)
        .map_err(|e| format!("JSON inválido: {}", e))?;

    let mut encontrada = false;

    let eliminar = |tareas: &mut Vec<Tarea>, imagenes: &mut Vec<Imagen>| {
        let tarea_idx = tareas.iter().position(|t| format!("{}: {}", t.nombre, t.descripcion) == registro);
        if let Some(idx) = tarea_idx {
            tareas.remove(idx);
            return true;
        }
        let imagen_idx = imagenes.iter().position(|i| format!("Imagen: {}", i.url) == registro);
        if let Some(idx) = imagen_idx {
            imagenes.remove(idx);
            return true;
        }
        false
    };

    if es_tutor {
        for tutor in data.tutores.iter_mut() {
            if tutor.correo == correo {
                encontrada = eliminar(&mut tutor.tareas, &mut tutor.imagenes);
                break;
            }
        }
    } else if es_tutorado1 {
        for tutorado in data.tutorado1.iter_mut() {
            if tutorado.correo == correo {
                encontrada = eliminar(&mut tutorado.tareas, &mut tutorado.imagenes);
                break;
            }
        }
    } else {
        for tutorado in data.tutorado2.iter_mut() {
            if tutorado.correo == correo {
                encontrada = eliminar(&mut tutorado.tareas, &mut tutorado.imagenes);
                break;
            }
        }
    }

    if !encontrada {
        return Err("No se encontró el registro".to_string());
    }

    // Actualizar progreso después de eliminar
    actualizar_tareas_y_progreso(&mut data.tutores, &mut data.tutorado1, &mut data.tutorado2);

    let json_string = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Error serializando JSON: {}", e))?;
    std::fs::write(&json_path, json_string)
        .map_err(|e| format!("Error al escribir el JSON: {}", e))?;

    Ok("Registro eliminado correctamente".to_string())
}

#[tauri::command]
pub fn editar_item_monitoreo(
    correo: String,
    registro_anterior: String,
    registro_nuevo: String,
    es_tutor: bool,
    es_tutorado1: bool,
) -> Result<String, String> {
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    let json_str = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("No se pudo leer el JSON: {}", e))?;
    let mut data: MonitoreoData = serde_json::from_str(&json_str)
        .map_err(|e| format!("JSON inválido: {}", e))?;

    let mut encontrada = false;

    let editar = |tareas: &mut Vec<Tarea>, imagenes: &mut Vec<Imagen>| {
        for tarea in tareas.iter_mut() {
            if format!("{}: {}", tarea.nombre, tarea.descripcion) == registro_anterior {
                // Suponemos que el nuevo registro es "nombre: descripcion"
                let partes: Vec<&str> = registro_nuevo.splitn(2, ':').collect();
                if partes.len() == 2 {
                    tarea.nombre = partes[0].trim().to_string();
                    tarea.descripcion = partes[1].trim().to_string();
                    return true;
                }
            }
        }
        for imagen in imagenes.iter_mut() {
            if format!("Imagen: {}", imagen.url) == registro_anterior {
                if registro_nuevo.starts_with("Imagen: ") {
                    imagen.url = registro_nuevo["Imagen: ".len()..].trim().to_string();
                    return true;
                }
            }
        }
        false
    };

    if es_tutor {
        for tutor in data.tutores.iter_mut() {
            if tutor.correo == correo {
                encontrada = editar(&mut tutor.tareas, &mut tutor.imagenes);
                break;
            }
        }
    } else if es_tutorado1 {
        for tutorado in data.tutorado1.iter_mut() {
            if tutorado.correo == correo {
                encontrada = editar(&mut tutorado.tareas, &mut tutorado.imagenes);
                break;
            }
        }
    } else {
        for tutorado in data.tutorado2.iter_mut() {
            if tutorado.correo == correo {
                encontrada = editar(&mut tutorado.tareas, &mut tutorado.imagenes);
                break;
            }
        }
    }

    if !encontrada {
        return Err("No se encontró el registro para editar".to_string());
    }

    // Actualizar progreso después de editar
    actualizar_tareas_y_progreso(&mut data.tutores, &mut data.tutorado1, &mut data.tutorado2);

    let json_string = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Error serializando JSON: {}", e))?;
    std::fs::write(&json_path, json_string)
        .map_err(|e| format!("Error al escribir el JSON: {}", e))?;

    Ok("Registro editado correctamente".to_string())
}


#[tauri::command]
pub fn toggle_hecho_monitoreo(
    correo: String,
    nombre_tarea: String
) -> Result<bool, String> {
    // Obtener la ruta del JSON
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    // Leer el JSON
    let json_str = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("No se pudo leer el JSON: {}", e))?;
    
    // Parsear el JSON
    let mut data: MonitoreoData = serde_json::from_str(&json_str)
        .map_err(|e| format!("JSON inválido: {}", e))?;

    // Variable para almacenar el nuevo estado
    let mut nuevo_estado = false;
    let mut encontrado = false;

    // Buscar la tarea en tutores
    for tutor in data.tutores.iter_mut() {
        if tutor.correo == correo {
            for tarea in tutor.tareas.iter_mut() {
                if tarea.nombre == nombre_tarea {
                    tarea.hecho = !tarea.hecho;
                    nuevo_estado = tarea.hecho;
                    encontrado = true;
                    println!("Tarea '{}' actualizada. Nuevo estado: {}", nombre_tarea, tarea.hecho);
                    break;
                }
            }
        }
    }

    // Si no se encontró en tutores, buscar en tutorado1
    if !encontrado {
        for tutorado in data.tutorado1.iter_mut() {
            if tutorado.correo == correo {
                for tarea in tutorado.tareas.iter_mut() {
                    if tarea.nombre == nombre_tarea {
                        tarea.hecho = !tarea.hecho;
                        nuevo_estado = tarea.hecho;
                        encontrado = true;
                        println!("Tarea '{}' actualizada. Nuevo estado: {}", nombre_tarea, tarea.hecho);
                        break;
                    }
                }
            }
        }
    }

    // Si no se encontró en tutorado1, buscar en tutorado2
    if !encontrado {
        for tutorado in data.tutorado2.iter_mut() {
            if tutorado.correo == correo {
                for tarea in tutorado.tareas.iter_mut() {
                    if tarea.nombre == nombre_tarea {
                        tarea.hecho = !tarea.hecho;
                        nuevo_estado = tarea.hecho;
                        encontrado = true;
                        println!("Tarea '{}' actualizada. Nuevo estado: {}", nombre_tarea, tarea.hecho);
                        break;
                    }
                }
            }
        }
    }

    if !encontrado {
        return Err("No se encontró la tarea especificada".to_string());
    }

    // Guardar los cambios en el JSON
    let json_string = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Error serializando JSON: {}", e))?;
    
    std::fs::write(&json_path, json_string)
        .map_err(|e| format!("Error escribiendo el JSON: {}", e))?;

    Ok(nuevo_estado)
}



