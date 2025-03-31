import { useState, useEffect, useMemo } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import * as XLSX from "xlsx";
import { saveAs } from "file-saver";
import {
  DragDropContext,
  Droppable,
  Draggable,
  DropResult,
  DragUpdate,
  DragStart,
} from "@hello-pangea/dnd";
import "./Emparejamiento.css";

// Definir el tipo de datos que se espera del backend (sin pendiente)
type EmparejamientoEntry = {
  tutor: string;
  disponibilidadTutor: string;
  materiaTutor: string;
  tutorado1: string;
  tutorado1_id: string;
  disponibilidadTutorado1: string;
  materiaTutorado1: string;
  colorOriginal1?: string; // Color fijo para tutorado1
  tutorado2: string;
  tutorado2_id: string;
  disponibilidadTutorado2: string;
  materiaTutorado2: string;
  colorOriginal2?: string; // Color fijo para tutorado2
};

// Funciones helper
function removeAccents(str: string): string {
  return str.normalize("NFD").replace(/[\u0300-\u036f]/g, "");
}
function normalize(str: string): string {
  return removeAccents(str).trim().toLowerCase();
}
function calcularColor(materia: string): string {
  const n = normalize(materia);
  if (n === "ingles") return "tutorado-ingles";
  if (n === "matematicas") return "tutorado-matematicas";
  return "";
}

const availabilityMapping: { [key: string]: string } = {
  "vacío": "",
  "Entre semana por la mañana": "0",
  "Entre semana de 2:00pm-3:00pm": "1",
  "Entre semana de 3:00pm-4:00pm": "2",
  "Entre semana de 4:00pm-5:00pm": "3",
  "Entre semana de 5:00pm-6:00pm": "4",
  "Entre semana de 6:00pm-8:00pm": "5",
  "Sábados en la mañana": "6",
  "Sábados en la tarde": "7",
};

const disponibilidadOptions = Object.keys(availabilityMapping);

