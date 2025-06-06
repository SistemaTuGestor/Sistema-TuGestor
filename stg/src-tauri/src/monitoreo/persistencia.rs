
use calamine::{open_workbook, Reader, Xlsx, XlsxError} ;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
use std::path::PathBuf;
use crate::servicios::logger::log_event;



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
            hecho: false,  // Por defecto, la tarea no está hecha
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
pub fn get_resource_path() -> PathBuf {
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

    log_event("Iniciando lectura de Excel".to_string()).unwrap();
    let base_path = get_resource_path();

    let json_path = base_path.join("monitoreo").join("monitoreo.json");
    let excel_path = base_path.join("EmparejamientoFINAL.xlsx");

    /*
        if json_path.exists() {
            println!("El archivo JSON ya existe, no es necesario regenerarlo.");
            return Err("Ya existe el archivo JSON".to_string());
        }
    */

    if json_path.exists() {
        // Si existe, leer y retornar los datos del JSON existente
        let json_str = std::fs::read_to_string(&json_path)
            .map_err(|e| format!("No se pudo leer el JSON existente: {}", e))?;
        
        let data: MonitoreoData = serde_json::from_str(&json_str)
            .map_err(|e| format!("Error parseando JSON existente: {}", e))?;
        
        return Ok((data.tutores, data.tutorado1, data.tutorado2));
    }


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
    let mut contador_tu= 0;
    
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
          nombre:"Curso de información sensible".to_string(), 
          descripcion: "Completa los modulos".to_string(), 
          hecho: false, // Por defecto, la tarea no está hecha
        };

        let mut tarea2 = Tarea{
            nombre:"Primer Contacto".to_string(), 
            descripcion: "Se contacta con su tutor o tutorados".to_string(), 
            hecho: false, // Por defecto, la tarea no está hecha
        };
        
        let mut tarea3 = Tarea{
            nombre:"Finaliza el curso".to_string(), 
            descripcion: "Diligencia formulario de experiencia".to_string(), 
            hecho: false, // Por defecto, la tarea no está hecha
        };

        let mut lista_tareas = Vec::new();
        lista_tareas.push(tarea);
        lista_tareas.push(tarea2);
        lista_tareas.push(tarea3);
        
        let mut lista_imagenes = Vec::new();

        let mut lista_tutoradonumeros = Vec::new();
        lista_tutoradonumeros.push(telefonotut1);
        lista_tutoradonumeros.push(telefono2tut1);

        let mut lista_tutoradonumeros2 = Vec::new();
        lista_tutoradonumeros2.push(telefonotut2);
        lista_tutoradonumeros2.push(telefono2tut2);
    
        contador_tu += 1;
        let mut tutor = Tutor{
            id: contador_tu.to_string(),
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
        contador_tu += 1;
        let mut tutorado1 = Tutorado{
            id: contador_tu.to_string(),
            nombre: nombretutorados1.clone(),
            cedula: cedulatutorados1.clone(),
            rol: "Tutorado".to_string(),
            institucion: instituciontut1.clone(),
            telefono: lista_tutoradonumeros,
            correo: correotut1.clone(),
            tareas: lista_tareas.clone(),
            imagenes: lista_imagenes.clone(),
            progreso: 0.0, // Por defecto, el progreso es 0
        };
        contador_tu += 1;
        let mut tutorado2 = Tutorado{
            id: contador_tu.to_string(),
            nombre: nombretutorados2.clone(),
            cedula: cedulatutorados2.clone(),
            rol: "Tutorado".to_string(),
            institucion: instituciontut2.clone(),
            telefono: lista_tutoradonumeros2,
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
    
    log_event("Lectura de Excel completada".to_string()).unwrap();
    Ok((tutores, tutorados1, tutorados2))
}


fn guardar_monitoreo_json(
    tutores: Vec<Tutor>,
    tutorado1: Vec<Tutorado>,
    tutorado2: Vec<Tutorado>,
) -> Result<(), String> {
    log_event("Guardando datos en JSON".to_string()).unwrap();
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
    log_event("Datos guardados en JSON".to_string()).unwrap();

    Ok(())
}


#[tauri::command]
pub fn cargar_datos_json() -> Result<String, String> {
    log_event("Cargando datos desde JSON".to_string()).unwrap();
    //let ruta = "C:\\Users\\Javier\\Desktop\\Proyecto TuGestor\\Sistema-TuGestor\\recursos\\monitoreo\\monitoreo.json";
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");
    log_event("Datos cargados desde JSON".to_string()).unwrap();

    std::fs::read_to_string(json_path).map_err(|e| format!("No se pudo leer el JSON: {}", e))
}

#[tauri::command] // Manejo de actualizaciones para tareas.
pub fn actualizar_json_monitoreo(json_data: String) -> Result<(), String> {
    log_event("Actualizando JSON de monitoreo".to_string()).unwrap();
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    // Verificar que el JSON es válido antes de escribir
    let data: serde_json::Value = serde_json::from_str(&json_data)
        .map_err(|e| format!("JSON inválido: {}", e))?;

    // Escribir el archivo
    std::fs::write(&json_path, json_data)
        .map_err(|e| format!("Error escribiendo el archivo: {}", e))?;

    println!("JSON actualizado correctamente");
    log_event("JSON de monitoreo actualizado".to_string()).unwrap();
    Ok(())
}

#[tauri::command]
pub fn guardar_datos_json(datos: String) -> Result<String, String> {
    log_event("Guardando datos en JSON".to_string()).unwrap();
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    // Verificar que los datos sean un JSON válido antes de escribir
    match serde_json::from_str::<serde_json::Value>(&datos) {
        Ok(_) => {
            // JSON válido, proceder a escribir
            match std::fs::write(&json_path, datos) {
                Ok(_) => {
                    log_event("Datos guardados en JSON".to_string()).unwrap();
                    Ok("Datos guardados correctamente".to_string())
                },
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
    // Actualizar progreso de tutores
    for tutor in tutores.iter_mut() {
        let total_tareas = tutor.tareas.len();
        if total_tareas > 0 {
            let tareas_completadas = tutor.tareas.iter().filter(|t| t.hecho).count();
            tutor.progreso = tareas_completadas as f32 / total_tareas as f32;
        } else {
            tutor.progreso = 0.0;
        }
        //Si quieres ver el progreso de los tutores en consola, descomenta la siguiente línea
        // println!("Tutor: {}, Tareas totales: {}, Completadas: {}, Progreso: {:.2}", 
        //     tutor.nombre, 
        //     total_tareas,
        //     tutor.tareas.iter().filter(|t| t.hecho).count(),
        //     tutor.progreso
        // );
    }

    // Actualizar progreso de tutorados (tanto tutorado1 como tutorado2)
    for tutorado in tutorados1.iter_mut().chain(tutorados2.iter_mut()) {
        let total_tareas = tutorado.tareas.len();
        if total_tareas > 0 {
            let tareas_completadas = tutorado.tareas.iter().filter(|t| t.hecho).count();
            tutorado.progreso = tareas_completadas as f32 / total_tareas as f32;
        } else {
            tutorado.progreso = 0.0;
        }
        //Si quieres ver el progreso de los tutorados en consola, descomenta la siguiente línea
        // println!("Tutorado: {}, Tareas totales: {}, Completadas: {}, Progreso: {:.2}", 
        //     tutorado.nombre, 
        //     total_tareas,
        //     tutorado.tareas.iter().filter(|t| t.hecho).count(),
        //     tutorado.progreso
        // );
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
    let base_path = get_resource_path();
    let json_path = base_path.join("monitoreo").join("monitoreo.json");

    // Leer el JSON
    let json_str = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("No se pudo leer el JSON: {}", e))?;
    
    // Parsear el JSON
    let mut data: MonitoreoData = serde_json::from_str(&json_str)
        .map_err(|e| format!("JSON inválido: {}", e))?;

    let mut nuevo_estado = false;
    let mut encontrado = false;

    // Buscar y actualizar la tarea
    for tutor in data.tutores.iter_mut() {
        if tutor.correo == correo {
            for tarea in tutor.tareas.iter_mut() {
                if tarea.nombre == nombre_tarea {
                    tarea.hecho = !tarea.hecho;
                    nuevo_estado = tarea.hecho;
                    encontrado = true;
                    break;
                }
            }
        }
    }

    if !encontrado {
        for tutorado in data.tutorado1.iter_mut() {
            if tutorado.correo == correo {
                for tarea in tutorado.tareas.iter_mut() {
                    if tarea.nombre == nombre_tarea {
                        tarea.hecho = !tarea.hecho;
                        nuevo_estado = tarea.hecho;
                        encontrado = true;
                        break;
                    }
                }
            }
        }
    }

    if !encontrado {
        for tutorado in data.tutorado2.iter_mut() {
            if tutorado.correo == correo {
                for tarea in tutorado.tareas.iter_mut() {
                    if tarea.nombre == nombre_tarea {
                        tarea.hecho = !tarea.hecho;
                        nuevo_estado = tarea.hecho;
                        encontrado = true;
                        break;
                    }
                }
            }
        }
    }

    if !encontrado {
        return Err("No se encontró la tarea especificada".to_string());
    }

    // Actualizar el progreso después de cambiar el estado de la tarea
    actualizar_tareas_y_progreso(&mut data.tutores, &mut data.tutorado1, &mut data.tutorado2);

    // Guardar los cambios en el JSON
    let json_string = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Error serializando JSON: {}", e))?;
    
    std::fs::write(&json_path, json_string)
        .map_err(|e| format!("Error escribiendo el JSON: {}", e))?;

    println!("Tarea '{}' actualizada. Nuevo estado: {}", nombre_tarea, nuevo_estado);
    Ok(nuevo_estado)
}

#[tauri::command]
pub fn leer_archivo_imagen(path: String) -> Result<Vec<u8>, String> {
    use std::fs;
    use std::io::Read;
    
    let mut file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(e) => return Err(format!("No se pudo abrir el archivo: {}", e)),
    };
    
    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        return Err(format!("Error al leer el archivo: {}", e));
    }
    
    Ok(buffer)
}


#[tauri::command]
pub fn guardar_imagen_persistente(
    file_data: Vec<u8>,
    file_name: String,
) -> Result<String, String> {
    // Obtener directorio de datos de la aplicación
    let app_data_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
        .ok_or("No se pudo obtener directorio de datos de la app")?;
    
    // Crear subdirectorio para imágenes si no existe
    let images_dir = app_data_dir.join("images");
    std::fs::create_dir_all(&images_dir)
        .map_err(|e| format!("No se pudo crear directorio de imágenes: {}", e))?;
    
    // Crear ruta completa del archivo
    let file_path = images_dir.join(&file_name);
    
    // Guardar el archivo
    std::fs::write(&file_path, &file_data)
        .map_err(|e| format!("No se pudo guardar la imagen: {}", e))?;
    
    // Devolver la ruta como string
    file_path.to_str()
        .map(|s| s.to_string())
        .ok_or("Error al convertir ruta a string".to_string())
}

