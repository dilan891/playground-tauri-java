use std::process::Command;
use std::process::Child;
use std::env;
use std::thread;
use std::sync::{Arc, Mutex};
use tauri::{Builder, WindowEvent};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn handler_event(java_process: Arc<Mutex<Option<Child>>>, event: WindowEvent) {
    // Escucha el evento de cerrar ventana
    if let WindowEvent::CloseRequested { .. } = event {
        println!("Cerrando la aplicación y matando el proceso...");
        let mut process_guard = java_process.lock().unwrap();
        if let Some(mut child) = process_guard.take() {
            let _ = child.kill(); // Mata el proceso
            println!("Proceso terminado.");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let java_process = Arc::new(Mutex::new(None));

    let java_process_clone = Arc::clone(&java_process);

    Builder::default()
         .setup(|_app| {
            // Llama a la función para ejecutar el .jar
            thread::spawn(move || {
                match run_java_jar() {
                    Ok(child) => {
                        println!("El archivo .jar se ejecutó correctamente.");
                        let mut process_guard = java_process_clone.lock().unwrap();
                        *process_guard = Some(child); // Guarda el proceso
                    }
                    Err(err) => println!("Error al ejecutar el archivo .jar: {}", err),
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .on_window_event(move |window, event| {
            handler_event(java_process.clone(),  event.clone());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn run_java_jar() -> Result<Child, String> {
    // Obtén la ruta actual del directorio del programa
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;

    // Construye la ruta completa al archivo .jar
    let jar_path = current_dir
        .join("extraFiles")
        .join("electron-react-java.jar");

    let java_jre_path = current_dir
        .join("extraFiles")
        .join("customjreOpenjdk")
        .join("bin")
        .join("java");

    // Verifica si el archivo existe
    if !jar_path.exists() {
        return Err(format!("El archivo no existe: {}", jar_path.display()));
    }

    // Ejecuta el comando para correr el archivo .jar
    let child = Command::new(java_jre_path)
        .args(&["-jar", jar_path.to_str().unwrap()])
        .spawn()
        .map_err(|e| format!("Error al ejecutar el proceso: {}", e))?;

    Ok(child)
}
