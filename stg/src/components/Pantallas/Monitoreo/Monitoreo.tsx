
import "./Monitoreo.css";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import EmergenteMonitoreo from "./EmergenteMonitoreo/EmergenteMonitoreo";


interface DatosMonitoreoIzq {
  id: string;
  rol: string;
  teleefono: string;
  email: string;
  nombre: string;
  institucion: string;
}

interface DatosMonitoreoDer {
  registro: string;
  esImagen?: boolean;
  urlImagen?: string;
  imagenData?: string; // Para almacenar datos base64 si es necesario
}

const ImagenPreview = ({ imagePath }: { imagePath: string }) => {
  const [imageData, setImageData] = useState<string | null>(null);

  useEffect(() => {
    const loadImage = async () => {
      try {
        // Usamos Tauri para leer el archivo como binario
        const imageBytes: number[] = await invoke("leer_archivo_imagen", {
          path: imagePath
        });

        // Convertimos los bytes a base64
        const binaryString = imageBytes.map(byte => String.fromCharCode(byte)).join('');
        const base64 = btoa(binaryString);
        setImageData(`data:image/jpeg;base64,${base64}`);
      } catch (error) {
        console.error("Error al cargar la imagen:", error);
        setImageData(null);
      }
    };

    if (imagePath) {
      loadImage();
    }
  }, [imagePath]);

  if (!imageData) {
    return (
      <div style={{ padding: '10px', color: '#666', fontStyle: 'italic' }}>
        Cargando imagen...
        <div style={{ fontSize: '0.8em' }}>{imagePath.split(/[\\/]/).pop()}</div>
      </div>
    );
  }

  return (
    <div style={{
      maxWidth: '200px',
      maxHeight: '200px',
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center'
    }}>
      <img
        src={imageData}
        alt="Preview"
        style={{
          maxWidth: '100%',
          maxHeight: '150px',
          objectFit: 'contain'
        }}
      />
      <p style={{
        fontSize: '0.8em',
        wordBreak: 'break-all',
        marginTop: '8px',
        textAlign: 'center'
      }}>
        {imagePath.split(/[\\/]/).pop()}
      </p>
    </div>
  );
};

