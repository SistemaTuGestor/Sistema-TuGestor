
import "./Reportes.css" ;

import Emergente from "../Emergente/Emergente" ;

import { useRef,useState,useEffect } from "react" ;
import { invoke } from "@tauri-apps/api/tauri";


function Reportes ( ) {

  //// Fecha

  const [fecha,setFecha] = useState("") ;

  useEffect ( ( ) => {
    invoke < {fecha:string} > ( "obtener_fecha" )
      .then ( (response) => setFecha(response.fecha) )
      .catch ( (err) => console.error("Failed to fetch date:", err) ) ;
  } , [] ) ;

  //// Control de ventana emergente.

  const [getEmergenteVisible,setEmergenteVisible] = useState ( false ) ;
  const [seccioonActual,setSeccioonActual] = useState ( "" ) ;

  const evento_clickGenerar = ( seccioon:string ) => {
    setSeccioonActual ( seccioon ) ;
    setEmergenteVisible ( true ) ;
  } ;

  const evento_clickCancelar = ( ) => {
    alert ( `Generación cancelada.` ) ;
    setEmergenteVisible ( false ) ;
  }

  const evento_clickVerificar = ( ) => {
    alert ( `Abrir explorador de archivos para visualizar reportes` ) ;
    handleFileClick() ;
    setEmergenteVisible ( true ) ;
  } ;
  
  const evento_clickEnviar = ( ) => {
    alert ( `Confirmar envío` ) ;
    setEmergenteVisible ( false ) ;
  } ;

  //// Apertura de explorador de archivos.

  const fileInputRef = useRef <HTMLInputElement|null> (null) ;
  // Handle file selection
  const handleFileChange = () => { } ;
  // Trigger file selection dialog.
  const handleFileClick = ( ) => {
    fileInputRef.current?.click() ;
  } ;


  return (

    <div className="reportes">

      { getEmergenteVisible && (
          <Emergente
            mensaje = {`¿Ya verificaste los reportes para ${seccioonActual}?`}
            cancelar = {evento_clickCancelar}
            verificar = {evento_clickVerificar}
            enviar = {evento_clickEnviar}
          />
      ) }
      
      <div className="seccioon">
        <div className="tiitulo">
          LEE
        </div>
        <ul className="lista">
          <li>
            {" "}
            <input
              type="date"
              value={fecha}
              onChange={(e) => setFecha(e.target.value)}
              onBlur={() => {
                invoke("actualizar_fecha", { nuevaFecha: fecha })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li>Ubicación de formularios</li>
          <li>Nombre de reportes</li>
          <li>Información adicional</li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickGenerar("LEE")}>
            Generar
          </button>
        </div>
      </div>
      <div className="seccioon">
        <div className="tiitulo">
          PUJ
        </div>
        <ul className="lista">
          <li>
            {" "}
            <input
              type="date"
              value={fecha}
              onChange={(e) => setFecha(e.target.value)}
              onBlur={() => {
                invoke("actualizar_fecha", { nuevaFecha: fecha })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li>Nombre de reportes</li>
          <li>Información adicional</li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickGenerar("PUJ")}>
            Generar
          </button>
        </div>
      </div>
      <div className="seccioon">
        <div className="tiitulo">
          Colegios
        </div>
        <ul className="lista">
          <li>
            {" "}
            <input
              type="date"
              value={fecha}
              onChange={(e) => setFecha(e.target.value)}
              onBlur={() => {
                invoke("actualizar_fecha", { nuevaFecha: fecha })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li>Nombre de reportes</li>
          <li>Información adicional</li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickGenerar("Colegios")}>
            Generar
          </button>
        </div>
      </div>
      <div className="seccioon">
        <div className="tiitulo">
          Constancias
        </div>
        <ul className="lista">
          <li>Nombre de reportes</li>
          <li>Información adicional</li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickGenerar("Participantes")}>
            Generar
          </button>
        </div>
      </div>
      <div className="seccioon">
        <div className="tiitulo">
          Sponsor
        </div>
        <ul className="lista">
          <li>Ubicación de inscripciones</li>
          <li>Nombre de reporte</li>
          <li>Información adicional</li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickGenerar("Sponsor")}>
            Generar
          </button>
        </div>
      </div>

      {/* Hidden file input for file selection */}
      <input
        type="file"
        ref={fileInputRef}
        style={{ display: "none" }}
        accept="application/pdf"
        onChange={handleFileChange}
      />
  
    </div>


  ) ;


}


export default Reportes ;

