import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import * as XLSX from "xlsx";
import { saveAs } from "file-saver";
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

  // Cargar datos desde localStorage al iniciar
  useEffect(() => {
    const datosGuardados = localStorage.getItem("emparejamientos");
    if (datosGuardados) {
      setEmparejamientos(JSON.parse(datosGuardados));
    }
  }, []);

  // Guardar en localStorage cada vez que se actualiza la lista
  useEffect(() => {
    if (emparejamientos.length > 0) {
      localStorage.setItem("emparejamientos", JSON.stringify(emparejamientos));
    }
  }, [emparejamientos]);

  const iniciarEmparejamiento = async () => {
    try {
      const data = await invoke<EmparejamientoEntry[]>("obtener_emparejamiento");
      const datosProcesados = data.map(entry => ({
        ...entry,
        materia:
          entry.materia?.trim() === "Ingles" ? "Inglés" :
          entry.materia?.trim() === "Matematicas" ? "Matemáticas" :
          entry.materia?.trim() || "",
      }));
      setEmparejamientos(datosProcesados);
    } catch (error) {
      console.error("Error al obtener emparejamiento:", error);
    }
  };

  // Función para exportar a Excel
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

  return (
    <div className="emparejamiento">
      <h2>Emparejamiento</h2>
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
        <tbody>
          {emparejamientos.map((fila, index) => (
            <tr key={index}>
              <td><input type="text" value={fila.tutor} onChange={(e) => {
                const nuevaLista = [...emparejamientos];
                nuevaLista[index].tutor = e.target.value;
                setEmparejamientos(nuevaLista);
              }}/></td>
              <td>
                <select value={fila.materia} onChange={(e) => {
                  const nuevaLista = [...emparejamientos];
                  nuevaLista[index].materia = e.target.value;
                  setEmparejamientos(nuevaLista);
                }}>
                  <option value="">(Vacío)</option>
                  <option value="Matemáticas">Matemáticas</option>
                  <option value="Inglés">Inglés</option>
                </select>
              </td>
              <td>
                <select value={fila.disponibilidad} onChange={(e) => {
                  const nuevaLista = [...emparejamientos];
                  nuevaLista[index].disponibilidad = e.target.value as "Mañana" | "Tarde" | "Noche";
                  setEmparejamientos(nuevaLista);
                }}>
                  <option value="Mañana">Mañana</option>
                  <option value="Tarde">Tarde</option>
                  <option value="Noche">Noche</option>
                </select>
              </td>
              <td><input type="text" value={fila.tutorado1} onChange={(e) => {
                const nuevaLista = [...emparejamientos];
                nuevaLista[index].tutorado1 = e.target.value;
                setEmparejamientos(nuevaLista);
              }}/></td>
              <td><input type="text" value={fila.tutorado2} onChange={(e) => {
                const nuevaLista = [...emparejamientos];
                nuevaLista[index].tutorado2 = e.target.value;
                setEmparejamientos(nuevaLista);
              }}/></td>
            </tr>
          ))}
        </tbody>
      </table>
      </div>
    </div>
  );
}

export default Emparejamiento;