function Monitoreo() {

  const [datosIzq, setDatosIzq] = useState<DatosMonitoreoIzq[]>([]);
  const [datosDer, setDatosDer] = useState<DatosMonitoreoDer[]>([]);

  const [datosOriginales, setDatosOriginales] = useState<any[]>([]); //Guarda datos originales de las tareas de todos los usuarios 
  const [editandoIndex, setEditandoIndex] = useState<number | null>(null);
  const [textoEditado, setTextoEditado] = useState<string>("");
  const [usuarioSeleccionado, setUsuarioSeleccionado] = useState<any>(null);
  const [mostrarEmergente, setMostrarEmergente] = useState(false);
  const [roles, setRoles] = useState<string[]>([]);
  const [instituciones, setInstituciones] = useState<string[]>([]);
  const [filtroRol, setFiltroRol] = useState<string[]>([]);
  const [filtroInstitucion, setFiltroInstitucion] = useState<string[]>([]);
  const [filtroProgreso, setFiltroProgreso] = useState<string | null>(null);
  const [textoBusqueda, setTextoBusqueda] = useState<string>("");

  // Función para manejar la selección de roles
  const handleRolChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const options = e.target.options;
    const selectedValues = [];

    for (let i = 0; i < options.length; i++) {
      if (options[i].selected && options[i].value !== "objetos") {
        selectedValues.push(options[i].value);
      }
    }

    setFiltroRol(selectedValues);
  };

  // Función para manejar la selección de instituciones
  const handleInstitucionChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const options = e.target.options;
    const selectedValues = [];

    for (let i = 0; i < options.length; i++) {
      if (options[i].selected && options[i].value !== "objetos") {
        selectedValues.push(options[i].value);
      }
    }

    setFiltroInstitucion(selectedValues);
  };

  // Función para manejar la selección de progreso
  const handleProgresoChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const options = e.target.options;
    let selectedValue = null;

    for (let i = 0; i < options.length; i++) {
      if (options[i].selected && options[i].value !== "objetos") {
        selectedValue = options[i].value;
        break;
      }
    }

    setFiltroProgreso(selectedValue);
  };

  // Función para filtrar los datos según los criterios seleccionados
  const getDatosFiltrados = () => {
    return datosOriginales.filter(persona => {
      // Filtro por rol
      if (filtroRol.length > 0 && !filtroRol.includes(persona.rol)) {
        return false;
      }

      // Filtro por institución
      if (filtroInstitucion.length > 0 && !filtroInstitucion.includes(persona.institucion)) {
        return false;
      }

      // Filtro por progreso
      if (filtroProgreso) {
        const progresoNumerico = parseFloat(filtroProgreso);
        if (!isNaN(progresoNumerico)) {
          if (filtroProgreso === "nulo") {
            if (persona.progreso !== undefined && persona.progreso !== null) {
              return false;
            }
          } else if (Math.abs((persona.progreso || 0) * 100 - progresoNumerico) > 10) {
            return false;
          }
        }
      }

      // Filtro por texto de búsqueda
      if (textoBusqueda.trim() !== "") {
        const busqueda = textoBusqueda.toLowerCase();
        const camposBusqueda = [
          persona.nombre?.toLowerCase(),
          persona.apellido?.toLowerCase(),
          persona.id?.toLowerCase(),
          persona.correo?.toLowerCase(),
          persona.institucion?.toLowerCase(),
          Array.isArray(persona.telefono)
            ? persona.telefono.join(' ')
            : persona.telefono?.toString()
        ].join(' ');

        return camposBusqueda.includes(busqueda);
      }

      return true;
    }).map((persona) => ({
      id: persona.id,
      rol: persona.rol,
      teleefono: Array.isArray(persona.telefono)
        ? persona.telefono[0]
        : persona.telefono || '',
      email: persona.correo,
      nombre: [persona.nombre, persona.apellido].filter(Boolean).join(' '),
      institucion: persona.institucion
    }));
  };


  // Modificado para usar esta función al renderizar
  const datosFiltrados = getDatosFiltrados();


  useEffect(() => {
    // Cargar roles
    invoke<string[]>("obtener_roles_unicos")
      .then((response) => setRoles(response))
      .catch((error) => console.error("Error al cargar roles:", error));

    // Cargar instituciones
    invoke<string[]>("obtener_instituciones_unicas")
      .then((response) => setInstituciones(response))
      .catch((error) => console.error("Error al cargar instituciones:", error));
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

        const mapPersona = (p: any): DatosMonitoreoIzq => ({
          id: `Usuario ${p.id}`,
          rol: p.rol,
          teleefono: Array.isArray(p.teleefono) ? p.telefono[0] : p.telefono,
          email: p.correo,
          nombre: [p.nombre, p.apellido].filter(Boolean).join(" "),
          institucion: p.institucion,
        });

        const personas = [
          ...jsonData.tutores,
          ...jsonData.tutorado1,
          ...jsonData.tutorado2,
        ];

        const datosIzquierda = personas.map(mapPersona);

        setDatosIzq(datosIzquierda);
        setDatosOriginales(personas);
      })
      .catch((err) => {
        console.error("Error cargando datos del JSON:", err);
      });
  }, []);

  const handleCasillaClick = (row: DatosMonitoreoIzq) => {
    const persona = datosOriginales.find(p => p.correo === row.email);
    if (!persona) return;

    setUsuarioSeleccionado(persona);

    const nuevasEntradas: DatosMonitoreoDer[] = [];

    // Agregar tareas (como antes)
    persona.tareas.forEach((tarea: any) => {
      nuevasEntradas.push({
        registro: `${tarea.nombre}: ${tarea.descripcion}`,
        esImagen: false
      });
    });

    // Agregar imágenes (una por cada entrada en el array)
    if (persona.imagenes && Array.isArray(persona.imagenes)) {
      for (const imagen of persona.imagenes) {
        if (imagen.url) {
          nuevasEntradas.push({
            registro: `Imagen: ${imagen.url}`,
            esImagen: true,
            urlImagen: imagen.url
          });
        }
      }
    }

    setDatosDer(nuevasEntradas);
  };


  // Función mejorada para manejar la eliminación
  const handleDeleteItem = async (index: number) => {
    const actualIndex = datosDer.length - 1 - index;
    const itemToDelete = datosDer[actualIndex];
    const isImage = itemToDelete.registro.startsWith("Imagen:");

    // Si no hay usuario seleccionado, no podemos proceder
    if (!usuarioSeleccionado) {
      console.error("No hay usuario seleccionado");
      return;
    }

    try {
      // Cargar el JSON actual
      const jsonResponse = await invoke("cargar_datos_json");
      const jsonData = JSON.parse(jsonResponse as string);

      // Buscar el usuario en el JSON usando el correo del usuario seleccionado
      let userType = "";
      let userIndex = -1;

      // Buscar en tutores
      userIndex = jsonData.tutores.findIndex((p: any) => p.correo === usuarioSeleccionado.correo);
      if (userIndex !== -1) userType = "tutores";

      // Si no se encontró en tutores, buscar en tutorado1
      if (userIndex === -1) {
        userIndex = jsonData.tutorado1.findIndex((p: any) => p.correo === usuarioSeleccionado.correo);
        if (userIndex !== -1) userType = "tutorado1";
      }

      // Si no se encontró en tutorado1, buscar en tutorado2
      if (userIndex === -1) {
        userIndex = jsonData.tutorado2.findIndex((p: any) => p.correo === usuarioSeleccionado.correo);
        if (userIndex !== -1) userType = "tutorado2";
      }

      if (userType === "" || userIndex === -1) {
        console.error("No se pudo encontrar al usuario en el JSON");
        return;
      }

      if (isImage) {
        // Para imágenes: extraer la URL de la imagen del registro
        const imagenUrl = itemToDelete.registro.substring(itemToDelete.registro.indexOf(":") + 1).trim();

        // Si imagenes es un array de objetos con url
        if (Array.isArray(jsonData[userType][userIndex].imagenes)) {
          jsonData[userType][userIndex].imagenes = jsonData[userType][userIndex].imagenes.filter(
            (img: any) => img.url !== imagenUrl
          );
        } else {
          // Si imagenes es un string simple, limpiarlo
          jsonData[userType][userIndex].imagenes = "";
        }
      } else {
        // Para tareas: eliminar del array de tareas
        const taskText = itemToDelete.registro;
        const taskName = taskText.split(":")[0].trim();

        jsonData[userType][userIndex].tareas = jsonData[userType][userIndex].tareas.filter(
          (tarea: any) => tarea.nombre !== taskName
        );
      }

      // Actualizar el JSON en el archivo
      await invoke("actualizar_json_monitoreo", {
        jsonData: JSON.stringify(jsonData)
      });

      // Actualizar la UI removiendo el item
      const newDatosDer = [...datosDer];
      newDatosDer.splice(actualIndex, 1);
      setDatosDer(newDatosDer);

      // Solo actualizar tareas y progreso si era una tarea (no imagen)
      if (!isImage) {
        await invoke("actualizar_tareas_y_progreso", {
          correo: usuarioSeleccionado.correo,
          esEliminacion: true
        });
      }

      // Actualizar los datos originales
      const personas = [
        ...jsonData.tutores,
        ...jsonData.tutorado1,
        ...jsonData.tutorado2,
      ];
      setDatosOriginales(personas);

      // Actualizar el usuario seleccionado con los nuevos datos
      const personaActualizada = personas.find(p => p.correo === usuarioSeleccionado.correo);
      if (personaActualizada) {
        setUsuarioSeleccionado(personaActualizada);
      }

      console.log("Item deleted successfully");
    } catch (error) {
      console.error("Error deleting item:", error);
    }
  };

  // Función para activar el modo de edición
  const handleActivarEdicion = (index: number) => {
    const actualIndex = datosDer.length - 1 - index;
    const item = datosDer[actualIndex];
    setEditandoIndex(actualIndex);
    setTextoEditado(item.registro);
  };

  // Función para cancelar la edición
  const handleCancelarEdicion = () => {
    setEditandoIndex(null);
    setTextoEditado("");
  };

  // Función para guardar los cambios de la edición
  // Función modificada para guardar los cambios
  const handleGuardarEdicion = async () => {
    if (editandoIndex === null) return;

    const itemToEdit = datosDer[editandoIndex];
    const isImage = itemToEdit.registro.startsWith("Imagen:");

    // Encontrar el usuario actual basado en los datos mostrados
    const currentUser = datosOriginales.find(p => {
      const matchingEmail = datosIzq.find(row => row.email === p.correo);
      return matchingEmail && datosDer.some(item => {
        if (isImage) {
          return item.registro.includes(p.imagenes);
        } else {
          return p.tareas.some((tarea: any) =>
            item.registro.includes(`${tarea.nombre}: ${tarea.descripcion}`)
          );
        }
      });
    });

    if (!currentUser) return;

    try {
      // Primero, cargamos el JSON completo actual desde el archivo
      const jsonActual = await invoke<string>("cargar_datos_json");
      const jsonCompleto = JSON.parse(jsonActual);

      // Determinamos en qué categoría está el usuario (tutores, tutorado1, tutorado2)
      let categoria = '';
      let userIndex = -1;

      // Buscar en tutores
      userIndex = jsonCompleto.tutores.findIndex((p: any) => p.correo === currentUser.correo);
      if (userIndex !== -1) categoria = 'tutores';

      // Si no se encontró en tutores, buscar en tutorado1
      if (userIndex === -1) {
        userIndex = jsonCompleto.tutorado1.findIndex((p: any) => p.correo === currentUser.correo);
        if (userIndex !== -1) categoria = 'tutorado1';
      }

      // Si no se encontró en tutorado1, buscar en tutorado2
      if (userIndex === -1) {
        userIndex = jsonCompleto.tutorado2.findIndex((p: any) => p.correo === currentUser.correo);
        if (userIndex !== -1) categoria = 'tutorado2';
      }

      if (categoria === '' || userIndex === -1) {
        console.error("No se encontró el usuario en ninguna categoría");
        return;
      }

      // Modificar los datos según corresponda
      if (isImage) {
        // Actualizar la imagen
        const newImageValue = textoEditado.substring(textoEditado.indexOf(":") + 1).trim();
        jsonCompleto[categoria][userIndex].imagenes = newImageValue;
      } else {
        // Actualizar la tarea
        const oldTaskText = itemToEdit.registro;
        const oldTaskName = oldTaskText.split(":")[0].trim();

        // Extraer el nuevo nombre y descripción
        const parts = textoEditado.split(":");
        const newTaskName = parts[0].trim();
        const newTaskDesc = parts.slice(1).join(":").trim();

        // Encontrar la tarea y actualizarla
        const taskIndex = jsonCompleto[categoria][userIndex].tareas.findIndex(
          (tarea: any) => tarea.nombre === oldTaskName
        );

        if (taskIndex !== -1) {
          const tareaExistente = jsonCompleto[categoria][userIndex].tareas[taskIndex];
          jsonCompleto[categoria][userIndex].tareas[taskIndex] = {
            nombre: newTaskName,
            descripcion: newTaskDesc,
            hecho: tareaExistente.hecho  // Faltaba el campo de "hecho" por eso no lo actualizaba correctamente
          };
        }
      }



      // Enviar los datos actualizados al backend para guardar en JSON
      await invoke("actualizar_json_monitoreo", {
        jsonData: JSON.stringify(jsonCompleto)
      });

      // Actualizar la UI
      const newDatosDer = [...datosDer];
      newDatosDer[editandoIndex] = { registro: textoEditado };
      setDatosDer(newDatosDer);

      // Actualizar los datos originales - importante para mantener consistencia
      // Esta vez debemos reconstruir el array plano de datosOriginales
      const nuevosOriginales = [
        ...jsonCompleto.tutores,
        ...jsonCompleto.tutorado1,
        ...jsonCompleto.tutorado2
      ];
      setDatosOriginales(nuevosOriginales);

      // Salir del modo edición
      setEditandoIndex(null);
      setTextoEditado("");

      console.log("Item actualizado exitosamente");
    } catch (error) {
      console.error("Error actualizando item:", error);
    }
  };

  const handleGuardarNuevoRegistro = async (tipo: 'tarea' | 'imagen', datos: any) => {
    if (!usuarioSeleccionado) return;

    try {
      const jsonResponse = await invoke("cargar_datos_json");
      const jsonData = JSON.parse(jsonResponse as string);

      let userType = "";
      let userIndex = -1;

      userIndex = jsonData.tutores.findIndex((p: any) => p.correo === usuarioSeleccionado.correo);
      if (userIndex !== -1) userType = "tutores";

      if (userIndex === -1) {
        userIndex = jsonData.tutorado1.findIndex((p: any) => p.correo === usuarioSeleccionado.correo);
        if (userIndex !== -1) userType = "tutorado1";
      }

      if (userIndex === -1) {
        userIndex = jsonData.tutorado2.findIndex((p: any) => p.correo === usuarioSeleccionado.correo);
        if (userIndex !== -1) userType = "tutorado2";
      }

      if (userType === "" || userIndex === -1) {
        console.error("No se pudo encontrar al usuario en el JSON");
        return;
      }

      if (tipo === 'tarea') {
        if (!jsonData[userType][userIndex].tareas) {
          jsonData[userType][userIndex].tareas = [];
        }
        jsonData[userType][userIndex].tareas.push(datos);
      } else if (tipo === 'imagen') {
        if (!jsonData[userType][userIndex].imagenes) {
          jsonData[userType][userIndex].imagenes = [];
        }

        // Aquí añadimos el objeto imagen con el formato correcto
        jsonData[userType][userIndex].imagenes.push({
          url: datos.url // Usamos la url que viene del emergente
        });
      }

      // Actualizar el JSON en el archivo
      await invoke("actualizar_json_monitoreo", {
        jsonData: JSON.stringify(jsonData)
      });

      // Actualizar la UI
      const nuevasEntradas = [...datosDer];
      if (tipo === 'tarea') {
        nuevasEntradas.push({
          registro: `${datos.nombre}: ${datos.descripcion}`,
          esImagen: false
        });
      } else {
        nuevasEntradas.push({
          registro: `Imagen: ${datos.url}`, // Mostramos la ruta completa
          esImagen: true,
          urlImagen: datos.url
        });
      }
      setDatosDer(nuevasEntradas);

      // Actualizar los datos originales
      const personas = [
        ...jsonData.tutores,
        ...jsonData.tutorado1,
        ...jsonData.tutorado2,
      ];
      setDatosOriginales(personas);

      // Actualizar el usuario seleccionado
      const personaActualizada = personas.find(p => p.correo === usuarioSeleccionado.correo);
      if (personaActualizada) {
        setUsuarioSeleccionado(personaActualizada);
      }

      setMostrarEmergente(false);

    } catch (error) {
      console.error("Error al guardar el nuevo registro:", error);
      alert("Error al guardar la imagen: " + (error as Error).message);
    }
  };

  const abrirEmergente = () => {
    if (!usuarioSeleccionado) {
      alert("Por favor selecciona un usuario primero");
      return;
    }
    setMostrarEmergente(true);
  };

  const handleEnviarItem = async (index: number) => {
    try {
      // Verificar que usuarioSeleccionado exista
      if (!usuarioSeleccionado) {
        alert("Por favor selecciona un usuario primero");
        return;
      }

      // Obtener el nombre de la tarea del texto mostrado en la UI
      const itemActual = datosDer[datosDer.length - 1 - index];
      if (!itemActual || !itemActual.registro) {
        alert("No se pudo identificar la tarea seleccionada");
        return;
      }

      // Extraer el nombre y la descripción del texto del registro
      const partes = itemActual.registro.split(":");
      if (partes.length < 2) {
        alert("El formato de la tarea no es válido");
        return;
      }

      const nombreTarea = partes[0].trim();
      const descripcionTarea = partes.slice(1).join(":").trim();

      // Buscar la tarea en el usuario seleccionado para verificar que existe
      if (!usuarioSeleccionado.tareas || !Array.isArray(usuarioSeleccionado.tareas)) {
        alert("El usuario seleccionado no tiene tareas");
        return;
      }

      const numeroTelefono = usuarioSeleccionado.rol === "Tutor"
        ? usuarioSeleccionado.telefono // Para tutores, usar el teléfono directamente
        : Array.isArray(usuarioSeleccionado.telefono)
          ? usuarioSeleccionado.telefono[0] // Para tutorados, usar el primer número del array
          : ""; // Fallback por si acaso

      // Invocar la función backend con los datos extraídos de la UI
      await invoke("monitoreo_enviar_tarea", {
        nombre: usuarioSeleccionado.nombre || "Usuario",
        titulo: nombreTarea,
        descripcion: descripcionTarea,
        telefono: numeroTelefono,
      });

      console.log("Tarea enviada correctamente");
    } catch (error) {
      console.error("Error al enviar la tarea:", error);
      alert("Ocurrió un error al enviar la tarea");
    }
  };

  const handleToggleHecho = async (taskName: string) => {
    if (!usuarioSeleccionado) return;

    try {
      // Cargar el JSON actual
      const jsonResponse = await invoke("cargar_datos_json");
      const jsonData = JSON.parse(jsonResponse as string);

      // Buscar el usuario en el JSON
      let userType = "";
      let userIndex = -1;

      userIndex = jsonData.tutores.findIndex((p: any) => p.correo === usuarioSeleccionado.correo);
      if (userIndex !== -1) userType = "tutores";

      if (userIndex === -1) {
        userIndex = jsonData.tutorado1.findIndex((p: any) => p.correo === usuarioSeleccionado.correo);
        if (userIndex !== -1) userType = "tutorado1";
      }

      if (userIndex === -1) {
        userIndex = jsonData.tutorado2.findIndex((p: any) => p.correo === usuarioSeleccionado.correo);
        if (userIndex !== -1) userType = "tutorado2";
      }

      if (userType === "" || userIndex === -1) return;

      // Buscar la tarea y cambiar el estado de hecho
      const tareas = jsonData[userType][userIndex].tareas;
      const tareaIndex = tareas.findIndex((t: any) => t.nombre === taskName);
      if (tareaIndex !== -1) {
        tareas[tareaIndex].hecho = !tareas[tareaIndex].hecho;
      }

      // Recalcular el progreso
      const total = tareas.length;
      const hechas = tareas.filter((t: any) => t.hecho).length;
      jsonData[userType][userIndex].progreso = total > 0 ? hechas / total : 0;

      // Guardar el JSON actualizado
      await invoke("actualizar_json_monitoreo", {
        jsonData: JSON.stringify(jsonData)
      });

      // Actualizar la UI
      const personas = [
        ...jsonData.tutores,
        ...jsonData.tutorado1,
        ...jsonData.tutorado2,
      ];
      setDatosOriginales(personas);

      // Actualizar usuarioSeleccionado y datosDer
      const personaActualizada = personas.find(p => p.correo === usuarioSeleccionado.correo);
      setUsuarioSeleccionado(personaActualizada);

      // Actualizar la vista derecha
      if (personaActualizada) {
        const nuevasEntradas: DatosMonitoreoDer[] = [];
        personaActualizada.tareas.forEach((tarea: any) => {
          nuevasEntradas.push({
            registro: `${tarea.nombre}: ${tarea.descripcion}`
          });
        });
        if (personaActualizada.imagenes && Array.isArray(personaActualizada.imagenes)) {
          personaActualizada.imagenes.forEach((imagen: any) => {
            if (imagen.url) {
              nuevasEntradas.push({
                registro: `Imagen: ${imagen.url}`
              });
            }
          });
        }
        setDatosDer(nuevasEntradas);
      }
    } catch (error) {
      console.error("Error actualizando el estado de la tarea:", error);
    }
  };

  return (
    <div className="monitoreo">
      <div className="contenedor_PanelIzquierdo">
        {/* Panel izquierdo con filtros */}
        <div className="opciones-izquierda">
          <select multiple onChange={handleRolChange}>
            <option value="objetos">Rol</option>
            {roles.map((rol, index) => (
              <option key={index} value={rol}>{rol}</option>
            ))}
          </select>
          <select multiple onChange={handleInstitucionChange}>
            <option value="objetos">Institución</option>
            {instituciones.map((institucion, index) => (
              <option key={index} value={institucion}>{institucion}</option>
            ))}
          </select>
          <select multiple onChange={handleProgresoChange}>
            <option value="objetos">Progreso</option>
            <option value="100">100%</option>
            <option value="80">80%</option>
            <option value="60">60%</option>
            <option value="40">40%</option>
            <option value="20">20%</option>
            <option value="0">0%</option>
            <option value="nulo">nulo</option>
          </select>
        </div>
        <div className="opciones-izquierda">
          <input
            type="text"
            placeholder="Buscar"
            className="barra-buusqueda"
            value={textoBusqueda}
            onChange={(e) => setTextoBusqueda(e.target.value)}
          />
        </div>
        <div className="desplazadora">
          {datosFiltrados.map((row, index) => {
            // Encontrar el progreso del usuario actual
            const persona = datosOriginales.find(p => p.correo === row.email);
            const progreso = persona?.progreso || 0;

            // Determinar el color de fondo según el progreso
            let backgroundColor;
            if (progreso === 0.0) {
              backgroundColor = '#DDDCDC'; // Blanco - 0%
            } else if (progreso > 0.0 && progreso <= 0.2) {
              backgroundColor = '#FF6B6B'; // Rojo - 1-20%
            } else if (progreso > 0.2 && progreso <= 0.4) {
              backgroundColor = '#FFEB3B'; // Amarillo - 21-40%
            } else if (progreso > 0.4 && progreso <= 0.6) {
              backgroundColor = '#4CAF50'; // Verde - 41-60%
            } else if (progreso > 0.6 && progreso < 1.0) {
              backgroundColor = '#2196F3'; // Azul - 61-99%
            } else if (progreso === 1.0) {
              backgroundColor = '#9C27B0'; // Morado - 100%
            }

            return (
              <div
                key={index}
                className="casilla"
                onClick={() => handleCasillaClick(row)}
                style={{
                  cursor: 'pointer',
                  backgroundColor: backgroundColor,
                  // Mantener los otros estilos existentes
                  border: '1px solid #8A2BE2',
                  borderRadius: '8px',
                  marginBottom: '8px',
                  padding: '12px'
                }}
              >
                <div className="header-usuario">
                  <div style={{
                    display: 'flex',

                    alignItems: 'center',
                    width: '100%'
                  }}>
                    <p className="rol-id">{row.rol} · ID: {row.id}</p>
                    <p className="progreso">· Progreso: {Math.round(progreso * 100)}%</p>
                  </div>
                  <p className="nombre">{row.nombre}</p>

                </div>
                <div className="detalles">
                  <p className="institucion">Institución: {row.institucion}</p>
                  <p className="contacto">Teléfono: {row.teleefono}</p>
                  <p className="email">Email: {row.email}</p>
                </div>
              </div>
            );
          })}
        </div>
      </div>

      <div className="contenedor_PanelDerecho">
        <div className="desplazadora">
          {datosDer.slice(0).reverse().map((row, index) => {
            const esImagen = row.esImagen || false;
            const esTarea = !esImagen;
            const actualIndex = datosDer.length - 1 - index;
            const isEditing = editandoIndex === actualIndex;

            return (
              <div
                key={index}
                className="registro"
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  padding: '8px',
                  border: '1px solid #ccc',
                  borderRadius: '8px',
                  marginBottom: '8px',
                  minHeight: '50px'
                }}
              >
                <div style={{ width: '24px', display: 'flex', justifyContent: 'center' }}>
                  {esTarea && (
                    <input
                      type="checkbox"
                      checked={(() => {
                        if (esTarea && usuarioSeleccionado && usuarioSeleccionado.tareas) {
                          const taskName = row.registro.split(":")[0].trim();
                          const tarea = usuarioSeleccionado.tareas.find(
                            (t: any) => t.nombre === taskName
                          );
                          return tarea ? tarea.hecho : false;
                        }
                        return false;
                      })()}
                      onChange={async () => {
                        try {
                          const taskName = row.registro.split(":")[0].trim();
                          const result = await invoke("toggle_hecho_monitoreo", {
                            correo: usuarioSeleccionado.correo,
                            nombreTarea: taskName
                          });
                          const jsonResponse = await invoke("cargar_datos_json");
                          const jsonData = JSON.parse(jsonResponse as string);
                          const personas = [
                            ...jsonData.tutores,
                            ...jsonData.tutorado1,
                            ...jsonData.tutorado2
                          ];
                          setDatosOriginales(personas);

                          const personaActualizada = personas.find(p => p.correo === usuarioSeleccionado.correo);
                          if (personaActualizada) {
                            setUsuarioSeleccionado(personaActualizada);
                          }

                          console.log(`Tarea ${taskName} cambió a estado: ${result}`);
                        } catch (error) {
                          console.error("Error llamando a toggle_hecho_monitoreo:", error);
                        }
                      }}
                    />
                  )}
                </div>

                {isEditing ? (
                  <input
                    type="text"
                    value={textoEditado}
                    onChange={(e) => setTextoEditado(e.target.value)}
                    style={{ flexGrow: 1, margin: '0 10px', padding: '5px' }}
                  />
                ) : esImagen ? (
                  <div style={{ flexGrow: 1, margin: '0 10px' }}>
                    <ImagenPreview imagePath={row.urlImagen || ''} />
                  </div>
                ) : (
                  <p
                    style={{
                      flexGrow: 1,
                      margin: '0 10px',
                      cursor: 'pointer'
                    }}
                    onClick={() => handleActivarEdicion(index)}
                  >
                    {row.registro}
                  </p>
                )}

                {isEditing ? (
                  <>
                    <button
                      style={{ marginLeft: '5px' }}
                      onClick={handleGuardarEdicion}
                    >
                      Actualizar
                    </button>
                    <button
                      style={{ marginLeft: '5px' }}
                      onClick={handleCancelarEdicion}
                    >
                      Cancelar
                    </button>
                  </>
                ) : (
                  <>
                    {esTarea && (
                      <button onClick={() => handleEnviarItem(index)}>
                        Enviar
                      </button>
                    )}
                    <button
                      style={{ marginLeft: '10px' }}
                      onClick={() => handleDeleteItem(index)}
                    >
                      Eliminar
                    </button>
                  </>
                )}
              </div>
            );
          })}
        </div>

        <div className="nuevo-registro">
          <button onClick={abrirEmergente}>
            +
          </button>
        </div>
      </div>

      {/* Ventana emergente tipo popup */}
      {mostrarEmergente && (
        <EmergenteMonitoreo
          mensaje={`Agregar nuevo registro a ${usuarioSeleccionado?.nombre || 'usuario seleccionado'}`}
          cancelar={() => setMostrarEmergente(false)}
          onGuardar={handleGuardarNuevoRegistro}
        />
      )}
    </div>
  );
}

export default Monitoreo;
