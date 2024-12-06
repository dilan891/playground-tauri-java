# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Proyect configuration
Se debe tener instalado bun.js y rustup para poder ejecutar el proyecto.

Añade los archivos de java en la carpeta `src-tauri/extraFiles` con el nombre de `electron-react-java.jar` y la carpeta `customjreOpenjdk` con el jre de java.

## iniciar el proyecto

Para el desarrollo de la aplicación se debe ejecutar el siguiente comando:
```bash 
bun install
bun run tauri dev
```

Para compilar la aplicación se debe ejecutar el siguiente comando:
```bash
bunx tauri build
```