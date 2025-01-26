
import "./Reportes.css" ;

import Emergente from "../Emergente/Emergente" ;

import { useRef,useState } from "react";
import { message } from "@tauri-apps/api/dialog";


function Reportes ( ) {

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
    message ( `Abrir explorador de archivos para visualizar reportes` ) ;
    handleFileClick() ;
    setEmergenteVisible ( true ) ;
  } ;
  
  const evento_clickEnviar = ( ) => {
    alert ( `Confirmar envío` ) ;
    setEmergenteVisible ( false ) ;
  } ;

  //// Apertura de explorador de archivos.

  const fileInputRef = useRef <HTMLInputElement|null> (null) ;  // Ref for file input
  const [filePath,setFilePath] = useState <string|null> (null) ;  // State to store the selected file path

  // Handle file selection
  const handleFileChange = ( event:React.ChangeEvent<HTMLInputElement> ) => {
    if ( event.target.files && event.target.files[0] ) {
      const file = event.target.files[0] ;
      if ( file.name.endsWith(".xlsx") ) {
        setFilePath ( file.name ) ;  // Set the file path if the file is valid (.xlsx)
        console.log ( "Selected file path:",file.name ) ;
        alert ( `Selected file: ${file.name}` ) ;
      } else {
        alert ( "Please select a valid .xlsx file." ) ;  // Alert if the file is not .xlsx
        setFilePath ( null ) ;  // Clear file path if an invalid file is selected
      }
    }
  } ;
  // Trigger file selection dialog.
  const handleFileClick = ( ) => {
    fileInputRef.current?.click() ;  // Trigger file input dialog
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
      
      { ["LEE","PUJ","Colegios","Participantes","Sponsor"].map (
        (reporte) => (
          <div className="seccioon" key={reporte}>
            <div className="tiitulo">
              {reporte}
            </div>
            <ul className="lista">
              <li>Archivo</li>
              <li>Archivo</li>
              <li>Archivo</li>
              <li>Archivo</li>
              <li>Archivo</li>
              <li>Info adicional</li>
            </ul>
            <div className="opciones">
              <button onClick={()=>evento_clickGenerar(reporte)}>
                Generar
              </button>
            </div>
          </div>
        )
      ) }

      {/* Hidden file input for file selection */}
      <input
        type="file"
        ref={fileInputRef}
        style={{ display: "none" }}
        accept=".xlsx,application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" // Restrict to .xlsx files
        onChange={handleFileChange}
      />
  
    </div>


  ) ;


}


export default Reportes ;

