use calamine::{open_workbook, Reader, Xlsx, DataType};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

const ARCHIVO_EXCEL: &str = "C:\\Users\\USER\\Documents\\GitHub\\Sistema-TuGestor\\recursos\\EmparejamientoFINAL.xlsx";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmparejamientoItem {  
    pub tutor: String,           
    pub disponibilidadTutor: String,
    pub materiaTutor: String,
    pub modalidad: String,
    pub max_tutorados: u8,
    pub tutorado1: String,
    pub tutorado1_id: String,
    pub disponibilidadTutorado1: String,
    pub materiaTutorado1: String,
    pub tutorado2: String,
    pub tutorado2_id: String,
    pub disponibilidadTutorado2: String,
    pub materiaTutorado2: String,
    pub colorOriginal1: Option<String>,
    pub colorOriginal2: Option<String>,
    pub grupoTutorado1: String,
    pub grupoTutorado2: String,
    pub contactoTutor: String,
    pub contactoTutorado1: String,
    pub contactoTutorado2: String,
}

// Funciones de utilidad para normalización y cálculo de color
fn remove_accents(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'á' | 'à' | 'ä' | 'â' | 'ã' => result.push('a'),
            'é' | 'è' | 'ë' | 'ê' => result.push('e'),
            'í' | 'ì' | 'ï' | 'î' => result.push('i'),
            'ó' | 'ò' | 'ö' | 'ô' | 'õ' => result.push('o'),
            'ú' | 'ù' | 'ü' | 'û' => result.push('u'),
            'ñ' => result.push('n'),
            'Á' | 'À' | 'Ä' | 'Â' | 'Ã' => result.push('A'),
            'É' | 'È' | 'Ë' | 'Ê' => result.push('E'),
            'Í' | 'Ì' | 'Ï' | 'Î' => result.push('I'),
            'Ó' | 'Ò' | 'Ö' | 'Ô' | 'Õ' => result.push('O'),
            'Ú' | 'Ù' | 'Ü' | 'Û' => result.push('U'),
            'Ñ' => result.push('N'),
           
            _ => result.push(c),
        }

    }
    
    result
}
fn first_upper(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize = true;

    for c in s.chars() {
        if capitalize {
            result.push(c.to_uppercase().to_string().chars().next().unwrap());
            capitalize = false;
        } else {
            result.push(c.to_lowercase().to_string().chars().next().unwrap());
        }
    }

    result
}

fn normalize(s: &str) -> String {
    remove_accents(s).trim().to_lowercase()
    
}
fn normalizeTutor(s: &str) -> String {
    first_upper(s)
    
    
}


fn calcular_color(materia: &str) -> String {
    match normalize(materia).as_str() {
        "ingles" => "tutorado-ingles".to_string(),
        "matematicas" | "matematica" => "tutorado-matematicas".to_string(),
        _ => "".to_string(),
    }
}


