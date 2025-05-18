import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import * as XLSX from "xlsx";
import { save } from "@tauri-apps/api/dialog";
import { open } from '@tauri-apps/api/dialog';
import { writeBinaryFile } from "@tauri-apps/api/fs";
import {
  DragDropContext,
  Droppable,
  Draggable,
  DropResult,
  DragUpdate,
  DragStart,
} from "@hello-pangea/dnd";
import "./Emparejamiento.css";

// Tipo de datos recibido del backend
type EmparejamientoEntry = {
  //datos de tutor
  // tutor
   nombretutor: string,
   apellidotutor: string,   
   correotutor: string,
   telefonotutor: string,
   instituciontutor: string,
   becariotutor: string,
   materiaTutor: string,
   horastutor: string,
   modalidad: string,
   disponibilidadTutor: string,
   max_tutorados: string,
   argostutor: string,
   descripcion_DE_LA_MODALIDAD: string,
   //datos de tutorado
    tutorado1: string,
    tutorado1_id: string,
    colegiotutorado1: string,
    tele1Tutorado1: string,
    tele2Tutorado1: string,
    contactoTutorado1: string,
    materiaTutorado1: string,
    vocabulariotutorado1: string,
    gramaticatutorado1: string,
    escuchatutorado1: string,
    lecturatutorado1: string,
    pensamientonumericotutorado1: string,
    pensamientoespacialtutorado1: string,
    pensamientoomtricotutorado1: string,
    pensamientoaleatoriotutorado1: string,
    pensamientovariacionalysistertudorado1: string,
    totalpuntuacionmathpretutorado1: string,
    totalpuntuacionenglishpretutorado1: string,
    disponibilidadTutorado1: string,
    grupoTutorado1: string,
   colorOriginal1?: string;
  // datos de tutorado 2
  tutorado2: string,
     tutorado2_id: string,
     colegiotutorado2: string,
     tele1Tutorado2: string,
     tele2Tutorado2: string,
     contactoTutorado2: string,
     materiaTutorado2: string,
     vocabulariotutorado2: string,
     gramaticatutorado2: string,
     escuchatutorado2: string,
     lecturatutorado2: string,
     pensamientonumericotutorado2: string,
     pensamientoespacialtutorado2: string,
     pensamientoomtricotutorado2: string,
     pensamientoaleatoriotutorado2: string,
     pensamientovariacionalysistertudorado2: string,
     totalpuntuacionmathpretutorado2: string,
     totalpuntuacionenglishpretutorado2: string,
     disponibilidadTutorado2: string,
     grupoTutorado2: string,
     colorOriginal2?: string;
 
};

const disponibilidadOptions = [
  "Entre semana por la mañana",
  "Entre semana de 2:00pm-3:00pm",
  "Entre semana de 3:00pm-4:00pm",
  "Entre semana de 4:00pm-5:00pm",
  "Entre semana de 5:00pm-6:00pm",
  "Entre semana de 6:00pm-8:00pm",
  "Sábados en la mañana",
  "Sábados en la tarde",
];

