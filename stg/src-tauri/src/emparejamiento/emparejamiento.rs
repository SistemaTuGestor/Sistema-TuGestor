
use calamine::{open_workbook, Reader, Xlsx, DataType};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::path::Path;



#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct EmparejamientoItem {
    // tutor
    pub nombretutor: String,
    pub apellidotutor: String,
    pub correotutor: String,
    pub telefonotutor: String,
    pub instituciontutor: String,
    pub becariotutor: String,
    pub materiatutor: String,
    pub horastutor: String,
    pub modalidad: String,
    pub disponibilidadtutor: String,
    pub max_tutorados: u8,
    pub argostutor: String,
    pub descripcion_de_la_modalidad: String,
    //tutorado1
    pub tutorado1: String,
    pub tutorado1_id: String,
    pub colegiotutorado1: String,
    pub tele1_tutorado1: String,
    pub tele2_tutorado1: String,
    pub contactoTutorado1: String,
    pub materiatutorado1: String,
    pub vocabulariotutorado1: String,
    pub gramaticatutorado1: String,
    pub escuchatutorado1: String,
    pub lecturatutorado1: String,
    pub pensamientonumericotutorado1: String,
    pub pensamientoespacialtutorado1: String,
    pub pensamientoomtricotutorado1: String,
    pub pensamientoaleatoriotutorado1: String,
    pub pensamientovariacionalysistertudorado1: String,
    pub totalpuntuacionmathpretutorado1: String,
    pub totalpuntuacionenglishpretutorado1: String,
    pub disponibilidadtutorado1: String,
    pub grupo_tutorado1: String,
    pub colorOriginal1: Option<String>,
    //tutorado2
    pub tutorado2: String,
    pub tutorado2_id: String,
    pub colegiotutorado2: String,
    pub tele1_tutorado2: String,
    pub tele2_tutorado2: String,
    pub contactoTutorado2: String,
    pub materiatutorado2: String,
    pub vocabulariotutorado2: String,
    pub gramaticatutorado2: String,
    pub escuchatutorado2: String,
    pub lecturatutorado2: String,
    pub pensamientonumericotutorado2: String,
    pub pensamientoespacialtutorado2: String,
    pub pensamientoomtricotutorado2: String,
    pub pensamientoaleatoriotutorado2: String,
    pub pensamientovariacionalysistertudorado2: String,
    pub totalpuntuacionmathpretutorado2: String,
    pub totalpuntuacionenglishpretutorado2: String,
    pub disponibilidadtutorado2: String,
    pub grupo_tutorado2: String,
    pub colorOriginal2: Option<String>,
    
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
pub fn obtener_emparejamiento(ruta: String) -> Result<Vec<EmparejamientoItem>, String> {
     println!("📁 Buscando en ruta: {}",ruta);
     println!("✅ Existe fichero? {}", Path::new(&ruta).exists());
     println!("📂 WD actual: {:?}", std::env::current_dir().unwrap()); 
    let mut workbook: Xlsx<_> = open_workbook(&ruta)
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
        let nombretutor = row.get(0)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let apellidotutor = row.get(1)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let correotutor = row.get(2)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let telefonotutor = row.get(3)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let instituciontutor = row.get(4)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let becariotutor = row.get(5)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materiatutor = row.get(6)
            .and_then(|c| c.as_string())
            .map(|s| normalizeTutor(&s))
            .unwrap_or_else(|| "VACÍO".to_string());
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
        let horastutor = row.get(8)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let disponibilidadtutor = row.get(9)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let descripcion_de_la_modalidad = row.get(50)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let argostutor = row.get(51)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
    
        
        // Datos del primer tutorado
        let tutorado1 = row.get(10)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let tutorado1_id = row.get(11)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let colegiotutorado1 = row.get(12).expect("REASON").as_string()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let tele1_tutorado1 = row.get(13)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let tele2_tutorado1 = row.get(14)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let contactoTutorado1 =row.get(15)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materiatutorado1 = row.get(16)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let vocabulariotutorado1 = row.get(17)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        println!("Vocabulario: {:?}", vocabulariotutorado1);
        let gramaticatutorado1 = row.get(18)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let escuchatutorado1 = row.get(19)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let lecturatutorado1 = row.get(20)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let pensamientonumericotutorado1 = row.get(21)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let pensamientoespacialtutorado1 = row.get(22)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let pensamientoomtricotutorado1 = row.get(23)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let pensamientoaleatoriotutorado1 = row.get(24)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let pensamientovariacionalysistertudorado1 = row.get(25)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let totalpuntuacionmathpretutorado1 = row.get(26)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let totalpuntuacionenglishpretutorado1 = row.get(27)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let disponibilidadtutorado1 = row.get(28)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let grupo_tutorado1 = row.get(29)
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
        let colegiotutorado2 = row.get(32)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let tele1_tutorado2 = row.get(33)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let tele2_tutorado2 = row.get(34)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let contactoTutorado2 = row.get(35)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let materiatutorado2 = row.get(36)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let vocabulariotutorado2 = row.get(37)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let gramaticatutorado2 = row.get(38)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let escuchatutorado2 = row.get(39)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let lecturatutorado2 = row.get(40)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let pensamientonumericotutorado2 = row.get(41)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let pensamientoespacialtutorado2 = row.get(42)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let pensamientoomtricotutorado2 = row.get(43)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let pensamientoaleatoriotutorado2 = row.get(44)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let pensamientovariacionalysistertudorado2 = row.get(45)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let totalpuntuacionmathpretutorado2 = row.get(46)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let totalpuntuacionenglishpretutorado2 = row.get(47)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let disponibilidadtutorado2 = row.get(48)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());
        let grupo_tutorado2 = row.get(49)
            .and_then(|c| c.as_string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "VACÍO".to_string());

        // Asignar colores basados en materias
        let colorOriginal1 = calcular_color(&materiatutorado1);
        let colorOriginal2 = calcular_color(&materiatutorado2);
         
     
        println!("👤 Tutor: {} (Disponibilidad: {}), Materia: {}, Contacto: {}| Tutorado1: {} (ID: {}, Disponibilidad: {}), Materia: {}, Contacto:{}, Grupo{}, | Tutorado2: {} (ID: {}, Disponibilidad: {}), Materia: {}, contacto: {}, grupo{}",
            nombretutor, disponibilidadtutor, materiatutor, correotutor,
            tutorado1, tutorado1_id, disponibilidadtutorado1, materiatutorado1, contactoTutorado1, grupo_tutorado1,
            tutorado2, tutorado2_id, disponibilidadtutorado2, materiatutorado2, contactoTutorado2, grupo_tutorado2
        );

        emparejamientos.push(EmparejamientoItem {
        //datos tutor
        nombretutor,
        apellidotutor,
        correotutor,
        telefonotutor,
        instituciontutor,
        becariotutor,
        materiatutor,
        modalidad,
        max_tutorados,
        horastutor,
        disponibilidadtutor,
        argostutor,
        descripcion_de_la_modalidad,
        //datos tutorado1
        tutorado1,
        tutorado1_id,
        colegiotutorado1,
        tele1_tutorado1,
        tele2_tutorado1,
        contactoTutorado1,
        materiatutorado1,
        vocabulariotutorado1,
        gramaticatutorado1,
        escuchatutorado1,
        lecturatutorado1,
        pensamientonumericotutorado1,
        pensamientoespacialtutorado1,
        pensamientoomtricotutorado1,
        pensamientoaleatoriotutorado1,
        pensamientovariacionalysistertudorado1,
        totalpuntuacionmathpretutorado1,
        totalpuntuacionenglishpretutorado1,
        disponibilidadtutorado1,
        grupo_tutorado1,
        colorOriginal1: Some(colorOriginal1),
        //datos tutorado2
        tutorado2,
        tutorado2_id,
        colegiotutorado2,
        tele1_tutorado2,
        tele2_tutorado2,
        contactoTutorado2,
        materiatutorado2,
        vocabulariotutorado2,
        gramaticatutorado2,
        escuchatutorado2,
        lecturatutorado2,
        pensamientonumericotutorado2,
        pensamientoespacialtutorado2,
        pensamientoomtricotutorado2,
        pensamientoaleatoriotutorado2,
        pensamientovariacionalysistertudorado2,
        totalpuntuacionmathpretutorado2,
        totalpuntuacionenglishpretutorado2,
        disponibilidadtutorado2,
        grupo_tutorado2,
        colorOriginal2: Some(colorOriginal2),
        });
    }

    

    println!("✅ Emparejamiento generado con {} elementos.", emparejamientos.len());
    Ok(emparejamientos)
}

