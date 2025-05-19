import { useState } from "react";

interface EmergenteMonitoreoProps {
  mensaje: string;
  cancelar: () => void;
  onGuardar: (tipo: 'tarea' | 'imagen', datos: any) => void;
}

function EmergenteMonitoreo({ mensaje, cancelar, onGuardar }: EmergenteMonitoreoProps) {
  const [tipo, setTipo] = useState<'tarea' | 'imagen'>('tarea');
  const [nombreTarea, setNombreTarea] = useState('');
  const [descripcion, setDescripcion] = useState('');
  const [urlImagen, setUrlImagen] = useState('');

  const handleGuardar = () => {
    if (tipo === 'tarea') {
      if (!nombreTarea || !descripcion) {
        alert('Por favor complete todos los campos de la tarea');
        return;
      }
      onGuardar('tarea', { nombre: nombreTarea, descripcion, hecho: false });
    } else {
      if (!urlImagen) {
        alert('Por favor ingrese la URL de la imagen');
        return;
      }
      onGuardar('imagen', { url: urlImagen });
    }
    cancelar();
  };

  return (
    <div className="emergente-monitoreo-overlay">
      <div className="emergente-monitoreo-content">
        <h3>{mensaje}</h3>
        
        <div className="tipo-seleccion">
          <button 
            className={tipo === 'tarea' ? 'active' : ''}
            onClick={() => setTipo('tarea')}
          >
            Tarea
          </button>
          <button 
            className={tipo === 'imagen' ? 'active' : ''}
            onClick={() => setTipo('imagen')}
          >
            Imagen
          </button>
        </div>

        {tipo === 'tarea' ? (
          <>
            <div className="form-group">
              <label>Nombre de la tarea:</label>
              <input
                type="text"
                value={nombreTarea}
                onChange={(e) => setNombreTarea(e.target.value)}
              />
            </div>
            <div className="form-group">
              <label>Descripción:</label>
              <textarea
                value={descripcion}
                onChange={(e) => setDescripcion(e.target.value)}
              />
            </div>
          </>
        ) : (
          <div className="form-group">
            <label>URL de la imagen:</label>
            <input
              type="text"
              value={urlImagen}
              onChange={(e) => setUrlImagen(e.target.value)}
              placeholder="C:\\ruta\\a\\la\\imagen.jpg"
            />
          </div>
        )}

        <div className="emergente-monitoreo-buttons">
          <button onClick={cancelar}>Cancelar</button>
          <button onClick={handleGuardar}>Guardar</button>
        </div>
      </div>
    </div>
  );
}

export default EmergenteMonitoreo;