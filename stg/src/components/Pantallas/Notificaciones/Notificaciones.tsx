
import "./Notificaciones.css" ;

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";



interface DatosNotificacionesIzq {
  asunto : string ;
  contactos : string ;
}
interface DatosNotificacionesDer {
  registro : string ;
}

function Notificaciones ( ) {

  const [datosIzq,setDatosIzq] = useState<DatosNotificacionesIzq[]>([]);
  
    useEffect ( () => {
      // Fetch data from the backend.
      invoke<DatosNotificacionesIzq[]> ( "notificaciones_izquierda" )
        .then ( (response) => setDatosIzq(response) )
        .catch ( (error) => console.error("Failed to fetch data:", error) ) ;
    }, []);
  
    const [datosDer,setDatosDer] = useState<DatosNotificacionesDer[]>([]) ;
  
    useEffect ( () => {
      // Fetch data from the backend.
      invoke<DatosNotificacionesDer[]> ( "notificaciones_derecha" )
        .then ( (response) => setDatosDer(response) )
        .catch ( (error) => console.error("Failed to fetch data:", error) ) ;
    }, [] ) ;


  return (

    <div className="notificaciones">
      <div className="contenedor_PanelIzquierdo">
        <div className="desplazadora">
          {datosIzq.map((row,index) => (
            <div key={index} className="casilla">
              <p className="asunto">{row.asunto}</p>
              <p className="contactos">{row.contactos}</p>
            </div>
          ))}
        </div>
      </div>
      <div className="contenedor_PanelDerecho">
        <div className="opciones">
          <select multiple className="botones-opciones">
            <option value="destinatarios">Destinatario</option>
            <option value="opt-2">Destinatario 2</option>
            <option value="opt-3">Destinatario 3</option>
          </select>
          <select className="botones-opciones">
            <option value="objetos">Objetos</option>
            <option value="opt-2">Opción 2</option>
            <option value="opt-3">Opción 3</option>
            <option value="opt-4">Opción 4</option>
            <option value="opt-5">Opción 5</option>
          </select>
        </div>
        <div className="desplazadora">
          {datosDer.map((row, index) => (
            <div className="registro">
              <p key={index}>{row.registro}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  
  ) ;


}


export default Notificaciones ;

