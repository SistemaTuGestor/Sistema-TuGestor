import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import * as XLSX from "xlsx";
import { saveAs } from "file-saver";
import { DragDropContext, Droppable, Draggable, DropResult, DragUpdate, DragStart } from "@hello-pangea/dnd";
import "./Emparejamiento.css";

type EmparejamientoEntry = {
  tutor: string;
  materiaTutor: string;
  disponibilidad: string;
  tutorado1: string;
  tutorado2: string;
  materiaTutorado1: string;
  materiaTutorado2: string;
};

function Emparejamiento() {
  const [emparejamientos, setEmparejamientos] = useState<EmparejamientoEntry[]>([]);
  const [draggingId, setDraggingId] = useState<string | null>(null);
  const [highlightedId, setHighlightedId] = useState<string | null>(null);

  // Cargar datos desde localStorage al iniciar
  useEffect(() => {
    const datosGuardados = localStorage.getItem("emparejamientos");
    if (datosGuardados) {
      setEmparejamientos(JSON.parse(datosGuardados));
    }
  }, []);

  // Guardar en localStorage cada vez que cambie emparejamientos
  useEffect(() => {
    if (emparejamientos.length > 0) {
      localStorage.setItem("emparejamientos", JSON.stringify(emparejamientos));
    }
  }, [emparejamientos]);

  // Obtener datos desde el backend y guardarlos en localStorage
  const iniciarEmparejamiento = async () => {
    try {
      const data = await invoke<EmparejamientoEntry[]>("obtener_emparejamiento");
      setEmparejamientos(data);
      localStorage.setItem("emparejamientos", JSON.stringify(data));
    } catch (error) {
      console.error("Error al obtener emparejamiento:", error);
    }
  };

  const exportarAExcel = () => {
    const hojaDatos = emparejamientos.map(({ tutor, materiaTutor, disponibilidad, tutorado1, tutorado2 }) => ({
      Tutor: tutor,
      Materia: materiaTutor,
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

  // Funciones de Drag & Drop
  const handleDragStart = (start: DragStart) => {
    setDraggingId(start.draggableId);
    setHighlightedId(null);
  };

  const handleDragUpdate = (update: DragUpdate) => {
    if (!update.destination) return;
    setHighlightedId(update.destination.droppableId);
  };

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

    // Guardamos temporalmente el tutorado y su materia del origen
    const tempTutorado = newEmparejamientos[sourceRow][sourceCol as 'tutorado1' | 'tutorado2'];
    const tempMateria =
      sourceCol === "tutorado1"
        ? newEmparejamientos[sourceRow].materiaTutorado1
        : newEmparejamientos[sourceRow].materiaTutorado2;

    // Intercambiamos en la fila de origen con la fila de destino
    newEmparejamientos[sourceRow][sourceCol as 'tutorado1' | 'tutorado2'] =
      newEmparejamientos[destRow][destCol as 'tutorado1' | 'tutorado2'];
    if (sourceCol === "tutorado1") {
      newEmparejamientos[sourceRow].materiaTutorado1 =
        destCol === "tutorado1"
          ? newEmparejamientos[destRow].materiaTutorado1
          : newEmparejamientos[destRow].materiaTutorado2;
    } else {
      newEmparejamientos[sourceRow].materiaTutorado2 =
        destCol === "tutorado1"
          ? newEmparejamientos[destRow].materiaTutorado1
          : newEmparejamientos[destRow].materiaTutorado2;
    }

    // Se asigna el valor temporal a la fila de destino
    newEmparejamientos[destRow][destCol as 'tutorado1' | 'tutorado2'] = tempTutorado;
    if (destCol === "tutorado1") {
      newEmparejamientos[destRow].materiaTutorado1 = tempMateria;
    } else {
      newEmparejamientos[destRow].materiaTutorado2 = tempMateria;
    }

    setEmparejamientos(newEmparejamientos);
  };

  // Actualizar lista desplegable en Materia o Disponibilidad
  const actualizarCampo = (index: number, campo: "materiaTutor" | "disponibilidad", valor: string) => {
    const nuevaLista = [...emparejamientos];
    nuevaLista[index][campo] = valor;
    setEmparejamientos(nuevaLista);
  };

  // Función de Emparejamiento Automático:
  // Reubica tutorados mal asignados a un tutor que dicta la materia correspondiente,
  // sin mover aquellos que ya están correctamente asignados.
  const emparejamientoAutomatico = () => {
    const nuevoEmparejamiento = emparejamientos.map(row => ({ ...row }));

    // Acumulamos los tutorados que están mal asignados.
    const tutoradosMisAsignados: { nombre: string; materia: string }[] = [];

    nuevoEmparejamiento.forEach((fila) => {
      // Para tutorado1: si está asignado y su materia registrada no coincide con la materia del tutor, se retira.
      if (fila.tutorado1 && fila.materiaTutorado1 !== fila.materiaTutor) {
        tutoradosMisAsignados.push({ nombre: fila.tutorado1, materia: fila.materiaTutorado1 });
        fila.tutorado1 = "";
        fila.materiaTutorado1 = "";
      }
      // Para tutorado2
      if (fila.tutorado2 && fila.materiaTutorado2 !== fila.materiaTutor) {
        tutoradosMisAsignados.push({ nombre: fila.tutorado2, materia: fila.materiaTutorado2 });
        fila.tutorado2 = "";
        fila.materiaTutorado2 = "";
      }
    });

    // Para cada tutorado mal asignado, buscar un tutor que dicte su materia y tenga espacio libre.
    tutoradosMisAsignados.forEach(({ nombre, materia }) => {
      const tutorDestino = nuevoEmparejamiento.find(fila =>
        fila.materiaTutor === materia && (!fila.tutorado1 || !fila.tutorado2)
      );

      if (tutorDestino) {
        if (!tutorDestino.tutorado1) {
          tutorDestino.tutorado1 = nombre;
          tutorDestino.materiaTutorado1 = materia;
        } else if (!tutorDestino.tutorado2) {
          tutorDestino.tutorado2 = nombre;
          tutorDestino.materiaTutorado2 = materia;
        }
      }
    });

    setEmparejamientos(nuevoEmparejamiento);
  };

  return (
    <div className="emparejamiento">
      <h2>Emparejamiento</h2>
      <button onClick={iniciarEmparejamiento} style={{ marginRight: "10px" }}>
        Iniciar Emparejamiento
      </button>
      <button onClick={emparejamientoAutomatico} style={{ marginRight: "10px" }}>
        Emparejamiento Automático
      </button>
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
                        <select
                          value={fila.materiaTutor}
                          onChange={(e) => actualizarCampo(rowIndex, "materiaTutor", e.target.value)}
                        >
                          <option value="">Vacío</option>
                          <option value="Ingles">Inglés</option>
                          <option value="Matematicas">Matemáticas</option>
                        </select>
                      </td>
                      <td>
                        <select
                          value={fila.disponibilidad}
                          onChange={(e) => actualizarCampo(rowIndex, "disponibilidad", e.target.value)}
                        >
                          <option value="Entre semana por la mañana">Entre semana por la mañana</option>
                          <option value="Entre semana de 2:00pm - 3:00pm">Entre semana de 2:00pm - 3:00pm</option>
                          <option value="Entre semana de 3:00pm - 4:00pm">Entre semana de 3:00pm - 4:00pm</option>
                          <option value="Entre semana de 4:00pm - 5:00pm">Entre semana de 4:00pm - 5:00pm</option>
                          <option value="Entre semana de 5:00pm - 6:00pm">Entre semana de 5:00pm - 6:00pm</option>
                          <option value="Entre semana de 6:00pm - 8:00pm">Entre semana de 6:00pm - 8:00pm</option>
                          <option value="Sábados en la mañana">Sábados en la mañana</option>
                          <option value="Sábados en la tarde">Sábados en la tarde</option>
                        </select>
                      </td>
                      {[fila.tutorado1, fila.tutorado2].map((tutorado, colIndex) => {
                        const materiaTutorado = colIndex === 0 ? fila.materiaTutorado1 : fila.materiaTutorado2;
                        const tutoradoId = `${rowIndex}-${colIndex}`;
                        const colorClase =
                          materiaTutorado === "Ingles"
                            ? "tutorado-ingles"
                            : materiaTutorado === "Matematicas"
                            ? "tutorado-matematicas"
                            : "";
                        return (
                          <td key={tutoradoId} className={highlightedId === tutoradoId ? "highlight" : ""}>
                            <Draggable draggableId={tutoradoId} index={rowIndex * 2 + colIndex}>
                              {(provided) => (
                                <div
                                  ref={provided.innerRef}
                                  {...provided.draggableProps}
                                  {...provided.dragHandleProps}
                                  className={`draggable-tutorado ${colorClase} ${draggingId === tutoradoId ? "dragging" : ""}`}
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