#[tauri::command] 
pub fn obtener_emparejamiento() -> Result<Vec<EmparejamientoItem>, String> {
    let mut workbook: Xlsx<_> = open_workbook(ARCHIVO_EXCEL)
        .map_err(|e| format!("❌ No se pudo abrir el archivo Excel: {}", e))?;

    println!("📂 Archivo Excel abierto correctamente.");
    let sheet_names = workbook.sheet_names();
    println!("📄 Hojas disponibles en el archivo: {:?}", sheet_names);

    // --- Procesar la hoja "Emparejamiento" ---
    let range = workbook
        .worksheet_range("Emparejamiento")
        .map_err(|e| format!("❌ No se pudo cargar la hoja 'Emparejamiento': {}", e))?;
    let mut emparejamientos = Vec::new();

    for (i, row) in range.rows().enumerate() {
        if i == 0 { continue; } // Saltar encabezado

        println!("➡ Procesando fila {}: {:?}", i, row);

        // Datos del tutor
        let tutor = format!(
            "{} {}",
            row.get(0)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string()),
            row.get(1)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string())
        );
        let disponibilidadTutor = row.get(9)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materiaTutor = row.get(6)
            .and_then(|c| c.as_string())
            .map(|s| normalizeTutor(&s))
            .unwrap_or_else(|| "VACÍO".to_string());
        let contactoTutor = format!(
            "correo:{}  telefono:{}",
            row.get(2)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string()),
            row.get(3)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string())
        );
        let modalidad = row.get(7)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
            let max_tutorados = match modalidad.as_str() {
                "40 horas - 1 tutorado" => 1,
                "80 horas - 1 tutorado" => 1,
                "100 horas - 1 tutorado" => 1,
                "80 horas - 2 tutorado" => 2,
                _ => 2 // Valor por defecto para códigos desconocidos
            };
        // Datos del primer tutorado
        let tutorado1 = row.get(10)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let tutorado1_id = row.get(11)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let disponibilidadTutorado1 = row.get(28)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materiaTutorado1 = row.get(16)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let contactoTutorado1 = format!(
            "tele1:{}  tele2:{} contacto:{}",
            row.get(13)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string()),
            row.get(14)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string()),
            row.get(15)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string())
        );
        let grupoTutorado1 = row.get(29)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        // Datos del segundo tutorado
        let tutorado2 = row.get(30)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let tutorado2_id = row.get(31)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let disponibilidadTutorado2 = row.get(48)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materiaTutorado2 = row.get(36)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let contactoTutorado2 = format!(
            "tele1:{}  tele2:{} contacto:{}",
            row.get(33)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string()),
            row.get(34)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string()),
            row.get(35)
                .and_then(|c| c.as_string())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "VACÍO".to_string())
            
        );
        let grupoTutorado2 = row.get(49)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());

        // Asignar colores basados en materias
        let colorOriginal1 = calcular_color(&materiaTutorado1);
        let colorOriginal2 = calcular_color(&materiaTutorado2);
         
     
        println!("👤 Tutor: {} (Disponibilidad: {}), Materia: {}, Contacto: {}| Tutorado1: {} (ID: {}, Disponibilidad: {}), Materia: {}, Contacto:{}, Grupo{}, | Tutorado2: {} (ID: {}, Disponibilidad: {}), Materia: {}, contacto: {}, grupo{}",
            tutor, disponibilidadTutor, materiaTutor, contactoTutor,
            tutorado1, tutorado1_id, disponibilidadTutorado1, materiaTutorado1, contactoTutorado1, grupoTutorado1,
            tutorado2, tutorado2_id, disponibilidadTutorado2, materiaTutorado2, contactoTutorado2, grupoTutorado2
        );

        emparejamientos.push(EmparejamientoItem {
            tutor,
            disponibilidadTutor,
            materiaTutor,
            tutorado1,
            tutorado1_id,
            disponibilidadTutorado1,
            materiaTutorado1,
            tutorado2,
            tutorado2_id,
            disponibilidadTutorado2,
            materiaTutorado2,
            grupoTutorado1,
            grupoTutorado2,
            contactoTutor,
            contactoTutorado1,
            contactoTutorado2,
            colorOriginal1: Some(colorOriginal1),
            colorOriginal2: Some(colorOriginal2),
            modalidad,
            max_tutorados,
        });
    }

    

    println!("✅ Emparejamiento generado con {} elementos.", emparejamientos.len());
    Ok(emparejamientos)
}

