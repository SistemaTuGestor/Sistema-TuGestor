
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import Reportes from '../Reportes';
import { jest } from '@jest/globals';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';


// Mock Tauri APIs antes de importar 'invoke' y 'open'
jest.mock('@tauri-apps/api/tauri', () => ({
  invoke: jest.fn(),
}));
jest.mock('@tauri-apps/api/dialog', () => ({
  open: jest.fn(),
}));


// Casts seguros de los mocks
const mockedInvoke = invoke as jest.Mock;
const mockedOpen = open as jest.Mock;

// Mock window.alert
const mockedAlert = jest.spyOn(window, 'alert').mockImplementation(() => {});

describe('Reportes Component', () => {
  
  beforeEach(() => {
    jest.clearAllMocks();
    mockedInvoke.mockImplementation((cmd) => {
      if (cmd === 'obtener_fecha') {
        return Promise.resolve({ fecha: '2023-01-01' });
      }
      return Promise.resolve({});
    });
  });


  ////    TESTING   ////

  // Prueba 1: Procesamiento inicial y entrada de fecha

  it('loads current date for all sections on mount', async () => {
    render(<Reportes />);
    
    await waitFor(() => {
      expect(mockedInvoke).toHaveBeenCalledWith('obtener_fecha');
    });
    
    expect(mockedInvoke).toHaveBeenCalledTimes(5);
    
    const dateInputs = screen.getAllByDisplayValue('2023-01-01');
    expect(dateInputs).toHaveLength(5);
  });

  // Prueba 2: Selección de archivos para Reporte de Emparejamiento.

  it('handles emparejamiento file selection', async () => {
    mockedOpen.mockResolvedValue('recursos/test_data/emparejamiento_final.xlsx');
    
    render(<Reportes />);
    
    const emparejamientoItem = screen.getByText('Ubicación de archivo Emparejamiento');
    fireEvent.click(emparejamientoItem);
    
    await waitFor(() => {
      expect(mockedInvoke).toHaveBeenCalledWith(
        'reportes_lee_recibir_emparejamiento',
        { path: 'recursos/test_data/emparejamiento_final.xlsx' }
      );
      expect(mockedInvoke).toHaveBeenCalledWith(
        'reportes_tutorados_recibir_emparejamiento', 
        { path: 'recursos/test_data/emparejamiento_final.xlsx' }
      );
      expect(screen.getByText('recursos/test_data/emparejamiento_final.xlsx')).toBeInTheDocument();
    });
  });

  // Prueba 3: Generación de Reporte LEE.

  it('generates LEE report when all requirements are met', async () => {
    mockedOpen
      .mockResolvedValueOnce('recursos/test_data/test_carpeta') // Folder selection
      .mockResolvedValueOnce('recursos/test_data/output/report.xlsx'); // Save dialog
    
    render(<Reportes />);
    
    const formsItem = screen.getByText('Ubicación de formularios');
    fireEvent.click(formsItem);
    
    const optionsBtn = screen.getAllByText('Opciones')[0];
    fireEvent.click(optionsBtn);
    
    const generateBtn = screen.getByText('Generar');
    fireEvent.click(generateBtn);
    
    await waitFor(() => {
      expect(mockedInvoke).toHaveBeenCalledWith(
        'reportes_lee_recibir_nombrereporte',
        { nombrereporte: 'recursos/test_data/output/report.xlsx' }
      );
      expect(mockedInvoke).toHaveBeenCalledWith('reportes_lee_leer_archivos_en_carpeta');
      expect(screen.getByText('recursos/test_data/output/report.xlsx')).toBeInTheDocument();
    });
  });

  // Prueba 4: Manejo de falta de requisitos

  it('shows alert when generating LEE report without folder', async () => {
    render(<Reportes />);
    
    const optionsBtn = screen.getAllByText('Opciones')[0];
    fireEvent.click(optionsBtn);
    
    const generateBtn = screen.getByText('Generar');
    fireEvent.click(generateBtn);
    
    await waitFor(() => {
      expect(mockedAlert).toHaveBeenCalledWith(
        'Por favor, selecciona un directorio de formularios antes de generar el reporte de LEE.'
      );
    });
  });

  // Prueba 5: Control de visibilidad modal

  it('shows and hides options modal correctly', async () => {
    render(<Reportes />);
    
    expect(screen.queryByText(/Opciones para los reportes de/)).not.toBeInTheDocument();
    
    const optionsBtn = screen.getAllByText('Opciones')[0];
    fireEvent.click(optionsBtn);
    
    expect(screen.getByText('Opciones para los reportes de LEE.')).toBeInTheDocument();
    
    const cancelBtn = screen.getByText('Cancelar');
    fireEvent.click(cancelBtn);
    
    expect(screen.queryByText(/Opciones para los reportes de/)).not.toBeInTheDocument();
  });

  // Prueba de integración completa

  it('completes Tutores report generation flow', async () => {
    mockedOpen
      .mockResolvedValueOnce('recursos/test_data/test_data.xlsx') // LEE file
      .mockResolvedValueOnce('recursos/Plantilla - Constancias Tutores.docx') // Template
      .mockResolvedValueOnce('recursos/test_data/output'); // Output dir
    
    render(<Reportes />);
    
    const leeFileItem = screen.getAllByText('Ubicación de archivo LEE')[0];
    fireEvent.click(leeFileItem);
    
    const templateItem = screen.getAllByText('Ubicación de plantilla')[2];
    fireEvent.click(templateItem);
    
    const optionsBtn = screen.getAllByText('Opciones')[3];
    fireEvent.click(optionsBtn);
    
    const generateBtn = screen.getByText('Generar');
    fireEvent.click(generateBtn);
    
    await waitFor(() => {
      expect(mockedInvoke).toHaveBeenCalledWith(
        'reportes_tutores_recibir_lee',
        { path: 'recursos/test_data/test_data.xlsx' }
      );
      expect(mockedInvoke).toHaveBeenCalledWith(
        'reportes_constanciastutores_recibir_pathplantilla',
        { path: 'recursos/Plantilla - Constancias Tutores.docx' }
      );
      expect(mockedInvoke).toHaveBeenCalledWith(
        'reportes_constanciastutores_recibir_nombrereporte',
        { nombrereporte: 'recursos/test_data/output' }
      );
      expect(mockedInvoke).toHaveBeenCalledWith('reportes_constanciastutores_generar');
      expect(screen.getByText('recursos/test_data/output')).toBeInTheDocument();
    });
  });

});

