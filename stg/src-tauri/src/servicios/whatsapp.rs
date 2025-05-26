
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use xlsxwriter::Workbook;
use urlencoding::encode;
use crate::notificaciones::leer_archivos::{leer_archivo_emparejados,leer_archivo_control};



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

#[derive(Serialize, Deserialize, Debug)]
pub struct Mensaje {
    correo: String,
    numero: String,
    nombre: String,
    asunto: String,
    mensaje: String,
    link: String,
}

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

    while !path.ends_with("Sistema-TuGestor") && path.parent().is_some() {
        path = path.parent().unwrap().to_path_buf();
    }

    path.push("recursos");
    path
}

#[tauri::command]
pub fn procesar_mensajes_desde_json() -> Result<Vec<Mensaje>, String> {
    let (
        tutores_puj_whatsapp,
        tutores_colegio_whatsapp,
        funcionarios_colegio_whatsapp,
        tutorados_emparejados_whatsapp,
        tutorados_control_whatsapp,
    ) = procesar_datos_para_whatsapp()?;

    let base_path = get_resource_path();
    let path = base_path.join("historiales").join("historial.json");
    let contenido = std::fs::read_to_string(&path)
        .map_err(|e| format!("Error al leer el archivo JSON: {}", e))?;
    let mensajes: Value =
        serde_json::from_str(&contenido).map_err(|e| format!("Error al parsear el JSON: {}", e))?;

    let mut mensajes_generados = Vec::new();

    if let Some(mensajes_array) = mensajes.as_array() {
        for mensaje in mensajes_array {
            if mensaje["estado"].as_bool().unwrap_or(true) {
                let destinatarios_default = Vec::new();
                let destinatarios = mensaje["destinatarios"].as_array().unwrap_or(&destinatarios_default);
                let mensaje_plantilla = mensaje["mensaje"].as_str().unwrap_or("");
                let asunto = mensaje["asunto"].as_str().unwrap_or("").to_string();

                for destinatario in destinatarios {
                    if let Some(destinatario_str) = destinatario.as_str() {
                        match destinatario_str {
                            "TutoresPUJ" => {
                                println!("Procesando mensajes para TutoresPUJ");
                                for tutor in &tutores_puj_whatsapp {
                                    let mensaje_real = mensaje_plantilla
                                        .replace("<<nombre TutoresPUJ>>", &tutor.nombre)
                                        .replace("<<apellido TutoresPUJ>>", &tutor.apellido)
                                        .replace("<<correo TutoresPUJ>>", &tutor.correo)
                                        .replace("<<institucion TutoresPUJ>>", &tutor.institucion)
                                        .replace("<<telefono TutoresPUJ>>", &tutor.telefono.join(", "))
                                        .replace("<<horas TutoresPUJ>>", &tutor.horas)
                                        .replace("<<tutorados TutoresPUJ>>", &tutor.tutorados.join(", "));
                                    let numero = tutor.telefono.get(0).cloned().unwrap_or_default();
                                    let nombre = format!("{} {}", tutor.nombre, tutor.apellido);
                                    let encoded_message = encode(&mensaje_real).to_string();
                                    let whatsapp_url = format!(
                                        "https://api.whatsapp.com/send?phone={}&text={}",
                                        numero, encoded_message
                                    );
                                    mensajes_generados.push(Mensaje {
                                        correo: tutor.correo.clone(),
                                        numero: numero.clone(),
                                        nombre: nombre.clone(),
                                        asunto: asunto.clone(),
                                        mensaje: mensaje_real.clone(),
                                        link: whatsapp_url.clone(),
                                    });
                                    println!("Mensaje generado para TutoresPUJ: {:?}", mensajes_generados.last());
                                }
                            }
                            "TutoresColegio" => {
                                println!("Procesando mensajes para TutoresColegio");
                                for tutor in &tutores_colegio_whatsapp {
                                    let mensaje_real = mensaje_plantilla
                                        .replace("<<nombre TutoresColegio>>", &tutor.nombre)
                                        .replace("<<apellido TutoresColegio>>", &tutor.apellido)
                                        .replace("<<correo TutoresColegio>>", &tutor.correo)
                                        .replace("<<institucion TutoresColegio>>", &tutor.institucion)
                                        .replace("<<telefono TutoresColegio>>", &tutor.telefono.join(", "))
                                        .replace("<<horas TutoresColegio>>", &tutor.horas)
                                        .replace("<<tutorados TutoresColegio>>", &tutor.tutorados.join(", "));
                                    let numero = tutor.telefono.get(0).cloned().unwrap_or_default();
                                    let nombre = format!("{} {}", tutor.nombre, tutor.apellido);
                                    let encoded_message = encode(&mensaje_real).to_string();
                                    let whatsapp_url = format!(
                                        "https://api.whatsapp.com/send?phone={}&text={}",
                                        numero, encoded_message
                                    );

                                    mensajes_generados.push(Mensaje {
                                        correo: tutor.correo.clone(),
                                        numero: numero.clone(),
                                        nombre: nombre.clone(),
                                        asunto: asunto.clone(),
                                        mensaje: mensaje_real.clone(),
                                        link: whatsapp_url.clone(),
                                    });
                                    println!("Mensaje generado: {:?}", mensajes_generados.last());
                                }
                            }
                            "FuncionariosColegio" => {
                                println!("Procesando mensajes para FuncionariosColegio");
                                for funcionario in &funcionarios_colegio_whatsapp {
                                    let mensaje_real = mensaje_plantilla
                                        .replace("<<nombre FuncionariosColegio>>", &funcionario.nombre)
                                        .replace("<<correo FuncionariosColegio>>", &funcionario.correo)
                                        .replace("<<telefono FuncionariosColegio>>", &funcionario.telefono.join(", "))
                                        .replace("<<institucion FuncionariosColegio>>", &funcionario.institucion);
                                    let numero = funcionario.telefono.get(0).cloned().unwrap_or_default();
                                    let nombre = funcionario.nombre.clone();
                                    let encoded_message = encode(&mensaje_real).to_string();
                                    let whatsapp_url = format!(
                                        "https://api.whatsapp.com/send?phone={}&text={}",
                                        numero, encoded_message
                                    );

                                    mensajes_generados.push(Mensaje {
                                        correo: funcionario.correo.clone(),
                                        numero: numero.clone(),
                                        nombre: nombre.clone(),
                                        asunto: asunto.clone(),
                                        mensaje: mensaje_real.clone(),
                                        link: whatsapp_url.clone(),
                                    });
                                    println!("Mensaje generado: {:?}", mensajes_generados.last());
                                }
                            }
                            "TutoradosEmparejados" => {
                                println!("Procesando mensajes para TutoradosEmparejados");
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
                                    let numero = tutorado.telefono.get(0).cloned().unwrap_or_default();
                                    let nombre = tutorado.nombre.clone();
                                    let encoded_message = encode(&mensaje_real).to_string();
                                    let whatsapp_url = format!(
                                        "https://api.whatsapp.com/send?phone={}&text={}",
                                        numero, encoded_message
                                    );

                                    mensajes_generados.push(Mensaje {
                                        correo: tutorado.correo.clone(),
                                        numero: numero.clone(),
                                        nombre: nombre.clone(),
                                        asunto: asunto.clone(),
                                        mensaje: mensaje_real.clone(),
                                        link: whatsapp_url.clone(),
                                    });
                                    println!("Mensaje generado: {:?}", mensajes_generados.last());
                                }
                            }
                            "TutoradosControl" => {
                                println!("Procesando mensajes para TutoradosControl");
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
                                    let numero = tutorado.telefono.get(0).cloned().unwrap_or_default();
                                    let nombre = tutorado.nombre.clone();
                                    let encoded_message = encode(&mensaje_real).to_string();
                                    let whatsapp_url = format!(
                                        "https://api.whatsapp.com/send?phone={}&text={}",
                                        numero, encoded_message
                                    );

                                    mensajes_generados.push(Mensaje {
                                        correo: tutorado.correo.clone(),
                                        numero: numero.clone(),
                                        nombre: nombre.clone(),
                                        asunto: asunto.clone(),
                                        mensaje: mensaje_real.clone(),
                                        link: whatsapp_url.clone(),
                                    });
                                    println!("Mensaje generado: {:?}", mensajes_generados.last());
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

#[tauri::command]
pub fn exportar_mensajes_a_excel(mensajes: Vec<Mensaje>, ruta: Option<String>) -> Result<String, String> {
    let fecha = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let nombre_archivo = format!("whatsapp_para_enviar_{}.xlsx", fecha);

    // Guardar en la carpeta recursos
    let mut ruta_archivo = get_resource_path();
    ruta_archivo.push(&nombre_archivo);

    // Si el usuario pasa una ruta personalizada, usarla
    let ruta_archivo = ruta.unwrap_or_else(|| ruta_archivo.to_string_lossy().to_string());

    let workbook = Workbook::new(&ruta_archivo).map_err(|e| e.to_string())?;
    let mut sheet = workbook.add_worksheet(None).map_err(|e| e.to_string())?;

    // Escribir encabezados
    sheet.write_string(0, 0, "Correo", None).unwrap();
    sheet.write_string(0, 1, "Número", None).unwrap();
    sheet.write_string(0, 2, "Nombre", None).unwrap();
    sheet.write_string(0, 3, "Asunto", None).unwrap();
    sheet.write_string(0, 4, "Mensaje", None).unwrap();
    sheet.write_string(0, 5, "Link", None).unwrap();

    // Escribir los datos
    for (i, m) in mensajes.iter().enumerate() {
        let fila = (i + 1) as u32;
        sheet.write_string(fila, 0, &m.correo, None).unwrap();
        sheet.write_string(fila, 1, &m.numero, None).unwrap();
        sheet.write_string(fila, 2, &m.nombre, None).unwrap();
        sheet.write_string(fila, 3, &m.asunto, None).unwrap();
        sheet.write_string(fila, 4, &m.mensaje, None).unwrap();
        sheet.write_string(fila, 5, &m.link, None).unwrap();
    }

    workbook.close().map_err(|e| e.to_string())?;
    println!("Archivo Excel generado en: {}", &ruta_archivo);
    Ok(ruta_archivo)
}


