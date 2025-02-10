
import "./Monitoreo.css" ;

import { useEffect, useState } from "react" ;
import { invoke } from "@tauri-apps/api/tauri" ;



interface DatosMonitoreoIzq {
  id : string ;
  rol : string ;
  contacto : string ;
}
interface DatosMonitoreoDer {
  registro : string ;
}

function Monitoreo ( ) {

  const [datosIzq,setDatosIzq] = useState<DatosMonitoreoIzq[]>([]);

  useEffect ( () => {
    // Fetch data from the backend
    invoke<DatosMonitoreoIzq[]>("monitoreo_izquierda")
      .then ( (response) => setDatosIzq(response) )
      .catch ( (error) => console.error("Failed to fetch data:", error) ) ;
  }, [] ) ;

  const [datosDer,setDatosDer] = useState<DatosMonitoreoDer[]>([]);

  useEffect ( () => {
    // Fetch data from the backend
    invoke<DatosMonitoreoDer[]>("monitoreo_derecha")
      .then ( (response) => setDatosDer(response) )
      .catch ( (error) => console.error("Failed to fetch data:", error) ) ;
  }, [] ) ;
  

  return (

    <div className="monitoreo">
      <div className="contenedor_PanelIzquierdo">
        <div className="desplazadora">
          {datosIzq.map((row, index) => (
            <div key={index} className="casilla">
              <div className="rootulo">
                <p className="id">{row.id}</p>
                <p className="rol">{row.rol}</p>
              </div>
              <p className="contacto">{row.contacto}</p>
            </div>
          ))}
        </div>
      </div>
      <div className="contenedor_PanelDerecho">
        <div className="desplazadora">
          {datosDer.slice(0).reverse().map((row, index) => (
            <div className="registro">
              <p key={index}>{row.registro}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  
  ) ;


}


export default Monitoreo ;

