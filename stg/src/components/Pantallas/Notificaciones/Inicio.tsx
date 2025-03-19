
import "./Inicio.css" ;

import { useState } from "react" ;
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog" ;



const Inicio = ( ) => {


  //// ARCHIVO EMPAREJAMIENTO

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

        // Enviar la ruta al backend.
        invoke ( "notificaciones_inicio_emparejamiento",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
      
      }

    } catch ( error ) {

      console.error ( "Error al seleccionar la archivo:",error ) ;

    }

  } ;


  //// ARCHIVO CONTROL

  const [archivoPath_Control,setArchivoPath_Control] = useState("Ubicación de archivo Control") ;

  const handleSelectArchivo_Control = async ( ) => {

    try {

      const selectedPath = await open ( {
        directory : false ,  // Permite seleccionar archivos.
        multiple : false ,  // Solo permite seleccionar uno.
      } ) ;

      if ( typeof selectedPath === "string" ) {

        // Imprimir por consola.
        console.log ( "Plantilla seleccionada:",selectedPath ) ;

        // Imprimir por GUI.
        setArchivoPath_Control ( selectedPath ) ;

        // Enviar la ruta al backend.
        invoke ( "notificaciones_inicio_control",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
      
      }

    } catch ( error ) {

      console.error ( "Error al seleccionar la archivo:",error ) ;

    }

  } ;


  //// ARCHIVO SEGUIMIENTO

  const [archivoPath_Seguimiento,setArchivoPath_Seguimiento] = useState("Ubicación de archivo Seguimiento") ;

  const handleSelectArchivo_Seguimiento = async ( ) => {

    try {

      const selectedPath = await open ( {
        directory : false ,  // Permite seleccionar archivos.
        multiple : false ,  // Solo permite seleccionar uno.
      } ) ;

      if ( typeof selectedPath === "string" ) {

        // Imprimir por consola.
        console.log ( "Plantilla seleccionada:",selectedPath ) ;

        // Imprimir por GUI.
        setArchivoPath_Seguimiento ( selectedPath ) ;

        // Enviar la ruta al backend.
        invoke ( "notificaciones_inicio_seguimiento",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
      
      }

    } catch ( error ) {

      console.error ( "Error al seleccionar la archivo:",error ) ;

    }

  } ;


  //// ARCHIVO LINKS

  const [archivoPath_Links,setArchivoPath_Links] = useState("Ubicación de archivo Links") ;

  const handleSelectArchivo_Links = async ( ) => {

    try {

      const selectedPath = await open ( {
        directory : false ,  // Permite seleccionar archivos.
        multiple : false ,  // Solo permite seleccionar uno.
      } ) ;

      if ( typeof selectedPath === "string" ) {

        // Imprimir por consola.
        console.log ( "Plantilla seleccionada:",selectedPath ) ;

        // Imprimir por GUI.
        setArchivoPath_Links ( selectedPath ) ;

        // Enviar la ruta al backend.
        invoke ( "notificaciones_inicio_links",{path:selectedPath} )
          .then ( () => console.log("Ruta enviada correctamente") )
          .catch ( (err) => console.error("Error al enviar la ruta:",err) ) ;
      
      }

    } catch ( error ) {

      console.error ( "Error al seleccionar la archivo:",error ) ;

    }

  } ;

  
  return (

    <div className="inicio">

      <img src="../dist/Logo.png" className="imagen"/>
      
      <ul className="lista">
        <li onClick={handleSelectArchivo_Emparejamiento}>
            {archivoPath_Emparejamiento}
        </li>
        <li onClick={handleSelectArchivo_Control}>
            {archivoPath_Control}
        </li>
        <li onClick={handleSelectArchivo_Seguimiento}>
            {archivoPath_Seguimiento}
        </li>
        <li onClick={handleSelectArchivo_Links}>
            {archivoPath_Links}
        </li>
      </ul>

    </div>
  
  ) ;


} ;


export default Inicio ;

