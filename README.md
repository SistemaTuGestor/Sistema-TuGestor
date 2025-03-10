# Sistema-TuGestor
Repositorio para implementación, testeo y despliegue del Sistema TuGestor (STG): software de coordinación para la Iniciativa Proyecto TuTutor de la Pontificia Universidad Javeriana sede Bogotá D.C.


## Previo a desarrollo
```
cd stg
npm install
cd src-tauri
cargo update
```

## Tecnologías
```
node: v23.7.0
npm: 11.0.0
rustc: 1.84.1
rustup: 1.27.1
```

## Ejecución
```
cd stg
npm run tauri dev
```

## Construcción
### Windows
```
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64
cd src-tauri/
cargo build --release --target x86_64-pc-windows-gnu
npm run tauri build
```
### Linux
```
rustup target add x86_64-unknown-linux-gnu
cd src-tauri/
cargo build --release --target x86_64-unknown-linux-gnu
npm run tauri build
```
