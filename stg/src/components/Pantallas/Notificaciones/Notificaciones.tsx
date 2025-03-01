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

interface Borrador {
  destinatarios: string[];
  asunto: string;
  mensaje: string;
}

function Notificaciones() {
  const [datosIzq, setDatosIzq] = useState<DatosNotificacionesIzq[]>([]);
  const [datosDer, setDatosDer] = useState<DatosNotificacionesDer[]>([]);
  const [asunto, setAsunto] = useState("");
  const [mensaje, setMensaje] = useState("");
  const [destinatarios, setDestinatarios] = useState<string[]>([]);

  // Fetch data from the backend.
  // useEffect(() => {
  //   invoke<DatosNotificacionesIzq[]>("notificaciones_izquierda")
  //     .then((response) => setDatosIzq(response))
  //     .catch((error) => console.error("Failed to fetch data:", error));
  // }, []);

  // useEffect(() => {
  //   invoke<DatosNotificacionesDer[]>("notificaciones_derecha")
  //     .then((response) => setDatosDer(response))
  //     .catch((error) => console.error("Failed to fetch data:", error));
  // }, []);

  useEffect(() => {
    const cargarHistorial = async () => {
      try {
        const historial = await invoke<Borrador[]>("leer_historial");
        const datosFormateados = historial.map(item => ({
          asunto: item.asunto,
          contactos: item.destinatarios.join(", ")
        }));
        setDatosIzq(datosFormateados);
      } catch (error) {
        console.error("Error al leer el historial:", error);
      }
    };

    cargarHistorial();
  }, []);

  // Maneja el cambio de que destinatario esta en ese momento
  const handleDestinatariosChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const value = event.target.value;
    setDestinatarios(value ? [value] : []);
  };

  const cargarHistorial = async () => {
    try {
      const historial: Borrador[] = await invoke("leer_historial");
      setDatosIzq(historial.map(item => ({
        asunto: item.asunto,
        contactos: item.destinatarios.join(", ") // Convertir array en string
      })));
    } catch (error) {
      console.error("Error al leer el historial:", error);
    }
  };
  
  // Cargar historial al montar el componente
  useEffect(() => {
    cargarHistorial();
  }, []);

  // Guardar en JSON y enviarlo al backend
  const handleGuardar = async () => {
    const data = {
      destinatarios,
      asunto,
      mensaje
    };

    console.log("Datos a enviar:", data);

    try {
      await invoke("guardar_historial", { data });
      alert("Historial guardado con éxito");
  
      // Recargar la lista después de guardar
      const historial = await invoke<Borrador[]>("leer_historial");
      const datosFormateados = historial.map(item => ({
        asunto: item.asunto,
        contactos: item.destinatarios.join(", ")
      }));
      setDatosIzq(datosFormateados);
    } catch (error) {
      console.error("Error al guardar el historial:", error);
    }
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
          <select data-multiselect onChange={handleDestinatariosChange}>
            <option value="">Destinatario</option>
            <option value="Destinatario 1">Destinatario 1</option>
            <option value="Destinatario 2">Destinatario 2</option>
            <option value="Destinatario 3">Destinatario 3</option>
          </select>
          <select>
            <option value="objetos">Objetos</option>
            <option value="opt-2">Opción 2</option>
            <option value="opt-3">Opción 3</option>
            <option value="opt-4">Opción 4</option>
            <option value="opt-5">Opción 5</option>
          </select>
        </div>
        <div className="mensaje">
          <div className="asunto-mensaje">
            <input
              placeholder="Asunto"
              value={asunto}
              onChange={(e) => setAsunto(e.target.value)}
            />
          </div>
          <div className="contenido-mensaje">
            <textarea
              placeholder="Mensaje"
              value={mensaje}
              onChange={(e) => setMensaje(e.target.value)}
            />
          </div>
        </div>
        <div className="botones">
          <button onClick={() => {handleGuardar();}}>
            Guardar
          </button>
          <button>
            Enviar
          </button>
        </div>
      </div>
    </div>
  );
}

export default Notificaciones;
