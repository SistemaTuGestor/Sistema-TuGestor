# 🗂️ Sistema TuGestor (STG)

Repositorio oficial para la implementación, pruebas y despliegue del **Sistema TuGestor (STG)**, una aplicación de escritorio desarrollada para apoyar la coordinación de la **Iniciativa Proyecto TuTutor (IPTT)** de la **Pontificia Universidad Javeriana**, sede Bogotá D.C.

---

## 🚀 Tecnologías empleadas

| Tecnología | Versión        |
|------------|----------------|
| Node.js    | v23.7.0        |
| npm        | 11.0.0         |
| Rust       | 1.84.1         |
| Rustup     | 1.27.1         |

> **Frameworks y herramientas clave**:  
> - Tauri para empaquetado multiplataforma  
> - React para la interfaz de usuario  
> - Rust para la lógica y seguridad del backend

---

## 🛠️ Preparación del entorno

Antes de comenzar el desarrollo, instala las dependencias necesarias:

```
cd stg
npm install
cd src-tauri
cargo update
```
## 🛠️ Preparación del entorno
Antes de comenzar el desarrollo, instala las dependencias necesarias:

```

cd stg
npm install
cd src-tauri
cargo update
```

## ▶️ Ejecución en modo desarrollo
Para iniciar la aplicación en modo desarrollo:

```

cd stg
npm run tauri dev
```
Esto lanzará la interfaz de usuario de React junto con la capa de backend en Rust a través de Tauri.

# 🏗️ Construcción (Build)
## 🔧 Windows
```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64
cd src-tauri
cargo build --release --target x86_64-pc-windows-gnu
cd ..
npm run tauri build
```
##🐧 Linux
```
rustup target add x86_64-unknown-linux-gnu
cd src-tauri
cargo build --release --target x86_64-unknown-linux-gnu
cd ..
npm run tauri build
```
## 📦 Output
El ejecutable generado se encuentra en:
```
src-tauri/target/{target}/release/
Y el paquete final empaquetado por Tauri se encuentra en:
```
```
stg/src-tauri/target/release/bundle/
```
## 🧪 Estado del Proyecto

- [x] Arquitectura React + Tauri + Rust  
- [x] Instalación multiplataforma  
- [x] Pruebas automatizadas  
- [x] Documentación de memoria  


## 🧠 Créditos
Desarrollado por el equipo del proyecto IPTT
Pontificia Universidad Javeriana – Ingeniería de Sistemas