#[tauri::command]
pub fn filtrar_emparejamientos(
    emparejamientos: Vec<EmparejamientoItem>,
    search_tutor: String,
    search_tutorado: String,
    search_tutorado_id: String,
    search_disponibilidad_tutor: String,
    search_disponibilidad_tutorado: String,
    sort_column: Option<String>,
    sort_direction: String,
) -> Vec<EmparejamientoItem> {
    let mut data = emparejamientos;
    
    // Filtrar por Tutor
    if !search_tutor.trim().is_empty() {
        let search_tutor_lower = search_tutor.to_lowercase();
        data.retain(|fila| fila.tutor.to_lowercase().contains(&search_tutor_lower));
    }
    
    // Filtrar por ID de Tutorado
    if !search_tutorado_id.trim().is_empty() {
        let search_id_lower = search_tutorado_id.to_lowercase();
        data.retain(|fila| 
            fila.tutorado1_id.to_lowercase().contains(&search_id_lower) || 
            fila.tutorado2_id.to_lowercase().contains(&search_id_lower)
        );
    }
    
    // Filtrar por nombre de Tutorado
    if !search_tutorado.trim().is_empty() {
        let search_tutorado_lower = search_tutorado.to_lowercase();
        data.retain(|fila| 
            fila.tutorado1.to_lowercase().contains(&search_tutorado_lower) || 
            fila.tutorado2.to_lowercase().contains(&search_tutorado_lower)
        );
    }
    
    // Filtrar por disponibilidad del Tutor
    if !search_disponibilidad_tutor.is_empty() {
        data.retain(|fila| fila.disponibilidadTutor == search_disponibilidad_tutor);
    }
    
    // Filtrar por disponibilidad de los Tutorados
    if !search_disponibilidad_tutorado.is_empty() {
        data.retain(|fila| 
            fila.disponibilidadTutorado1 == search_disponibilidad_tutorado || 
            fila.disponibilidadTutorado2 == search_disponibilidad_tutorado
        );
    }
    
    // Ordenar si hay una columna definida
    if let Some(column) = sort_column {
        data.sort_by(|a, b| {
            let is_asc = sort_direction == "asc";
            
            match column.as_str() {
                "tutor" => {
                    let a_val = a.tutor.to_lowercase();
                    let b_val = b.tutor.to_lowercase();
                    if is_asc { a_val.cmp(&b_val) } else { b_val.cmp(&a_val) }
                },
                "materiaTutor" => {
                    let a_val = a.materiaTutor.to_lowercase();
                    let b_val = b.materiaTutor.to_lowercase();
                    if is_asc { a_val.cmp(&b_val) } else { b_val.cmp(&a_val) }
                },
                "disponibilidadTutor" => {
                    let a_val = a.disponibilidadTutor.to_lowercase();
                    let b_val = b.disponibilidadTutor.to_lowercase();
                    if is_asc { a_val.cmp(&b_val) } else { b_val.cmp(&a_val) }
                },
                _ => std::cmp::Ordering::Equal,
            }
        });
    }
    
    data
}


