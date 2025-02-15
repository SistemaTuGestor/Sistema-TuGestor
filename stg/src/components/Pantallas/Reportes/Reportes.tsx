
import "./Reportes.css" ;

import Emergente from "../Emergente/Emergente" ;

import { useRef,useState,useEffect } from "react" ;
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";



function Reportes ( ) {

  //// Fecha

  const [fechaLee,setFechaLee] = useState("") ;

  useEffect ( ( ) => {
    invoke < {fecha:string} > ( "obtener_fecha" )
      .then ( (response) => setFechaLee(response.fecha) )
      .catch ( (err) => console.error("Failed to fetch date:", err) ) ;
  } , [] ) ;

  const [fechaPUJ,setFechaPUJ] = useState("") ;

  useEffect ( ( ) => {
    invoke < {fecha:string} > ( "obtener_fecha" )
      .then ( (response) => setFechaPUJ(response.fecha) )
      .catch ( (err) => console.error("Failed to fetch date:", err) ) ;
  } , [] ) ;

  const [fechaColegios,setFechaColegios] = useState("") ;

  useEffect ( ( ) => {
    invoke < {fecha:string} > ( "obtener_fecha" )
      .then ( (response) => setFechaColegios(response.fecha) )
      .catch ( (err) => console.error("Failed to fetch date:", err) ) ;
  } , [] ) ;

  //// Apertura de explorador de archivos.

  const [folderPath, setFolderPath] = useState<string | null>("Ubicación de formularios") ;

  const handleSelectFolder = async () => {

    try {

      const selectedPath = await open ( {
        directory : true,  // Permite seleccionar una carpeta.
        multiple : false ,  // Solo permite seleccionar una.
      } ) ;

      if ( typeof selectedPath === "string" ) {

        // Imprimir por consola.
        console.log ( "Carpeta seleccionada:",selectedPath ) ;

        // Imprimir por GUI.
        const folderName = selectedPath.split(/[\\/]/).pop() || "Carpeta seleccionada" ;
        setFolderPath ( folderName ) ;

        // Enviar la ruta al backend.
        invoke("reportes_lee_recibir_pathcarpeta", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));
      
      }

    } catch (error) {

      console.error ( "Error al seleccionar la carpeta:",error ) ;

    }

  } ;

  //// Control de ventana emergente.

  const [getEmergenteVisible,setEmergenteVisible] = useState ( false ) ;
  const [seccioonActual,setSeccioonActual] = useState ( "" ) ;

  const evento_clickGenerar = ( seccioon:string ) => {
    setSeccioonActual ( seccioon ) ;
    setEmergenteVisible ( true ) ;
  } ;

  const evento_clickCancelar = ( ) => {
    setEmergenteVisible ( false ) ;
  }

  const evento_clickVerificar = ( ) => {
    handleFileClick() ;
    setEmergenteVisible ( true ) ;
  } ;
  
  const evento_clickEnviar = ( ) => {
    alert ( `¡Envío exitoso!` ) ;
    setEmergenteVisible ( false ) ;
  } ;

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
              value={fechaLee}
              onChange={(e) => setFechaLee(e.target.value)}
              onBlur={() => {
                invoke("reportes_lee_actualizar_fecha", { nuevaFecha: fechaLee })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={()=>handleSelectFolder()}>
            {folderPath}
          </li>
          <li>Nombre de reportes</li>
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
              value={fechaPUJ}
              onChange={(e) => setFechaPUJ(e.target.value)}
              onBlur={() => {
                invoke("reportes_puj_actualizar_fecha", { nuevaFecha: fechaPUJ })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li>Nombre de reportes</li>
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
              value={fechaColegios}
              onChange={(e) => setFechaColegios(e.target.value)}
              onBlur={() => {
                invoke("reportes_colegios_actualizar_fecha", { nuevaFecha: fechaColegios })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li>Nombre de reportes</li>
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

