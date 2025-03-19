
import "./Notificaciones.css";
import Inicio from "./Inicio" ;

import { useEffect,useState } from "react" ;
import { invoke } from "@tauri-apps/api/tauri" ;



interface DatosNotificacionesIzq {
  asunto : string ;
  contactos : string ;
}

interface Borrador {
  destinatarios : string[] ;
  asunto : string ;
  mensaje : string ;
}

const estructuras : Record<string,string[]> = {
  TutoresPUJ : ["nombre", "apellido", "correo", "institucion", "telefono", "horas", "tutorados"] ,
  TutoresColegio : ["nombre", "apellido", "correo", "institucion", "telefono", "horas", "tutorados"] ,
  FuncionariosColegio : ["nombre", "correo", "telefono", "institucion"] ,
  TutoradosEmparejados : ["nombre", "correo", "telefono", "id", "colegio", "vocabulario", "gramatica", "escucha", "lectura", "a", "b", "c", "d", "e", "f", "g"] ,
  TutoradosControl : ["nombre", "correo", "telefono", "id", "colegio", "vocabulario", "gramatica", "escucha", "lectura", "a", "b", "c", "d", "e", "f", "g"]
} ;

function Notificaciones ( ) {

  const [datosIzq, setDatosIzq] = useState<DatosNotificacionesIzq[]>([]);
  const [estructurasSeleccionadas, setEstructurasSeleccionadas] = useState<string[]>([]);
  const [atributos, setAtributos] = useState<string[]>([]);
  const [controlData, setControlData] = useState<any[]>([]);
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

  // Botón de inicio.

  const [showInicio, setShowInicio] = useState(false);

  const handleInicioClick = () => {
    setShowInicio(true);
  };

  return (
    <div className="notificaciones">
      <div className="contenedor_PanelIzquierdo">
        <div className="opciones-izquierda">
          <button onClick={handleInicioClick}>
            Inicio
          </button>
          <button>
            +
          </button>
        </div>
        <ul className="desplazadora">
          {datosIzq.map((row, index) => (
            <li key={index} className="casilla">
              <p className="asunto-casilla">{row.asunto}</p>
              <p className="contactos-casilla">{"contactos"}</p>
            </li>
          ))}
        </ul>
      </div>
      <div className="contenedor_PanelDerecho">
        {showInicio ? (
          <Inicio />
        ) : (
          <>
            <div className="opciones-derecha">
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
              <button onClick={handleGuardar}>
                Guardar
              </button>
              <button>
                Enviar
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
  
}


export default Notificaciones ;

