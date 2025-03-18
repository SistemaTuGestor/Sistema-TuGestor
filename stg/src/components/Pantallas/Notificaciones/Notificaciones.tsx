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
  const [controlData, setControlData] = useState<any[]>([]);

  // Contenido

  const [asunto, setAsunto] = useState("");
  const [mensaje, setMensaje] = useState("");
  const [destinatarios, setDestinatarios] = useState<string[]>([]);

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

  // // Fetch data from the backend.
  // useEffect(() => {
  //   invoke<DatosNotificacionesIzq[]>("notificaciones_izquierda")
  //     .then((response) => setDatosIzq(response))
  //     .catch((error) => console.error("Failed to fetch data:", error));
  // }, []);

  // // Fetch data from the backend.
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

  // Maneja el cambio de que destinatario está en ese momento
  const handleDestinatariosChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const value = event.target.value;
    setDestinatarios(value ? [value] : []);
  };

  // Función para manejar el cambio en la lista de objetos
  const handleObjetoChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selected = event.target.value;
    // Si se selecciona una opción distinta a la opción por defecto "objetos"
    if (selected !== "objetos") {
      setMensaje(prevMensaje => prevMensaje + selected);
    }
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

  // Enviar
  
  // Add new state for loading and QR code
  const [loading, setLoading] = useState(false);
  // const [qrCode, setQrCode] = useState("");

  /*
  // Add this useEffect for QR code monitoring
  useEffect(() => {
    const checkQrCode = async () => {
      try {
        const qr = await invoke<string>("leer_qr_code");
        if (qr) setQrCode(qr);
      } catch (error) {
        console.error("Error reading QR code:", error);
      }
    };
    
    const interval = setInterval(checkQrCode, 1000);
    return () => clearInterval(interval);
  }, []);
  */

  // Update handleEnviar function
  const handleEnviar = async () => {

    setLoading(true);

    try {
      await invoke("enviar_mensaje_whatsapp");  // Invoke Rust without arguments
      alert("Mensajes enviados con éxito");
    } catch (error) {
      console.error("Error:", error);
      alert("Error al enviar mensajes");
    } finally {
      setLoading(false);
    }   

    /*
    if (destinatarios.length === 0) {
      alert("Selecciona al menos un destinatario");
      return;
    }

    setLoading(true);
    try {
      await invoke("enviar_mensaje_whatsapp", {
        destinatarios,
        asunto,
        mensaje,
      });
      alert("Mensajes enviados con éxito");
    } catch (error) {
      console.error("Error:", error);
      alert("Error al enviar mensajes");
    } finally {
      setLoading(false);
    }
    */
   
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
          <select onChange={handleObjetoChange}>
            <option value="objetos">Objetos</option>
            <option value="opción 1">Opción 1</option>
            <option value="opción 2">Opción 2</option>
            <option value="opción 3">Opción 3</option>
            <option value="opción 4">Opción 4</option>
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
          <button onClick={handleEnviar} disabled={loading}>
            {loading ? "Enviando..." : "Enviar"}
          </button>
        </div>
      </div>
    </div>
  );
}


export default Notificaciones ;
