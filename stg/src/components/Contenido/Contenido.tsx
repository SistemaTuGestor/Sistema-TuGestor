
import "./Contenido.css";

import { ReactNode } from "react";



function Contenido ( { children } : { children: ReactNode } )  {
  

  return (

    <div className="contenido">
      {children} {/* This renders the current module (e.g., Monitoreo, Notificaciones, etc.) */}
    </div>
  
  );


}


export default Contenido ;

