
import "./Barra.css" ;

import { NavLink } from "react-router-dom" ;


function Barra ( ) {

  return (

    <div className="barra">

      <div className="botoon">
        <NavLink 
          to="/monitoreo" 
          className={({ isActive }) => (isActive ? "botoon-activo" : "")}
        >
          <button>Monitoreo</button>
        </NavLink>
      </div>
      
      <div className="botoon">
        <NavLink 
          to="/notificaciones" 
          className={({ isActive }) => (isActive ? "botoon-activo" : "")}
        >
          <button>Notificaciones</button>
        </NavLink>
      </div>

      <div className="botoon">
        <NavLink 
          to="/reportes" 
          className={({ isActive }) => (isActive ? "botoon-activo" : "")}
        >
          <button>Reportes</button>
        </NavLink>
      </div>

      <div className="botoon">
        <NavLink 
          to="/emparejamiento" 
          className={({ isActive }) => (isActive ? "botoon-activo" : "")}
        >
          <button>Emparejamiento</button>
        </NavLink>
      </div>

    </div>

  ) ;

}


export default Barra ;