#[tauri::command]
pub fn filtrar_emparejamientos(
    emparejamientos: Vec<EmparejamientoItem>,
    searchtutor: String,
    searchtutorado: String,
    searchtutorado_id: String,
    searchdisponibilidad_tutor: String,
    searchdisponibilidad_tutorado: String,
    sort_column: Option<String>,
    sort_direction: String,
) -> Vec<EmparejamientoItem> {
    let mut data = emparejamientos;
    
    // Filtrar por Tutor (nombre + apellido)
if !searchtutor.trim().is_empty() {
    let searchtutor_lower = searchtutor.to_lowercase();
    data.retain(|fila| {
        let nombre_completo = format!("{} {}", fila.nombretutor, fila.apellidotutor).to_lowercase();
        nombre_completo.contains(&searchtutor_lower)
    });
}
    // Filtrar por ID de Tutorado
    if !searchtutorado_id.trim().is_empty() {
        let searchid_lower = searchtutorado_id.to_lowercase();
        data.retain(|fila| 
            fila.tutorado1_id.to_lowercase().contains(&searchid_lower) || 
            fila.tutorado2_id.to_lowercase().contains(&searchid_lower)
        );
    }
    
    // Filtrar por nombre de Tutorado
    if !searchtutorado.trim().is_empty() {
        let searchtutorado_lower = searchtutorado.to_lowercase();
        data.retain(|fila| 
            fila.tutorado1.to_lowercase().contains(&searchtutorado_lower) || 
            fila.tutorado2.to_lowercase().contains(&searchtutorado_lower)
        );
    }
    
    // Filtrar por disponibilidad del Tutor
    if !searchdisponibilidad_tutor.is_empty() {
        data.retain(|fila| fila.disponibilidadtutor == searchdisponibilidad_tutor);
    }
    
    // Filtrar por disponibilidad de los Tutorados
    if !searchdisponibilidad_tutorado.is_empty() {
        data.retain(|fila| 
            fila.disponibilidadtutorado1 == searchdisponibilidad_tutorado || 
            fila.disponibilidadtutorado2 == searchdisponibilidad_tutorado
        );
    }
    
    // Ordenar si hay una columna definida
    if let Some(column) = sort_column {
        data.sort_by(|a, b| {
            let is_asc = sort_direction == "asc";
            
            match column.as_str() {
                "tutor" => {
                    let a_val = a.nombretutor.to_lowercase();
                    let b_val = b.nombretutor.to_lowercase();
                    if is_asc { a_val.cmp(&b_val) } else { b_val.cmp(&a_val) }
                },
                "materiaTutor" => {
                    let a_val = a.materiatutor.to_lowercase();
                    let b_val = b.materiatutor.to_lowercase();
                    if is_asc { a_val.cmp(&b_val) } else { b_val.cmp(&a_val) }
                },
                "disponibilidadTutor" => {
                    let a_val = a.disponibilidadtutor.to_lowercase();
                    let b_val = b.disponibilidadtutor.to_lowercase();
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
    let mut tutorados_pendientes: Vec<(EmparejamientoItem, u8)> = vec![];

    // --- Etapa 1: Sacar tutorados que no cumplen condiciones ---
    for fila in &mut nuevo_emparejamiento {
        // Solo procesar filas que tengan tutor y materia válidos
        if fila.nombretutor.trim().is_empty() || fila.materiatutor == "VACÍO" {
            continue;
        }

        // Revisar tutorado1
        if !fila.tutorado1.trim().is_empty() && fila.tutorado1 != "VACÍO" {
            // Depuración
            println!("Comparando: '{}' con '{}' | '{}' con '{}'", 
                normalize(&fila.materiatutorado1), normalize(&fila.materiatutor),
                fila.disponibilidadtutorado1, fila.disponibilidadtutor);
                
            if normalize(&fila.materiatutorado1) != normalize(&fila.materiatutor) ||
               fila.disponibilidadtutorado1 != fila.disponibilidadtutor {
                
                println!("🛑 Tutorados incompatibles -> Tutorado1: {} (mat: {}, disp: {}) con Tutor (mat: {}, disp: {})",
                    fila.tutorado1, fila.materiatutorado1, fila.disponibilidadtutorado1,
                    fila.materiatutor, fila.disponibilidadtutor
                );

                 let mut tutorado = EmparejamientoItem::default();
                 copiar_datos_tutorado(&fila, &mut tutorado, 1);
                 tutorados_pendientes.push((tutorado, 1));


                fila.tutorado1 = "".to_string();
                fila.tutorado1_id = "".to_string();
                fila.colegiotutorado1 = "".to_string();
                fila.tele1_tutorado1 = "".to_string();
                fila.tele2_tutorado1 = "".to_string();
                fila.contactoTutorado1 = "".to_string();
                fila.materiatutorado1 = "VACÍO".to_string();
                fila.vocabulariotutorado1 = "".to_string();
                fila.gramaticatutorado1 = "".to_string();
                fila.escuchatutorado1 = "".to_string();
                fila.lecturatutorado1 = "".to_string();
                fila.pensamientonumericotutorado1 = "".to_string();
                fila.pensamientoespacialtutorado1 = "".to_string();
                fila.pensamientoomtricotutorado1 = "".to_string();
                fila.pensamientoaleatoriotutorado1 = "".to_string();
                fila.pensamientovariacionalysistertudorado1 = "".to_string();
                fila.totalpuntuacionmathpretutorado1 = "".to_string();
                fila.totalpuntuacionenglishpretutorado1 = "".to_string();
                fila.disponibilidadtutorado1 = "VACÍO".to_string();
                fila.grupo_tutorado1 = "".to_string();
                fila.colorOriginal1 = Some("".to_string());
            }
        }

        // Revisar tutorado2
        if !fila.tutorado2.trim().is_empty() && fila.tutorado2 != "VACÍO" {
            // Depuración
            println!("Comparando: '{}' con '{}' | '{}' con '{}'", 
                normalize(&fila.materiatutorado2), normalize(&fila.materiatutor),
                fila.disponibilidadtutorado2, fila.disponibilidadtutor);
                
            if normalize(&fila.materiatutorado2) != normalize(&fila.materiatutor) ||
               fila.disponibilidadtutorado2 != fila.disponibilidadtutor {
                
                println!("🛑 Tutorados incompatibles -> Tutorado2: {} (mat: {}, disp: {}) con Tutor (mat: {}, disp: {})",
                    fila.tutorado2, fila.materiatutorado2, fila.disponibilidadtutorado2,
                    fila.materiatutor, fila.disponibilidadtutor
                );

                 let mut tutorado = EmparejamientoItem::default();
                 copiar_datos_tutorado(&fila, &mut tutorado, 2);
                 tutorados_pendientes.push((tutorado, 2));


                fila.tutorado2 = "".to_string();
                fila.tutorado2_id = "".to_string();
                fila.colegiotutorado2 = "".to_string();
                fila.tele1_tutorado2 = "".to_string();
                fila.tele2_tutorado2 = "".to_string();
                fila.contactoTutorado2 = "".to_string();
                fila.materiatutorado2 = "VACÍO".to_string();
                fila.vocabulariotutorado2 = "".to_string();
                fila.gramaticatutorado2 = "".to_string();
                fila.escuchatutorado2 = "".to_string();
                fila.lecturatutorado2 = "".to_string();
                fila.pensamientonumericotutorado2 = "".to_string();
                fila.pensamientoespacialtutorado2 = "".to_string();
                fila.pensamientoomtricotutorado2 = "".to_string();
                fila.pensamientoaleatoriotutorado2 = "".to_string();
                fila.pensamientovariacionalysistertudorado2 = "".to_string();
                fila.totalpuntuacionmathpretutorado2 = "".to_string();
                fila.totalpuntuacionenglishpretutorado2 = "".to_string();
                fila.disponibilidadtutorado2 = "VACÍO".to_string();
                fila.grupo_tutorado2 = "".to_string();
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
                fila.materiatutorado1.clone(),
                fila.disponibilidadtutorado1.clone(),
                fila.colorOriginal1.clone().unwrap_or_default(),
                fila.tutorado1_id.clone(),
            ));
        }
        if !fila.tutorado2.trim().is_empty() && fila.tutorado2 != "VACÍO" {
            actuales.push((
                2,
                fila.tutorado2.clone(),
                fila.materiatutorado2.clone(),
                fila.disponibilidadtutorado2.clone(),
                fila.colorOriginal2.clone().unwrap_or_default(),
                fila.tutorado2_id.clone(),
            ));
        }

        let to_remove = actuales.len().saturating_sub(fila.max_tutorados as usize);
        if to_remove > 0 {
            for (slot, ..) in actuales.into_iter().rev().take(to_remove)
            {
            let mut pendiente = EmparejamientoItem::default();
            copiar_datos_tutorado(&fila, &mut pendiente, slot);
            tutorados_pendientes.push((pendiente, slot));            
            match slot {
                    1 => {
                        fila.tutorado1.clear();
                        fila.tutorado1_id.clear();
                        fila.materiatutorado1 = "VACÍO".into();
                        fila.disponibilidadtutorado1 = "VACÍO".into();
                        fila.colorOriginal1 = Some("".into());
                    }
                    2 => {
                        fila.tutorado2.clear();
                        fila.tutorado2_id.clear();
                        fila.materiatutorado2 = "VACÍO".into();
                        fila.disponibilidadtutorado2 = "VACÍO".into();
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
        let disp_cmp = a.0.disponibilidadtutorado1.cmp(&b.0.disponibilidadtutorado1);
        if disp_cmp == std::cmp::Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            disp_cmp
        }
    });

    // Filtrar tutorados vacíos
    tutorados_pendientes.retain(|(t, _)| !t.tutorado1.trim().is_empty() && t.tutorado1 != "VACÍO");

   // --- Etapa 3: Reubicar los tutorados pendientes ---
    let mut asignados: HashSet<String> = HashSet::new();

    for (tutorado_origen, _) in &tutorados_pendientes {
        for fila in &mut nuevo_emparejamiento {
            if !fila.nombretutor.trim().is_empty()
                && normalize(&fila.materiatutor) == normalize(&tutorado_origen.materiatutorado1)
                && fila.disponibilidadtutor == tutorado_origen.disponibilidadtutorado1
            {
                let actuales = [
                    !fila.tutorado1.trim().is_empty() && fila.tutorado1 != "VACÍO",
                    !fila.tutorado2.trim().is_empty() && fila.tutorado2 != "VACÍO",
                ];
                let count = actuales.iter().filter(|&&b| b).count();
                if count < fila.max_tutorados as usize {
                    if fila.tutorado1.trim().is_empty() || fila.tutorado1 == "VACÍO" {
                        copiar_datos_tutorado(tutorado_origen, fila, 1);
                    } else {
                        copiar_datos_tutorado(tutorado_origen, fila, 2);
                    }
                    asignados.insert(tutorado_origen.tutorado1_id.clone());
                    break;
                }
            }
        }
    }


    // --- Solo los no asignados generan fila vacía ---
    for (tutorado_origen, _) in tutorados_pendientes {
        if asignados.contains(&tutorado_origen.tutorado1_id) {
            continue;
        }

        let mut fila_vacia = EmparejamientoItem {
            nombretutor: "".into(),
            apellidotutor: "".into(),
            correotutor: "".into(),
            telefonotutor: "".into(),
            instituciontutor: "".into(),
            becariotutor: "".into(),
            argostutor: "".into(),
            descripcion_de_la_modalidad: "".into(),
            horastutor: "VACÍO".into(),
            disponibilidadtutor: "VACÍO".into(),
            materiatutor: "VACÍO".into(),
            modalidad: "VACÍO".into(),
            max_tutorados: 2,
            

            tutorado1: "VACÍO".into(),
            tutorado1_id: "VACÍO".into(),
            colegiotutorado1: "VACÍO".into(),
            tele1_tutorado1: "VACÍO".into(),
            tele2_tutorado1: "VACÍO".into(),
            contactoTutorado1: "VACÍO".into(),
            vocabulariotutorado1: "VACÍO".into(),
            gramaticatutorado1: "VACÍO".into(),
            escuchatutorado1: "VACÍO".into(),
            lecturatutorado1: "VACÍO".into(),
            pensamientonumericotutorado1: "VACÍO".into(),
            pensamientoespacialtutorado1: "VACÍO".into(),
            pensamientoomtricotutorado1: "VACÍO".into(),
            pensamientoaleatoriotutorado1: "VACÍO".into(),
            pensamientovariacionalysistertudorado1: "VACÍO".into(),
            totalpuntuacionmathpretutorado1: "VACÍO".into(),
            totalpuntuacionenglishpretutorado1: "VACÍO".into(),       
            materiatutorado1: "VACÍO".into(),
            disponibilidadtutorado1: "VACÍO".into(),
            grupo_tutorado1: "VACÍO".into(),
            colorOriginal1: Some("".into()),

            tutorado2: "VACÍO".into(),
            tutorado2_id: "VACÍO".into(),
            colegiotutorado2: "VACÍO".into(),
            tele1_tutorado2: "VACÍO".into(),
            tele2_tutorado2: "VACÍO".into(),
            contactoTutorado2: "VACÍO".into(),
            vocabulariotutorado2: "VACÍO".into(),
            gramaticatutorado2: "VACÍO".into(),
            escuchatutorado2: "VACÍO".into(),
            lecturatutorado2: "VACÍO".into(),
            pensamientonumericotutorado2: "VACÍO".into(),
            pensamientoespacialtutorado2: "VACÍO".into(),
            pensamientoomtricotutorado2: "VACÍO".into(),
            pensamientoaleatoriotutorado2: "VACÍO".into(),
            pensamientovariacionalysistertudorado2: "VACÍO".into(),
            totalpuntuacionmathpretutorado2: "VACÍO".into(),
            totalpuntuacionenglishpretutorado2: "VACÍO".into(),
            materiatutorado2: "VACÍO".into(),
            disponibilidadtutorado2: "VACÍO".into(),
            grupo_tutorado2: "VACÍO".into(),
            colorOriginal2: Some("".into()),

            ..Default::default()


        };

        copiar_datos_tutorado(&tutorado_origen, &mut fila_vacia, 1);
        nuevo_emparejamiento.push(fila_vacia);
    }



    // Eliminar elementos que no tienen tutores ni tutorados
    nuevo_emparejamiento.retain(|fila| {
        !fila.nombretutor.trim().is_empty() || 
        (!fila.tutorado1.trim().is_empty() && fila.tutorado1 != "VACÍO") || 
        (!fila.tutorado2.trim().is_empty() && fila.tutorado2 != "VACÍO")
    });

    println!("📦 Emparejamiento final contiene {} filas", nuevo_emparejamiento.len());
    nuevo_emparejamiento
}

fn copiar_datos_tutorado(origen: &EmparejamientoItem, destino: &mut EmparejamientoItem, slot: u8) {
    match slot {
        1 => {
            destino.tutorado1 = origen.tutorado1.clone();
            destino.tutorado1_id = origen.tutorado1_id.clone();
            destino.colegiotutorado1 = origen.colegiotutorado1.clone();
            destino.tele1_tutorado1 = origen.tele1_tutorado1.clone();
            destino.tele2_tutorado1 = origen.tele2_tutorado1.clone();
            destino.contactoTutorado1 = origen.contactoTutorado1.clone();
            destino.materiatutorado1 = origen.materiatutorado1.clone();
            destino.vocabulariotutorado1 = origen.vocabulariotutorado1.clone();
            destino.gramaticatutorado1 = origen.gramaticatutorado1.clone();
            destino.escuchatutorado1 = origen.escuchatutorado1.clone();
            destino.lecturatutorado1 = origen.lecturatutorado1.clone();
            destino.pensamientonumericotutorado1 = origen.pensamientonumericotutorado1.clone();
            destino.pensamientoespacialtutorado1 = origen.pensamientoespacialtutorado1.clone();
            destino.pensamientoomtricotutorado1 = origen.pensamientoomtricotutorado1.clone();
            destino.pensamientoaleatoriotutorado1 = origen.pensamientoaleatoriotutorado1.clone();
            destino.pensamientovariacionalysistertudorado1 = origen.pensamientovariacionalysistertudorado1.clone();
            destino.totalpuntuacionmathpretutorado1 = origen.totalpuntuacionmathpretutorado1.clone();
            destino.totalpuntuacionenglishpretutorado1 = origen.totalpuntuacionenglishpretutorado1.clone();
            destino.disponibilidadtutorado1 = origen.disponibilidadtutorado1.clone();
            destino.grupo_tutorado1 = origen.grupo_tutorado1.clone();
            destino.colorOriginal1 = origen.colorOriginal1.clone();
        }
        2 => {
            destino.tutorado2 = origen.tutorado2.clone();
            destino.tutorado2_id = origen.tutorado2_id.clone();
            destino.colegiotutorado2 = origen.colegiotutorado2.clone();
            destino.tele1_tutorado2 = origen.tele1_tutorado2.clone();
            destino.tele2_tutorado2 = origen.tele2_tutorado2.clone();
            destino.contactoTutorado2 = origen.contactoTutorado2.clone();
            destino.materiatutorado2 = origen.materiatutorado2.clone();
            destino.vocabulariotutorado2 = origen.vocabulariotutorado2.clone();
            destino.gramaticatutorado2 = origen.gramaticatutorado2.clone();
            destino.escuchatutorado2 = origen.escuchatutorado2.clone();
            destino.lecturatutorado2 = origen.lecturatutorado2.clone();
            destino.pensamientonumericotutorado2 = origen.pensamientonumericotutorado2.clone();
            destino.pensamientoespacialtutorado2 = origen.pensamientoespacialtutorado2.clone();
            destino.pensamientoomtricotutorado2 = origen.pensamientoomtricotutorado2.clone();
            destino.pensamientoaleatoriotutorado2 = origen.pensamientoaleatoriotutorado2.clone();
            destino.pensamientovariacionalysistertudorado2 = origen.pensamientovariacionalysistertudorado2.clone();
            destino.totalpuntuacionmathpretutorado2 = origen.totalpuntuacionmathpretutorado2.clone();
            destino.totalpuntuacionenglishpretutorado2 = origen.totalpuntuacionenglishpretutorado2.clone();
            destino.disponibilidadtutorado2 = origen.disponibilidadtutorado2.clone();
            destino.grupo_tutorado2 = origen.grupo_tutorado2.clone();
            destino.colorOriginal2 = origen.colorOriginal2.clone();
        }
        _ => {
            eprintln!("❌ Slot inválido: {}", slot);
        }
    }
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
        "materiaTutor" => nuevos_emparejamientos[index].materiatutor = valor,
        "disponibilidadTutor" => nuevos_emparejamientos[index].disponibilidadtutor = valor,
        _ => return Err(format!("Campo no reconocido: {}", campo)),
    }
    
    Ok(nuevos_emparejamientos)
}