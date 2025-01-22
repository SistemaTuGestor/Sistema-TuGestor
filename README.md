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
node: v20.18.0
npm: 11.0.0
rustc: 1.81.0
rustup: 1.27.1
```

## Ejecución
```
cd stg
npm run tauri dev
```

## Construcción
```
npm run tauri build
```