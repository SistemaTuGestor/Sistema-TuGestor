import "./Notificaciones.css";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface DatosNotificacionesIzq {
  asunto: string;
  contactos: string;
}

interface DatosNotificacionesDer {
  registro: string;
}

const estructuras: Record<string, string[]> = {
  TutoresPUJ: ["nombre", "apellido", "correo", "institucion", "telefono", "horas", "tutorados"],
  TutoresColegio: ["nombre", "apellido", "correo", "institucion", "telefono", "horas", "tutorados"],
  FuncionariosColegio: ["nombre", "correo", "telefono", "institucion"],
  TutoradosEmparejados: ["nombre", "correo", "telefono", "id", "colegio", "vocabulario", "gramatica", "escucha", "lectura", "a", "b", "c", "d", "e", "f", "g"],
  TutoradosControl: ["nombre", "correo", "telefono", "id", "colegio", "vocabulario", "gramatica", "escucha", "lectura", "a", "b", "c", "d", "e", "f", "g"]
};

function Notificaciones() {
  const [datosIzq, setDatosIzq] = useState<DatosNotificacionesIzq[]>([]);
  const [datosDer, setDatosDer] = useState<DatosNotificacionesDer[]>([]);
  const [estructurasSeleccionadas, setEstructurasSeleccionadas] = useState<string[]>([]);
  const [atributos, setAtributos] = useState<string[]>([]);
  const [controlData, setControlData] = useState<any[]>([]);

  useEffect(() => {
    invoke("leer_archivo_emparejados")
      .then(() => console.log("Archivo de emparejados leído correctamente."))
      .catch((error) => console.error("Error al leer el archivo:", error));
  }, []);

  useEffect(() => {
    invoke<any[]>("leer_archivo_control")
      .then((response: any[]) => {
        console.log("Archivo de control leído correctamente.");
        setControlData(response);
      })
      .catch((error) => console.error("Error al leer el archivo de control:", error));
  }, []);

  useEffect(() => {
    invoke<DatosNotificacionesIzq[]>("notificaciones_izquierda")
      .then((response) => setDatosIzq(response))
      .catch((error) => console.error("Failed to fetch data:", error));
  }, []);

  useEffect(() => {
    invoke<DatosNotificacionesDer[]>("notificaciones_derecha")
      .then((response) => setDatosDer(response))
      .catch((error) => console.error("Failed to fetch data:", error));
  }, []);

  useEffect(() => {
    if (estructurasSeleccionadas.length > 0) {
      const nuevosAtributos = estructurasSeleccionadas.flatMap((estructura) => estructuras[estructura] || []);
      setAtributos([...new Set(nuevosAtributos)]);
    } else {
      setAtributos([]);
    }
  }, [estructurasSeleccionadas]);

  const handleSeleccionDestinatario = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const opcionesSeleccionadas = Array.from(e.target.selectedOptions, (option) => option.value);
    setEstructurasSeleccionadas(opcionesSeleccionadas);
  };

  return (
    <div className="notificaciones">
      <div className="contenedor_PanelIzquierdo">
        <div className="desplazadora">
          {datosIzq.map((row, index) => (
            <div key={index} className="casilla">
              <p className="asunto-casilla">{row.asunto}</p>
              <p className="contactos-casilla">{row.contactos}</p>
            </div>
          ))}
        </div>
      </div>
      <div className="contenedor_PanelDerecho">
        <div className="opciones">
          <select multiple onChange={handleSeleccionDestinatario}>
            {Object.keys(estructuras).map((estructura) => (
              <option key={estructura} value={estructura}>{estructura}</option>
            ))}
          </select>
          <select multiple>
            <option value="">Seleccionar Objeto</option>
            {atributos.length > 0 ? (
              atributos.map((atributo) => (
                <option key={atributo} value={atributo}>{atributo}</option>
              ))
            ) : (
              <option disabled>No hay atributos disponibles</option>
            )}
          </select>
        </div>
        <div className="mensaje">
          <div className="asunto-mensaje">
            <input placeholder="Asunto" />
          </div>
          <div className="contenido-mensaje">
            <textarea placeholder="Mensaje"></textarea>
          </div>
        </div>
        <div className="botones">
          <button>Guardar</button>
          <button>Enviar</button>
        </div>
      </div>
    </div>
  );
}

export default Notificaciones;
