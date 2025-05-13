import "./Notificaciones.css";
import Inicio from "./Inicio";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface TutoresPUJ {
  nombre: string;
  apellido: string;
  correo: string;
  institucion: string;
  telefono: string[];
  horas: string;
  tutorados: string[];
  link: string;
}

interface DatosNotificacionesIzq {
  asunto : string ;
  contactos : string ;
}

interface Borrador {
  destinatarios : string[] ;
  asunto : string ;
  mensaje : string ;
}

const estructuras: Record<string, string[]> = {
  TutoresPUJ: ["nombre", "apellido", "correo", "institucion", "telefono", "horas", "tutorados"],
  TutoresColegio: ["nombre", "apellido", "correo", "institucion", "telefono", "horas", "tutorados"],
  FuncionariosColegio: ["nombre", "correo", "telefono", "institucion"],
  TutoradosEmparejados: ["nombre", "correo", "telefono", "id", "colegio", "vocabulario", "gramatica", "escucha", "lectura", "a", "b", "c", "d", "e", "f", "g"],
  TutoradosControl: ["nombre", "correo", "telefono", "id", "colegio", "vocabulario", "gramatica", "escucha", "lectura", "a", "b", "c", "d", "e", "f", "g"],
};

function Notificaciones() {
  const [datosIzq, setDatosIzq] = useState<DatosNotificacionesIzq[]>([]);
  const [estructurasSeleccionadas, setEstructurasSeleccionadas] = useState<string[]>([]);
  const [atributos, setAtributos] = useState<string[]>([]);
  const [, setControlData] = useState<any[]>([]);
  const [asunto, setAsunto] = useState("");
  const [mensaje, setMensaje] = useState("");
  const [destinatarios, setDestinatarios] = useState<string[]>([]);
  const [modoEdicion, setModoEdicion] = useState(false); // Estado para ver lo que se edita
  const [asuntoOriginal, setAsuntoOriginal] = useState(""); // Para recordar el asunto original a editar

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
  // const handleDestinatariosChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
  //   const value = event.target.value;
  //   setDestinatarios(value ? [value] : []);
  // };

  // Función para manejar el cambio en la lista de objetos
  const handleObjetoChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selected = event.target.value;
    
    if (selected !== "") {
      //Escoge la estructura de origen
      const estructuraOrigen = estructurasSeleccionadas.find(estructura => 
        estructuras[estructura] && estructuras[estructura].includes(selected)
      ) || "";
            
      //Mensaje que se muestra 
      setMensaje(prevMensaje => prevMensaje + " <<" + selected + " " + estructuraOrigen + ">> ");
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
    setDestinatarios(opcionesSeleccionadas);
  };

  // Guardar en JSON y enviarlo al backend
  const handleGuardar = async () => {
    try {
      // Prepara los datos para guardar
      const data = {
        destinatarios,
        asunto,
        mensaje,
        estado: false, // Asegúrate de incluir el estado inicial como `false`
      };

      console.log("Datos a guardar:", data);

      // Llama al backend para guardar el historial
      if (modoEdicion) {
        // Si está en modo edición, actualiza el historial existente
        await invoke("actualizar_historial", {
          asuntoOriginal,
          data,
        });
        alert("Historial actualizado con éxito");
        setModoEdicion(false);
        setAsuntoOriginal("");
      } else {
        // Si no está en modo edición, guarda un nuevo historial
        await invoke("guardar_historial", { data });
        alert("Historial guardado con éxito");
      }

      // Recarga la lista de historiales después de guardar
      const historial = await invoke<Borrador[]>("leer_historial");
      const datosFormateados = historial.map((item) => ({
        asunto: item.asunto,
        contactos: item.destinatarios.join(", "),
      }));
      setDatosIzq(datosFormateados);

      // Limpia el formulario después de guardar
      setAsunto("");
      setMensaje("");
      setDestinatarios([]);
    } catch (error) {
      console.error("Error al guardar el historial:", error);
      alert("Error al guardar el historial: " + error);
    }
  };

const handleCancelarEdicion = () => {
  setModoEdicion(false);
  setAsuntoOriginal("");
  setAsunto("");
  setMensaje("");
  setDestinatarios([]);
};

const handleEliminar = async (asunto: string, event: React.MouseEvent) => {
  // Detener la propagación para evitar que se active el onClick del <li>
  event.stopPropagation();
  event.preventDefault(); // Agregar esto también para asegurarnos
  
  // Ejecutar la confirmación en un setTimeout para separarlo del flujo de eventos
  setTimeout(() => {

      (async () => {
        try {
          await invoke("eliminar_historial", { asunto });
          
          // Notificar éxito
          setTimeout(() => {
            alert("Entrada eliminada con éxito");
          }, 100);
          
          // Actualizar la interfaz
          const historial = await invoke<Borrador[]>("leer_historial");
          const datosFormateados = historial.map(item => ({
            asunto: item.asunto,
            contactos: item.destinatarios.join(", ")
          }));
          setDatosIzq(datosFormateados);
          
          // Reiniciar estado si es necesario
          if (modoEdicion && asuntoOriginal === asunto) {
            setModoEdicion(false);
            setAsuntoOriginal("");
            setAsunto("");
            setMensaje("");
            setDestinatarios([]);
          }
        } catch (error) {
          console.error("Error al eliminar la entrada:", error);
          alert("Error al eliminar la entrada: " + error);
        }
      })();
    
  },); 
};

  //Boton de envio.
  const handleEnviar = async () => {
    try {
      const historiales = await invoke<Borrador[]>("enviar_historiales");
      
      console.log("Historiales enviados:");
      historiales.forEach((item, index) => {
        console.log(`🔹 Historial ${index + 1}`);
        console.log(`   📌 Asunto: ${item.asunto}`);
        console.log(`   ✉️ Destinatarios: ${item.destinatarios.join(", ")}`);
        console.log(`   📝 Mensaje: ${item.mensaje}`);
        console.log("-----------------------------------");
      });
  
      alert("Historiales enviados exitosamente");
    } catch (error) {
      console.error("Error al enviar los historiales:", error);
      alert("Error al enviar los historiales: " + error);
    }
  };
  

  // Botón de inicio.

  const [showInicio, setShowInicio] = useState(false);

  const handleInicioClick = () => {
    setShowInicio(true);
  };

  const handleNuevoClick = () => {
    setAsunto("");
    setMensaje("");
  };

  async function handleCasillaClick(row: DatosNotificacionesIzq): Promise<void> {
    try {
      const historial = await invoke<Borrador[]>("editar_historial", { asunto: row.asunto });
      console.log("Historial recibido:", historial);
      
      if (historial && historial.length > 0) {
        const borrador = historial[0];
        
        setAsunto(borrador.asunto);
        setMensaje(borrador.mensaje);
        setDestinatarios(borrador.destinatarios);

        setModoEdicion(true);
        setAsuntoOriginal(borrador.asunto);
        
        console.log("Formulario actualizado con los datos del historial");
      } else {
        console.log("No se encontró ningún historial con ese asunto");
      }
    } catch (error) {
      console.error("Error al leer el historial:", error);
    }
  }

  return (
    <div className="notificaciones">
      <div className="contenedor_PanelIzquierdo">
        <div className="opciones-izquierda">
          <button onClick={handleInicioClick}>
            Inicio
          </button>
          <button  onClick={handleNuevoClick}>
            +
          </button>
        </div>
        <ul className="desplazadora">
          {datosIzq.map((row, index) => (
            <li key={index} className="casilla" onClick={() => handleCasillaClick(row)}
            style={{ cursor: 'pointer' }}>
              <p className="asunto-casilla">{row.asunto}</p>
              <p className="contactos-casilla">{row.contactos}</p>
              <button onClick={(e) => handleEliminar(row.asunto, e)}>
                Eliminar
              </button>
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
              <select multiple onChange={handleObjetoChange}>
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
              {modoEdicion ? "Actualizar" : "Guardar"}
              </button>
              <button onClick={handleEnviar}>
                Enviar
              </button>
              {modoEdicion && (
                <button onClick={handleCancelarEdicion}>
                  Cancelar edición
                </button>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
  
}


export default Notificaciones ;

