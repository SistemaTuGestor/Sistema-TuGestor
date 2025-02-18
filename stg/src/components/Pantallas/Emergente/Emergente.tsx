
import "./Emergente.css" ;



interface PropiedadesEmergente {
  mensaje : string ;
  cancelar : () => void ;
  generar : () => void ;
  verificar : () => void ;
  enviar : () => void ;
  modulo?: string;
}

function Emergente ( {mensaje,cancelar,generar,verificar,enviar} : PropiedadesEmergente ) {


  return (

    <div className="emergente">
      <div className="ventana">
      
        <p className="mensaje">
            {mensaje}
        </p>
        <div className="botones">
          <button onClick={cancelar} className="cancelar">
            Cancelar
          </button>
          <button onClick={generar} className="generar">
            Generar
          </button>
          <button onClick={verificar} className="verificar">
            Verificar
          </button>
          <button onClick={enviar} className="enviar">
            Enviar
          </button>
        </div>
      </div>
    </div>
  
    ) ;


}


export default Emergente ;