function Emparejamiento() {
  const [allData, setAllData] = useState<EmparejamientoEntry[]>(() => {
    const saved = localStorage.getItem("emparejamientoData");
    return saved ? JSON.parse(saved) : [];
  });
  const [filtered, setFiltered] = useState<EmparejamientoEntry[]>([]);
  const [draggingId, setDraggingId] = useState<string | null>(null);
  const [highlightedId, setHighlightedId] = useState<string | null>(null);
  const [invalidDropId, setInvalidDropId] = useState<string | null>(null);

  // Estados de búsqueda y ordenamiento
  const [searchTutor, setSearchTutor] = useState("");
  const [searchTutorado, setSearchTutorado] = useState("");
  const [searchTutoradoId, setSearchTutoradoId] = useState("");
  const [searchDisponibilidadTutor, setSearchDisponibilidadTutor] = useState("");
  const [searchDisponibilidadTutorado, setSearchDisponibilidadTutorado] = useState("");
  const [sortColumn, setSortColumn] = useState<"tutor" | "materiaTutor" | "disponibilidadTutor" | null>(null);
  const [sortDirection, setSortDirection] = useState<"asc" | "desc">("asc");

  // Carga inicial de emparejamientos
  // Sólo escribe en localStorage cuando allData cambia
useEffect(() => {
  localStorage.setItem("emparejamientoData", JSON.stringify(allData));
}, [allData]);

// Y al montar, restaura, pero **luego** recarga siempre del backend
useEffect(() => {
  const saved = localStorage.getItem("emparejamientoData");
  if (saved) setAllData(JSON.parse(saved));
  invoke<EmparejamientoEntry[]>("obtener_emparejamiento")
    .then((data) => setAllData(data))
    .catch(console.error);
}, []);


  // Función para seleccionar archivo Excel
  const seleccionarArchivo = async () => {
    const selected = await open({
      filters: [{ name: 'Excel', extensions: ['xlsx'] }],
      multiple: false,
    });
  
    if (typeof selected === 'string') {
      console.log("📂 Archivo seleccionado:", selected);
      const datos = await invoke<EmparejamientoEntry[]>("obtener_emparejamiento", { ruta: selected });
      if (datos) {
        setAllData(datos);
      }
    }
  };

  // Filtrar y ordenar usando backend
  useEffect(() => {
    async function applyFilter() {
      const args = {
        emparejamientos:               allData,
        searchtutor:                   searchTutor,
        searchtutorado:                searchTutorado,
        searchtutoradoId:              searchTutoradoId,
        searchdisponibilidadTutor:     searchDisponibilidadTutor,
        searchdisponibilidadTutorado:  searchDisponibilidadTutorado,
        sortColumn:                   sortColumn,
        sortDirection:                sortDirection,
      };
  
      console.log("Invocando filtro con", args);
  
      try {
        const result = await invoke<EmparejamientoEntry[]>("filtrar_emparejamientos", args);
        setFiltered(result);
      } catch (e) {
        console.error("❌ Error al invocar filtro:", e);
      }
    }
  
    applyFilter();
  }, [
    allData,
    searchTutor,
    searchTutorado,
    searchTutoradoId,
    searchDisponibilidadTutor,
    searchDisponibilidadTutorado,
    sortColumn,
    sortDirection,
  ]);
  
  // Exportar a Excel
  const exportarAExcel = async () => {
    const dataForSheet = filtered.map(f => ({
      // Datos del Tutor
      NOM_TUTOR:          f.nombretutor          !== "VACÍO" ? f.nombretutor             : "",
      Apellido_TUTOR:    f.apellidotutor         !== "VACÍO" ? f.apellidotutor           : "",
      CORREO_TUTOR:      f.correotutor           !== "VACÍO" ? f.correotutor             : "",
      CELULAR_TUTOR:    f.telefonotutor         !== "VACÍO" ? f.telefonotutor           : "",
      INSTITUCION_TUTOR: f.instituciontutor      !== "VACÍO" ? f.instituciontutor        : "",
      MATERIA:        f.materiaTutor     !== "VACÍO" ? f.materiaTutor      : "",
      MODALIDAD:      f.modalidad        !== "VACÍO" ? f.modalidad         : "",
      HORAS:          f.horastutor       !== "VACÍO" ? f.horastutor        : "",
      HORARARIOS: f.disponibilidadTutor !== "VACÍO" ? f.disponibilidadTutor : "",
    // Datos del Tutorado 1
  
      TUTORADO1:      f.tutorado1      !== "VACÍO" ? f.tutorado1      : "",
      ID_T1:           f.tutorado1      !== "VACÍO" ? f.tutorado1_id   : "",
      COLEGIO1: f.colegiotutorado1 !== "VACÍO" ? f.colegiotutorado1 : "",
      TELE1_T1:       f.tele1Tutorado1 !== "VACÍO" ? f.tele1Tutorado1 : "",
      TELE2_T1:       f.tele2Tutorado1 !== "VACÍO" ? f.tele2Tutorado1 : "",
      CONTACTO_T1:    f.contactoTutorado1 !== "VACÍO" ? f.contactoTutorado1 : "",
      MATERIA_T1:      f.materiaTutorado1       !== "VACÍO" ? f.materiaTutorado1        : "",
      becartiotutor: f.becariotutor !== "VACÍO" ? f.becariotutor : "",
      VOCABULARIO_T1: f.vocabulariotutorado1 !== "VACÍO" ? f.vocabulariotutorado1 : "",
      GRAMATICA_T1:   f.gramaticatutorado1 !== "VACÍO" ? f.gramaticatutorado1 : "",
      ESCUCHA_T1:     f.escuchatutorado1 !== "VACÍO" ? f.escuchatutorado1 : "",
      LECTURA_T1:     f.lecturatutorado1 !== "VACÍO" ? f.lecturatutorado1 : "",
      PENSAMIENTO_NUMERICO_T1: f.pensamientonumericotutorado1 !== "VACÍO" ? f.pensamientonumericotutorado1 : "",
      PENSAMIENTO_ESPACIAL_T1: f.pensamientoespacialtutorado1 !== "VACÍO" ? f.pensamientoespacialtutorado1 : "",
      PENSAMIENTO_OMTRIC_T1: f.pensamientoomtricotutorado1 !== "VACÍO" ? f.pensamientoomtricotutorado1 : "",
      PENSAMIENTO_ALETORIO_T1: f.pensamientoaleatoriotutorado1 !== "VACÍO" ? f.pensamientoaleatoriotutorado1 : "",
      PENSAMIENTO_VARIACIONAL_T1: f.pensamientovariacionalysistertudorado1 !== "VACÍO" ? f.pensamientovariacionalysistertudorado1 : "",
      TOTAL_PUNTUACION_MATH_PRE_T1: f.totalpuntuacionmathpretutorado1 !== "VACÍO" ? f.totalpuntuacionmathpretutorado1 : "",
      TOTAL_PUNTUACION_ENGLISH_PRE_T1: f.totalpuntuacionenglishpretutorado1 !== "VACÍO" ? f.totalpuntuacionenglishpretutorado1 : "",
      DISP_T1:f.disponibilidadTutorado1 !== "VACÍO" ? f.disponibilidadTutorado1 : "",
      GRUPO_T1: f.grupoTutorado1 !== "VACÍO" ? f.grupoTutorado1 : "",
      // Datos del Tutorado 2
      TUTORADO_2:      f.tutorado2      !== "VACÍO" ? f.tutorado2      : "",
      ID_T2:           f.tutorado2      !== "VACÍO" ? f.tutorado2_id   : "",
      COLEGIO2: f.colegiotutorado2 !== "VACÍO" ? f.colegiotutorado2 : "",
      TELE1_T2:       f.tele1Tutorado2 !== "VACÍO" ? f.tele1Tutorado2 : "",
      TELE2_T2:       f.tele2Tutorado2 !== "VACÍO" ? f.tele2Tutorado2 : "",
      CONTACTO_T2:    f.contactoTutorado2 !== "VACÍO" ? f.contactoTutorado2 : "",
      MATERIA_T2:      f.materiaTutorado2       !== "VACÍO" ? f.materiaTutorado2        : "",
      VOCABULARIO_T2: f.vocabulariotutorado2 !== "VACÍO" ? f.vocabulariotutorado2 : "",
      GRAMATICA_T2:   f.gramaticatutorado2 !== "VACÍO" ? f.gramaticatutorado2 : "",
      ESCUCHA_T2:     f.escuchatutorado2 !== "VACÍO" ? f.escuchatutorado2 : "",
      LECTURA_T2:     f.lecturatutorado2 !== "VACÍO" ? f.lecturatutorado2 : "",
      PENSAMIENTO_NUMERICO_T2: f.pensamientonumericotutorado2 !== "VACÍO" ? f.pensamientonumericotutorado2 : "",
      PENSAMIENTO_ESPACIAL_T2: f.pensamientoespacialtutorado2 !== "VACÍO" ? f.pensamientoespacialtutorado2 : "",
      PENSAMIENTO_OMTRIC_T2: f.pensamientoomtricotutorado2 !== "VACÍO" ? f.pensamientoomtricotutorado2 : "",
      PENSAMIENTO_ALETORIO_T2: f.pensamientoaleatoriotutorado2 !== "VACÍO" ? f.pensamientoaleatoriotutorado2 : "",
      PENSAMIENTO_VARIACIONAL_T2: f.pensamientovariacionalysistertudorado2 !== "VACÍO" ? f.pensamientovariacionalysistertudorado2 : "",
      TOTAL_PUNTUACION_MATH_PRE_T2: f.totalpuntuacionmathpretutorado2 !== "VACÍO" ? f.totalpuntuacionmathpretutorado2 : "",
      TOTAL_PUNTUACION_ENGLISH_PRE_T2: f.totalpuntuacionenglishpretutorado2 !== "VACÍO" ? f.totalpuntuacionenglishpretutorado2 : "",      
      DISP_T2:         f.disponibilidadTutorado2 !== "VACÍO" ? f.disponibilidadTutorado2 : "",
      GRUPO_T2: f.grupoTutorado2 !== "VACÍO" ? f.grupoTutorado2 : "",
     DESCRIPCIO_DE_LA_MODALIDAD: f.descripcion_DE_LA_MODALIDAD !== "VACÍO" ? f.descripcion_DE_LA_MODALIDAD : "",
    ARGOS: f.argostutor !== "VACÍO" ? f.argostutor : "",
    }));
    const ws = XLSX.utils.json_to_sheet(dataForSheet);
    const wb = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(wb, ws, "Emparejamiento");
    const buf = XLSX.write(wb, { bookType: "xlsx", type: "array" });

    const path = await save({ defaultPath: "Emparejamiento.xlsx" });
    if (path) await writeBinaryFile({ path, contents: new Uint8Array(buf) });
  };


  // Emparejamiento automático via backend
  const emparejamientoAutomatico = async () => {
    // Usar allData en lugar de filtered
    const result = await invoke<EmparejamientoEntry[]>(
      "emparejamiento_automatico",
      { emparejamientos: allData }
    );
    
    // Actualizar allData con el resultado completo
    setAllData(result);
    
    // Luego aplicar los filtros actuales al resultado completo
    const filteredResult = await invoke<EmparejamientoEntry[]>("filtrar_emparejamientos", {
      emparejamientos: result,
      searchtutor: searchTutor,
      searchtutorado: searchTutorado,
      searchtutoradoId: searchTutoradoId, // Corregido: cambiar searchtutorado_id a searchtutoradoId
      searchdisponibilidadTutor: searchDisponibilidadTutor, // Corregido: cambiar searchdisponibilidad_tutor a searchdisponibilidadTutor
      searchdisponibilidadTutorado: searchDisponibilidadTutorado, // Corregido: cambiar searchdisponibilidad_tutorado a searchdisponibilidadTutorado
      sortcolumn: sortColumn,
      sortdirection: sortDirection,
    });
    
    
    // Actualizar filtered con el resultado filtrado
    setFiltered(filteredResult);
  };

  // Handler de ordenamiento
  const handleSort = (col: typeof sortColumn) => {
    if (sortColumn === col) setSortDirection(sortDirection === "asc" ? "desc" : "asc");
    else { setSortColumn(col); setSortDirection("asc"); }
  };

  // Verificar si un tutor ya ha alcanzado su máximo de tutorados
  const checkMaxTutoradosLimit = (rowIndex: number) => {
    const targetRow = filtered[Math.floor(rowIndex / 2)];
    
    // Si no hay datos o el tutor está vacío, no hay restricción
    if (!targetRow || !targetRow.nombretutor) return false;
    
    // Contar cuántos tutorados ya tiene este tutor
    const tieneTutorado1 = targetRow.tutorado1 && targetRow.tutorado1.trim() !== "" && targetRow.tutorado1 !== "VACÍO";
    const tieneTutorado2 = targetRow.tutorado2 && targetRow.tutorado2.trim() !== "" && targetRow.tutorado2 !== "VACÍO";
    const tutoradosActuales = (tieneTutorado1 ? 1 : 0) + (tieneTutorado2 ? 1 : 0);
    
    // Si la columna que estamos intentando llenar ya tiene un tutorado, no contamos ese
    const colIdx = rowIndex % 2;
    if ((colIdx === 0 && tieneTutorado1) || (colIdx === 1 && tieneTutorado2)) {
      return false; // El slot ya está ocupado pero estamos reemplazándolo
    }
    
    // Si ya alcanzó o superó el límite, no se puede agregar más
    return tutoradosActuales >= parseInt(targetRow.max_tutorados, 10);
  };

  // Drag & Drop handlers con swap completo de campos y validación de max_tutorados
  const handleDragStart = (start: DragStart) => { 
    setDraggingId(start.draggableId); 
    setHighlightedId(null); 
    setInvalidDropId(null);
  };
  
  const handleDragUpdate = (update: DragUpdate) => {
    if (!update.destination) {
      setHighlightedId(null);
      setInvalidDropId(null);
      return;
    }
    
    const idx = update.destination.index;
    const row = Math.floor(idx / 2);
    const col = idx % 2;
    const cellId = `${row}-${col}`;
    
    // Verificar si el destino excedería el límite de tutorados
    if (checkMaxTutoradosLimit(idx)) {
      setHighlightedId(null);
      setInvalidDropId(cellId); // Marcar como inválido
    } else {
      setHighlightedId(cellId); // Resaltar como válido
      setInvalidDropId(null);
    }
  };
  
 // Función corregida handleDragEnd que incluye todos los campos relevantes para la exportación a Excel
const handleDragEnd = (result: DropResult) => {
  setDraggingId(null);
  setHighlightedId(null);
  setInvalidDropId(null);
  
  if (!result.destination) return;

  const { source, destination } = result;
  const srcRow = Math.floor(source.index / 2);
  const dstRow = Math.floor(destination.index / 2);
  const srcCol = source.index % 2 === 0 ? "tutorado1" : "tutorado2";
  const dstCol = destination.index % 2 === 0 ? "tutorado1" : "tutorado2";
  
  // Verificar si el destino excedería el límite de max_tutorados
  if (checkMaxTutoradosLimit(destination.index)) {
    console.warn("No se puede soltar aquí: el tutor ya alcanzó su límite de tutorados");
    return; // Cancelar la operación de drag and drop
  }

  // Obtener los objetos reales del arreglo filtered
  const srcFilteredRow = filtered[srcRow];
  const dstFilteredRow = filtered[dstRow];

  // Encontrar los índices correspondientes en allData
  const srcAllDataIdx = allData.findIndex(item => 
    item.nombretutor === srcFilteredRow.nombretutor && 
    item.tutorado1_id === srcFilteredRow.tutorado1_id && 
    item.tutorado2_id === srcFilteredRow.tutorado2_id
  );
  
  const dstAllDataIdx = allData.findIndex(item => 
    item.nombretutor === dstFilteredRow.nombretutor && 
    item.tutorado1_id === dstFilteredRow.tutorado1_id && 
    item.tutorado2_id === dstFilteredRow.tutorado2_id
  );

  if (srcAllDataIdx === -1 || dstAllDataIdx === -1) {
    console.error("No se encontraron elementos correspondientes en allData");
    return;
  }

  const updated = [...allData];

  // Helper para extraer TODOS los campos del tutorado
  const getFields = (rowIdx: number, col: string) => {
    const isTutorado1 = col === "tutorado1";
    
    return {
      // Campos básicos
      name: updated[rowIdx][col as keyof EmparejamientoEntry] as string,
      id: isTutorado1 ? updated[rowIdx].tutorado1_id : updated[rowIdx].tutorado2_id,
      disp: isTutorado1 ? updated[rowIdx].disponibilidadTutorado1 : updated[rowIdx].disponibilidadTutorado2,
      materia: isTutorado1 ? updated[rowIdx].materiaTutorado1 : updated[rowIdx].materiaTutorado2,
      color: isTutorado1 ? updated[rowIdx].colorOriginal1 : updated[rowIdx].colorOriginal2,
      grupo: isTutorado1 ? updated[rowIdx].grupoTutorado1 : updated[rowIdx].grupoTutorado2,
      contacto: isTutorado1 ? updated[rowIdx].contactoTutorado1 : updated[rowIdx].contactoTutorado2,
      
      // Campos adicionales para Excel
      colegio: isTutorado1 ? updated[rowIdx].colegiotutorado1 : updated[rowIdx].colegiotutorado2,
      tele1: isTutorado1 ? updated[rowIdx].tele1Tutorado1 : updated[rowIdx].tele1Tutorado2,
      tele2: isTutorado1 ? updated[rowIdx].tele2Tutorado1 : updated[rowIdx].tele2Tutorado2,
      
      // Campos de evaluación
      vocabulario: isTutorado1 ? updated[rowIdx].vocabulariotutorado1 : updated[rowIdx].vocabulariotutorado2,
      gramatica: isTutorado1 ? updated[rowIdx].gramaticatutorado1 : updated[rowIdx].gramaticatutorado2,
      escucha: isTutorado1 ? updated[rowIdx].escuchatutorado1 : updated[rowIdx].escuchatutorado2,
      lectura: isTutorado1 ? updated[rowIdx].lecturatutorado1 : updated[rowIdx].lecturatutorado2,
      
      // Campos de pensamiento matemático
      pensamientoNumerico: isTutorado1 ? updated[rowIdx].pensamientonumericotutorado1 : updated[rowIdx].pensamientonumericotutorado2,
      pensamientoEspacial: isTutorado1 ? updated[rowIdx].pensamientoespacialtutorado1 : updated[rowIdx].pensamientoespacialtutorado2,
      pensamientoOmtrico: isTutorado1 ? updated[rowIdx].pensamientoomtricotutorado1 : updated[rowIdx].pensamientoomtricotutorado2,
      pensamientoAleatorio: isTutorado1 ? updated[rowIdx].pensamientoaleatoriotutorado1 : updated[rowIdx].pensamientoaleatoriotutorado2,
      pensamientoVariacional: isTutorado1 ? updated[rowIdx].pensamientovariacionalysistertudorado1 : updated[rowIdx].pensamientovariacionalysistertudorado2,
      
      // Puntuaciones totales
      totalMath: isTutorado1 ? updated[rowIdx].totalpuntuacionmathpretutorado1 : updated[rowIdx].totalpuntuacionmathpretutorado2,
      totalEnglish: isTutorado1 ? updated[rowIdx].totalpuntuacionenglishpretutorado1 : updated[rowIdx].totalpuntuacionenglishpretutorado2,
    };
  };

  const src = getFields(srcAllDataIdx, srcCol);
  const dst = getFields(dstAllDataIdx, dstCol);

  // Asignar TODOS los campos de dst en src
  if (srcCol === "tutorado1") {
    // Campos básicos
    updated[srcAllDataIdx].tutorado1 = dst.name;
    updated[srcAllDataIdx].tutorado1_id = dst.id;
    updated[srcAllDataIdx].disponibilidadTutorado1 = dst.disp;
    updated[srcAllDataIdx].materiaTutorado1 = dst.materia;
    updated[srcAllDataIdx].colorOriginal1 = dst.color;
    updated[srcAllDataIdx].grupoTutorado1 = dst.grupo;
    updated[srcAllDataIdx].contactoTutorado1 = dst.contacto;
    
    // Campos adicionales
    updated[srcAllDataIdx].colegiotutorado1 = dst.colegio;
    updated[srcAllDataIdx].tele1Tutorado1 = dst.tele1;
    updated[srcAllDataIdx].tele2Tutorado1 = dst.tele2;
    
    // Campos de evaluación
    updated[srcAllDataIdx].vocabulariotutorado1 = dst.vocabulario;
    updated[srcAllDataIdx].gramaticatutorado1 = dst.gramatica;
    updated[srcAllDataIdx].escuchatutorado1 = dst.escucha;
    updated[srcAllDataIdx].lecturatutorado1 = dst.lectura;
    
    // Campos de pensamiento matemático
    updated[srcAllDataIdx].pensamientonumericotutorado1 = dst.pensamientoNumerico;
    updated[srcAllDataIdx].pensamientoespacialtutorado1 = dst.pensamientoEspacial;
    updated[srcAllDataIdx].pensamientoomtricotutorado1 = dst.pensamientoOmtrico;
    updated[srcAllDataIdx].pensamientoaleatoriotutorado1 = dst.pensamientoAleatorio;
    updated[srcAllDataIdx].pensamientovariacionalysistertudorado1 = dst.pensamientoVariacional;
    
    // Puntuaciones totales
    updated[srcAllDataIdx].totalpuntuacionmathpretutorado1 = dst.totalMath;
    updated[srcAllDataIdx].totalpuntuacionenglishpretutorado1 = dst.totalEnglish;
  } else {
    // Campos básicos
    updated[srcAllDataIdx].tutorado2 = dst.name;
    updated[srcAllDataIdx].tutorado2_id = dst.id;
    updated[srcAllDataIdx].disponibilidadTutorado2 = dst.disp;
    updated[srcAllDataIdx].materiaTutorado2 = dst.materia;
    updated[srcAllDataIdx].colorOriginal2 = dst.color;
    updated[srcAllDataIdx].grupoTutorado2 = dst.grupo;
    updated[srcAllDataIdx].contactoTutorado2 = dst.contacto;
    
    // Campos adicionales
    updated[srcAllDataIdx].colegiotutorado2 = dst.colegio;
    updated[srcAllDataIdx].tele1Tutorado2 = dst.tele1;
    updated[srcAllDataIdx].tele2Tutorado2 = dst.tele2;
    
    // Campos de evaluación
    updated[srcAllDataIdx].vocabulariotutorado2 = dst.vocabulario;
    updated[srcAllDataIdx].gramaticatutorado2 = dst.gramatica;
    updated[srcAllDataIdx].escuchatutorado2 = dst.escucha;
    updated[srcAllDataIdx].lecturatutorado2 = dst.lectura;
    
    // Campos de pensamiento matemático
    updated[srcAllDataIdx].pensamientonumericotutorado2 = dst.pensamientoNumerico;
    updated[srcAllDataIdx].pensamientoespacialtutorado2 = dst.pensamientoEspacial;
    updated[srcAllDataIdx].pensamientoomtricotutorado2 = dst.pensamientoOmtrico;
    updated[srcAllDataIdx].pensamientoaleatoriotutorado2 = dst.pensamientoAleatorio;
    updated[srcAllDataIdx].pensamientovariacionalysistertudorado2 = dst.pensamientoVariacional;
    
    // Puntuaciones totales
    updated[srcAllDataIdx].totalpuntuacionmathpretutorado2 = dst.totalMath;
    updated[srcAllDataIdx].totalpuntuacionenglishpretutorado2 = dst.totalEnglish;
  }

  // Asignar TODOS los campos de src en dst
  if (dstCol === "tutorado1") {
    // Campos básicos
    updated[dstAllDataIdx].tutorado1 = src.name;
    updated[dstAllDataIdx].tutorado1_id = src.id;
    updated[dstAllDataIdx].disponibilidadTutorado1 = src.disp;
    updated[dstAllDataIdx].materiaTutorado1 = src.materia;
    updated[dstAllDataIdx].colorOriginal1 = src.color;
    updated[dstAllDataIdx].grupoTutorado1 = src.grupo;
    updated[dstAllDataIdx].contactoTutorado1 = src.contacto;
    
    // Campos adicionales
    updated[dstAllDataIdx].colegiotutorado1 = src.colegio;
    updated[dstAllDataIdx].tele1Tutorado1 = src.tele1;
    updated[dstAllDataIdx].tele2Tutorado1 = src.tele2;
    
    // Campos de evaluación
    updated[dstAllDataIdx].vocabulariotutorado1 = src.vocabulario;
    updated[dstAllDataIdx].gramaticatutorado1 = src.gramatica;
    updated[dstAllDataIdx].escuchatutorado1 = src.escucha;
    updated[dstAllDataIdx].lecturatutorado1 = src.lectura;
    
    // Campos de pensamiento matemático
    updated[dstAllDataIdx].pensamientonumericotutorado1 = src.pensamientoNumerico;
    updated[dstAllDataIdx].pensamientoespacialtutorado1 = src.pensamientoEspacial;
    updated[dstAllDataIdx].pensamientoomtricotutorado1 = src.pensamientoOmtrico;
    updated[dstAllDataIdx].pensamientoaleatoriotutorado1 = src.pensamientoAleatorio;
    updated[dstAllDataIdx].pensamientovariacionalysistertudorado1 = src.pensamientoVariacional;
    
    // Puntuaciones totales
    updated[dstAllDataIdx].totalpuntuacionmathpretutorado1 = src.totalMath;
    updated[dstAllDataIdx].totalpuntuacionenglishpretutorado1 = src.totalEnglish;
  } else {
    // Campos básicos
    updated[dstAllDataIdx].tutorado2 = src.name;
    updated[dstAllDataIdx].tutorado2_id = src.id;
    updated[dstAllDataIdx].disponibilidadTutorado2 = src.disp;
    updated[dstAllDataIdx].materiaTutorado2 = src.materia;
    updated[dstAllDataIdx].colorOriginal2 = src.color;
    updated[dstAllDataIdx].grupoTutorado2 = src.grupo;
    updated[dstAllDataIdx].contactoTutorado2 = src.contacto;
    
    // Campos adicionales
    updated[dstAllDataIdx].colegiotutorado2 = src.colegio;
    updated[dstAllDataIdx].tele1Tutorado2 = src.tele1;
    updated[dstAllDataIdx].tele2Tutorado2 = src.tele2;
    
    // Campos de evaluación
    updated[dstAllDataIdx].vocabulariotutorado2 = src.vocabulario;
    updated[dstAllDataIdx].gramaticatutorado2 = src.gramatica;
    updated[dstAllDataIdx].escuchatutorado2 = src.escucha;
    updated[dstAllDataIdx].lecturatutorado2 = src.lectura;
    
    // Campos de pensamiento matemático
    updated[dstAllDataIdx].pensamientonumericotutorado2 = src.pensamientoNumerico;
    updated[dstAllDataIdx].pensamientoespacialtutorado2 = src.pensamientoEspacial;
    updated[dstAllDataIdx].pensamientoomtricotutorado2 = src.pensamientoOmtrico;
    updated[dstAllDataIdx].pensamientoaleatoriotutorado2 = src.pensamientoAleatorio;
    updated[dstAllDataIdx].pensamientovariacionalysistertudorado2 = src.pensamientoVariacional;
    
    // Puntuaciones totales
    updated[dstAllDataIdx].totalpuntuacionmathpretutorado2 = src.totalMath;
    updated[dstAllDataIdx].totalpuntuacionenglishpretutorado2 = src.totalEnglish;
  }

  setAllData(updated);
};
  // Actualizar campo tutor
  const actualizarTutor = async (index: number, campo: "materiaTutor" | "disponibilidadTutor"| "contactoTutor", valor: string) => {
    const updated = await invoke<EmparejamientoEntry[]>("actualizar_campoTutor", { emparejamientos: allData, index, campo, valor });
    setAllData(updated);
  };

  return (
    <div className="emparejamiento">
      <div style={{ display: "flex", gap: "10px", justifyContent: "center", marginBottom: "15px" }}>
        <button onClick={seleccionarArchivo} style={{ flex: "1 1 160px", padding: "10px", fontSize: "16px" }}>Seleccionar hoja de cálculo</button>
        <button onClick={emparejamientoAutomatico} style={{ flex: "1 1 200px", padding: "10px", fontSize: "16px" }}>Emparejamiento Automático</button>
        <button onClick={() => {localStorage.removeItem("emparejamientoData");window.location.reload();}}>Reiniciar Tabla</button>
        <button onClick={exportarAExcel} style={{ flex: "1 1 200px", padding: "10px", fontSize: "16px" }}>Exportar a Excel</button>



      </div>
      <div className="search-bar" style={{ display: "flex", flexWrap: "wrap", alignItems: "center", gap: "10px", maxWidth: "900px", margin: "0 auto 20px" }}>
        <input type="text" placeholder="Buscar Tutor" value={searchTutor} onChange={(e) => setSearchTutor(e.target.value)} style={{ flex: "1 1 150px", padding: "8px" }} />
        <input type="text" placeholder="Buscar Tutorados" value={searchTutorado} onChange={(e) => setSearchTutorado(e.target.value)} style={{ flex: "1 1 150px", padding: "8px" }} />
        <input type="text" placeholder="Buscar ID Tutorados" value={searchTutoradoId} onChange={(e) => setSearchTutoradoId(e.target.value)} style={{ flex: "1 1 150px", padding: "8px" }} />
        <select value={searchDisponibilidadTutor} onChange={(e) => setSearchDisponibilidadTutor(e.target.value)} style={{ flex: "1 1 180px", padding: "8px" }}>
          <option value="">Disponibilidad Tutor: Todos</option>
          {disponibilidadOptions.map((opt) => (<option key={opt} value={opt}>{opt}</option>))}
        </select>
        <select value={searchDisponibilidadTutorado} onChange={(e) => setSearchDisponibilidadTutorado(e.target.value)} style={{ flex: "1 1 180px", padding: "8px" }}>
          <option value="">Disponibilidad Tutorados: Todos</option>
          {disponibilidadOptions.map((opt) => (<option key={opt} value={opt}>{opt}</option>))}
        </select>
      </div>
      <div className="table-container">
        <table>
          <thead>
            <tr>
              <th onClick={() => handleSort("tutor")}>Tutor {sortColumn === "tutor" && (sortDirection === "asc" ? "▲" : "▼")}</th>
              <th onClick={() => handleSort("materiaTutor")}>Materia {sortColumn === "materiaTutor" && (sortDirection === "asc" ? "▲" : "▼")}</th>
              <th onClick={() => handleSort("disponibilidadTutor")}>Disponibilidad {sortColumn === "disponibilidadTutor" && (sortDirection === "asc" ? "▲" : "▼")}</th>
              <th>Tutorado 1</th>
              <th>Tutorado 2</th>
            </tr>
          </thead>
          <DragDropContext onDragStart={handleDragStart} onDragUpdate={handleDragUpdate} onDragEnd={handleDragEnd}>
            <Droppable droppableId="tutorados" direction="vertical">
              {(provided) => (
                <tbody ref={provided.innerRef} {...provided.droppableProps}>
                  {filtered.map((fila, rowIndex) => (
                    <tr key={rowIndex}>
                      <td>
                        {fila.nombretutor+" " + fila.apellidotutor}
                        {parseInt(fila.max_tutorados, 10) === 1 && (
                          <span style={{ marginLeft: '5px', color: '#ff6b6b', fontWeight: 'bold', fontSize: '11px' }}>
                            (Max: 1)
                          </span>
                        )}
                      </td>
                      <td>
                        <select 
                          value={fila.materiaTutor} 
                          onChange={(e) => actualizarTutor(rowIndex, "materiaTutor", e.target.value)} 
                          style={{ padding: "6px" }}
                        >
                          <option value="">Vacío</option>
                          <option value="Ingles">Inglés</option>
                          <option value="Matematicas">Matemáticas</option>
                        </select>
                      </td>
                      <td>
                        <select 
                          value={fila.disponibilidadTutor} 
                          onChange={(e) => actualizarTutor(rowIndex, "disponibilidadTutor", e.target.value)} 
                          style={{ padding: "6px" }}
                        >
                          {disponibilidadOptions.map((opt) => (
                            <option key={opt} value={opt}>{opt}</option>
                          ))}
                        </select>
                      </td>
                      {[fila.tutorado1, fila.tutorado2].map((_, colIdx) => {
                        const cellId = `${rowIndex}-${colIdx}`;
                        const item = colIdx === 0 ? {
                          name: fila.tutorado1,
                          id: fila.tutorado1_id,
                          disp: fila.disponibilidadTutorado1,
                          contacto: fila.contactoTutorado1,
                          color: fila.colorOriginal1
                        } : {
                          name: fila.tutorado2,
                          id: fila.tutorado2_id,
                          disp: fila.disponibilidadTutorado2,
                          contacto: fila.contactoTutorado2,
                          color: fila.colorOriginal2
                        };
                        const isEmpty = !item.name.trim() || item.name.toUpperCase() === "VACÍO";
                        
                        // Si es la segunda columna y el tutor solo puede tener 1 tutorado y ya tiene uno en la primera columna
                        const isForbiddenSlot = colIdx === 1 && 
                                             parseInt(fila.max_tutorados, 10) === 1 && 
                                             fila.tutorado1 && 
                                             fila.tutorado1.trim() !== "" && 
                                             fila.tutorado1 !== "VACÍO";
                        
                        return (
                          <td 
                            key={cellId} 
                            className={`
                              drop-target 
                              ${highlightedId === cellId ? "highlight" : ""} 
                              ${invalidDropId === cellId ? "invalid-drop" : ""}
                              ${isForbiddenSlot ? "forbidden-slot" : ""}
                            `}
                          >
                            <Draggable 
                              draggableId={cellId} 
                              index={rowIndex * 2 + colIdx}
                              isDragDisabled={!!(isForbiddenSlot && isEmpty)}
                            >
                              {(prov) => (
                                <div 
                                  ref={prov.innerRef} 
                                  {...prov.draggableProps} 
                                  {...prov.dragHandleProps} 
                                  className={`
                                    draggable 
                                    ${item.color} 
                                    ${draggingId === cellId ? "dragging" : ""} 
                                    ${isForbiddenSlot ? "forbidden-tutorado" : ""}
                                  `}
                                >
                                  {isEmpty ? (
                                    <div className="tutorado-vacio">
                                      {isForbiddenSlot ? "NO DISPONIBLE" : "VACÍO"}
                                    </div>
                                  ) : (
                                    <div className="tutorado-info">
                                      <div className="tutorado-name">{item.name}</div>
                                      <div className="tutorado-id">ID: {item.id}</div>
                                      <div className="tutorado-disp">Disp: {item.disp}</div>
                                    </div>
                                  )}
                                </div>
                              )}
                            </Draggable>
                          </td>
                        );
                      })}
                    </tr>
                  ))}
                  {provided.placeholder}
                </tbody>
              )}
            </Droppable>
          </DragDropContext>
        </table>
      </div>
    </div>
  );
}

export default Emparejamiento;