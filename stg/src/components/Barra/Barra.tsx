
import "./Barra.css";

import { NavLink } from "react-router-dom";



function Barra ( )  {
  

  return (

    <div className="barra">
      <div className="botoon">
        <NavLink to="/monitoreo" className={({ isActive }) => (isActive ? "active" : "")}>
          <button>Monitoreo</button>
        </NavLink>
      </div>
      <div className="botoon">
        <NavLink to="/notificaciones" className={({ isActive }) => (isActive ? "active" : "")}>
          <button>Notificaciones</button>
        </NavLink>
      </div>
      <div className="botoon">
        <NavLink to="/reportes" className={({ isActive }) => (isActive ? "active" : "")}>
          <button>Reportes</button>
        </NavLink>
      </div>
      <div className="botoon">
        <NavLink to="/emparejamiento" className={({ isActive }) => (isActive ? "active" : "")}>
          <button>Emparejamiento</button>
        </NavLink>
      </div>
    </div>
  
  ) ;


}


export default Barra ;

