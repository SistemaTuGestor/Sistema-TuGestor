// EmergenteMonitoreo.tsx
import { invoke } from "@tauri-apps/api";
import { useState, useRef } from "react";

interface EmergenteMonitoreoProps {
  mensaje: string;
  cancelar: () => void;
  onGuardar: (tipo: 'tarea' | 'imagen', datos: any) => void;
}

function EmergenteMonitoreo({ mensaje, cancelar, onGuardar }: EmergenteMonitoreoProps) {
  const [tipo, setTipo] = useState<'tarea' | 'imagen'>('tarea');
  const [nombreTarea, setNombreTarea] = useState('');
  const [descripcion, setDescripcion] = useState('');
  const [archivoImagen, setArchivoImagen] = useState<File | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      setArchivoImagen(e.target.files[0]);
    }
  };

  const handleGuardar = async () => {
    if (tipo === 'tarea') {
      if (!nombreTarea || !descripcion) {
        alert('Por favor complete todos los campos de la tarea');
        return;
      }
      onGuardar('tarea', { nombre: nombreTarea, descripcion, hecho: false });
    } else {
      if (!archivoImagen) {
        alert('Por favor seleccione una imagen');
        return;
      }

      try {
        // Leer el archivo como array de bytes
        const fileBytes = await readFileAsArrayBuffer(archivoImagen);

        // Enviar al backend para guardar en ubicación persistente
        const savedPath: string = await invoke("guardar_imagen_persistente", {
          fileData: Array.from(fileBytes), // Convertir Uint8Array a array normal
          fileName: archivoImagen.name
        });

        onGuardar('imagen', {
          url: savedPath // Usar la ruta devuelta por el backend
        });
      } catch (error) {
        console.error("Error al guardar la imagen:", error);
        alert('Error al guardar la imagen');
        return;
      }
    }
    cancelar();
  };

  // Función auxiliar para leer el archivo como ArrayBuffer
  const readFileAsArrayBuffer = (file: File): Promise<Uint8Array> => {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => {
        if (reader.result) {
          resolve(new Uint8Array(reader.result as ArrayBuffer));
        } else {
          reject(new Error("No se pudo leer el archivo"));
        }
      };
      reader.onerror = reject;
      reader.readAsArrayBuffer(file);
    });
  };


  return (
    <div className="emergente-monitoreo-overlay">
      <div className="emergente-monitoreo-content">
        <h3 className="emergente-titulo">{mensaje}</h3>

        <div className="tipo-seleccion">
          <button
            className={`tipo-boton ${tipo === 'tarea' ? 'active' : ''}`}
            onClick={() => setTipo('tarea')}
          >
            Tarea
          </button>
          <button
            className={`tipo-boton ${tipo === 'imagen' ? 'active' : ''}`}
            onClick={() => setTipo('imagen')}
          >
            Imagen
          </button>
        </div>

        {tipo === 'tarea' ? (
          <div className="formulario-tarea">
            <div className="form-group">
              <label className="form-label">Nombre de la tarea:</label>
              <input
                type="text"
                className="form-input"
                value={nombreTarea}
                onChange={(e) => setNombreTarea(e.target.value)}
              />
            </div>
            <div className="form-group">
              <label className="form-label">Descripción:</label>
              <textarea
                className="form-textarea"
                value={descripcion}
                onChange={(e) => setDescripcion(e.target.value)}
              />
            </div>
          </div>
        ) : (
          <div className="formulario-imagen">
            <div className="form-group">
              <label className="form-label">Seleccionar imagen:</label>
              <input
                type="file"
                ref={fileInputRef}
                onChange={handleFileChange}
                accept="image/*"
                className="file-input"
              />
              <div
                className="file-selector"
                onClick={() => fileInputRef.current?.click()}
              >
                <span className="file-selector-text">
                  {archivoImagen ? archivoImagen.name : "Seleccionar archivo"}
                </span>
              </div>

              {archivoImagen && (
                <div className="image-preview-container">
                  <img
                    src={URL.createObjectURL(archivoImagen)}
                    alt="Preview"
                    style={{ maxWidth: '200px', maxHeight: '200px' }}
                  />
                </div>
              )}
            </div>
          </div>
        )}

        <div className="emergente-acciones">
          <button
            className="boton boton-secundario"
            onClick={cancelar}
          >
            Cancelar
          </button>
          <button
            className="boton boton-primario"
            onClick={handleGuardar}
          >
            Guardar
          </button>
        </div>
      </div>
    </div>
  );
}

export default EmergenteMonitoreo;