use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use xlsxwriter::{Workbook, Format};
use crate::notificaciones::leer_archivos::{
    TutoresPUJ, TutoresColegio, FuncionariosColegio, TutoradosEmparejados, TutoradosControl,
    leer_archivo_emparejados, leer_archivo_control, generar_tutores, generar_tutores_enlaces,
};

#[derive(Deserialize)]
struct Borrador {
    destinatarios: Vec<String>,
    asunto: String,
    mensaje: String,
}


#[derive(Serialize, Debug)]
pub struct TutoresPUJwhatsapp {
    nombre: String,
    apellido: String,
    correo: String,
    institucion: String,
    telefono: Vec<String>,
    horas: String,
    tutorados: Vec<String>,
    link: String,
}

#[derive(Serialize, Debug)]
pub struct TutoresColegiowhatsapp {
    nombre: String,
    apellido: String,
    correo: String,
    institucion: String,
    telefono: Vec<String>,
    horas: String,
    tutorados: Vec<String>,
    link: String,
}

#[derive(Serialize, Debug)]
pub struct FuncionariosColegiowhatsapp {
    nombre: String,
    correo: String,
    telefono: Vec<String>,
    institucion: String,
    
}

#[derive(Serialize, Debug)]
pub struct TutoradosEmparejadoswhatsapp {
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
pub struct TutoradosControlwhatsapp {
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


/*
#[tauri::command]
pub fn generar_excel_whatsapp() -> Result<String, String> {
    let fecha_actual = "2025-05-13".to_string();
    let nombre_archivo = format!("Historial_{}.xlsx", fecha_actual);

    let workbook = Workbook::new(&nombre_archivo).map_err(|e| e.to_string())?;
    let mut worksheet = workbook.add_worksheet(None).map_err(|e| e.to_string())?;

    let header_format = Format::new().set_bold();

    // Escribir encabezados
    worksheet.write_string(0, 0, "Nombre", Some(&header_format))?;
    worksheet.write_string(0, 1, "Apellido", Some(&header_format))?;
    worksheet.write_string(0, 2, "Correo", Some(&header_format))?;
    worksheet.write_string(0, 3, "Institución", Some(&header_format))?;
    worksheet.write_string(0, 4, "Teléfono", Some(&header_format))?;
    worksheet.write_string(0, 5, "Horas", Some(&header_format))?;
    worksheet.write_string(0, 6, "Tutorados", Some(&header_format))?;
    worksheet.write_string(0, 7, "Link", Some(&header_format))?;

    // Leer datos desde las funciones de `leer_archivos`
    let (tutores_puj, _, _, _) = leer_archivo_emparejados().map_err(|e| format!("Error al leer emparejados: {}", e))?;

    // Escribir datos de Tutores PUJ
    let mut row = 1;
    for tutor in tutores_puj {
        worksheet.write_string(row, 0, &tutor.nombre, None)?;
        worksheet.write_string(row, 1, &tutor.apellido, None)?;
        worksheet.write_string(row, 2, &tutor.correo, None)?;
        worksheet.write_string(row, 3, &tutor.institucion, None)?;
        worksheet.write_string(row, 4, &tutor.telefono.join(", "), None)?;
        worksheet.write_string(row, 5, &tutor.horas, None)?;
        worksheet.write_string(row, 6, &tutor.tutorados.join(", "), None)?;
        worksheet.write_string(row, 7, &tutor.link, None)?;
        row += 1;
    }

    workbook.close().map_err(|e| e.to_string())?;
    Ok(nombre_archivo)
}
*/

#[tauri::command]
pub fn procesar_datos_para_whatsapp() -> Result<
    (
        Vec<TutoresPUJwhatsapp>,
        Vec<TutoresColegiowhatsapp>,
        Vec<FuncionariosColegiowhatsapp>,
        Vec<TutoradosEmparejadoswhatsapp>,
        Vec<TutoradosControlwhatsapp>,
    ),
    String,
> {
    let (tutores_puj, tutores_colegio, funcionarios_colegio, tutorados_emparejados) =
        leer_archivo_emparejados().map_err(|e| format!("Error al leer emparejados: {}", e))?;

    let tutorados_control = leer_archivo_control().map_err(|e| format!("Error al leer control: {}", e))?;

    // Convertir los datos a las estructuras de `whatsapp.rs`
    let tutores_puj_whatsapp: Vec<TutoresPUJwhatsapp> = tutores_puj
        .into_iter()
        .map(|tutor| TutoresPUJwhatsapp {
            nombre: tutor.nombre,
            apellido: tutor.apellido,
            correo: tutor.correo,
            institucion: tutor.institucion,
            telefono: tutor.telefono,
            horas: tutor.horas,
            tutorados: tutor.tutorados,
            link: tutor.link,
        })
        .collect();

    let tutores_colegio_whatsapp: Vec<TutoresColegiowhatsapp> = tutores_colegio
        .into_iter()
        .map(|tutor| TutoresColegiowhatsapp {
            nombre: tutor.nombre,
            apellido: tutor.apellido,
            correo: tutor.correo,
            institucion: tutor.institucion,
            telefono: tutor.telefono,
            horas: tutor.horas,
            tutorados: tutor.tutorados,
            link: tutor.link,
        })
        .collect();

    let funcionarios_colegio_whatsapp: Vec<FuncionariosColegiowhatsapp> = funcionarios_colegio
        .into_iter()
        .map(|funcionario| FuncionariosColegiowhatsapp {
            nombre: funcionario.nombre,
            correo: funcionario.correo,
            telefono: funcionario.telefono,
            institucion: funcionario.institucion,
        })
        .collect();

    let tutorados_emparejados_whatsapp: Vec<TutoradosEmparejadoswhatsapp> = tutorados_emparejados
        .into_iter()
        .map(|tutorado| TutoradosEmparejadoswhatsapp {
            nombre: tutorado.nombre,
            correo: tutorado.correo,
            telefono: tutorado.telefono,
            id: tutorado.id,
            colegio: tutorado.colegio,
            vocabulario: tutorado.vocabulario,
            gramatica: tutorado.gramatica,
            escucha: tutorado.escucha,
            lectura: tutorado.lectura,
            a: tutorado.a,
            b: tutorado.b,
            c: tutorado.c,
            d: tutorado.d,
            e: tutorado.e,
            f: tutorado.f,
            g: tutorado.g,
        })
        .collect();

    let tutorados_control_whatsapp: Vec<TutoradosControlwhatsapp> = tutorados_control
        .into_iter()
        .map(|tutorado| TutoradosControlwhatsapp {
            nombre: tutorado.nombre,
            correo: tutorado.correo,
            telefono: tutorado.telefono,
            id: tutorado.id,
            colegio: tutorado.colegio,
            vocabulario: tutorado.vocabulario,
            gramatica: tutorado.gramatica,
            escucha: tutorado.escucha,
            lectura: tutorado.lectura,
            a: tutorado.a,
            b: tutorado.b,
            c: tutorado.c,
            d: tutorado.d,
            e: tutorado.e,
            f: tutorado.f,
            g: tutorado.g,
        })
        .collect();

    /* 
    println!("Tutores PUJ: {:?}", tutores_puj_whatsapp);
    println!("Tutores Colegio: {:?}", tutores_colegio_whatsapp);
    println!("Funcionarios Colegio: {:?}", funcionarios_colegio_whatsapp);
    println!("Tutorados Emparejados: {:?}", tutorados_emparejados_whatsapp);
    println!("Tutorados Control: {:?}", tutorados_control_whatsapp);
*/
    Ok((
        tutores_puj_whatsapp,
        tutores_colegio_whatsapp,
        funcionarios_colegio_whatsapp,
        tutorados_emparejados_whatsapp,
        tutorados_control_whatsapp,

    ))
}

// Función para obtener la ruta base del recurso
fn get_resource_path() -> PathBuf {
    let current_exe = std::env::current_exe().expect("Failed to get current executable path");
    let mut path = current_exe.parent().unwrap().to_path_buf();

    // Navegar hacia arriba hasta encontrar la carpeta "Sistema-TuGestor"
    while !path.ends_with("Sistema-TuGestor") && path.parent().is_some() {
        path = path.parent().unwrap().to_path_buf();
    }

    // Agregar la carpeta "recursos"
    path.push("recursos");
    path
}

#[tauri::command]
pub fn procesar_mensajes_desde_json() -> Result<Vec<String>, String> {
   // println!("Procesando mensajes desde JSON...1");

    // Llamar a `procesar_datos_para_whatsapp` para obtener los datos procesados
    let (tutores_puj_whatsapp, tutores_colegio_whatsapp, funcionarios_colegio_whatsapp, tutorados_emparejados_whatsapp, tutorados_control_whatsapp) =
        procesar_datos_para_whatsapp()?;
//println!("Datos procesados correctamente.2");
    // Construir la ruta del archivo JSON
    let base_path = get_resource_path();
    let path = base_path.join("historiales").join("historial.json");
//println!("Ruta del archivo 2");
    // Leer el archivo JSON
    let contenido = std::fs::read_to_string(&path).map_err(|e| format!("Error al leer el archivo JSON: {}", e))?;
    let mensajes: Value = serde_json::from_str(&contenido).map_err(|e| format!("Error al parsear el JSON: {}", e))?;
//println!("Archivo JSON leído y parseado correctamente.3");
    let mut mensajes_generados = Vec::new();

    // Procesar los mensajes
    if let Some(mensajes_array) = mensajes.as_array() {
       // println!("Mensajes encontrados en el JSON.4");
       // println!("Mensajes encontrados: {:?}", mensajes_array);
        for mensaje in mensajes_array {
           // println!("Mensajes dentro del array.5");
           // println!("Mensaje encontrado: {:?}", mensaje);
            if mensaje["estado"].as_bool().unwrap_or(true) {
                let destinatarios_default = Vec::new();
                let destinatarios = mensaje["destinatarios"].as_array().unwrap_or(&destinatarios_default);
                let mensaje_plantilla = mensaje["mensaje"].as_str().unwrap_or("");

                for destinatario in destinatarios {
                    if let Some(destinatario_str) = destinatario.as_str() {
                        match destinatario_str {
                            "TutoresPUJ" => {
                                for tutor in &tutores_puj_whatsapp {
                                    let mensaje_real = mensaje_plantilla
                                        .replace("<<nombre TutoresPUJ>>", &tutor.nombre)
                                        .replace("<<apellido TutoresPUJ>>", &tutor.apellido)
                                        .replace("<<correo TutoresPUJ>>", &tutor.correo)
                                        .replace("<<institucion TutoresPUJ>>", &tutor.institucion)
                                        .replace("<<telefono TutoresPUJ>>", &tutor.telefono.join(", "))
                                        .replace("<<horas TutoresPUJ>>", &tutor.horas)
                                        .replace("<<tutorados TutoresPUJ>>", &tutor.tutorados.join(", "));
                                    println!("🧑‍🎓🧑‍🏫TutoresPUJ");
                                    println!("Mensaje real: {}", mensaje_real);
                                    mensajes_generados.push(mensaje_real);
                                }
                            }
                            "TutoresColegio" => {
                                for tutor in &tutores_colegio_whatsapp {
                                    let mensaje_real = mensaje_plantilla
                                        .replace("<<nombre TutoresColegio>>", &tutor.nombre)
                                        .replace("<<apellido TutoresColegio>>", &tutor.apellido)
                                        .replace("<<correo TutoresColegio>>", &tutor.correo)
                                        .replace("<<institucion TutoresColegio>>", &tutor.institucion)
                                        .replace("<<telefono TutoresColegio>>", &tutor.telefono.join(", "))
                                        .replace("<<horas TutoresColegio>>", &tutor.horas)
                                        .replace("<<tutorados TutoresColegio>>", &tutor.tutorados.join(", "));
                                    println!("🏫👳‍♂️TutoresColegio");
                                    println!("Mensaje real: {}", mensaje_real);
                                    mensajes_generados.push(mensaje_real);
                                }
                            }
                            "FuncionariosColegio" => {
                                for funcionario in &funcionarios_colegio_whatsapp {
                                    let mensaje_real = mensaje_plantilla
                                        .replace("<<nombre FuncionariosColegio>>", &funcionario.nombre)
                                        .replace("<<correo FuncionariosColegio>>", &funcionario.correo)
                                        .replace("<<telefono FuncionariosColegio>>", &funcionario.telefono.join(", "))
                                        .replace("<<institucion FuncionariosColegio>>", &funcionario.institucion);
                                    println!("🏫Funcionarios Colegio");
                                    println!("Mensaje real: {}", mensaje_real);
                                    mensajes_generados.push(mensaje_real);
                                }
                            }
                            "TutoradosEmparejados" => {
                                for tutorado in &tutorados_emparejados_whatsapp {
                                    let mensaje_real = mensaje_plantilla
                                        .replace("<<nombre TutoradosEmparejados>>", &tutorado.nombre)
                                        .replace("<<correo TutoradosEmparejados>>", &tutorado.correo)
                                        .replace("<<telefono TutoradosEmparejados>>", &tutorado.telefono.join(", "))
                                        .replace("<<id TutoradosEmparejados>>", &tutorado.id)
                                        .replace("<<colegio TutoradosEmparejados>>", &tutorado.colegio)
                                        .replace("<<vocabulario TutoradosEmparejados>>", &tutorado.vocabulario)
                                        .replace("<<gramatica TutoradosEmparejados>>", &tutorado.gramatica)
                                        .replace("<<escucha TutoradosEmparejados>>", &tutorado.escucha)
                                        .replace("<<lectura TutoradosEmparejados>>", &tutorado.lectura)
                                        .replace("<<a TutoradosEmparejados>>", &tutorado.a)
                                        .replace("<<b TutoradosEmparejados>>", &tutorado.b)
                                        .replace("<<c TutoradosEmparejados>>", &tutorado.c)
                                        .replace("<<d TutoradosEmparejados>>", &tutorado.d)
                                        .replace("<<e TutoradosEmparejados>>", &tutorado.e)
                                        .replace("<<f TutoradosEmparejados>>", &tutorado.f)
                                        .replace("<<g TutoradosEmparejados>>", &tutorado.g);
                                    println!("🥪Tutorados Emparejados");
                                    println!("Mensaje real: {}", mensaje_real);
                                    mensajes_generados.push(mensaje_real);
                                }
                            }
                            "TutoradosControl" => {
                                for tutorado in &tutorados_control_whatsapp {
                                    let mensaje_real = mensaje_plantilla
                                        .replace("<<nombre TutoradosControl>>", &tutorado.nombre)
                                        .replace("<<correo TutoradosControl>>", &tutorado.correo)
                                        .replace("<<telefono TutoradosControl>>", &tutorado.telefono.join(", "))
                                        .replace("<<colegio TutoradosControl>>", &tutorado.colegio)
                                        .replace("<<id TutoradosControl>>", &tutorado.id)
                                        .replace("<<vocabulario TutoradosControl>>", &tutorado.vocabulario)
                                        .replace("<<gramatica TutoradosControl>>", &tutorado.gramatica)
                                        .replace("<<escucha TutoradosControl>>", &tutorado.escucha)
                                        .replace("<<lectura TutoradosControl>>", &tutorado.lectura)
                                        .replace("<<a TutoradosControl>>", &tutorado.a)
                                        .replace("<<b TutoradosControl>>", &tutorado.b)
                                        .replace("<<c TutoradosControl>>", &tutorado.c)
                                        .replace("<<d TutoradosControl>>", &tutorado.d)
                                        .replace("<<e TutoradosControl>>", &tutorado.e)
                                        .replace("<<f TutoradosControl>>", &tutorado.f)
                                        .replace("<<g TutoradosControl>>", &tutorado.g);
                                    println!("🔑Tutorados Contol");
                                    println!("Mensaje real: {}", mensaje_real);
                                    mensajes_generados.push(mensaje_real);
                                }
                            }
                            _ => {
                                println!("Destinatario desconocido: {}", destinatario_str);
                            }
                        }
                    }
                }
            }
        }
    } else {
        return Err("El archivo JSON no contiene un array de mensajes.".to_string());
    }

    Ok(mensajes_generados)
}


