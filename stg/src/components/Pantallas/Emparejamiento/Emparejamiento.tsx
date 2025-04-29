import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import * as XLSX from "xlsx";
import { save } from "@tauri-apps/api/dialog";
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
  tutor: string;
  disponibilidadTutor: string;
  materiaTutor: string;
  modalidad: string;
  max_tutorados: number;
  tutorado1: string;
  tutorado1_id: string;
  disponibilidadTutorado1: string;
  materiaTutorado1: string;
  colorOriginal1?: string;
  tutorado2: string;
  tutorado2_id: string;
  disponibilidadTutorado2: string;
  materiaTutorado2: string;
  colorOriginal2?: string;
  contactoTutor: string;
  contactoTutorado1: string;
  contactoTutorado2: string;
  grupoTutorado1: string;
  grupoTutorado2: string;
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
  useEffect(() => {
    localStorage.setItem("emparejamientoData", JSON.stringify(allData));
    if (allData.length === 0) {
          async function load() {
            const data = await invoke<EmparejamientoEntry[]>("obtener_emparejamiento");
           setAllData(data);
         }
         load();
        }
  }, [allData]);

  // Filtrar y ordenar usando backend
  useEffect(() => {
    async function applyFilter() {
      console.log("Invocando filtro con", {
        searchTutor: searchTutor,
        searchTutorado: searchTutorado,
        searchTutorado_id: searchTutoradoId,
        search_disponibilidadTutor: searchDisponibilidadTutor,
        search_disponibilidadTutorado: searchDisponibilidadTutorado,
        sort_column: sortColumn,
        sort_direction: sortDirection,
      });
      const result = await invoke<EmparejamientoEntry[]>("filtrar_emparejamientos", {
               emparejamientos: allData,
               searchTutor:        searchTutor,
               searchTutorado:     searchTutorado,
               searchTutoradoId:   searchTutoradoId,
               searchDisponibilidadTutor:     searchDisponibilidadTutor,
               searchDisponibilidadTutorado:  searchDisponibilidadTutorado,
               sortColumn:  sortColumn,
               sortDirection: sortDirection,
            });
      setFiltered(result);
    }
    applyFilter();
  }, [allData, searchTutor, searchTutorado, searchTutoradoId, searchDisponibilidadTutor, searchDisponibilidadTutorado, sortColumn, sortDirection]);

  // Exportar a Excel
  const exportarAExcel = async () => {
    const dataForSheet = filtered.map(f => ({
      TUTOR:          f.tutor            !== "VACÍO" ? f.tutor             : "",
      CONTACTO:       f.contactoTutor    !== "VACÍO" ? f.contactoTutor     : "",
      MATERIA:        f.materiaTutor     !== "VACÍO" ? f.materiaTutor      : "",
      DISPONIBILIDAD: f.disponibilidadTutor !== "VACÍO" ? f.disponibilidadTutor : "",
      MODALIDAD:      f.modalidad        !== "VACÍO" ? f.modalidad         : "",
    
  
      TUTORADO_1:      f.tutorado1      !== "VACÍO" ? f.tutorado1      : "",
      ID_T1:           f.tutorado1      !== "VACÍO" ? f.tutorado1_id   : "",
      DISP_T1:         f.disponibilidadTutorado1 !== "VACÍO" ? f.disponibilidadTutorado1 : "",
      MATERIA_T1:      f.materiaTutorado1       !== "VACÍO" ? f.materiaTutorado1        : "",
  
      TUTORADO_2:      f.tutorado2      !== "VACÍO" ? f.tutorado2      : "",
      ID_T2:           f.tutorado2      !== "VACÍO" ? f.tutorado2_id   : "",
      DISP_T2:         f.disponibilidadTutorado2 !== "VACÍO" ? f.disponibilidadTutorado2 : "",
      MATERIA_T2:      f.materiaTutorado2       !== "VACÍO" ? f.materiaTutorado2        : "",
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
      searchTutor: searchTutor,
      searchTutorado: searchTutorado,
      searchTutoradoId: searchTutoradoId,
      searchDisponibilidadTutor: searchDisponibilidadTutor,
      searchDisponibilidadTutorado: searchDisponibilidadTutorado,
      sortColumn: sortColumn,
      sortDirection: sortDirection,
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
  const checkMaxTutoradosLimit = (rowIndex: number, colIndex: number) => {
    const targetRow = filtered[Math.floor(rowIndex / 2)];
    
    // Si no hay datos o el tutor está vacío, no hay restricción
    if (!targetRow || !targetRow.tutor) return false;
    
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
    return tutoradosActuales >= targetRow.max_tutorados;
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
    if (checkMaxTutoradosLimit(idx, col)) {
      setHighlightedId(null);
      setInvalidDropId(cellId); // Marcar como inválido
    } else {
      setHighlightedId(cellId); // Resaltar como válido
      setInvalidDropId(null);
    }
  };
  
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
    if (checkMaxTutoradosLimit(destination.index, destination.index % 2)) {
      console.warn("No se puede soltar aquí: el tutor ya alcanzó su límite de tutorados");
      return; // Cancelar la operación de drag and drop
    }
  
    // Obtener los objetos reales del arreglo filtered
    const srcFilteredRow = filtered[srcRow];
    const dstFilteredRow = filtered[dstRow];
  
    // Encontrar los índices correspondientes en allData
    const srcAllDataIdx = allData.findIndex(item => 
      item.tutor === srcFilteredRow.tutor && 
      item.tutorado1_id === srcFilteredRow.tutorado1_id && 
      item.tutorado2_id === srcFilteredRow.tutorado2_id
    );
    
    const dstAllDataIdx = allData.findIndex(item => 
      item.tutor === dstFilteredRow.tutor && 
      item.tutorado1_id === dstFilteredRow.tutorado1_id && 
      item.tutorado2_id === dstFilteredRow.tutorado2_id
    );
  
    if (srcAllDataIdx === -1 || dstAllDataIdx === -1) {
      console.error("No se encontraron elementos correspondientes en allData");
      return;
    }
  
    const updated = [...allData];
  
    // Helper para extraer campos del tutorado
    const getFields = (rowIdx: number, col: string) => {
      return {
        name: updated[rowIdx][col as keyof EmparejamientoEntry] as string,
        id: col === "tutorado1" ? updated[rowIdx].tutorado1_id : updated[rowIdx].tutorado2_id,
        disp: col === "tutorado1" ? updated[rowIdx].disponibilidadTutorado1 : updated[rowIdx].disponibilidadTutorado2,
        materia: col === "tutorado1" ? updated[rowIdx].materiaTutorado1 : updated[rowIdx].materiaTutorado2,
        color: col === "tutorado1" ? updated[rowIdx].colorOriginal1 : updated[rowIdx].colorOriginal2,
        grupo: col === "tutorado1" ? updated[rowIdx].grupoTutorado1 : updated[rowIdx].grupoTutorado2,
        contacto: col === "tutorado1" ? updated[rowIdx].contactoTutorado1 : updated[rowIdx].contactoTutorado2,
      };
    };
  
    const src = getFields(srcAllDataIdx, srcCol);
    const dst = getFields(dstAllDataIdx, dstCol);
  
    // Asignar dst en src
    if (srcCol === "tutorado1") {
      updated[srcAllDataIdx].tutorado1 = dst.name;
      updated[srcAllDataIdx].tutorado1_id = dst.id;
      updated[srcAllDataIdx].disponibilidadTutorado1 = dst.disp;
      updated[srcAllDataIdx].materiaTutorado1 = dst.materia;
      updated[srcAllDataIdx].colorOriginal1 = dst.color;
      updated[srcAllDataIdx].grupoTutorado1 = dst.grupo;
      updated[srcAllDataIdx].contactoTutorado1 = dst.contacto;
    } else {
      updated[srcAllDataIdx].tutorado2 = dst.name;
      updated[srcAllDataIdx].tutorado2_id = dst.id;
      updated[srcAllDataIdx].disponibilidadTutorado2 = dst.disp;
      updated[srcAllDataIdx].materiaTutorado2 = dst.materia;
      updated[srcAllDataIdx].colorOriginal2 = dst.color;
      updated[srcAllDataIdx].grupoTutorado2 = dst.grupo;
      updated[srcAllDataIdx].contactoTutorado2 = dst.contacto; 
    }
  
    // Asignar src en dst
    if (dstCol === "tutorado1") {
      updated[dstAllDataIdx].tutorado1 = src.name;
      updated[dstAllDataIdx].tutorado1_id = src.id;
      updated[dstAllDataIdx].disponibilidadTutorado1 = src.disp;
      updated[dstAllDataIdx].materiaTutorado1 = src.materia;
      updated[dstAllDataIdx].colorOriginal1 = src.color;
      updated[dstAllDataIdx].grupoTutorado1 = src.grupo;
      updated[dstAllDataIdx].contactoTutorado1 = src.contacto;
    } else {
      updated[dstAllDataIdx].tutorado2 = src.name;
      updated[dstAllDataIdx].tutorado2_id = src.id;
      updated[dstAllDataIdx].disponibilidadTutorado2 = src.disp;
      updated[dstAllDataIdx].materiaTutorado2 = src.materia;
      updated[dstAllDataIdx].colorOriginal2 = src.color;
      updated[dstAllDataIdx].grupoTutorado2 = src.grupo;
      updated[dstAllDataIdx].contactoTutorado2 = src.contacto;
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
        <button onClick={emparejamientoAutomatico} style={{ flex: "1 1 200px", padding: "10px", fontSize: "16px" }}>Emparejamiento Automático</button>
        <button onClick={exportarAExcel} style={{ flex: "1 1 200px", padding: "10px", fontSize: "16px" }}>Exportar a Excel</button>
        <button onClick={() => {
  localStorage.removeItem("emparejamientoData");
  window.location.reload();
}}>
  Reiniciar Tabla
</button>
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
                        {fila.tutor}
                        {fila.max_tutorados === 1 && (
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
                                             fila.max_tutorados === 1 && 
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