function Emparejamiento() {
  const [emparejamientos, setEmparejamientos] = useState<EmparejamientoEntry[]>([]);
  const [draggingId, setDraggingId] = useState<string | null>(null);
  const [highlightedId, setHighlightedId] = useState<string | null>(null);

  // Estados para búsqueda y ordenamiento
  const [searchTutor, setSearchTutor] = useState("");
  const [searchTutorado, setSearchTutorado] = useState("");
  const [searchDisponibilidadTutor, setSearchDisponibilidadTutor] = useState("");
  const [searchDisponibilidadTutorado, setSearchDisponibilidadTutorado] = useState("");
  const [sortColumn, setSortColumn] = useState<"tutor" | "materiaTutor" | "disponibilidadTutor" | null>(null);
  const [sortDirection, setSortDirection] = useState<"asc" | "desc">("asc");
  const [searchTutoradoId, setSearchTutoradoId] = useState("");

  // Cargar datos desde localStorage al iniciar
  useEffect(() => {
    const datosGuardados = localStorage.getItem("emparejamientos");
    if (datosGuardados) {
      setEmparejamientos(JSON.parse(datosGuardados));
    }
  }, []);

  // Guardar en localStorage cada vez que cambien los emparejamientos
  useEffect(() => {
    if (emparejamientos.length > 0) {
      localStorage.setItem("emparejamientos", JSON.stringify(emparejamientos));
    }
  }, [emparejamientos]);

  // Obtener datos desde el backend y guardarlos en localStorage
  const iniciarEmparejamiento = async () => {
    try {
      localStorage.removeItem("emparejamientos");
      const data = await invoke<EmparejamientoEntry[]>("obtener_emparejamiento");
      // Asignar el colorOriginal a cada tutorado usando calcularColor
      const dataConColor = data.map((item) => ({
        ...item,
        colorOriginal1: calcularColor(item.materiaTutorado1),
        colorOriginal2: calcularColor(item.materiaTutorado2),
      }));
      setEmparejamientos(dataConColor);
      localStorage.setItem("emparejamientos", JSON.stringify(dataConColor));
    } catch (error) {
      console.error("Error al obtener emparejamiento:", error);
    }
  };

  const exportarAExcel = () => {
    const hojaDatos = emparejamientos.map(
      ({ tutor, materiaTutor, disponibilidadTutor, tutorado1, tutorado2 }) => ({
        Tutor: tutor,
        Materia: materiaTutor,
        Disponibilidad: disponibilidadTutor,
        "Tutorado 1": tutorado1,
        "Tutorado 2": tutorado2,
      })
    );

    const hoja = XLSX.utils.json_to_sheet(hojaDatos);
    const libro = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(libro, hoja, "Emparejamiento");

    const excelBuffer = XLSX.write(libro, { bookType: "xlsx", type: "array" });
    const archivo = new Blob([excelBuffer], {
      type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    });

    saveAs(archivo, "Emparejamiento.xlsx");
  };

  // Función para manejar ordenamiento
  const handleSort = (column: "tutor" | "materiaTutor" | "disponibilidadTutor") => {
    if (sortColumn === column) {
      setSortDirection(sortDirection === "asc" ? "desc" : "asc");
    } else {
      setSortColumn(column);
      setSortDirection("asc");
    }
  };

  // Filtrado y ordenamiento usando useMemo
  const filteredEmparejamientos = useMemo(() => {
    let data = [...emparejamientos];

    // Filtrar por Tutor
    if (searchTutor.trim() !== "") {
      data = data.filter((fila) =>
        fila.tutor.toLowerCase().includes(searchTutor.toLowerCase())
      );
    }
    if (searchTutoradoId.trim() !== "") {
      data = data.filter(
        (fila) =>
          fila.tutorado1_id.toLowerCase().includes(searchTutoradoId.toLowerCase()) ||
          fila.tutorado2_id.toLowerCase().includes(searchTutoradoId.toLowerCase())
      );
    }

    // Filtrar por Tutorados: si alguno de los dos tutorados incluye el texto
    if (searchTutorado.trim() !== "") {
      data = data.filter(
        (fila) =>
          fila.tutorado1.toLowerCase().includes(searchTutorado.toLowerCase()) ||
          fila.tutorado2.toLowerCase().includes(searchTutorado.toLowerCase())
      );
    }
    // Filtrar por disponibilidad del Tutor
    if (searchDisponibilidadTutor !== "") {
      data = data.filter(
        (fila) => fila.disponibilidadTutor === searchDisponibilidadTutor
      );
    }
    // Filtrar por disponibilidad de los Tutorados: si al menos uno coincide
    if (searchDisponibilidadTutorado !== "") {
      data = data.filter(
        (fila) =>
          fila.disponibilidadTutorado1 === searchDisponibilidadTutorado ||
          fila.disponibilidadTutorado2 === searchDisponibilidadTutorado
      );
    }

    if (sortColumn) {
      data.sort((a, b) => {
        const aValue = a[sortColumn].toLowerCase();
        const bValue = b[sortColumn].toLowerCase();
        if (aValue < bValue) return sortDirection === "asc" ? -1 : 1;
        if (aValue > bValue) return sortDirection === "asc" ? 1 : -1;
        return 0;
      });
    }

    return data;
  }, [
    emparejamientos,
    searchTutor,
    searchTutorado,
    searchTutoradoId,
    searchDisponibilidadTutor,
    searchDisponibilidadTutorado,
    sortColumn,
    sortDirection,
  ]);

  // DRAG & DROP
  const handleDragStart = (start: DragStart) => {
    setDraggingId(start.draggableId);
    setHighlightedId(null);
  };

  const handleDragUpdate = (update: DragUpdate) => {
    if (!update.destination) return;
    setHighlightedId(update.destination.droppableId);
  };

  // Intercambio de tutorados sin lógica pendiente
  const handleDragEnd = (result: DropResult) => {
    setDraggingId(null);
    setHighlightedId(null);
    if (!result.destination) return;

    const { source, destination } = result;
    const sourceRow = Math.floor(source.index / 2);
    const destRow = Math.floor(destination.index / 2);
    const sourceCol = source.index % 2 === 0 ? "tutorado1" : "tutorado2";
    const destCol = destination.index % 2 === 0 ? "tutorado1" : "tutorado2";

    const newEmparejamientos = [...emparejamientos];

    // Función helper para extraer la data completa del tutorado
    const getTutoradoData = (
      rowIndex: number,
      col: "tutorado1" | "tutorado2"
    ) => ({
      nombre: newEmparejamientos[rowIndex][col],
      materia:
        col === "tutorado1"
          ? newEmparejamientos[rowIndex].materiaTutorado1
          : newEmparejamientos[rowIndex].materiaTutorado2,
      disponibilidad:
        col === "tutorado1"
          ? newEmparejamientos[rowIndex].disponibilidadTutorado1
          : newEmparejamientos[rowIndex].disponibilidadTutorado2,
      color:
        col === "tutorado1"
          ? newEmparejamientos[rowIndex].colorOriginal1
          : newEmparejamientos[rowIndex].colorOriginal2,
    });

    const sourceData = getTutoradoData(sourceRow, sourceCol);
    const destData = getTutoradoData(destRow, destCol);

    // Intercambio: asignar la data de destino en la posición de origen
    newEmparejamientos[sourceRow][sourceCol] = destData.nombre;
    if (sourceCol === "tutorado1") {
      newEmparejamientos[sourceRow].materiaTutorado1 = destData.materia;
      newEmparejamientos[sourceRow].disponibilidadTutorado1 = destData.disponibilidad;
      newEmparejamientos[sourceRow].colorOriginal1 = destData.color;
    } else {
      newEmparejamientos[sourceRow].materiaTutorado2 = destData.materia;
      newEmparejamientos[sourceRow].disponibilidadTutorado2 = destData.disponibilidad;
      newEmparejamientos[sourceRow].colorOriginal2 = destData.color;
    }

    // Intercambio: asignar la data de origen en la posición de destino
    newEmparejamientos[destRow][destCol] = sourceData.nombre;
    if (destCol === "tutorado1") {
      newEmparejamientos[destRow].materiaTutorado1 = sourceData.materia;
      newEmparejamientos[destRow].disponibilidadTutorado1 = sourceData.disponibilidad;
      newEmparejamientos[destRow].colorOriginal1 = sourceData.color;
    } else {
      newEmparejamientos[destRow].materiaTutorado2 = sourceData.materia;
      newEmparejamientos[destRow].disponibilidadTutorado2 = sourceData.disponibilidad;
      newEmparejamientos[destRow].colorOriginal2 = sourceData.color;
    }

    // Si alguna celda quedó vacía, se reemplaza con un ítem "nuevo" (sin datos heredados)
    const reinitializeCell = (
      rowIndex: number,
      col: "tutorado1" | "tutorado2"
    ) => {
      newEmparejamientos[rowIndex][col] = "";
      if (col === "tutorado1") {
        newEmparejamientos[rowIndex].materiaTutorado1 = "VACÍO";
        newEmparejamientos[rowIndex].disponibilidadTutorado1 = "VACÍO";
        newEmparejamientos[rowIndex].colorOriginal1 = "";
      } else {
        newEmparejamientos[rowIndex].materiaTutorado2 = "VACÍO";
        newEmparejamientos[rowIndex].disponibilidadTutorado2 = "VACÍO";
        newEmparejamientos[rowIndex].colorOriginal2 = "";
      }
    };

    if (newEmparejamientos[sourceRow][sourceCol].trim() === "") {
      reinitializeCell(sourceRow, sourceCol);
    }
    if (newEmparejamientos[destRow][destCol].trim() === "") {
      reinitializeCell(destRow, destCol);
    }

    setEmparejamientos(newEmparejamientos);
  };

  // Actualizar select de materia o disponibilidad del tutor
  const actualizarCampo = (
    index: number,
    campo: "materiaTutor" | "disponibilidadTutor",
    valor: string
  ) => {
    const nuevaLista = [...emparejamientos];
    nuevaLista[index][campo] = valor;
    setEmparejamientos(nuevaLista);
  };

  // EMPAREJAMIENTO AUTOMÁTICO
  const emparejamientoAutomatico = () => {
    if (draggingId) return;

    const nuevoEmparejamiento = emparejamientos.map((row) => ({ ...row }));

    // Lista de tutorados pendientes (se extraen y se vacían para evitar duplicados)
    const tutoradosPendientes: {
      nombre: string;
      materia: string;
      disponibilidad: string;
      columna: 1 | 2;
    }[] = [];

    nuevoEmparejamiento.forEach((fila) => {
      if (
        fila.tutorado1 &&
        (normalize(fila.materiaTutorado1) !== normalize(fila.materiaTutor) ||
          fila.disponibilidadTutorado1 !== fila.disponibilidadTutor)
      ) {
        tutoradosPendientes.push({
          nombre: fila.tutorado1,
          materia: fila.materiaTutorado1,
          disponibilidad: fila.disponibilidadTutorado1,
          columna: 1,
        });
        // Vaciamos para evitar duplicados
        fila.tutorado1 = "";
        fila.tutorado1_id = "";
        fila.materiaTutorado1 = "VACÍO";
        fila.disponibilidadTutorado1 = "VACÍO";
        fila.colorOriginal1 = "";
      }

      if (
        fila.tutorado2 &&
        (normalize(fila.materiaTutorado2) !== normalize(fila.materiaTutor) ||
          fila.disponibilidadTutorado2 !== fila.disponibilidadTutor)
      ) {
        tutoradosPendientes.push({
          nombre: fila.tutorado2,
          materia: fila.materiaTutorado2,
          disponibilidad: fila.disponibilidadTutorado2,
          columna: 2,
        });
        fila.tutorado2 = "";
        fila.tutorado2_id = "";
        fila.materiaTutorado2 = "VACÍO";
        fila.disponibilidadTutorado2 = "VACÍO";
        fila.colorOriginal2 = "";
      }
    });

    tutoradosPendientes.forEach(({ nombre, materia, disponibilidad, columna }) => {
      const normalizedMateria = normalize(materia);
      const tutorDestino = nuevoEmparejamiento.find((fila) => {
        return (
          normalize(fila.materiaTutor) === normalizedMateria &&
          fila.disponibilidadTutor === disponibilidad &&
          ((columna === 1 && fila.tutorado1 === "") ||
            (columna === 2 && fila.tutorado2 === ""))
        );
      });

      if (tutorDestino) {
        if (columna === 1) {
          tutorDestino.tutorado1 = nombre;
          tutorDestino.materiaTutorado1 = materia;
          tutorDestino.disponibilidadTutorado1 = disponibilidad;
          tutorDestino.colorOriginal1 = calcularColor(materia);
        } else {
          tutorDestino.tutorado2 = nombre;
          tutorDestino.materiaTutorado2 = materia;
          tutorDestino.disponibilidadTutorado2 = disponibilidad;
          tutorDestino.colorOriginal2 = calcularColor(materia);
        }
      } else {
        // Crear nueva fila solo si el nombre no es vacío
        if (nombre.trim() !== "" && nombre !== "N/A") {
          const yaExiste = nuevoEmparejamiento.some(
            (fila) =>
              fila.tutor === "" &&
              (fila.tutorado1 === nombre || fila.tutorado2 === nombre)
          );
          if (!yaExiste) {
            if (columna === 1) {
              nuevoEmparejamiento.push({
                tutor: "",
                disponibilidadTutor: "",
                materiaTutor: "", // materia vacía para nueva fila
                tutorado1: nombre,
                tutorado1_id: "",
                disponibilidadTutorado1: disponibilidad,
                materiaTutorado1: materia,
                colorOriginal1: calcularColor(materia),
                tutorado2: "",
                tutorado2_id: "",
                disponibilidadTutorado2: "VACÍO",
                materiaTutorado2: "VACÍO",
                colorOriginal2: "",
              });
            } else {
              nuevoEmparejamiento.push({
                tutor: "",
                disponibilidadTutor: "",
                materiaTutor: "", // materia vacía para nueva fila
                tutorado1: "",
                tutorado1_id: "",
                disponibilidadTutorado1: "VACÍO",
                materiaTutorado1: "VACÍO",
                colorOriginal1: "",
                tutorado2: nombre,
                tutorado2_id: "",
                disponibilidadTutorado2: disponibilidad,
                materiaTutorado2: materia,
                colorOriginal2: calcularColor(materia),
              });
            }
          }
        }
      }
    });

    // Aquí ya no llamamos a reinitializeCell para todas las filas

    // Filtrar: eliminar filas que tengan disponibilidad y materia del tutor vacíos o "VACÍO"
    // y que además tengan ambos tutorados vacíos o "VACÍO".
    // En las filas que se van a eliminar, se borra el id (aunque al final no se incluyen en el arreglo final).
    const filtrado = nuevoEmparejamiento.filter((fila) => {
      const disponibilidadVacia =
        fila.disponibilidadTutor.trim().toUpperCase() === "VACÍO" ||
        fila.disponibilidadTutor.trim() === "";
      const materiaVacia =
        fila.materiaTutor.trim().toUpperCase() === "VACÍO" ||
        fila.materiaTutor.trim() === "";
      const tutorado1Vacio =
        fila.tutorado1.trim() === "" ||
        fila.tutorado1.trim().toUpperCase() === "VACÍO";
      const tutorado2Vacio =
        fila.tutorado2.trim() === "" ||
        fila.tutorado2.trim().toUpperCase() === "VACÍO";

      const eliminarFila = disponibilidadVacia && materiaVacia && tutorado1Vacio && tutorado2Vacio;
      if (eliminarFila) {
        // Borrar los id de esta fila
        fila.tutorado1_id = "";
        fila.tutorado2_id = "";
      }
      return !eliminarFila;
    });

    console.log("Nuevo emparejamiento:", filtrado);
    setEmparejamientos(filtrado);
  };





  return (
    <div className="emparejamiento">
     
      <div
        style={{
          display: "flex",
          gap: "10px",
          justifyContent: "center",
          marginBottom: "10px"
        }}
      >
        <button
          onClick={iniciarEmparejamiento}
          style={{
            flex: "1 1 200px",
            padding: "10px",
            fontSize: "16px"
          }}
        >
          Iniciar Emparejamiento
        </button>
        <button
          onClick={emparejamientoAutomatico}
          style={{
            flex: "1 1 200px",
            padding: "10px",
            fontSize: "16px"
          }}
        >
          Emparejamiento Automático
        </button>
        <button
          onClick={exportarAExcel}
          style={{
            flex: "1 1 200px",
            padding: "10px",
            fontSize: "16px"
          }}
        >
          Exportar a Excel
        </button>
        <div
          className="search-bar"
          style={{
            display: "flex",
            flexWrap: "wrap",
            alignItems: "center",
            gap: "10px",
            maxWidth: "800px",
            margin: "10px auto"
          }}
        >
          <input
            type="text"
            placeholder="Buscar Tutor"
            value={searchTutor}
            onChange={(e) => setSearchTutor(e.target.value)}
            style={{ flex: "1 1 150px", maxWidth: "200px" }}
          />
          <input
            type="text"
            placeholder="Buscar Tutorados"
            value={searchTutorado}
            onChange={(e) => setSearchTutorado(e.target.value)}
            style={{ flex: "1 1 150px", maxWidth: "200px" }}
          />
          <input
            type="text"
            placeholder="Buscar ID Tutorados"
            value={searchTutoradoId}
            onChange={(e) => setSearchTutoradoId(e.target.value)}
            style={{ flex: "1 1 150px", maxWidth: "200px" }}
          />
          <select
            value={searchDisponibilidadTutor}
            onChange={(e) => setSearchDisponibilidadTutor(e.target.value)}
            style={{ flex: "1 1 150px", maxWidth: "200px" }}
          >
            <option value="">Disponibilidad Tutor: Todos</option>
            {disponibilidadOptions.map((opt) => (
              <option key={opt} value={opt}>
                {opt}
              </option>
            ))}
          </select>
          <select
            value={searchDisponibilidadTutorado}
            onChange={(e) => setSearchDisponibilidadTutorado(e.target.value)}
            style={{ flex: "1 1 150px", maxWidth: "200px" }}
          >
            <option value="">Disponibilidad Tutorados: Todos</option>
            {disponibilidadOptions.map((opt) => (
              <option key={opt} value={opt}>
                {opt}
              </option>
            ))}
          </select>
        </div>
      </div>

      <div className="table-container">
        <table>
          <thead>
            <tr>
              <th onClick={() => handleSort("tutor")}>
                Tutor {sortColumn === "tutor" && (sortDirection === "asc" ? "▲" : "▼")}
              </th>
              <th onClick={() => handleSort("materiaTutor")}>
                Materia {sortColumn === "materiaTutor" && (sortDirection === "asc" ? "▲" : "▼")}
              </th>
              <th onClick={() => handleSort("disponibilidadTutor")}>
                Disponibilidad {sortColumn === "disponibilidadTutor" && (sortDirection === "asc" ? "▲" : "▼")}
              </th>
              <th>Tutorado 1</th>
              <th>Tutorado 2</th>
            </tr>
          </thead>
          <DragDropContext
            onDragStart={handleDragStart}
            onDragUpdate={handleDragUpdate}
            onDragEnd={handleDragEnd}
          >
            <Droppable droppableId="tutorados" direction="vertical">
              {(provided) => (
                <tbody ref={provided.innerRef} {...provided.droppableProps}>
                  {filteredEmparejamientos.map((fila, rowIndex) => (
                    <tr key={rowIndex}>
                      <td>{fila.tutor}</td>
                      <td>
                        <select
                          value={fila.materiaTutor}
                          onChange={(e) =>
                            actualizarCampo(rowIndex, "materiaTutor", e.target.value)
                          }
                        >
                          <option value="">Vacío</option>
                          <option value="Ingles">Inglés</option>
                          <option value="ingles">Inglés</option>
                          <option value="Matematicas">Matemáticas</option>
                          <option value="matematicas">Matemáticas</option>
                        </select>
                      </td>
                      <td>
                        <select
                          value={fila.disponibilidadTutor}
                          onChange={(e) =>
                            actualizarCampo(rowIndex, "disponibilidadTutor", e.target.value)
                          }
                        >
                          {disponibilidadOptions.map((opt, index) => (
                            <option key={index} value={opt}>
                              {index}. {opt}
                            </option>
                          ))}
                        </select>
                      </td>
                      {[fila.tutorado1, fila.tutorado2].map((tutorado, colIndex) => {
                        const tutoradoId = `${rowIndex}-${colIndex}`;
                        const dispNumber = availabilityMapping[
                          colIndex === 0 ? fila.disponibilidadTutorado1 : fila.disponibilidadTutorado2
                        ];
                        const realId = colIndex === 0 ? fila.tutorado1_id : fila.tutorado2_id;
                        const tutoradoDisplay = tutorado.trim()
                          ? dispNumber
                            ? `-nombre: ${tutorado} \n id ${realId} \n disp(${dispNumber})`
                            : tutorado
                          : "VACÍO";
                        return (
                          <td key={tutoradoId} className={highlightedId === tutoradoId ? "highlight" : ""}>
                            <Draggable draggableId={tutoradoId} index={rowIndex * 2 + colIndex}>
                              {(provided) => (
                                <div
                                  ref={provided.innerRef}
                                  {...provided.draggableProps}
                                  {...provided.dragHandleProps}
                                  className={`draggable-tutorado ${colIndex === 0 ? fila.colorOriginal1 : fila.colorOriginal2
                                    } ${draggingId === tutoradoId ? "dragging" : ""}`}
                                >
                                  {tutoradoDisplay}
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
