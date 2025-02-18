
import "./Reportes.css" ;

import Emergente from "../Emergente/Emergente" ;

import { useRef,useState,useEffect } from "react" ;
import { invoke } from "@tauri-apps/api/tauri";
import { open,save } from "@tauri-apps/api/dialog";



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

  const [fechaConstancias,setFechaConstancias] = useState("") ;

  useEffect ( ( ) => {
    invoke < {fecha:string} > ( "obtener_fecha" )
      .then ( (response) => setFechaConstancias(response.fecha) )
      .catch ( (err) => console.error("Failed to fetch date:", err) ) ;
  } , [] ) ;

  const [fechaSponsor,setFechaSponsor] = useState("") ;

  useEffect ( ( ) => {
    invoke < {fecha:string} > ( "obtener_fecha" )
      .then ( (response) => setFechaSponsor(response.fecha) )
      .catch ( (err) => console.error("Failed to fetch date:", err) ) ;
  } , [] ) ;


  //// Apertura de explorador de archivos para formularios.


  const [folderPath_LEE, setFolderPath_LEE] = useState<string | null>("Ubicación de formularios") ;

  const handleSelectFolder_LEE = async ( ) => {

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

  const handleSelectFolder_Sponsor = async ( ) => {

    try {

      const selectedPath = await open ( {
        directory : true ,  // Permite seleccionar una carpeta.
        multiple : false ,  // Solo permite seleccionar una.
      } ) ;

      if ( typeof selectedPath === "string" ) {

        // Imprimir por consola.
        console.log ( "Carpeta seleccionada:",selectedPath ) ;

        // Imprimir por GUI.
        const folderName = selectedPath.split(/[\\/]/).pop() || "Carpeta seleccionada" ;
        setFolderPath_Sponsor ( folderName ) ;

        // Enviar la ruta al backend.
        invoke ( "reportes_sponsor_recibir_pathcarpeta",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
      
      }

    } catch (error) {

      console.error ( "Error al seleccionar la carpeta:",error ) ;

    }

  } ;

  
  //// Ubicación de los reportes.


  const [directorioReporteLee, setDirectorioReporteLee] = useState("Directorio del reporte");
  
  const [directorioReportePUJ, setDirectorioReportePUJ] = useState("Directorio de reportes");
  
  const [directorioReporteColegios, setDirectorioReporteColegios] = useState("Directorio de reportes");
  
  const [directorioReporteConstancias, setDirectorioReporteConstancias] = useState("Directorio de reportes");

  const [directorioReporteSponsor, setDirectorioReporteSponsor] = useState("Directorio del reporte");
  

  //// Nombre de los reportes.


  const [nombreReporteLee, setNombreReporteLee] = useState("Nombre del reporte");
  
  const [nombreReportePUJ, setNombreReportePUJ] = useState("Nombre de reportes");
  
  const [nombreReporteColegios, setNombreReporteColegios] = useState("Nombre de reportes");
  
  const [nombreReporteConstancias, setNombreReporteConstancias] = useState("Nombre de reportes");

  const [nombreReporteSponsor, setNombreReporteSponsor] = useState("Nombre del reporte");


  //// Control de ventana emergente.


  const [seccioonActual,setSeccioonActual] = useState ( "" ) ;
  const [getEmergenteVisible,setEmergenteVisible] = useState ( false ) ;

  const evento_clickOpciones = async ( seccioon:string ) => {
    setSeccioonActual ( seccioon ) ;
    setEmergenteVisible ( true ) ;
  }

  const evento_clickCancelar = ( ) => {
    setEmergenteVisible ( false ) ;
  }

  const evento_clickGenerar = async ( seccioon:string ) => {

    if ( seccioon === "LEE" ) {

      if ( folderPath_LEE === "Ubicación de formularios" ) {
        alert ( `Por favor, selecciona un directorio de formularios antes de generar el reporte de `+seccioon+`.` ) ;
        setEmergenteVisible ( false ) ;
        return ;
      }

      try {

        const filePath = await save({
          defaultPath : seccioon+".xlsx" ,
          filters : [ { name:"Excel Files" , extensions:["xlsx"] } ]
        } ) ;

        if ( filePath ) {
          await invoke("reportes_lee_recibir_nombrereporte", { nombrereporte: filePath }) ;
          await invoke ( "leer_archivos_en_carpeta" ) ;
          setDirectorioReporteLee ( filePath ) ;
          setNombreReporteLee ( filePath.split(/[\\/]/).pop() || "Nombre del reporte" ) ;
          alert ( `Reporte de `+seccioon+` guardado en: `+filePath ) ;
        } else {
          alert ( `¡Generación de `+seccioon+` cancelada!` ) ;
          return ;
        }
        
      } catch ( error ) {
      
        alert ( `¡Error al abrir dialogo en sección de `+seccioon+`!` ) ;
      
      }

    } else if ( seccioon === "PUJ" ) {

      try {

        const filePath = await save({
          defaultPath : seccioon ,
          filters : [ { name:"All Files" , extensions:["*"] } ]
        } ) ;

        if ( filePath ) {
          // Leer estudiantes aprobados.
          const estudiantesAprobados = await invoke<string[]>("leer_universitarios_aprobados");
          if ( estudiantesAprobados.length === 0 ) {
            alert ( `No hay tutores aprobados para generar el reporte.` ) ;
            return;
          }
          await invoke ( "generar_reporte_puj",{estudiantes:estudiantesAprobados} ) ;
          setDirectorioReportePUJ ( filePath ) ;
          setNombreReportePUJ ( filePath.split(/[\\/]/).pop() || "Nombre de reportes" ) ;
          alert ( `Reporte de `+seccioon+` guardado en: `+filePath ) ;
        } else {
          alert ( `¡Generación de `+seccioon+` cancelada!` ) ;
          return ;
        }

      } catch ( error ) {
      
        alert ( `¡Error al abrir dialogo en sección de `+seccioon+`!` ) ;
      
      }
      
    } else if ( seccioon === "Colegios" ) {

      try {

        const filePath = await save({
          defaultPath : seccioon ,
          filters : [ { name:"All Files" , extensions:["*"] } ]
        } ) ;
        
        if ( filePath ) {
          // Leer estudiantes aprobados
          const estudiantesAprobados = await invoke<string[]>("leer_estudiantes_aprobados");
          if (estudiantesAprobados.length === 0) {
            alert("No hay tutores aprobados para generar el reporte.");
            return;
          }
          await invoke ("generar_reporte_colegios",{estudiantes:estudiantesAprobados } ) ;
          setDirectorioReporteColegios ( filePath ) ;
          setNombreReporteColegios ( filePath.split(/[\\/]/).pop() || "Nombre de reportes" ) ;
          alert ( `Reporte de `+seccioon+` guardado en: `+filePath ) ;
        } else {
          alert ( `¡Generación de `+seccioon+` cancelada!` ) ;
          return ;
        }

      } catch ( error ) {
      
        alert ( `¡Error al abrir dialogo en sección de `+seccioon+`!` ) ;
      
      }

    } else if ( seccioon === "Constancias" ) {

      try {

        const filePath = await save({
          defaultPath : seccioon ,
          filters : [ { name:"Word Files" , extensions:["docx"] } ]
        } ) ;

        if ( filePath ) {
          await invoke ( "generar_constancias" ) ;
          setDirectorioReporteConstancias ( filePath ) ;
          setNombreReporteConstancias ( filePath.split(/[\\/]/).pop() || "Nombre de reportes" ) ;
          alert ( `Reporte de `+seccioon+` guardado en: `+filePath ) ;
        } else {
          alert ( `¡Generación de `+seccioon+` cancelada!` ) ;
          return ;
        }

      } catch ( error ) {
      
        alert ( `¡Error al abrir dialogo en sección de `+seccioon+`!` ) ;
      
      }

    } else if ( seccioon === "Sponsor" ) {

      if ( folderPath_Sponsor === "Ubicación de inscripciones" ) {
        alert ( `Por favor, selecciona un directorio de inscripciones antes de generar el reporte de `+seccioon+`.` ) ;
        setEmergenteVisible ( false ) ;
        return ;
      }

      try {

        const filePath = await save ( {
          defaultPath : seccioon+".xlsx" ,
          filters : [ { name:"Excel Files" , extensions:["xlsx"] } ]
        } ) ;

        if ( filePath ) {
          setDirectorioReporteSponsor ( filePath ) ;
          setNombreReporteSponsor ( filePath.split(/[\\/]/).pop() || "Nombre del reporte" ) ;
          alert ( `Reporte de `+seccioon+` guardado en: `+filePath ) ;
        } else {
          alert ( `¡Generación de `+seccioon+` cancelada!` ) ;
          return ;
        }

      } catch ( error ) {
      
        alert ( `¡Error al abrir dialogo en sección de `+seccioon+`!` ) ;
      
      }

    } else {

      alert ( `¡Error en la selección de sección!` ) ;
    
    }

    /*
    try {

      if ( seccioon === "LEE" ) {
        try {
          const datos = await invoke("leer_archivos_en_carpeta");
          console.log("Datos procesados:", datos);
          setFolderPath_LEE ( fileName ) ;
        } catch (error) {
          console.error("Error al procesar los archivos de la carpeta Qualtrics:", error);
          alert("Hubo un error al generar el reporte.");
        }
      }

      if (seccioon === "Colegios") {
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
      if (seccioon === "Constancias") {
        try {
            await invoke("generar_constancias");
            alert("¡Envío exitoso! Se han generado las constancias.");
        } catch (err) {
            console.error("Error al generar constancias:", err);
            alert("Hubo un error al generar las constancias.");
        }
      }
      else {
        console.log("📌 Otra sección seleccionada, no se generará reporte de colegios.");
      }

    } catch (err) {
      
      console.error("Error al generar el reporte de colegios:", err);
      alert("Hubo un error al generar el reporte.");
    
    }
    */
  
    setEmergenteVisible ( true ) ;
  
  } ;

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
  const handleFileChange = ( ) => { } ;
  // Trigger file selection dialog.
  const handleFileClick = ( ) => {
    fileInputRef.current?.click() ;
  } ;


  return (
  

    <div className="reportes">


      { getEmergenteVisible && (
          <Emergente
            mensaje   = {`¿Ya verificaste los reportes para ${seccioonActual}?`}
            cancelar  = {evento_clickCancelar}
            generar   = {()=>evento_clickGenerar(seccioonActual)}
            verificar = {evento_clickVerificar}
            enviar    = {evento_clickEnviar}
            modulo    = {seccioonActual}
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
          <li onClick={() => handleSelectFolder_LEE()} className="hover-underline">
            {folderPath_LEE}
          </li>
          <li className="base">
            {directorioReporteLee}
          </li>
          <li className="base">
            {nombreReporteLee}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickOpciones("LEE")}>
            Opciones
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
          <li className="base">
            {directorioReportePUJ}
          </li>
          <li className="base">
            {nombreReportePUJ}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickOpciones("PUJ")}>
            Opciones
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
          <li className="base">
            {directorioReporteColegios}
          </li>
          <li className="base">
            {nombreReporteColegios}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickOpciones("Colegios")}>
            Opciones
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
              type="date"
              value={fechaConstancias}
              onChange={(e) => setFechaConstancias(e.target.value)}
              onBlur={() => {
                invoke("reportes_constancias_actualizar_fecha", { nuevaFecha: fechaConstancias })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li className="base">
            {directorioReporteConstancias}
          </li>
          <li className="base">
            {nombreReporteConstancias}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickOpciones("Constancias")}>
            Opciones
          </button>
        </div>
      </div>


      <div className="seccioon">
        <div className="tiitulo">
          Sponsor
        </div>
        <ul className="lista">
          <li>
            <input
              type="date"
              value={fechaSponsor}
              onChange={(e) => setFechaSponsor(e.target.value)}
              onBlur={() => {
                invoke("reportes_sponsor_actualizar_fecha", { nuevaFecha: fechaSponsor })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={() => handleSelectFolder_Sponsor()} className="hover-underline">
            {folderPath_Sponsor}
          </li>
          <li className="base">
            {directorioReporteSponsor}
          </li>
          <li className="base">
            {nombreReporteSponsor}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickOpciones("Sponsor")}>
            Opciones
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