#[tauri::command]
pub fn emparejamiento_automatico(emparejamientos: Vec<EmparejamientoItem>) -> Vec<EmparejamientoItem> {
    let mut nuevo_emparejamiento = emparejamientos.clone();
    let mut tutorados_pendientes: Vec<(String, String, String, String, String, i32)> = Vec::new();

    // --- Etapa 1: Sacar tutorados que no cumplen condiciones ---
    for fila in &mut nuevo_emparejamiento {
        // Solo procesar filas que tengan tutor y materia válidos
        if fila.tutor.trim().is_empty() || fila.materiaTutor == "VACÍO" {
            continue;
        }

        // Revisar tutorado1
        if !fila.tutorado1.trim().is_empty() && fila.tutorado1 != "VACÍO" {
            // Depuración
            println!("Comparando: '{}' con '{}' | '{}' con '{}'", 
                normalize(&fila.materiaTutorado1), normalize(&fila.materiaTutor),
                fila.disponibilidadTutorado1, fila.disponibilidadTutor);
                
            if normalize(&fila.materiaTutorado1) != normalize(&fila.materiaTutor) ||
               fila.disponibilidadTutorado1 != fila.disponibilidadTutor {
                
                println!("🛑 Tutorados incompatibles -> Tutorado1: {} (mat: {}, disp: {}) con Tutor (mat: {}, disp: {})",
                    fila.tutorado1, fila.materiaTutorado1, fila.disponibilidadTutorado1,
                    fila.materiaTutor, fila.disponibilidadTutor
                );

                tutorados_pendientes.push((
                    fila.tutorado1.clone(),
                    fila.materiaTutorado1.clone(),
                    fila.disponibilidadTutorado1.clone(),
                    calcular_color(&fila.materiaTutorado1),
                    fila.tutorado1_id.clone(),
                    1
                ));

                fila.tutorado1 = "".to_string();
                fila.tutorado1_id = "".to_string();
                fila.materiaTutorado1 = "VACÍO".to_string();
                fila.disponibilidadTutorado1 = "VACÍO".to_string();
                fila.colorOriginal1 = Some("".to_string());
            }
        }

        // Revisar tutorado2
        if !fila.tutorado2.trim().is_empty() && fila.tutorado2 != "VACÍO" {
            // Depuración
            println!("Comparando: '{}' con '{}' | '{}' con '{}'", 
                normalize(&fila.materiaTutorado2), normalize(&fila.materiaTutor),
                fila.disponibilidadTutorado2, fila.disponibilidadTutor);
                
            if normalize(&fila.materiaTutorado2) != normalize(&fila.materiaTutor) ||
               fila.disponibilidadTutorado2 != fila.disponibilidadTutor {
                
                println!("🛑 Tutorados incompatibles -> Tutorado2: {} (mat: {}, disp: {}) con Tutor (mat: {}, disp: {})",
                    fila.tutorado2, fila.materiaTutorado2, fila.disponibilidadTutorado2,
                    fila.materiaTutor, fila.disponibilidadTutor
                );

                tutorados_pendientes.push((
                    fila.tutorado2.clone(),
                    fila.materiaTutorado2.clone(),
                    fila.disponibilidadTutorado2.clone(),
                    calcular_color(&fila.materiaTutorado2),
                    fila.tutorado2_id.clone(),
                    2
                ));

                fila.tutorado2 = "".to_string();
                fila.tutorado2_id = "".to_string();
                fila.materiaTutorado2 = "VACÍO".to_string();
                fila.disponibilidadTutorado2 = "VACÍO".to_string();
                fila.colorOriginal2 = Some("".to_string());
            }
        }
    }
// --- Stage 1B: Asegurar que ningún tutor supere su max_tutorados ---
for fila in &mut nuevo_emparejamiento {
    let mut actuales = Vec::new();
    if !fila.tutorado1.trim().is_empty() && fila.tutorado1 != "VACÍO" {
        actuales.push((
            1,
            fila.tutorado1.clone(),
            fila.materiaTutorado1.clone(),
            fila.disponibilidadTutorado1.clone(),
            fila.colorOriginal1.clone().unwrap_or_default(),
            fila.tutorado1_id.clone(),
        ));
    }
    if !fila.tutorado2.trim().is_empty() && fila.tutorado2 != "VACÍO" {
        actuales.push((
            2,
            fila.tutorado2.clone(),
            fila.materiaTutorado2.clone(),
            fila.disponibilidadTutorado2.clone(),
            fila.colorOriginal2.clone().unwrap_or_default(),
            fila.tutorado2_id.clone(),
        ));
    }

    let to_remove = actuales.len().saturating_sub(fila.max_tutorados as usize);
    if to_remove > 0 {
        for (slot, nombre, materia, dispo, color, id) 
            in actuales.into_iter().rev().take(to_remove)
        {
            tutorados_pendientes.push((nombre, materia, dispo, color, id, slot as i32));
            match slot {
                1 => {
                    fila.tutorado1.clear();
                    fila.tutorado1_id.clear();
                    fila.materiaTutorado1 = "VACÍO".into();
                    fila.disponibilidadTutorado1 = "VACÍO".into();
                    fila.colorOriginal1 = Some("".into());
                }
                2 => {
                    fila.tutorado2.clear();
                    fila.tutorado2_id.clear();
                    fila.materiaTutorado2 = "VACÍO".into();
                    fila.disponibilidadTutorado2 = "VACÍO".into();
                    fila.colorOriginal2 = Some("".into());
                }
                _ => {}
            }
        }
    }
}


    // --- Etapa 2: Ordenar tutorados pendientes para mejorar asignación ---
    // Ordenamos primero por disponibilidad y luego por materia para agrupar casos similares
    tutorados_pendientes.sort_by(|a, b| {
        let disp_cmp = a.2.cmp(&b.2);
        if disp_cmp == std::cmp::Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            disp_cmp
        }
    });

    // Filtrar tutorados vacíos
    tutorados_pendientes.retain(|(nombre, _, _, _, _, _)| !nombre.trim().is_empty() && nombre != "VACÍO");

   // --- Etapa 3: Reubicar los tutorados pendientes ---
