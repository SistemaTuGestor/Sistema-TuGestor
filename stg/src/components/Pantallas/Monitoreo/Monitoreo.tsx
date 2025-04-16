
import "./Monitoreo.css" ;

import { useEffect, useState } from "react" ;
import { invoke } from "@tauri-apps/api/tauri" ;



interface DatosMonitoreoIzq {
  id : string ;
  rol : string ;
  teleefono : string ;
  email : string ;
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
        <div className="opciones-izquierda">
          <select multiple>
            <option value="objetos">Rol</option>
            <option value="opción 1">Tutor</option>
            <option value="opción 2">Prioridad</option>
            <option value="opción 3">Emparejado</option>
            <option value="opción 4">Control</option>
          </select>
          <select multiple>
            <option value="objetos">Institución</option>
            <option value="opción 1">PUJ</option>
            <option value="opción 2">Colegio 1</option>
            <option value="opción 3">Colegio 2</option>
            <option value="opción 4">Colegio 3</option>
          </select>
          <select multiple>
            <option value="objetos">Progreso</option>
            <option value="opción 1">100%</option>
            <option value="opción 2">80%</option>
            <option value="opción 3">60%</option>
            <option value="opción 4">40%</option>
            <option value="opción 5">20%</option>
            <option value="opción 6">0%</option>
            <option value="opción 7">nulo</option>
          </select>
        </div>
        <div className="opciones-izquierda">
          <input
            type="text"
            placeholder="Buscar"
            className="barra-buusqueda"
          />
        </div>
        <div className="desplazadora">
          {datosIzq.map((row, index) => (
            <div key={index} className="casilla">
              <div className="rootulo">
                <p className="id">{row.id}</p>
                <p className="rol">{row.rol}</p>
              </div>
              <p className="contacto">{row.teleefono}</p>
              <p className="contacto">{row.email}</p>
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

