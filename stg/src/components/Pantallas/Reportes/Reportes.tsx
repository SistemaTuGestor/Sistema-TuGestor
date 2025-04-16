
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

  const [fechaConstanciasTutores,setFechaConstanciasTutores] = useState("") ;

  useEffect ( ( ) => {
    invoke < {fecha:string} > ( "obtener_fecha" )
      .then ( (response) => setFechaConstanciasTutores(response.fecha) )
      .catch ( (err) => console.error("Failed to fetch date:", err) ) ;
  } , [] ) ;

  const [fechaConstanciasTutorados,setFechaConstanciasTutorados] = useState("") ;

  useEffect ( ( ) => {
    invoke < {fecha:string} > ( "obtener_fecha" )
      .then ( (response) => setFechaConstanciasTutorados(response.fecha) )
      .catch ( (err) => console.error("Failed to fetch date:", err) ) ;
  } , [] ) ;
  
  
  //// Apertura de explorador de archivos para XLSX de Emparejamiento en LEE.


  const [archivoPath_Emparejamiento,setArchivoPath_Emparejamiento] = useState("Ubicación de archivo Emparejamiento") ;
  
  const handleSelectArchivo_Emparejamiento = async ( ) => {

    try {

      const selectedPath = await open ( {
        directory : false ,  // Permite seleccionar archivos.
        multiple : false ,  // Solo permite seleccionar uno.
      } ) ;

      if ( typeof selectedPath === "string" ) {

        // Imprimir por consola.
        console.log ( "Plantilla seleccionada:",selectedPath ) ;

        // Imprimir por GUI.
        setArchivoPath_Emparejamiento ( selectedPath ) ;

        // Enviar la ruta al backend de LEE.
        invoke ( "reportes_lee_recibir_emparejamiento",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
        // Enviar la ruta al backend de Tutorados.
        invoke ( "reportes_tutorados_recibir_emparejamiento",{path:selectedPath} )
        .then ( () => console.log("Ruta enviada correctamente") )
        .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
      
      }

    } catch ( error ) {

      console.error ( "Error al seleccionar la archivo:",error ) ;

    }

  } ;


  //// Apertura de explorador de archivos para formularios.


  const [folderPath_LEE, setFolderPath_LEE] = useState<string | null>("Ubicación de formularios") ;

  const handleSelectFolder_LEE = async ( ) => {

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


  //// Apertura de explorador para lectura de archivo LEE para otros reportes.


  const [archivoPath_LEE,setArchivoPath_LEE] = useState("Ubicación de archivo LEE") ;
  
  const handleSelectArchivo_LEE = async ( ) => {

    try {

      const selectedPath = await open ( {
        directory : false ,  // Permite seleccionar archivos.
        multiple : false ,  // Solo permite seleccionar uno.
      } ) ;

      if ( typeof selectedPath === "string" ) {

        // Imprimir por consola.
        console.log ( "Plantilla seleccionada:",selectedPath ) ;

        // Imprimir por GUI.
        setArchivoPath_LEE ( selectedPath ) ;

        // Enviar la ruta al backend para PUJ.
        invoke ( "reportes_puj_recibir_lee",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
        // Enviar la ruta al backend para Colegios.
        invoke ( "reportes_colegios_recibir_lee",{path:selectedPath} )
        .then ( () => console.log("Ruta enviada correctamente") )
        .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
        // Enviar la ruta al backend para Tutores.
        invoke ( "reportes_tutores_recibir_lee",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;

      }

    } catch ( error ) {

      console.error ( "Error al seleccionar la archivo:",error ) ;

    }

  } ;


  //// Apertura de explorador de archivos para plantilla de PUJ.


  const [plantillaPath_PUJ,setPlantillaPath_PUJ] = useState<string | null>("Ubicación de plantilla") ;

  const handleSelectPlantilla_PUJ = async ( ) => {

    try {

      const selectedPath = await open ( {
        directory : false ,  // Permite seleccionar archivos.
        multiple : false ,  // Solo permite seleccionar uno.
      } ) ;

      if ( typeof selectedPath === "string" ) {

        // Imprimir por consola.
        console.log ( "Plantilla seleccionada:",selectedPath ) ;

        // Imprimir por GUI.
        const fileName = selectedPath.split(/[\\/]/).pop() || "Plantilla seleccionada" ;
        setPlantillaPath_PUJ ( fileName ) ;

        // Enviar la ruta al backend.
        invoke ( "reportes_puj_recibir_pathplantilla",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
      
      }

    } catch ( error ) {

      console.error ( "Error al seleccionar la plantilla:",error ) ;

    }

  } ;
  

  //// Apertura de explorador de archivos para plantilla de Colegios.


  const [plantillaPath_Colegios,setPlantillaPath_Colegios] = useState<string | null>("Ubicación de plantilla") ;

  const handleSelectPlantilla_Colegios = async ( ) => {

    try {

      const selectedPath = await open ( {
        directory : false ,  // Permite seleccionar archivos.
        multiple : false ,  // Solo permite seleccionar uno.
      } ) ;

      if ( typeof selectedPath === "string" ) {

        // Imprimir por consola.
        console.log ( "Plantilla seleccionada:",selectedPath ) ;

        // Imprimir por GUI.
        const fileName = selectedPath.split(/[\\/]/).pop() || "Plantilla seleccionada" ;
        setPlantillaPath_Colegios ( fileName ) ;

        // Enviar la ruta al backend.
        invoke ( "reportes_colegios_recibir_pathplantilla",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
      
      }

    } catch ( error ) {

      console.error ( "Error al seleccionar la plantilla:",error ) ;

    }

  } ;


  //// Apertura de explorador de archivos para plantilla de Tutores.


  const [plantillaPath_ConstanciasTutores,setPlantillaPath_ConstanciasTutores] = useState<string | null>("Ubicación de plantilla") ;

  const handleSelectPlantilla_ConstanciasTutores = async ( ) => {

    try {

      const selectedPath = await open ( {
        directory : false ,  // Permite seleccionar archivos.
        multiple : false ,  // Solo permite seleccionar uno.
      } ) ;

      if ( typeof selectedPath === "string" ) {

        // Imprimir por consola.
        console.log ( "Plantilla seleccionada:",selectedPath ) ;

        // Imprimir por GUI.
        const fileName = selectedPath.split(/[\\/]/).pop() || "Plantilla seleccionada" ;
        setPlantillaPath_ConstanciasTutores ( fileName ) ;

        // Enviar la ruta al backend.
        invoke ( "reportes_constanciastutores_recibir_pathplantilla",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
      
      }

    } catch ( error ) {

      console.error ( "Error al seleccionar la plantilla:",error ) ;

    }

  } ;


  //// Apertura de explorador de archivos para plantilla de Tutorados.


  const [plantillaPath_ConstanciasTutorados,setPlantillaPath_ConstanciasTutorados] = useState<string | null>("Ubicación de plantilla") ;

  const handleSelectPlantilla_ConstanciasTutorados = async ( ) => {

    try {

      const selectedPath = await open ( {
        directory : false ,  // Permite seleccionar una carpeta.
        multiple : false ,  // Solo permite seleccionar una.
      } ) ;

      if ( typeof selectedPath === "string" ) {

        // Imprimir por consola.
        console.log ( "Plantilla seleccionada:",selectedPath ) ;

        // Imprimir por GUI.
        const fileName = selectedPath.split(/[\\/]/).pop() || "Plantilla seleccionada" ;
        setPlantillaPath_ConstanciasTutorados ( fileName ) ;

        // Enviar la ruta al backend.
        invoke ( "reportes_constanciastutorados_recibir_pathplantilla",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
      
      }

    } catch ( error ) {

      console.error ( "Error al seleccionar la plantilla:",error ) ;

    }

  } ;

  
  //// Ubicación de los reportes.


  const [directorioReporteLee,setDirectorioReporteLee] = useState("Directorio del reporte") ;
  
  const [directorioReportePUJ,setDirectorioReportePUJ] = useState("Directorio de reportes") ;
  
  const [directorioReporteColegios,setDirectorioReporteColegios] = useState("Directorio de reportes") ;
  
  const [directorioReporteConstanciasTutores,setDirectorioReporteConstanciasTutores] = useState("Directorio de reportes") ;

  const [directorioReporteConstanciasTutorados,setDirectorioReporteConstanciasTutorados] = useState("Directorio de reportes") ;
  

  //// Nombre de los reportes.


  const [nombreReporteLee,setNombreReporteLee] = useState("Nombre del reporte") ;
  
  const [nombreReportePUJ,setNombreReportePUJ] = useState("Nombre de reportes") ;
  
  const [nombreReporteColegios,setNombreReporteColegios] = useState("Nombre de reportes") ;
  
  const [nombreReporteConstanciasTutores,setNombreReporteConstanciasTutores] = useState("Nombre de reportes") ;

  const [nombreReporteConstanciasTutorados,setNombreReporteConstanciasTutorados] = useState("Nombre de reportes") ;


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

        const filePath = await save ( {
          defaultPath : seccioon ,
          filters : [ { name:"Excel Files" , extensions:["xlsx"] } ]
        } ) ;

        if ( filePath ) {
          // await invoke ( "reportes_lee_actualizarfecha",{nueva_fecha:fechaLee} ) ;
          await invoke ( "reportes_lee_recibir_nombrereporte",{nombrereporte:filePath} ) ;
          await invoke ( "reportes_lee_leer_archivos_en_carpeta" ) ;
          setDirectorioReporteLee ( filePath ) ;
          setNombreReporteLee ( filePath.split(/[\\/]/).pop() || "Nombre del reporte" ) ;
          alert ( `Reporte de `+seccioon+` guardado en: `+filePath ) ;
        } else {
          alert ( `¡Generación de `+seccioon+` cancelada!` ) ;
          return ;
        }
        
      } catch ( error ) {
      
        alert ( `¡Error en opciones de la sección `+seccioon+`!` ) ;
      
      }

    } else if ( seccioon === "PUJ" ) {

      if ( plantillaPath_PUJ === "Ubicación de plantilla" ) {
        alert ( `Por favor, selecciona una plantilla de constancias para tutores antes de generar el reporte de `+seccioon+`.` ) ;
        setEmergenteVisible ( false ) ;
        return ;
      }

      try {

        const filePath = await save ( {
          defaultPath : seccioon ,
          filters : [ { name:"Word Files" , extensions:["docx"] } ]
        } ) ;

        if ( filePath ) {
          // Leer estudiantes aprobados.
          const estudiantesAprobados = await invoke<string[]>("reportes_puj_leer_universitarios_aprobados");
          if ( estudiantesAprobados.length === 0 ) {
            alert ( `No hay tutores aprobados para generar el reporte.` ) ;
            return;
          }
          // await invoke ( "reportes_puj_actualizarfecha",{nueva_fecha:fechaPUJ} ) ;
          await invoke ( "reportes_puj_recibir_nombrereporte",{nombrereporte:filePath} ) ;
          await invoke ( "reporte_puj_generar",{estudiantes:estudiantesAprobados} ) ;
          setDirectorioReportePUJ ( filePath ) ;
          setNombreReportePUJ ( filePath.split(/[\\/]/).pop() || "Nombre de reportes" ) ;
          alert ( `Reporte de `+seccioon+` guardado en: `+filePath ) ;
        } else {
          alert ( `¡Generación de `+seccioon+` cancelada!` ) ;
          return ;
        }

      } catch ( error ) {
      
        alert ( `¡Error en opciones de la sección `+seccioon+`!` ) ;
      
      }
      
    } else if ( seccioon === "Colegios" ) {

      if ( plantillaPath_Colegios === "Ubicación de plantilla" ) {
        alert ( `Por favor, selecciona una plantilla de constancias para tutores antes de generar el reporte de `+seccioon+`.` ) ;
        setEmergenteVisible ( false ) ;
        return ;
      }

      try {

        const filePath = await save ( {
          defaultPath : seccioon ,
          filters : [ { name:"Word Files" , extensions:["docx"] } ]
        } ) ;
        
        if ( filePath ) {
          // Leer estudiantes aprobados
          const estudiantesAprobados = await invoke<string[]>("reportes_colegios_leer_estudiantes_aprobados") ;
          if (estudiantesAprobados.length === 0) {
            alert("No hay tutores aprobados para generar el reporte.");
            return;
          }
          // await invoke ( "reportes_colegios_actualizarfecha",{nueva_fecha:fechaColegios} ) ;
          await invoke ( "reportes_colegios_recibir_nombrereporte",{nombrereporte:filePath} ) ;
          await invoke ("reportes_colegios_generar",{estudiantes:estudiantesAprobados } ) ;
          setDirectorioReporteColegios ( filePath ) ;
          setNombreReporteColegios ( filePath.split(/[\\/]/).pop() || "Nombre de reportes" ) ;
          alert ( `Reporte de `+seccioon+` guardado en: `+filePath ) ;
        } else {
          alert ( `¡Generación de `+seccioon+` cancelada!` ) ;
          return ;
        }

      } catch ( error ) {
      
        alert ( `¡Error en opciones de la sección `+seccioon+`!` ) ;
      
      }

    } else if ( seccioon === "Tutores" ) {

      if ( plantillaPath_ConstanciasTutores === "Ubicación de plantilla" ) {
        alert ( `Por favor, selecciona una plantilla de constancias para tutores antes de generar el reporte de `+seccioon+`.` ) ;
        setEmergenteVisible ( false ) ;
        return ;
      }

      try {

        const dirPath = await open ( {
          directory : true ,  // Permite seleccionar una carpeta.
          multiple : false ,  // Solo permite seleccionar una.
        } ) ;

        if ( dirPath ) {
          // await invoke ( "reportes_constanciastutores_actualizarfecha",{nueva_fecha:fechaConstanciasTutores} ) ;
          await invoke ( "reportes_constanciastutores_recibir_nombrereporte",{nombrereporte:dirPath.toString()} ) ;
          await invoke ( "reportes_constanciastutores_generar" ) ;
          setDirectorioReporteConstanciasTutores ( dirPath.toString() ) ;
          setNombreReporteConstanciasTutores ( "Constancia Tutor" ) ;
          alert ( `Reporte de `+seccioon+` guardado en: `+dirPath ) ;
        } else {
          alert ( `¡Generación de `+seccioon+` cancelada!` ) ;
          return ;
        }

      } catch ( error ) {
      
        alert ( `¡Error en opciones de la sección `+seccioon+`!` ) ;
      
      }

    } else if ( seccioon === "Tutorados" ) {

      if ( plantillaPath_ConstanciasTutorados === "Ubicación de plantilla" ) {
        alert ( `Por favor, selecciona una plantilla de constancias para tutorados antes de generar el reporte de `+seccioon+`.` ) ;
        setEmergenteVisible ( false ) ;
        return ;
      }

      try {

        const dirPath = await open ( {
          directory : true ,  // Permite seleccionar una carpeta.
          multiple : false ,  // Solo permite seleccionar una.
        } ) ;

        if ( dirPath ) {
          // await invoke ( "reportes_constanciastutorados_actualizarfecha",{nueva_fecha:fechaConstanciasTutorados} ) ;
          await invoke ( "reportes_constanciastutorados_recibir_nombrereporte",{nombrereporte:dirPath.toString()} ) ;
          await invoke ( "reportes_constanciastutorados_generar" ) ;
          setDirectorioReporteConstanciasTutorados ( dirPath.toString() ) ;
          setNombreReporteConstanciasTutorados ( "Constancia Tutorado" ) ;
          alert ( `Reporte de `+seccioon+` guardado en: `+dirPath ) ;
        } else {
          alert ( `¡Generación de `+seccioon+` cancelada!` ) ;
          return ;
        }

      } catch ( error ) {
      
        alert ( `¡Error en opciones de la sección `+seccioon+`!` ) ;
      
      }

    } else {

      alert ( `¡Error en la selección de sección!` ) ;
    
    }

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
            mensaje   = {`Opciones para los reportes de ${seccioonActual}.`}
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
                invoke("reportes_lee_actualizarfecha", { nuevaFecha: fechaLee })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={handleSelectArchivo_Emparejamiento} className="hover-underline">
            {archivoPath_Emparejamiento}
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
                invoke("reportes_puj_actualizarfecha", { nuevaFecha: fechaPUJ })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={() => handleSelectArchivo_LEE()} className="hover-underline">
            {archivoPath_LEE}
          </li>
          <li onClick={() => handleSelectPlantilla_PUJ()} className="hover-underline">
            {plantillaPath_PUJ}
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
                invoke("reportes_colegios_actualizarfecha", { nuevaFecha: fechaColegios })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={() => handleSelectArchivo_LEE()} className="hover-underline">
            {archivoPath_LEE}
          </li>
          <li onClick={() => handleSelectPlantilla_Colegios()} className="hover-underline">
            {plantillaPath_Colegios}
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
          Tutores
        </div>
        <ul className="lista">
          <li>
            <input
              type="date"
              value={fechaConstanciasTutores}
              onChange={(e) => setFechaConstanciasTutores(e.target.value)}
              onBlur={() => {
                invoke("reportes_constanciastutores_actualizarfecha", { nuevaFecha: fechaConstanciasTutores })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={() => handleSelectArchivo_LEE()} className="hover-underline">
            {archivoPath_LEE}
          </li>
          <li onClick={() => handleSelectPlantilla_ConstanciasTutores()} className="hover-underline">
            {plantillaPath_ConstanciasTutores}
          </li>
          <li className="base">
            {directorioReporteConstanciasTutores}
          </li>
          <li className="base">
            {nombreReporteConstanciasTutores}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickOpciones("Tutores")}>
            Opciones
          </button>
        </div>
      </div>


      <div className="seccioon">
        <div className="tiitulo">
          Tutorados
        </div>
        <ul className="lista">
          <li>
            <input
              type="date"
              value={fechaConstanciasTutorados}
              onChange={(e) => setFechaConstanciasTutorados(e.target.value)}
              onBlur={() => {
                invoke("reportes_constanciastutorados_actualizarfecha", { nuevaFecha: fechaConstanciasTutorados })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={() => handleSelectArchivo_Emparejamiento()} className="hover-underline">
            {archivoPath_Emparejamiento}
          </li>
          <li onClick={() => handleSelectPlantilla_ConstanciasTutorados()} className="hover-underline">
            {plantillaPath_ConstanciasTutorados}
          </li>
          <li className="base">
            {directorioReporteConstanciasTutorados}
          </li>
          <li className="base">
            {nombreReporteConstanciasTutorados}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={()=>evento_clickOpciones("Tutorados")}>
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

