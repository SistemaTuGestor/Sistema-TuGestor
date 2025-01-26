
import "./Reportes.css" ;



function Reportes ( ) {

  /* ... */

  //// Constantes de estilo para cada Título de sección.
  const const_styleLEE = {
    color : "#8A2BE2" , border : "2px solid #F18200" ,
    backgroundColor : "white" ,
  } ;
  const const_stylePUJ = {
    color : "#77BFC7" , border : "2px solid #FFC901" ,
    backgroundColor : "white" ,
  } ;
  const const_styleColegios = {
    color : "white" , border : "2px solid white" ,
    backgroundColor : "#F18200" ,
  } ;
  const const_styleParticipantes = {
    color : "#F18200" , border : "2px solid #8A2BE2" ,
    backgroundColor : "white" ,
  } ;
  const const_styleSponsor = {
    color : "black" , border : "2px solid black" ,
    backgroundColor : "#77BFC7" ,
  } ;
  

  return (

    <div className="reportes">
      <div className="seccioon">
        <div className="tiitulo" style={const_styleLEE}>
          LEE
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
          <button>Enviar</button>
        </div>
      </div>
      <div className="seccioon">
        <div className="tiitulo" style={const_stylePUJ}>
          PUJ
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
          <button>Enviar</button>
        </div>
      </div>
      <div className="seccioon">
        <div className="tiitulo" style={const_styleColegios}>
          Colegios
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
          <button>Enviar</button>
        </div>
      </div>
      <div className="seccioon">
        <div className="tiitulo" style={const_styleParticipantes}>
          Participantes
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
          <button>Enviar</button>
        </div>
      </div>
      <div className="seccioon">
        <div className="tiitulo" style={const_styleSponsor}>
          Sponsor
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
          <button>Enviar</button>
        </div>
      </div>
    </div>
  
  ) ;


}


export default Reportes ;

