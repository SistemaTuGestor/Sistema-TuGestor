import "./Barra.css";
import { NavLink } from "react-router-dom";
import { useState } from "react";

function Barra() {
  const [mostrarContenido, setMostrarContenido] = useState(false);

  const toggleContenido = () => {
    setMostrarContenido(!mostrarContenido);
  };

  return (
    <div className="barra">
      <div className="botoon">
        <NavLink 
          to="/monitoreo" 
          className={({ isActive }) => (isActive ? "active-button" : "")}
        >
          <button>Monitoreo</button>
        </NavLink>
      </div>
      <div className="botoon">
        <NavLink 
          to="/notificaciones" 
          className={({ isActive }) => (isActive ? "active-button" : "")}
        >
          <button>Notificaciones</button>
        </NavLink>
      </div>
      <div className="botoon">
        <NavLink 
          to="/reportes" 
          className={({ isActive }) => (isActive ? "active-button" : "")}
        >
          <button>Reportes</button>
        </NavLink>
      </div>
      <div className="botoon">
        <NavLink 
          to="/emparejamiento" 
          className={({ isActive }) => (isActive ? "active-button" : "")}
        >
          <button>Emparejamiento</button>
        </NavLink>
      </div>

      {/* Contenido emergente */}
      <div className={`contenido-hover ${mostrarContenido ? "mostrar" : ""}`}>
        Este es el contenido emergente al presionar "Monitoreo".
      </div>
    </div>
  );
}

export default Barra;
