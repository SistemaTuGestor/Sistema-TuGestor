import "./Reportes.css";

import Emergente from "./Emergente/Emergente";

import { useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open, save } from "@tauri-apps/api/dialog";



function Reportes() {

  const [isLoading, setIsLoading] = useState(false);

  //// Fecha


  const [fechaLee, setFechaLee] = useState("");
  const [fechaPUJ, setFechaPUJ] = useState("");
  const [fechaColegios, setFechaColegios] = useState("");
  const [fechaConstanciasTutores, setFechaConstanciasTutores] = useState("");
  const [fechaConstanciasTutorados, setFechaConstanciasTutorados] = useState("");

  

  //// Apertura de explorador de archivos para XLSX de Emparejamiento en LEE.


  const [archivoPath_Emparejamiento, setArchivoPath_Emparejamiento] = useState("Ubicación de archivo Emparejamiento");

  const handleSelectArchivo_Emparejamiento = async () => {

    try {

      const selectedPath = await open({
        directory: false,  // Permite seleccionar archivos.
        multiple: false,  // Solo permite seleccionar uno.
      });

      if (typeof selectedPath === "string") {

        // Imprimir por consola.
        console.log("Plantilla seleccionada:", selectedPath);

        // Imprimir por GUI.
        setArchivoPath_Emparejamiento(selectedPath);

        // Enviar la ruta al backend de LEE.
        invoke("reportes_lee_recibir_emparejamiento", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));
        // Enviar la ruta al backend de Tutorados.
        invoke("reportes_tutorados_recibir_emparejamiento", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));

      }

    } catch (error) {

      console.error("Error al seleccionar la archivo:", error);

    }

  };


  //// Apertura de explorador de archivos para formularios.


  const [folderPath_LEE, setFolderPath_LEE] = useState<string | null>("Ubicación de formularios");

  const handleSelectFolder_LEE = async () => {

    try {

      const selectedPath = await open({
        directory: true,  // Permite seleccionar una carpeta.
        multiple: false,  // Solo permite seleccionar una.
      });

      if (typeof selectedPath === "string") {

        // Imprimir por consola.
        console.log("Carpeta seleccionada:", selectedPath);

        // Imprimir por GUI.
        const folderName = selectedPath.split(/[\\/]/).pop() || "Carpeta seleccionada";
        setFolderPath_LEE(folderName);

        // Enviar la ruta al backend.
        invoke("reportes_lee_recibir_pathcarpeta", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));

      }

    } catch (error) {

      console.error("Error al seleccionar la carpeta:", error);

    }

  };


  //// Apertura de explorador para lectura de archivo LEE para otros reportes.


  const [archivoPath_LEE, setArchivoPath_LEE] = useState("Ubicación de archivo LEE");
  

  const handleSelectArchivo_LEE = async () => {

    try {

      const selectedPath = await open({
        directory: false,  // Permite seleccionar archivos.
        multiple: false,  // Solo permite seleccionar uno.
      });

      if (typeof selectedPath === "string") {

        // Imprimir por consola.
        console.log("Plantilla seleccionada:", selectedPath);

        // Imprimir por GUI.
        setArchivoPath_LEE(selectedPath);

        // Enviar la ruta al backend para PUJ.
        invoke("reportes_puj_recibir_lee", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));
        // Enviar la ruta al backend para Colegios.
        invoke("reportes_colegios_recibir_lee", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));
        // Enviar la ruta al backend para Tutores.
        invoke("reportes_tutores_recibir_lee", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));

      }

    } catch (error) {

      console.error("Error al seleccionar la archivo:", error);

    }

  };


  //// Apertura de explorador de archivos para plantilla de PUJ.


  const [plantillaPath_PUJ, setPlantillaPath_PUJ] = useState<string | null>("Ubicación de plantilla");

  const handleSelectPlantilla_PUJ = async () => {

    try {

      const selectedPath = await open({
        directory: false,  // Permite seleccionar archivos.
        multiple: false,  // Solo permite seleccionar uno.
      });

      if (typeof selectedPath === "string") {

        // Imprimir por consola.
        console.log("Plantilla seleccionada:", selectedPath);

        // Imprimir por GUI.
        const fileName = selectedPath.split(/[\\/]/).pop() || "Plantilla seleccionada";
        setPlantillaPath_PUJ(fileName);

        // Enviar la ruta al backend.
        invoke("reportes_puj_recibir_pathplantilla", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));

      }

    } catch (error) {

      console.error("Error al seleccionar la plantilla:", error);

    }

  };


  //// Apertura de explorador de archivos para plantilla de Colegios.


  const [plantillaPath_Colegios, setPlantillaPath_Colegios] = useState<string | null>("Ubicación de plantilla");

  const handleSelectPlantilla_Colegios = async () => {

    try {

      const selectedPath = await open({
        directory: false,  // Permite seleccionar archivos.
        multiple: false,  // Solo permite seleccionar uno.
      });

      if (typeof selectedPath === "string") {

        // Imprimir por consola.
        console.log("Plantilla seleccionada:", selectedPath);

        // Imprimir por GUI.
        const fileName = selectedPath.split(/[\\/]/).pop() || "Plantilla seleccionada";
        setPlantillaPath_Colegios(fileName);

        // Enviar la ruta al backend.
        invoke("reportes_colegios_recibir_pathplantilla", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));

      }

    } catch (error) {

      console.error("Error al seleccionar la plantilla:", error);

    }

  };


  //// Apertura de explorador de archivos para plantilla de Tutores.


  const [plantillaPath_ConstanciasTutores, setPlantillaPath_ConstanciasTutores] = useState<string | null>("Ubicación de plantilla");

  const handleSelectPlantilla_ConstanciasTutores = async () => {

    try {

      const selectedPath = await open({
        directory: false,  // Permite seleccionar archivos.
        multiple: false,  // Solo permite seleccionar uno.
      });

      if (typeof selectedPath === "string") {

        // Imprimir por consola.
        console.log("Plantilla seleccionada:", selectedPath);

        // Imprimir por GUI.
        const fileName = selectedPath.split(/[\\/]/).pop() || "Plantilla seleccionada";
        setPlantillaPath_ConstanciasTutores(fileName);

        // Enviar la ruta al backend.
        invoke("reportes_constanciastutores_recibir_pathplantilla", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));

      }

    } catch (error) {

      console.error("Error al seleccionar la plantilla:", error);

    }

  };


  //// Apertura de explorador de archivos para plantilla de Tutorados.


  const [plantillaPath_ConstanciasTutorados, setPlantillaPath_ConstanciasTutorados] = useState<string | null>("Ubicación de plantilla");

  const handleSelectPlantilla_ConstanciasTutorados = async () => {

    try {

      const selectedPath = await open({
        directory: false,  // Permite seleccionar una carpeta.
        multiple: false,  // Solo permite seleccionar una.
      });

      if (typeof selectedPath === "string") {

        // Imprimir por consola.
        console.log("Plantilla seleccionada:", selectedPath);

        // Imprimir por GUI.
        const fileName = selectedPath.split(/[\\/]/).pop() || "Plantilla seleccionada";
        setPlantillaPath_ConstanciasTutorados(fileName);

        // Enviar la ruta al backend.
        invoke("reportes_constanciastutorados_recibir_pathplantilla", { path: selectedPath })
          .then(() => console.log("Ruta enviada correctamente"))
          .catch((err) => console.error("Error al enviar la ruta:", err));

      }

    } catch (error) {

      console.error("Error al seleccionar la plantilla:", error);

    }

  };


  //// Ubicación de los reportes.


  const [directorioReporteLee, setDirectorioReporteLee] = useState("Directorio del reporte");

  const [directorioReportePUJ, setDirectorioReportePUJ] = useState("Directorio de reportes");

  const [directorioReporteColegios, setDirectorioReporteColegios] = useState("Directorio de reportes");

  const [directorioReporteConstanciasTutores, setDirectorioReporteConstanciasTutores] = useState("Directorio de reportes");

  const [directorioReporteConstanciasTutorados, setDirectorioReporteConstanciasTutorados] = useState("Directorio de reportes");


  //// Nombre de los reportes.


  const [nombreReporteLee, setNombreReporteLee] = useState("Nombre del reporte");

  const [nombreReportePUJ, setNombreReportePUJ] = useState("Nombre de reportes");

  const [nombreReporteColegios, setNombreReporteColegios] = useState("Nombre de reportes");

  const [nombreReporteConstanciasTutores, setNombreReporteConstanciasTutores] = useState("Nombre de reportes");

  const [nombreReporteConstanciasTutorados, setNombreReporteConstanciasTutorados] = useState("Nombre de reportes");


  //// Control de ventana emergente.


  const [seccioonActual, setSeccioonActual] = useState("");
  const [getEmergenteVisible, setEmergenteVisible] = useState(false);

  const evento_clickOpciones = async (seccioon: string) => {
    setSeccioonActual(seccioon);
    setEmergenteVisible(true);
  }

  const evento_clickCancelar = () => {
    setEmergenteVisible(false);
  }

  const evento_clickGenerar = async (seccioon: string) => {

  if (seccioon === "LEE") {

    if (folderPath_LEE === "Ubicación de formularios" || archivoPath_Emparejamiento === "Ubicación de archivo Emparejamiento" || !fechaLee || fechaLee.trim() === "") {
      let mensaje = `Por favor, completa los siguientes campos antes de generar el reporte de ${seccioon}:`;
      
      if (folderPath_LEE === "Ubicación de formularios") {
        mensaje += "\n- Directorio de formularios";
      }
      
      if (archivoPath_Emparejamiento === "Ubicación de archivo Emparejamiento") {
        mensaje += "\n- Archivo de emparejamiento";
      }
      
      if (!fechaLee || fechaLee.trim() === "") {
        mensaje += "\n- Fecha";
      }
      
      alert(mensaje);
      setEmergenteVisible(false);
      return;
    }

    try {

      const filePath = await save({
        defaultPath: seccioon,
        filters: [{ name: "Excel Files", extensions: ["xlsx"] }]
      });

      if (filePath) {
        // await invoke ( "reportes_lee_actualizarfecha",{nueva_fecha:fechaLee} ) ;
        await invoke("reportes_lee_recibir_nombrereporte", { nombrereporte: filePath });
        await invoke("reportes_lee_leer_archivos_en_carpeta");
        setDirectorioReporteLee(filePath);
        setNombreReporteLee(filePath.split(/[\\/]/).pop() || "Nombre del reporte");
        alert(`Reporte de ` + seccioon + ` guardado en: ` + filePath);
      } else {
        alert(`¡Generación de ` + seccioon + ` cancelada!`);
        return;
      }

    } catch (error) {

      alert(`¡Error en opciones de la sección ` + seccioon + `!`);

    }

  } else if (seccioon === "PUJ") {

    if (plantillaPath_PUJ === "Ubicación de plantilla" || archivoPath_LEE === "Ubicación de archivo LEE" || !fechaPUJ || fechaPUJ.trim() === "") {
      let mensaje = `Por favor, completa los siguientes campos antes de generar el reporte de ${seccioon}:`;
      
      if (plantillaPath_PUJ === "Ubicación de plantilla") {
        mensaje += "\n- Plantilla de constancias";
      }
      
      if (archivoPath_LEE === "Ubicación de archivo LEE") {
        mensaje += "\n- Archivo LEE";
      }
      
      if (!fechaPUJ || fechaPUJ.trim() === "") {
        mensaje += "\n- Fecha";
      }
      
      alert(mensaje);
      setEmergenteVisible(false);
      return;
    }

    try {

      const filePath = await save({
        defaultPath: seccioon,
        filters: [{ name: "Word Files", extensions: ["docx"] }]
      });

      if (filePath) {
        // Leer estudiantes aprobados.
        const estudiantesAprobados = await invoke<string[]>("reportes_puj_leer_universitarios_aprobados");
        if (estudiantesAprobados.length === 0) {
          alert(`No hay tutores aprobados para generar el reporte.`);
          return;
        }
        // await invoke ( "reportes_puj_actualizarfecha",{nueva_fecha:fechaPUJ} ) ;
        await invoke("reportes_puj_recibir_nombrereporte", { nombrereporte: filePath });
        await invoke("reporte_puj_generar", { estudiantes: estudiantesAprobados });
        setDirectorioReportePUJ(filePath);
        setNombreReportePUJ(filePath.split(/[\\/]/).pop() || "Nombre de reportes");
        alert(`Reporte de ` + seccioon + ` guardado en: ` + filePath);
      } else {
        alert(`¡Generación de ` + seccioon + ` cancelada!`);
        return;
      }

    } catch (error) {

      alert(`¡Error en opciones de la sección ` + seccioon + `!`);

    }

  } else if (seccioon === "Colegios") {

    if (plantillaPath_Colegios === "Ubicación de plantilla" || archivoPath_LEE === "Ubicación de archivo LEE" || !fechaColegios || fechaColegios.trim() === "") {
      let mensaje = `Por favor, completa los siguientes campos antes de generar el reporte de ${seccioon}:`;
      
      if (plantillaPath_Colegios === "Ubicación de plantilla") {
        mensaje += "\n- Plantilla de constancias";
      }
      
      if (archivoPath_LEE === "Ubicación de archivo LEE") {
        mensaje += "\n- Archivo LEE";
      }
      
      if (!fechaColegios || fechaColegios.trim() === "") {
        mensaje += "\n- Fecha";
      }
      
      alert(mensaje);
      setEmergenteVisible(false);
      return;
    }

    try {
      const filePath = await save({
        defaultPath: seccioon,
        filters: [{ name: "Word Files", extensions: ["docx"] }]
      });

      if (filePath) {
        // Leer estudiantes aprobados
        const estudiantesAprobados = await invoke<string[]>("reportes_colegios_leer_estudiantes_aprobados");
        if (estudiantesAprobados.length === 0) {
          alert("No hay tutores aprobados para generar el reporte.");
          return;
        }
        // await invoke ( "reportes_colegios_actualizarfecha",{nueva_fecha:fechaColegios} ) ;
        await invoke("reportes_colegios_recibir_nombrereporte", { nombrereporte: filePath });
        await invoke("reportes_colegios_generar", { estudiantes: estudiantesAprobados });
        setDirectorioReporteColegios(filePath);
        setNombreReporteColegios(filePath.split(/[\\/]/).pop() || "Nombre de reportes");
        alert(`Reporte de ` + seccioon + ` guardado en: ` + filePath);
      } else {
        alert(`¡Generación de ` + seccioon + ` cancelada!`);
        return;
      }
    } catch (error) {
      alert(`¡Error en opciones de la sección ` + seccioon + `!`);
    }
  } else if (seccioon === "Tutores") {

    if (plantillaPath_ConstanciasTutores === "Ubicación de plantilla" || 
        archivoPath_LEE === "Ubicación de archivo LEE" ||
        archivoPath_Emparejamiento === "Ubicación de archivo Emparejamiento Tutores" || 
        !fechaConstanciasTutores || fechaConstanciasTutores.trim() === "") {
      
      let mensaje = `Por favor, completa los siguientes campos antes de generar el reporte de ${seccioon}:`;
      
      if (plantillaPath_ConstanciasTutores === "Ubicación de plantilla") {
        mensaje += "\n- Plantilla de constancias";
      }
      
      if (archivoPath_LEE === "Ubicación de archivo LEE") {
        mensaje += "\n- Archivo LEE";
      }
      
      if (archivoPath_Emparejamiento === "Ubicación de archivo Emparejamiento Tutores") {
        mensaje += "\n- Archivo de Emparejamiento para Tutores";
      }
      
      if (!fechaConstanciasTutores || fechaConstanciasTutores.trim() === "") {
        mensaje += "\n- Fecha";
      }
      
      alert(mensaje);
      setEmergenteVisible(false);
      return;
    }

    try {

      const dirPath = await save({
        defaultPath: seccioon,
        filters: [{ name: "Word Files", extensions: ["docx"] }]
      });

      if (dirPath) {
        // await invoke ( "reportes_constanciastutores_actualizarfecha",{nueva_fecha:fechaConstanciasTutores} ) ;
        await invoke("reportes_constanciastutores_recibir_nombrereporte", { nombrereporte: dirPath.toString() });
        await invoke("reportes_constanciastutores_generar");
        setDirectorioReporteConstanciasTutores(dirPath.toString());
        setNombreReporteConstanciasTutores("Constancia Tutor");
        alert(`Reporte de ` + seccioon + ` guardado en: ` + dirPath);
      } else {
        alert(`¡Generación de ` + seccioon + ` cancelada!`);
        return;
      }

    } catch (error) {

      alert(`¡Error en opciones de la sección ` + seccioon + `!`);

    }

  } else if (seccioon === "Tutorados") {

    if (plantillaPath_ConstanciasTutorados === "Ubicación de plantilla" || archivoPath_Emparejamiento === "Ubicación de archivo Emparejamiento" || !fechaConstanciasTutorados || fechaConstanciasTutorados.trim() === "") {
      let mensaje = `Por favor, completa los siguientes campos antes de generar el reporte de ${seccioon}:`;
      
      if (plantillaPath_ConstanciasTutorados === "Ubicación de plantilla") {
        mensaje += "\n- Plantilla de constancias";
      }
      
      if (archivoPath_Emparejamiento === "Ubicación de archivo Emparejamiento") {
        mensaje += "\n- Archivo de emparejamiento";
      }
      
      if (!fechaConstanciasTutorados || fechaConstanciasTutorados.trim() === "") {
        mensaje += "\n- Fecha";
      }
      
      alert(mensaje);
      setEmergenteVisible(false);
      return;
    }

    try {

      const dirPath = await save({
        defaultPath: seccioon,
        filters: [{ name: "Word Files", extensions: ["docx"] }]
      });

      if (dirPath) {
        // await invoke ( "reportes_constanciastutorados_actualizarfecha",{nueva_fecha:fechaConstanciasTutorados} ) ;
        await invoke("reportes_constanciastutorados_recibir_nombrereporte", { nombrereporte: dirPath.toString() });
        await invoke("reportes_constanciastutorados_generar");
        setDirectorioReporteConstanciasTutorados(dirPath.toString());
        setNombreReporteConstanciasTutorados("Constancia Tutorado");
        alert(`Reporte de ` + seccioon + ` guardado en: ` + dirPath);
      } else {
        alert(`¡Generación de ` + seccioon + ` cancelada!`);
        return;
      }

    } catch (error) {

      alert(`¡Error en opciones de la sección ` + seccioon + `!`);

    }

  } else {

    alert(`¡Error en la selección de sección!`);

  }

  setEmergenteVisible(true);

};
  const evento_clickVerificar = async (seccioon: string) => {
    try {
      // Mostrar indicador de carga
      setIsLoading(true);
      
      if (seccioon === "LEE") {
        // La sección LEE no tiene conversión a PDF
        alert("No hay archivos para verificar en la sección LEE");
      }
      else if (seccioon === "PUJ") {
        const filePath = directorioReportePUJ;
        
        if (filePath === "Directorio de reportes" || !filePath) {
          alert("Por favor, genera primero el reporte de PUJ para convertir a PDF");
          setIsLoading(false);
          return;
        }
        
        // Extraer el directorio donde realmente estarán los archivos
        const dirPath = filePath.substring(0, filePath.lastIndexOf('\\'));
        console.log("Verificando PDFs de PUJ en:", dirPath);
        
        try {
          // Primero verificar si ya existen PDFs
          const existenPDFs = await invoke<boolean>("verificar_pdfs_existentes_puj", {
            directorioReportes: dirPath,
            tipo: "puj"
          });
          
          if (existenPDFs) {
            alert(`Los reportes de ${seccioon} ya están convertidos a PDF y listos para ser enviados.`);
          } else {
            // Si no existen PDFs, intentar convertir
            console.log("No se encontraron PDFs, convirtiendo documentos...");
            await invoke("convertir_puj_pdf", {
              urldocs: dirPath
            });
            
            // Verificar nuevamente después de la conversión
            const pdfsConvertidos = await invoke<boolean>("verificar_pdfs_existentes_puj", {
              directorioReportes: dirPath,
              tipo: "puj"
            });
            
            if (pdfsConvertidos) {
              alert(`Los reportes de ${seccioon} han sido convertidos a PDF y están listos para ser enviados.`);
            } else {
              alert(`No se pudieron convertir los documentos a PDF. Verifica que existan los archivos DOCX en el directorio.`);
            }
          }
        } catch (error) {
          console.error("Error al verificar reportes de PUJ:", error);
          alert(`Error: ${error}`);
        }
      }
      else if (seccioon === "Colegios") {
        const filePath = directorioReporteColegios;
        
        if (filePath === "Directorio de reportes" || !filePath) {
          alert("Por favor, genera primero los reportes de colegios para convertir a PDF");
          setIsLoading(false);
          return;
        }
        
        // Extraer el directorio donde realmente estarán los archivos
        const dirPath = filePath.substring(0, filePath.lastIndexOf('\\'));
        console.log("Verificando PDFs de Colegios en:", dirPath);
        
        try {
          // Primero verificar si ya existen PDFs
          const existenPDFs = await invoke<boolean>("verificar_pdfs_existentes_colegios", {
            directorioReportes: dirPath,
            tipo: "colegios"
          });
          
          if (existenPDFs) {
            alert(`Los reportes de ${seccioon} ya están convertidos a PDF y listos para ser enviados.`);
          } else {
            // Si no existen PDFs, intentar convertir
            console.log("No se encontraron PDFs, convirtiendo documentos...");
            await invoke("convertir_colegios_pdf", {
              urldocs: dirPath
            });
            
            // Verificar nuevamente después de la conversión
            const pdfsConvertidos = await invoke<boolean>("verificar_pdfs_existentes_colegios", {
              directorioReportes: dirPath,
              tipo: "colegios"
            });
            
            if (pdfsConvertidos) {
              alert(`Los reportes de ${seccioon} han sido convertidos a PDF y están listos para ser enviados.`);
            } else {
              alert(`No se pudieron convertir los documentos a PDF. Verifica que existan los archivos DOCX en el directorio.`);
            }
          }
        } catch (error) {
          console.error("Error al verificar reportes de Colegios:", error);
          alert(`Error: ${error}`);
        }
      }
      else if (seccioon === "Tutores") {
        const dirPath = directorioReporteConstanciasTutores;
        
        if (dirPath === "Directorio de reportes") {
          alert("Por favor, genera primero las constancias de tutores para convertir a PDF");
          setIsLoading(false);
          return;
        }
        
        try {
          // Primero verificar si ya existen PDFs
          const existenPDFs = await invoke<boolean>("verificar_pdfs_existentes_tutores", {
            directorioReportes: dirPath,
            tipo: "tutores"
          });
          
          if (existenPDFs) {
            alert(`Las constancias de ${seccioon} ya están convertidas a PDF y listas para ser enviadas.`);
          } else {
            // Si no existen PDFs, intentar convertir
            console.log("No se encontraron PDFs, convirtiendo documentos...");
            await invoke("convertir_tutores_pdf", {
              urldocs: dirPath
            });
            
            // Verificar nuevamente después de la conversión
            const pdfsConvertidos = await invoke<boolean>("verificar_pdfs_existentes_tutores", {
              directorioReportes: dirPath,
              tipo: "tutores"
            });
            
            if (pdfsConvertidos) {
              alert(`Las constancias de ${seccioon} han sido convertidas a PDF y están listas para ser enviadas.`);
            } else {
              alert(`No se pudieron convertir los documentos a PDF. Verifica que existan los archivos DOCX en el directorio.`);
            }
          }
        } catch (error) {
          console.error("Error al verificar constancias de Tutores:", error);
          alert(`Error: ${error}`);
        }
      }
      else if (seccioon === "Tutorados") {
        const dirPath = directorioReporteConstanciasTutorados;
        
        if (dirPath === "Directorio de reportes") {
          alert("Por favor, genera primero las constancias de tutorados para convertir a PDF");
          setIsLoading(false);
          return;
        }
        
        try {
          // Primero verificar si ya existen PDFs
          const existenPDFs = await invoke<boolean>("verificar_pdfs_existentes_tutorados", {
            directorioReportes: dirPath,
            tipo: "tutorados"
          });
          
          if (existenPDFs) {
            alert(`Las constancias de ${seccioon} ya están convertidas a PDF y listas para ser enviadas.`);
          } else {
            // Si no existen PDFs, intentar convertir
            console.log("No se encontraron PDFs, convirtiendo documentos...");
            await invoke("convertir_tutorados_pdf", {
              urldocs: dirPath
            });
            
            // Verificar nuevamente después de la conversión
            const pdfsConvertidos = await invoke<boolean>("verificar_pdfs_existentes_tutorados", {
              directorioReportes: dirPath,
              tipo: "tutorados"
            });
            
            if (pdfsConvertidos) {
              alert(`Las constancias de ${seccioon} han sido convertidas a PDF y están listas para ser enviadas.`);
            } else {
              alert(`No se pudieron convertir los documentos a PDF. Verifica que existan los archivos DOCX en el directorio.`);
            }
          }
        } catch (error) {
          console.error("Error al verificar constancias de Tutorados:", error);
          alert(`Error: ${error}`);
        }
      }
      else {
        alert(`¡Error en la selección de sección!`);
      }
    } catch (error) {
      console.error(`Error al verificar ${seccioon}:`, error);
      alert(`Error: ${error}`);
    } finally {
      setIsLoading(false);
      setEmergenteVisible(false);
    }
  };

  // Función modificada para enviar documentos solo si ya existen los PDFs
  const evento_clickEnviar = async (seccioon: string) => {
    try {
      if (seccioon === "LEE") {
        alert(`¡Envío exitoso del módulo ${seccioon}!`);
      }
      else if (seccioon === "PUJ") {
        const filePath = directorioReportePUJ;
        
        if (filePath === "Directorio de reportes" || !filePath) {
          alert("Por favor, genera primero el reporte de PUJ");
          return;
        }
        
        // Extraer el directorio donde realmente estarán los archivos
        const dirPath = filePath.substring(0, filePath.lastIndexOf('\\'));
        console.log("Usando directorio para búsqueda:", dirPath);
        
        // Mostrar indicador de carga
        setIsLoading(true);
        
        try {
          // Verificar si existen PDFs antes de enviar
          const existenPDFs = await invoke<boolean>("verificar_pdfs_existentes_puj", {
            directorioReportes: dirPath,
            tipo: "puj"
          });
          
          if (!existenPDFs) {
            // Si no existen, intentar convertir
            try {
              await invoke("convertir_puj_pdf", {
                urldocs: dirPath
              });
              console.log("Se han convertido los documentos a PDF");
            } catch (e) {
              console.error("Error al convertir a PDF:", e);
              alert("Los documentos no pudieron ser convertidos a PDF. Por favor use la opción 'Verificar' primero.");
              setIsLoading(false);
              return;
            }
          } else {
            console.log("Los PDFs ya existen, no es necesario convertir");
          }
          
          // Llamar al backend con la ruta del DIRECTORIO
          const destinatarios = await invoke<any[]>("reportes_puj_enviar_por_whatsapp", {
            directorioReportes: dirPath
          });
          
          if (destinatarios && destinatarios.length > 0) {
            // Confirmar el envío automático
            const confirmar = window.confirm(
              `Se encontraron ${destinatarios.length} destinatarios para PUJ. ¿Deseas enviar los mensajes automáticamente?`
            );
            
            if (confirmar) {
              // Abrir enlaces de WhatsApp con un pequeño retraso entre cada uno
              destinatarios.forEach((destinatario, index) => {
                setTimeout(() => {
                  window.open(destinatario.whatsapp_url, "_blank");
                }, index * 800); // 800ms de retraso para evitar bloqueos
              });
              
              alert(`Se iniciaron ${destinatarios.length} envíos para los reportes de PUJ`);
            } else {
              alert("Los enlaces de WhatsApp están preparados pero no se enviaron automáticamente");
            }
          } else {
            alert("No se encontraron destinatarios para enviar los reportes de PUJ");
          }
        } catch (error) {
          console.error("Error al procesar envíos de PUJ:", error);
          alert(`Error: ${error}`);
        } finally {
          setIsLoading(false);
        }
      }
      else if (seccioon === "Colegios") {
        const filePath = directorioReporteColegios;
        
        if (filePath === "Directorio de reportes" || !filePath) {
          alert("Por favor, genera primero los reportes de colegios");
          return;
        }
        
        // Extraer el directorio donde realmente estarán los archivos
        const dirPath = filePath.substring(0, filePath.lastIndexOf('\\'));
        console.log("Usando directorio para búsqueda:", dirPath);
        
        // Mostrar indicador de carga
        setIsLoading(true);
        
        try {
          // Verificar si existen PDFs antes de enviar
          const existenPDFs = await invoke<boolean>("verificar_pdfs_existentes_colegios", {
            directorioReportes: dirPath,
            tipo: "colegios"
          });
          
          if (!existenPDFs) {
            // Si no existen, intentar convertir
            try {
              await invoke("convertir_colegios_pdf", {
                urldocs: dirPath
              });
              console.log("Se han convertido los documentos a PDF");
            } catch (e) {
              console.error("Error al convertir a PDF:", e);
              alert("Los documentos no pudieron ser convertidos a PDF. Por favor use la opción 'Verificar' primero.");
              setIsLoading(false);
              return;
            }
          } else {
            console.log("Los PDFs ya existen, no es necesario convertir");
          }
          
          // Llamar al backend con la ruta del DIRECTORIO
          const destinatarios = await invoke<any[]>("reportes_colegios_enviar_por_whatsapp", {
            directorioReportes: dirPath
          });
          
          if (destinatarios && destinatarios.length > 0) {
            // Confirmar el envío automático
            const confirmar = window.confirm(
              `Se encontraron ${destinatarios.length} destinatarios. ¿Deseas enviar los mensajes automáticamente?`
            );
            
            if (confirmar) {
              // Abrir enlaces de WhatsApp with un pequeño retraso entre cada uno
              destinatarios.forEach((destinatario, index) => {
                setTimeout(() => {
                  window.open(destinatario.whatsapp_url, "_blank");
                }, index * 800); // 800ms de retraso para evitar bloqueos
              });
              
              alert(`Se iniciaron ${destinatarios.length} envíos para los reportes de colegios`);
            } else {
              alert("Los enlaces de WhatsApp están preparados pero no se enviaron automáticamente");
            }
          } else {
            alert("No se encontraron destinatarios para enviar los reportes de colegios");
          }
        } catch (error) {
          console.error("Error al procesar envíos de Colegios:", error);
          alert(`Error: ${error}`);
        } finally {
          setIsLoading(false);
        }
      }
      else if (seccioon === "Tutores") {
        // Verificar si existe el archivo de emparejamiento
        if (!archivoPath_Emparejamiento || archivoPath_Emparejamiento === "") {
          alert("Primero debes seleccionar un archivo de emparejamiento para los tutores");
          return;
        }

        // Enviar primero la ruta del archivo de emparejamiento al backend
        try {
          await invoke("reportes_tutores_recibir_emparejamiento", {
            archivoPathEmparejamiento: archivoPath_Emparejamiento
          });
          console.log("Archivo de emparejamiento enviado correctamente");
        } catch (error) {
          console.error("Error al enviar archivo de emparejamiento:", error);
          alert(`Error: No se pudo procesar el archivo de emparejamiento: ${error}`);
          return;
        }

        const dirPath = directorioReporteConstanciasTutores;
        
        if (dirPath === "Directorio de reportes") {
          alert("Por favor, genera primero las constancias de tutores");
          return;
        }
        
        // Mostrar indicador de carga
        setIsLoading(true);
        
        try {
          // Verificar si existen PDFs antes de enviar
          const existenPDFs = await invoke<boolean>("verificar_pdfs_existentes_tutores", {
            directorioReportes: dirPath,
            tipo: "tutores"
          });
          
          if (!existenPDFs) {
            // Si no existen, intentar convertir
            try {
              await invoke("convertir_tutores_pdf", {
                urldocs: dirPath
              });
              console.log("Se han convertido los documentos a PDF");
            } catch (e) {
              console.error("Error al convertir a PDF:", e);
              alert("Los documentos no pudieron ser convertidos a PDF. Por favor use la opción 'Verificar' primero.");
              setIsLoading(false);
              return;
            }
          } else {
            console.log("Los PDFs ya existen, no es necesario convertir");
          }
          
          // Llamar a la función para generar el Excel de envíos y preparar mensajes
          const destinatarios = await invoke<any[]>("reportes_tutores_enviar_por_whatsapp", {
            directorioReportes: dirPath,
            archivoEmparejamiento: archivoPath_Emparejamiento  // Usamos el archivo específico para tutores
          });
          
          if (destinatarios && destinatarios.length > 0) {
            // Confirmar el envío automático
            const confirmar = window.confirm(
              `Se encontraron ${destinatarios.length} tutores con constancias. ¿Deseas enviar los mensajes automáticamente?`
            );
            
            if (confirmar) {
              // Abrir enlaces de WhatsApp with un pequeño retraso entre cada uno
              destinatarios.forEach((destinatario, index) => {
                setTimeout(() => {
                  // Solo abrir si tiene URL de WhatsApp
                  if (destinatario.whatsapp_url) {
                    window.open(destinatario.whatsapp_url, "_blank");
                  }
                }, index * 800); // 800ms de retraso para evitar bloqueos
              });
              
              alert(`Se iniciaron ${destinatarios.length} envíos para las constancias de tutores`);
            } else {
              alert("Los enlaces de WhatsApp están preparados pero no se enviaron automáticamente");
            }
          } else {
            alert("No se encontraron tutores a los que enviar constancias. Verifica que los nombres coincidan con los archivos generados.");
          }
        } catch (error) {
          console.error("Error al procesar envíos de constancias de tutores:", error);
          alert(`Error: ${error}`);
        } finally {
          setIsLoading(false);
        }
      }
      else if (seccioon === "Tutorados") {
        const dirPath = directorioReporteConstanciasTutorados;
        
        if (dirPath === "Directorio de reportes") {
          alert("Por favor, genera primero las constancias de tutorados");
          return;
        }
        
        // Mostrar indicador de carga
        setIsLoading(true);
        
        try {
          // Verificar si existen PDFs antes de enviar
          const existenPDFs = await invoke<boolean>("verificar_pdfs_existentes_tutorados", {
            directorioReportes: dirPath,
            tipo: "tutorados"
          });
          
          if (!existenPDFs) {
            // Si no existen, intentar convertir
            try {
              await invoke("convertir_tutorados_pdf", {
                urldocs: dirPath
              });
              console.log("Se han convertido los documentos a PDF");
            } catch (e) {
              console.error("Error al convertir a PDF:", e);
              alert("Los documentos no pudieron ser convertidos a PDF. Por favor use la opción 'Verificar' primero.");
              setIsLoading(false);
              return;
            }
          }
          
          // Llamar a la función para generar el Excel de envíos y preparar mensajes
          const destinatarios = await invoke<any[]>("reportes_tutorados_enviar_por_whatsapp", {
            directorioReportes: dirPath
          });
          
          if (destinatarios && destinatarios.length > 0) {
            // Confirmar el envío automático
            const confirmar = window.confirm(
              `Se encontraron ${destinatarios.length} tutorados con constancias. ¿Deseas enviar los mensajes automáticamente?`
            );
            
            if (confirmar) {
              // Abrir enlaces de WhatsApp with un pequeño retraso entre cada uno
              destinatarios.forEach((destinatario, index) => {
                setTimeout(() => {
                  // Solo abrir si tiene URL de WhatsApp
                  if (destinatario.whatsapp_url) {
                    window.open(destinatario.whatsapp_url, "_blank");
                  }
                }, index * 800); // 800ms de retraso para evitar bloqueos
              });
              
              alert(`Se iniciaron ${destinatarios.length} envíos para las constancias de tutorados`);
            } else {
              alert("Los enlaces de WhatsApp están preparados pero no se enviaron automáticamente");
            }
          } else {
            alert("No se encontraron tutorados a los que enviar constancias. Verifica que los nombres coincidan con los archivos generados.");
          }
        } catch (error) {
          console.error("Error al procesar envíos de constancias de tutorados:", error);
          alert(`Error: ${error}`);
        } finally {
          setIsLoading(false);
        }
      }
      else {
        alert(`¡Error en la selección de sección!`);
      }
    } catch (error) {
      setIsLoading(false);
      console.error(`Error al procesar envíos de ${seccioon}:`, error);
      alert(`Error: ${error}`);
    }

    setEmergenteVisible(false);
  };

  const fileInputRef = useRef<HTMLInputElement | null>(null);
  // Handle file selection
  const handleFileChange = () => { };
  // Trigger file selection dialog.
  const handleFileClick = () => {
    fileInputRef.current?.click();
  };

  // En la función que maneja el cambio de archivo de emparejamiento
  const handleEmparejamientoFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      // El objeto File no tiene 'path', así que solo podemos obtener el nombre
      const fileName = file.name;
      setArchivoPath_Emparejamiento(fileName);
      
      // Si necesitas la ruta completa, debes obtenerla desde el diálogo de Tauri, no desde el input file
      // Aquí solo enviamos el nombre al backend como ejemplo
      try {
        await invoke("reportes_tutores_recibir_emparejamiento", {
          urlarchivo: fileName
        });
        console.log("Archivo de emparejamiento enviado correctamente");
      } catch (error) {
        console.error("Error al enviar archivo de emparejamiento:", error);
        alert(`Error: ${error}`);
      }
    }
  };


  return (


    <div className="reportes">


      {getEmergenteVisible && (
        <Emergente
          mensaje={`Opciones para los reportes de ${seccioonActual}.`}
          cancelar={evento_clickCancelar}
          generar={() => evento_clickGenerar(seccioonActual)}
          verificar={() => evento_clickVerificar(seccioonActual)} // Usando función anónima
          enviar={() => evento_clickEnviar(seccioonActual)}
          modulo={seccioonActual}
        />
      )}


      <div className="seccioon">
        <div className="tiitulo">
          LEE
        </div>
        <ul className="lista">
          <li>
            <input
              type="date"
              value={fechaLee}
              onChange={(e) => setFechaLee(e.target.value)}
              onBlur={() => {
                invoke("reportes_lee_actualizarfecha", { nuevaFecha: fechaLee })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={handleSelectArchivo_Emparejamiento} className="hover-underline">
            {archivoPath_Emparejamiento}
          </li>
          <li onClick={() => handleSelectFolder_LEE()} className="hover-underline">
            {folderPath_LEE}
          </li>
          <li className="base">
            {directorioReporteLee}
          </li>
          <li className="base">
            {nombreReporteLee}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={() => evento_clickOpciones("LEE")}>
            Opciones
          </button>
        </div>
      </div>


      <div className="seccioon">
        <div className="tiitulo">
          PUJ
        </div>
        <ul className="lista">
          <li>
            <input
              type="date"
              value={fechaPUJ}
              onChange={(e) => setFechaPUJ(e.target.value)}
              onBlur={() => {
                invoke("reportes_puj_actualizarfecha", { nuevaFecha: fechaPUJ })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={() => handleSelectArchivo_LEE()} className="hover-underline">
            {archivoPath_LEE}
          </li>
          <li onClick={() => handleSelectPlantilla_PUJ()} className="hover-underline">
            {plantillaPath_PUJ}
          </li>
          <li className="base">
            {directorioReportePUJ}
          </li>
          <li className="base">
            {nombreReportePUJ}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={() => evento_clickOpciones("PUJ")}>
            Opciones
          </button>
        </div>
      </div>


      <div className="seccioon">
        <div className="tiitulo">
          Colegios
        </div>
        <ul className="lista">
          <li>
            <input
              type="date"
              value={fechaColegios}
              onChange={(e) => setFechaColegios(e.target.value)}
              onBlur={() => {
                invoke("reportes_colegios_actualizarfecha", { nuevaFecha: fechaColegios })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={() => handleSelectArchivo_LEE()} className="hover-underline">
            {archivoPath_LEE}
          </li>
          <li onClick={() => handleSelectPlantilla_Colegios()} className="hover-underline">
            {plantillaPath_Colegios}
          </li>
          <li className="base">
            {directorioReporteColegios}
          </li>
          <li className="base">
            {nombreReporteColegios}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={() => evento_clickOpciones("Colegios")}>
            Opciones
          </button>
        </div>
      </div>


      <div className="seccioon">
        <div className="tiitulo">
          Tutores
        </div>
        <ul className="lista">
          <li>
            <input
              type="date"
              value={fechaConstanciasTutores}
              onChange={(e) => setFechaConstanciasTutores(e.target.value)}
              onBlur={() => {
                invoke("reportes_constanciastutores_actualizarfecha", { nuevaFecha: fechaConstanciasTutores })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={() => handleSelectArchivo_LEE()} className="hover-underline">
            {archivoPath_LEE}
          </li>
          <li onClick={() => handleSelectArchivo_Emparejamiento()} className="hover-underline">
            {archivoPath_Emparejamiento}
          </li>
          <li onClick={() => handleSelectPlantilla_ConstanciasTutores()} className="hover-underline">
            {plantillaPath_ConstanciasTutores}
          </li>
          <li className="base">
            {directorioReporteConstanciasTutores}
          </li>
          <li className="base">
            {nombreReporteConstanciasTutores}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={() => evento_clickOpciones("Tutores")}>
            Opciones
          </button>
        </div>
      </div>


      <div className="seccioon">
        <div className="tiitulo">
          Tutorados
        </div>
        <ul className="lista">
          <li>
            <input
              type="date"
              value={fechaConstanciasTutorados}
              onChange={(e) => setFechaConstanciasTutorados(e.target.value)}
              onBlur={() => {
                invoke("reportes_constanciastutorados_actualizarfecha", { nuevaFecha: fechaConstanciasTutorados })
                  .then(() => console.log("Fecha actualizada"))
                  .catch((err) => console.error("Failed to update date:", err));
              }}
            />
          </li>
          <li onClick={() => handleSelectArchivo_Emparejamiento()} className="hover-underline">
            {archivoPath_Emparejamiento}
          </li>
          <li onClick={() => handleSelectPlantilla_ConstanciasTutorados()} className="hover-underline">
            {plantillaPath_ConstanciasTutorados}
          </li>
          <li className="base">
            {directorioReporteConstanciasTutorados}
          </li>
          <li className="base">
            {nombreReporteConstanciasTutorados}
          </li>
        </ul>
        <div className="opciones">
          <button onClick={() => evento_clickOpciones("Tutorados")}>
            Opciones
          </button>
        </div>
      </div>


      {/* Hidden file input for file selection */}
      <input
        type="file"
        ref={fileInputRef}
        style={{ display: "none" }}
        accept="application/pdf"
        onChange={handleFileChange}
      />


      {/* Loading spinner */}
      {isLoading && (
        <div className="loading-overlay">
          <div className="loading-spinner"></div>
          <p>Procesando Conversion de PDF...</p>
        </div>
      )}

    </div>


  );


}


export default Reportes;

