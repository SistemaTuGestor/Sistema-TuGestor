
import "./Monitoreo.css";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";



interface DatosMonitoreoIzq {
  id: string;
  rol: string;
  teleefono: string;
  email: string;
}
interface DatosMonitoreoDer {
  registro: string;
}

function Monitoreo() {

  const [datosIzq, setDatosIzq] = useState<DatosMonitoreoIzq[]>([]);

  useEffect(() => {
    // Fetch data from the backend
    invoke<DatosMonitoreoIzq[]>("monitoreo_izquierda")
      .then((response) => setDatosIzq(response))
      .catch((error) => console.error("Failed to fetch data:", error));
  }, []);

  const [datosDer, setDatosDer] = useState<DatosMonitoreoDer[]>([]);
  const [datosOriginales, setDatosOriginales] = useState<any[]>([]); //Guarda datos originales de las tareas de todos los usuarios 

  useEffect(() => {
    // Fetch data from the backend
    invoke<DatosMonitoreoDer[]>("monitoreo_derecha")
      .then((response) => setDatosDer(response))
      .catch((error) => console.error("Failed to fetch data:", error));
  }, []);

  useEffect(() => {
    const cargarEmparejamiento = async () => {
      try {
        const resultado = await invoke("leer_excel_emparejamiento");
        console.log("Emparejamiento cargado automáticamente:", resultado);
      } catch (error) {
        console.error("Error al cargar el emparejamiento al inicio:", error);
      }
    };

    cargarEmparejamiento();
  }, []);


  useEffect(() => {
    invoke("cargar_datos_json")
      .then((res) => {
        const jsonData = JSON.parse(res as string);
        let contador = 1;

        const mapPersona = (p: any): DatosMonitoreoIzq => ({
          id: `Usuario ${contador++}`,
          rol: p.rol,
          teleefono: Array.isArray(p.telefono) ? p.telefono[0] : p.telefono,
          email: p.correo,
        });

        const personas = [
          ...jsonData.tutores,
          ...jsonData.tutorado1,
          ...jsonData.tutorado2,
        ];

        const datosIzquierda = personas.map(mapPersona);

        setDatosIzq(datosIzquierda);
        setDatosOriginales(personas); // 🔥 Guardamos todo el contenido original
      })
      .catch((err) => {
        console.error("Error cargando datos del JSON:", err);
      });
  }, []);

  const handleCasillaClick = (row: DatosMonitoreoIzq) => {
    const persona = datosOriginales.find(p => p.correo === row.email);
    if (!persona) return;
  
    const nuevasEntradas: DatosMonitoreoDer[] = [];
  
    persona.tareas.forEach((tarea: any) => {
      nuevasEntradas.push({
        registro: `${tarea.nombre}: ${tarea.descripcion}`
      });
    });
  
    nuevasEntradas.push({
      registro: `Imagen: ${persona.imagenes}`
    });
  
    setDatosDer(nuevasEntradas);
  };



  return (

    <div className="monitoreo">
      <div className="contenedor_PanelIzquierdo">
        <div className="opciones-izquierda">
          <select multiple>
            <option value="objetos">Rol</option>
            <option value="opción 1">Tutor</option>
            <option value="opción 2">Prioridad</option>
            <option value="opción 3">Emparejado</option>
            <option value="opción 4">Control</option>
          </select>
          <select multiple>
            <option value="objetos">Institución</option>
            <option value="opción 1">PUJ</option>
            <option value="opción 2">Colegio 1</option>
            <option value="opción 3">Colegio 2</option>
            <option value="opción 4">Colegio 3</option>
          </select>
          <select multiple>
            <option value="objetos">Progreso</option>
            <option value="opción 1">100%</option>
            <option value="opción 2">80%</option>
            <option value="opción 3">60%</option>
            <option value="opción 4">40%</option>
            <option value="opción 5">20%</option>
            <option value="opción 6">0%</option>
            <option value="opción 7">nulo</option>
          </select>
        </div>
        <div className="opciones-izquierda">
          <input
            type="text"
            placeholder="Buscar"
            className="barra-buusqueda"
          />
        </div>
        <div className="desplazadora">
          {datosIzq.map((row, index) => (
            <div
              key={index}
              className="casilla"
              onClick={() => handleCasillaClick(row)}
              style={{ cursor: 'pointer' }}
            >
              <div className="rootulo">
                <p className="id">{`${row.rol}, ${row.id}`}</p>
              </div>
              <p className="contacto">{row.teleefono}</p>
              <p className="contacto">{row.email}</p>
            </div>
          ))}
        </div>
      </div>
      <div className="contenedor_PanelDerecho">
        <div className="desplazadora">
          {datosDer.slice(0).reverse().map((row, index) => (
            <div className="registro">
              <p key={index}>{row.registro}</p>
            </div>
          ))}
        </div>
        <div className="nuevo-registro">
          <button>
            +
          </button>
        </div>
      </div>
    </div>

  );


}


export default Monitoreo;

