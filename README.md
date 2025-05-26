# 🗂️ Sistema TuGestor (STG) – Gestión automatizada para la Iniciativa Proyecto TuTutor (IPTT)

Repositorio oficial para la implementación, pruebas y despliegue del **Sistema TuGestor (STG)**, una aplicación de escritorio desarrollada para apoyar la coordinación de la **Iniciativa Proyecto TuTutor (IPTT)** de la **Pontificia Universidad Javeriana**, sede Bogotá D.C.

---

## 📋 Descripción General

Sistema TuGestor (STG) es una aplicación multiplataforma diseñada para facilitar la gestión de tutorías académicas en la Pontificia Universidad Javeriana.  
Su objetivo principal es automatizar procesos clave del programa TuTutor, como el emparejamiento tutor-tutorado, notificaciones, monitoreo y generación de reportes, optimizando el tiempo y mejorando la calidad del programa.

---

## ✨ Características Principales

- 🔄 Emparejamiento automático y manual entre tutores y tutorados según criterios configurables (disponibilidad, áreas de conocimiento, necesidades).  
- 📱 Envío automático de notificaciones vía WhatsApp y correo institucional.  
- 📊 Panel de monitoreo visual para seguimiento del avance y cumplimiento de actividades.  
- 📄 Generación automática de reportes en formatos PDF y CSV, adaptados a los estándares institucionales.  
- 💻 Aplicación multiplataforma: compatible con Windows, MacOS y GNU/Linux (Debian).  
- 🔓 Código abierto y sin costo, facilitando su acceso y mantenimiento.  

---

## 📑 Tabla de Contenidos

