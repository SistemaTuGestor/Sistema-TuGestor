
import "./App.css" ;

/* COMPONENTES FIJOS */
import Barra from "./components/Barra/Barra" ;
import Contenido from "./components/Contenido/Contenido" ;
/* COMPONENTES DINÁMICOS */
import Monitoreo from "./components/Pantallas/Monitoreo/Monitoreo" ;
import Notificaciones from "./components/Pantallas/Notificaciones/Notificaciones" ;
import Reportes from "./components/Pantallas/Reportes/Reportes" ;
import Emparejamiento from "./components/Pantallas/Emparejamiento/Emparejamiento" ;

import { BrowserRouter as Router , Routes , Route, Navigate } from "react-router-dom" ;



function App ( ) {

  return (

    <Router>
      <div>
        <Barra />
        <Contenido>
          <Routes>
            <Route path="/" element={<Navigate to="/monitoreo" />} />
            <Route path="/monitoreo" element={<Monitoreo />} />
            <Route path="/notificaciones" element={<Notificaciones />} />
            <Route path="/reportes" element={<Reportes />} />
            <Route path="/emparejamiento" element={<Emparejamiento />} />
          </Routes>
        </Contenido>
      </div>
    </Router>
  
  ) ;


}


export default App ;

