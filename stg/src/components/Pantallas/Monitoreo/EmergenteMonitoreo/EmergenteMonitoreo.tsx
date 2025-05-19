import { useState } from "react";

interface EmergenteMonitoreoProps {
  mensaje: string;
  cancelar: () => void;
  onGuardar: (tipo: 'tarea' | 'imagen', datos: any) => void;
}

function EmergenteMonitoreo({ mensaje, cancelar, onGuardar }: EmergenteMonitoreoProps) {
  // const [tipo, setTipo] = useState<'tarea' | 'imagen'>('tarea');
  const [nombreTarea, setNombreTarea] = useState('');
  const [descripcion, setDescripcion] = useState('');
  // const [urlImagen, setUrlImagen] = useState('');

  const handleGuardar = () => {
    if (!nombreTarea || !descripcion) {
      alert('Por favor complete todos los campos de la tarea');
      return;
    }
    onGuardar('tarea', { nombre: nombreTarea, descripcion, hecho: false });
    cancelar();
  };

  return (
    <div className="emergente-monitoreo-overlay">
      <div className="emergente-monitoreo-content">
        <h3>{mensaje}</h3>

        {/* Removed tipo-seleccion buttons */}

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

        <div className="emergente-monitoreo-buttons">
          <button onClick={cancelar}>Cancelar</button>
          <button onClick={handleGuardar}>Guardar</button>
        </div>
      </div>
    </div>
  );
}

export default EmergenteMonitoreo;