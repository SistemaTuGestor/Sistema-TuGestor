
import "./Barra.css" ;

import { NavLink } from "react-router-dom" ;
import { invoke } from "@tauri-apps/api/tauri";


function Barra ( ) {

  // Función para invocar leer_excel_path_fijo cuando se haga clic en "Reportes"
  const handleReportesClick = async () => {
    try {
      const datos = await invoke("leer_archivos_en_carpeta"); // Llamar a la nueva función en Rust
      console.log("Datos procesados:", datos);
    } catch (error) {
      console.error("Error al procesar los archivos de la carpeta Qualtrics:", error);
    }
  };
  
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
          onClick={handleReportesClick} // Llamar a la función al hacer clic
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