let mut asignados: HashSet<String> = HashSet::new();

for (nombre, materia, disponibilidad, color, id, _) in &tutorados_pendientes {
    for fila in &mut nuevo_emparejamiento {
        if !fila.tutor.trim().is_empty()
            && normalize(&fila.materiaTutor) == normalize(materia)
            && fila.disponibilidadTutor == *disponibilidad
        {
            let actuales = [
                !fila.tutorado1.trim().is_empty() && fila.tutorado1 != "VACÍO",
                !fila.tutorado2.trim().is_empty() && fila.tutorado2 != "VACÍO",
            ];
            let count = actuales.iter().filter(|&&b| b).count();
            if count < fila.max_tutorados as usize {
                // intentamos slot 1
                if fila.tutorado1.trim().is_empty() || fila.tutorado1 == "VACÍO" {
                    fila.tutorado1 = nombre.clone();
                    fila.tutorado1_id = id.clone();
                    fila.materiaTutorado1 = materia.clone();
                    fila.disponibilidadTutorado1 = disponibilidad.clone();
                    fila.colorOriginal1 = Some(color.clone());
                } else {
                    // slot 2
                    fila.tutorado2 = nombre.clone();
                    fila.tutorado2_id = id.clone();
                    fila.materiaTutorado2 = materia.clone();
                    fila.disponibilidadTutorado2 = disponibilidad.clone();
                    fila.colorOriginal2 = Some(color.clone());
                }
                asignados.insert(id.clone());
                break;
            }
        }
    }
}

// --- Solo los no asignados generan fila vacía ---
for (nombre, materia, disponibilidad, color, id, _) in tutorados_pendientes {
    if asignados.contains(&id) {
        continue;
    }
    let fila_vacia = EmparejamientoItem {
        tutor: "".into(),
        disponibilidadTutor: "VACÍO".into(),
        materiaTutor: "VACÍO".into(),
        modalidad: "VACÍO".into(),
        max_tutorados: 2,
        tutorado1: nombre.clone(),
        tutorado1_id: id.clone(),
        disponibilidadTutorado1: disponibilidad.clone(),
        materiaTutorado1: materia.clone(),
        colorOriginal1: Some(color.clone()),
        tutorado2: "".into(),
        tutorado2_id: "".into(),
        disponibilidadTutorado2: "VACÍO".into(),
        materiaTutorado2: "VACÍO".into(),
        colorOriginal2: Some("".into()),
        grupoTutorado1: "VACÍO".into(),
        grupoTutorado2: "VACÍO".into(),
        contactoTutor: "VACÍO".into(),
        contactoTutorado1: "VACÍO".into(),
        contactoTutorado2: "VACÍO".into(),
    };
    nuevo_emparejamiento.push(fila_vacia);
}


    // Eliminar elementos que no tienen tutores ni tutorados
    nuevo_emparejamiento.retain(|fila| {
        !fila.tutor.trim().is_empty() || 
        (!fila.tutorado1.trim().is_empty() && fila.tutorado1 != "VACÍO") || 
        (!fila.tutorado2.trim().is_empty() && fila.tutorado2 != "VACÍO")
    });

    println!("📦 Emparejamiento final contiene {} filas", nuevo_emparejamiento.len());
    nuevo_emparejamiento
}


#[tauri::command]
pub fn actualizar_campo_tutor(
    emparejamientos: Vec<EmparejamientoItem>, 
    index: usize,
    campo: String, 
    valor: String
) -> Result<Vec<EmparejamientoItem>, String> {
    let mut nuevos_emparejamientos = emparejamientos;
    
    if index >= nuevos_emparejamientos.len() {
        return Err("Índice fuera de rango".to_string());
    }
    
    match campo.as_str() {
        "materiaTutor" => nuevos_emparejamientos[index].materiaTutor = valor,
        "disponibilidadTutor" => nuevos_emparejamientos[index].disponibilidadTutor = valor,
        _ => return Err(format!("Campo no reconocido: {}", campo)),
    }
    
    Ok(nuevos_emparejamientos)
}