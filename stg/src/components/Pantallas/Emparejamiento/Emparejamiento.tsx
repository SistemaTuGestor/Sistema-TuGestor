import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import * as XLSX from "xlsx";
import { saveAs } from "file-saver";
import { DragDropContext, Droppable, Draggable, DropResult, DragUpdate, DragStart } from "@hello-pangea/dnd";
import "./Emparejamiento.css";

type EmparejamientoEntry = {
  tutor: string;
  materia: string;
  disponibilidad: "Mañana" | "Tarde" | "Noche";
  tutorado1: string;
  tutorado2: string;
};

function Emparejamiento() {
  const [emparejamientos, setEmparejamientos] = useState<EmparejamientoEntry[]>([]);
  const [draggingId, setDraggingId] = useState<string | null>(null);
  const [highlightedId, setHighlightedId] = useState<string | null>(null);

  // 1️⃣ Cargar datos desde localStorage al iniciar
  useEffect(() => {
    const datosGuardados = localStorage.getItem("emparejamientos");
    if (datosGuardados) {
      setEmparejamientos(JSON.parse(datosGuardados));
    }
  }, []);

  // 2️⃣ Guardar en localStorage cada vez que cambie emparejamientos
  useEffect(() => {
    if (emparejamientos.length > 0) { // Evitar guardar un array vacío al inicio
      localStorage.setItem("emparejamientos", JSON.stringify(emparejamientos));
    }
  }, [emparejamientos]);

  // 3️⃣ Obtener datos desde el backend y guardar en localStorage
  const iniciarEmparejamiento = async () => {
    try {
      const data = await invoke<EmparejamientoEntry[]>("obtener_emparejamiento");
      setEmparejamientos(data);
      localStorage.setItem("emparejamientos", JSON.stringify(data)); // Guardar en localStorage
    } catch (error) {
      console.error("Error al obtener emparejamiento:", error);
    }
  };

  const exportarAExcel = () => {
    const hojaDatos = emparejamientos.map(({ tutor, materia, disponibilidad, tutorado1, tutorado2 }) => ({
      Tutor: tutor,
      Materia: materia,
      Disponibilidad: disponibilidad,
      "Tutorado 1": tutorado1,
      "Tutorado 2": tutorado2,
    }));

    const hoja = XLSX.utils.json_to_sheet(hojaDatos);
    const libro = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(libro, hoja, "Emparejamiento");

    const excelBuffer = XLSX.write(libro, { bookType: "xlsx", type: "array" });
    const archivo = new Blob([excelBuffer], { type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" });

    saveAs(archivo, "Emparejamiento.xlsx");
  };

  // Manejo de inicio de arrastre
  const handleDragStart = (start: DragStart) => {
    setDraggingId(start.draggableId);
    setHighlightedId(null);
  };

  // Manejo de actualización de arrastre (muestra el destino)
  const handleDragUpdate = (update: DragUpdate) => {
    if (!update.destination) return;
    setHighlightedId(update.destination.droppableId);
  };

  // Manejo del Drop: intercambio entre filas y columnas
  const handleDragEnd = (result: DropResult) => {
    setDraggingId(null);
    setHighlightedId(null);

    if (!result.destination) return;

    const { source, destination } = result;
    const sourceRow = Math.floor(source.index / 2);
    const sourceCol = source.index % 2 === 0 ? "tutorado1" : "tutorado2";
    const destRow = Math.floor(destination.index / 2);
    const destCol = destination.index % 2 === 0 ? "tutorado1" : "tutorado2";

    const newEmparejamientos = [...emparejamientos];

    // Intercambiar posiciones
    [newEmparejamientos[sourceRow][sourceCol as 'tutorado1' | 'tutorado2'],
    newEmparejamientos[destRow][destCol as 'tutorado1' | 'tutorado2']] =
      [newEmparejamientos[destRow][destCol as 'tutorado1' | 'tutorado2'],
      newEmparejamientos[sourceRow][sourceCol as 'tutorado1' | 'tutorado2']];

    setEmparejamientos(newEmparejamientos);
  };

  // Actualizar lista desplegable en Materia o Disponibilidad
  const actualizarCampo = (index: number, campo: "materia" | "disponibilidad", valor: string) => {
    const nuevaLista = [...emparejamientos];
    nuevaLista[index][campo] = valor as any;
    setEmparejamientos(nuevaLista);
  };

  return (
    <div className="emparejamiento">
      <h2>Emparejamiento Manual</h2>
      <button onClick={iniciarEmparejamiento}>Iniciar Emparejamiento</button>
      <button onClick={exportarAExcel}>Exportar a Excel</button>

      <div className="table-container">
        <table>
          <thead>
            <tr>
              <th>Tutor</th>
              <th>Materia</th>
              <th>Disponibilidad</th>
              <th>Tutorado 1</th>
              <th>Tutorado 2</th>
            </tr>
          </thead>
          <DragDropContext onDragStart={handleDragStart} onDragUpdate={handleDragUpdate} onDragEnd={handleDragEnd}>
            <Droppable droppableId="tutorados" direction="vertical">
              {(provided) => (
                <tbody ref={provided.innerRef} {...provided.droppableProps}>
                  {emparejamientos.map((fila, rowIndex) => (
                    <tr key={rowIndex}>
                      <td>{fila.tutor}</td>
                      <td>
                        <select value={fila.materia} onChange={(e) => actualizarCampo(rowIndex, "materia", e.target.value)}>
                          <option value="">Vacío</option>
                          <option value="Ingles">Inglés</option>
                          <option value="Matemáticas">Matemáticas</option>
                        </select>
                      </td>
                      <td>
                        <select value={fila.disponibilidad} onChange={(e) => actualizarCampo(rowIndex, "disponibilidad", e.target.value)}>
                          <option value="Mañana">Mañana</option>
                          <option value="Tarde">Tarde</option>
                          <option value="Noche">Noche</option>
                        </select>
                      </td>
                      {[fila.tutorado1, fila.tutorado2].map((tutorado, colIndex) => {
                        const tutoradoId = `${rowIndex}-${colIndex}`;
                        return (
                          <td key={tutoradoId} className={highlightedId === tutoradoId ? "highlight" : ""}>
                            <Draggable draggableId={tutoradoId} index={rowIndex * 2 + colIndex}>
                              {(provided) => (
                                <div
                                  ref={provided.innerRef}
                                  {...provided.draggableProps}
                                  {...provided.dragHandleProps}
                                  className={`draggable-tutorado ${draggingId === tutoradoId ? "dragging" : ""}`}
                                >
                                  {tutorado}
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
