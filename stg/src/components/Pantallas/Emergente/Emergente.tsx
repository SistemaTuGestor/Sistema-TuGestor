
import "./Emergente.css" ;



interface PropiedadesEmergente {
  mensaje : string ;
  cancelar : () => void ;
  verificar : () => void ;
  enviar : () => void ;
}

function Emergente ( {mensaje,cancelar,verificar,enviar} : PropiedadesEmergente ) {


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

