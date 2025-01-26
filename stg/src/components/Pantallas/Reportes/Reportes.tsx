
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

