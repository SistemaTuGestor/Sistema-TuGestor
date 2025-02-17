
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

  //// Apertura de explorador de archivos para formularios.

  const [folderPath_LEE, setFolderPath_LEE] = useState<string | null>("Ubicación de formularios") ;

  const handleSelectFolder_LEE = async () => {

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
        setFolderPath_LEE ( folderName ) ;

        // Enviar la ruta al backend.
        invoke("reportes_lee_recibir_pathcarpeta", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));
      
      }

    } catch (error) {

      console.error ( "Error al seleccionar la carpeta:",error ) ;

    }

  } ;

  //// Apertura de explorador de archivos para inscripciones.

  const [folderPath_Sponsor, setFolderPath_Sponsor] = useState<string | null>("Ubicación de inscripciones") ;

  const handleSelectFolder_Sponsor = async () => {

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
        setFolderPath_Sponsor ( folderName ) ;

        // Enviar la ruta al backend.
        invoke("reportes_sponsor_recibir_pathcarpeta", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));
      
      }

    } catch (error) {

      console.error ( "Error al seleccionar la carpeta:",error ) ;

    }

  } ;

  //// Nombre de los reportes.

  const [nombreReporteLee, setNombreReporteLee] = useState("");
  
  const confirmarNombreReporteLee = ( ) => {
    invoke("reportes_lee_recibir_nombrereporte", { nombrereporte: nombreReporteLee })
      .then(() => console.log("Nombre del reporte guardado:", nombreReporteLee))
      .catch((err) => console.error("Error al guardar el nombre del reporte:", err));
  } ;

  const [nombreReportePUJ, setNombreReportePUJ] = useState("");
  
  const confirmarNombreReportePUJ = ( ) => {
    invoke("reportes_puj_recibir_nombrereporte", { nombrereporte: nombreReportePUJ })
      .then(() => console.log("Nombre del reporte guardado:", nombreReportePUJ))
      .catch((err) => console.error("Error al guardar el nombre del reporte:", err));
  } ;

  const [nombreReporteColegios, setNombreReporteColegios] = useState("");
  
  const confirmarNombreReporteColegios = ( ) => {
    invoke("reportes_colegios_recibir_nombrereporte", { nombrereporte: nombreReporteColegios })
      .then(() => console.log("Nombre del reporte guardado:", nombreReporteColegios))
      .catch((err) => console.error("Error al guardar el nombre del reporte:", err));
  } ;

  const [nombreReporteConstancias, setNombreReporteConstancias] = useState("");
  
  const confirmarNombreReporteConstancias = ( ) => {
    invoke("reportes_constancias_recibir_nombrereporte", { nombrereporte: nombreReporteConstancias })
      .then(() => console.log("Nombre del reporte guardado:", nombreReporteConstancias))
      .catch((err) => console.error("Error al guardar el nombre del reporte:", err));
  } ;

  const [nombreReporteSponsor, setNombreReporteSponsor] = useState("");
  
  const confirmarNombreReporteSponsor = ( ) => {
    invoke("reportes_sponsor_recibir_nombrereporte", { nombrereporte: nombreReporteSponsor })
      .then(() => console.log("Nombre del reporte guardado:", nombreReporteSponsor))
      .catch((err) => console.error("Error al guardar el nombre del reporte:", err));
  } ;
  
  //// Control de ventana emergente.
  
  const evento_clickGenerar_LEE = async ( ) => {
    
    try {
      const datos = await invoke("leer_archivos_en_carpeta");
      console.log("Datos procesados:", datos);
    } catch (error) {
      console.error("Error al procesar los archivos de la carpeta Qualtrics:", error);
      alert("Hubo un error al generar el reporte.");
    }

    setSeccioonActual ( "LEE" ) ;
    setEmergenteVisible ( true ) ;
  
  } ;

  const [getEmergenteVisible,setEmergenteVisible] = useState ( false ) ;
  const [seccioonActual,setSeccioonActual] = useState ( "" ) ;

  const evento_clickGenerar = async ( seccioon:string ) => {

    try {
      if (seccioonActual === "Colegios") {
        // Leer estudiantes aprobados
        const estudiantesAprobados = await invoke<string[]>("leer_estudiantes_aprobados");
  
        if (estudiantesAprobados.length === 0) {
          alert("No hay tutores aprobados para generar el reporte.");
          return;
        }
  
        // Generar el reporte con la lista de estudiantes aprobados
        await invoke("generar_reporte_colegios", { estudiantes: estudiantesAprobados });
  
        alert("¡Envío exitoso! El reporte de Colegios se ha generado.");
      }
      if (seccioon === "PUJ") {
         // Leer estudiantes aprobados
         const estudiantesAprobados = await invoke<string[]>("leer_universitarios_aprobados");
  
         if (estudiantesAprobados.length === 0) {
           alert("No hay tutores aprobados para generar el reporte.");
           return;
         }
   
         // Generar el reporte con la lista de estudiantes aprobados
         await invoke("generar_reporte_puj", { estudiantes: estudiantesAprobados });
   
         alert("¡Envío exitoso! El reporte de puj se ha generado.");
      }
      else {
        console.log("📌 Otra sección seleccionada, no se generará reporte de colegios.");
      }
    } catch (err) {
      console.error("Error al generar el reporte de colegios:", err);
      alert("Hubo un error al generar el reporte.");
    }
  
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
    // alert ( `¡Envío exitoso!` ) ;
    setEmergenteVisible ( false ) ;
  } ;

  const fileInputRef = useRef <HTMLInputElement|null> (null) ;
  // Handle file selection
  const handleFileChange = ( ) => { } ;
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
            modulo={seccioonActual}
          />
      ) }

      
      <div className="seccioon">
        <div className="tiitulo">
          LEE
        </div>
        <ul className="lista">
          <li>
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
          <li onClick={()=>handleSelectFolder_LEE()}>
            {folderPath_LEE}
          </li>
          <li>
            <input
              type="text"
              placeholder="Nombre del reporte"
              value={nombreReporteLee}
              onChange={(e) => {
                setNombreReporteLee(e.target.value) ;
                confirmarNombreReporteLee() ;
              }}
            />
          </li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickGenerar_LEE()}>
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
          <li>
            <input
              type="text"
              placeholder="Nombre de reportes"
              value={nombreReportePUJ}
              onChange={(e) => {
                setNombreReportePUJ(e.target.value) ;
                confirmarNombreReportePUJ() ;
              }}
            />
          </li>
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
          <li>
            <input
              type="text"
              placeholder="Nombre de reportes"
              value={nombreReporteColegios}
              onChange={(e) => {
                setNombreReporteColegios(e.target.value) ;
                confirmarNombreReporteColegios() ;
              }}
            />
          </li>
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
          <li>
            <input
              type="text"
              placeholder="Nombre de reportes"
              value={nombreReporteConstancias}
              onChange={(e) => {
                setNombreReporteConstancias(e.target.value) ;
                confirmarNombreReporteConstancias() ;
              }}
            />
          </li>
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
          <li onClick={()=>handleSelectFolder_Sponsor()}>
            {folderPath_Sponsor}
          </li>
          <li>
            <input
              type="text"
              placeholder="Nombre del reporte"
              value={nombreReporteSponsor}
              onChange={(e) => {
                setNombreReporteSponsor(e.target.value) ;
                confirmarNombreReporteSponsor() ;
              }}
            />
          </li>
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