- [Descripción General](#-descripción-general)  
- [Características Principales](#-características-principales)  
- [Tecnologías Empleadas](#-tecnologías-empleadas)  
- [Requisitos del Sistema](#-requisitos-del-sistema)  
- [Preparación del Entorno](#️-preparación-del-entorno)  
- [Ejecución en Modo Desarrollo](#️-ejecución-en-modo-desarrollo)  
- [Construcción (Build)](#️-construcción-build)  
- [Uso](#-uso)  
- [Estructura del Proyecto](#-estructura-del-proyecto)  
- [Estado del Proyecto](#-estado-del-proyecto)  
- [Requisitos Funcionales y No Funcionales](#-requisitos-funcionales-y-no-funcionales)  
- [Restricciones](#-restricciones)  
- [Cómo Contribuir](#-cómo-contribuir)  
- [Licencia](#-licencia)  
- [Contacto](#-contacto)  
- [Créditos](#-créditos)

---

## 🚀 Tecnologías Empleadas

| Tecnología | Versión        |
|------------|----------------|
| Node.js    | v23.7.0        |
| npm        | 11.0.0         |
| Rust       | 1.84.1         |
| Rustup     | 1.27.1         |

> **Frameworks y herramientas clave**:  
> - 📦 Tauri para empaquetado multiplataforma  
> - ⚛️ React para la interfaz de usuario  
> - 🦀 Rust para la lógica y seguridad del backend
> - 📝 TypeScript para desarrollo de interfaz gráfica
> - 🔄 JSON para almacenamiento y persistencia de datos

---

## 💻 Requisitos del Sistema

### Hardware mínimo

- 💾 16 MB de almacenamiento libre en disco.  
- 🖱️ Teclado, ratón, monitor, tarjeta de red.  
- 🔐 Permisos de ejecución adecuados en el sistema operativo.  

### Software

- 🖥️ Sistemas operativos compatibles: Windows, MacOS, GNU/Linux (Debian).  
- 📚 Dependencias:  
  - Node.js  
  - Rust  
  - TypeScript  
  - Tauri  
- 🌐 Conexión TCP/IP para funciones de notificación (puertos 80 y 443).  

---

## 🛠️ Preparación del entorno

Antes de comenzar el desarrollo, instala las dependencias necesarias:

```bash
cd stg
npm install
cd src-tauri
cargo update
```
### ▶️ Ejecución en modo desarrollo
Para iniciar la aplicación en modo desarrollo:

```bash
cd stg
npm run tauri dev
Esto lanzará la interfaz de usuario de React junto con la capa de backend en Rust a través de Tauri.
```

## 🏗️ Construcción (Build)
### 🪟 Windows
```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64
cd src-tauri
cargo build --release --target x86_64-pc-windows-gnu
cd ..
npm run tauri build
```
###🐧 Linux
bash
Mostrar siempre los detalles

Copiar
rustup target add x86_64-unknown-linux-gnu
cd src-tauri
cargo build --release --target x86_64-unknown-linux-gnu
cd ..
npm run tauri build
📦 Output
El ejecutable generado se encuentra en:

bash
Mostrar siempre los detalles

Copiar
src-tauri/target/{target}/release/
Y el paquete final empaquetado por Tauri se encuentra en:

```swift
stg/src-tauri/target/release/bundle/
```
### 🚀 Instalación
Clonar el repositorio:

```bash
git clone https://github.com/SistemaTuGestor/Sistema-TuGestor.git
cd tuGestor
```
Instalar dependencias:
```bash
npm install
Instalar Rust y Tauri (si no están instalados):
```
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
npm install -g @tauri-apps/cli
```
Construir la aplicación:
```bash

npm run tauri build
```
Ejecutar la aplicación:

```bash
npm run tauri dev
```
## 📱 Uso
### 📂 Cargar los archivos de datos de tutores y tutorados desde la interfaz gráfica.

### 🔄 Ejecutar el módulo de emparejamiento para asignar automáticamente o manualmente los pares.

### 📲 Enviar notificaciones masivas a través de WhatsApp o correo institucional desde el módulo de notificaciones.

### 📊 Monitorear el progreso y cumplimiento de actividades mediante el panel visual.

### 📄 Generar reportes automáticos en PDF o CSV para presentación ante entes aliados y directivos.

📁 Estructura del Proyecto
```
/stg # Directorio raíz del proyecto
├── /src # Código fuente del frontend (React/TypeScript)
│ ├── /components # Componentes de React reutilizables
│ │ ├── /Comunes # Componentes compartidos (botones, inputs, etc.)
│ │ ├── /Pantallas # Componentes principales de cada módulo
│ │ │ ├── /Emparejamiento # Pantallas para el módulo de emparejamiento
│ │ │ ├── /Notificaciones # Pantallas para el módulo de notificaciones
│ │ │ ├── /Reportes # Pantallas para el módulo de reportes
│ │ │ └── /Monitoreo # Pantallas para el módulo de monitoreo
│ ├── /assets # Recursos estáticos (imágenes, iconos, estilos)
│ │ ├── /img # Imágenes e iconos
│ │ └── /styles # Hojas de estilo CSS
│ ├── App.tsx # Componente principal de la aplicación
│ └── main.tsx # Punto de entrada de React
│
├── /src-tauri # Código fuente del backend (Rust)
│ ├── /src # Código Rust
│ │ ├── /emparejamiento # Lógica para el emparejamiento tutor-tutorado
│ │ ├── /notificaciones # Lógica para envío de notificaciones
│ │ ├── /reportes # Lógica para generación de reportes
│ │ ├── /utils # Utilidades y funciones auxiliares
│ │ └── main.rs # Punto de entrada del backend
│ ├── Cargo.toml # Configuración de dependencias Rust
│ └── tauri.conf.json # Configuración de Tauri
│
├── /node_modules # Dependencias de Node.js (generado automáticamente)
├── /public # Archivos públicos accesibles directamente
├── /dist # Código compilado para producción (generado)
├── package.json # Configuración de dependencias npm
├── tsconfig.json # Configuración de TypeScript
└── README.md # Documentación del proyecto
```
## 📋 Estructura de Archivos por Módulo
### 1️⃣ Módulo de Emparejamiento

```bash

/src/components/Pantallas/Emparejamiento
  ├── Emparejamiento.tsx   # Componente principal del módulo
  ├── Emparejamiento.css   # Estilos específicos del módulo
  ├── ListaTutores.tsx     # Componente para mostrar lista de tutores
  ├── ListaTutorados.tsx   # Componente para mostrar lista de tutorados
  └── FormularioEmparejamiento.tsx  # Formulario para emparejar manualmente
```
```
/src-tauri/src/emparejamiento
  ├── mod.rs               # Definición del módulo en Rust
  ├── algoritmo.rs         # Algoritmo de emparejamiento automático
  ├── importacion.rs       # Funciones para importar datos desde Excel/CSV
  └── exportacion.rs       # Funciones para exportar resultados
```
### 2️⃣ Módulo de Notificaciones

```bash

/src/components/Pantallas/Notificaciones
  ├── Notificaciones.tsx   # Componente principal del módulo
  ├── Notificaciones.css   # Estilos específicos del módulo
  ├── PlantillaMensaje.tsx # Editor de plantillas de mensajes
  └── ProgramacionEnvios.tsx # Programación de envíos automatizados

/src-tauri/src/notificaciones
  ├── mod.rs               # Definición del módulo en Rust
  ├── whatsapp.rs          # Integración con WhatsApp
  ├── correo.rs            # Integración con servicio de correo
  └── programacion.rs      # Lógica para programar envíos
```
### 3️⃣ Módulo de Reportes
```
/src/components/Pantallas/Reportes
  ├── Reportes.tsx         # Componente principal del módulo
  ├── Reportes.css         # Estilos específicos del módulo
  ├── GeneradorPDF.tsx     # Componente para generar reportes PDF
  └── GeneradorCSV.tsx     # Componente para generar reportes CSV

/src-tauri/src/reportes
  ├── mod.rs               # Definición del módulo en Rust
  ├── pdf.rs               # Generación de documentos PDF
  ├── csv.rs               # Generación de documentos CSV
  └── utils.rs             # Funciones auxiliares para reportes
```
### 4️⃣ Módulo de Monitoreo

```bash

/src/components/Pantallas/Monitoreo
  ├── Monitoreo.tsx        # Componente principal del módulo
  ├── Monitoreo.css        # Estilos específicos del módulo
  ├── PanelAvance.tsx      # Panel visual de avance
  └── Estadisticas.tsx     # Estadísticas y gráficos

/src-tauri/src/monitoreo
  ├── mod.rs               # Definición del módulo en Rust
  ├── dashboard.rs         # Lógica para panel y estadísticas
  └── data.rs              # Manejo de datos de monitoreo
```
### 🛠 Estado del Proyecto
 Módulo de emparejamiento: funcional.

 Módulo de notificaciones: funcional (WhatsApp y correo).

 Módulo de reportes: funcional (PDF y CSV).

 Módulo de monitoreo: en desarrollo.

 Integración completa y pruebas finales.

 Documentación y empaquetado final.

### 📋 Requisitos Funcionales y No Funcionales
(Se documentan en el archivo REQUERIMIENTOS.md del repositorio, con detalle de atributos de calidad según ISO 25010:2014.)

## 🚫 Restricciones
Limitaciones técnicas: sólo multiplataforma Windows, MacOS y Debian Linux.

Requiere conexión TCP/IP para envío de notificaciones.

Uso obligatorio de cuentas institucionales para acceso.

Uso de servicios externos como WhatsApp y Outlook para notificaciones.

Archivos soportados para carga: Excel (.xlsx), CSV, JSON.

## 🤝 Cómo Contribuir
Forkear el repositorio.

Crear una rama con la funcionalidad o corrección: git checkout -b feature/nombre-funcion.

Realizar commits con mensajes claros.

Hacer push a la rama remota.

Crear Pull Request explicando los cambios.


## 🎉 Créditos
David Alejandro Roa Velásquez - road.a@javeriana.edu.co
Javier Emiro Useche Acosta - javierusechea@javeriana.edu.co
Jhoseph Samirt Lizarazo Murcia - jhosephs_lizarazom@javeriana.edu.co
Nicolás Quintana Cuartas - quintana.nicolas@javeriana.edu.co
