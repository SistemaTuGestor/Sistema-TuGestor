
import "./Reportes.css" ;

import Emergente from "../Emergente/Emergente" ;

import { useRef,useState } from "react" ;


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
  const [, setFilePath] = useState<string> ("/home/user/Downloads") ;

  // Handle file selection
  const handleFileChange = ( event:React.ChangeEvent<HTMLInputElement> ) => {
    if ( event.target.files && event.target.files[0] ) {
      const file = event.target.files[0] ;
      if ( file.name.endsWith(".pdf") ) {
        setFilePath ( file.name ) ;
        alert ( `Archivo seleccionado: ${file.name}` ) ;
        console.log ( "Ruta de archivo seleccionado:",file.name ) ;
      } else {
        alert ( "Por favor, seleccionar un archivo PDF válido." ) ;
        setFilePath ( "/home/user/Downloads" ) ;
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
        accept="application/pdf"
        onChange={handleFileChange}
      />
  
    </div>


  ) ;


}


export default Reportes ;

