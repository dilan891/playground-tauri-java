use std::process::Command;
use std::env;
use std::thread;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        /* .setup(|_app| {
            // Llama a la función para ejecutar el .jar
            match run_java_jar() {
                Ok(_) => {
                    println!("El archivo .jar se ejecutó correctamente.");
                }
                Err(err) => {
                    println!("Error al ejecutar el archivo .jar: {}", err);
                    // Si falla, podrías terminar la app si es crítico
                    return Err(Box::from(err));
                }
            }
            Ok(())
        })*/
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// This function will be called from the main.rs file
pub fn call_java() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    let jar_path = current_dir
        .join("extraFiles")
        .join("electron-react-java.jar");

    if !jar_path.exists() {
        return format!("El archivo no existe: {}", jar_path.display());
    }

    // Ejecuta el comando para correr el archivo .jar
    let output = Command::new("java")
        .args(&["-jar", jar_path.to_str().unwrap()])
        .output()
        .expect("No se pudo ejecutar el proceso");

    if output.status.success() {
        return "Java code executed".to_string();
    } else {
        return "Error executing Java code".to_string();
    }
}

fn run_java_jar() -> Result<(), String> {
    // Obtén la ruta actual del directorio del programa
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;

    // Construye la ruta completa al archivo .jar
    let jar_path = current_dir
        .join("extraFiles")
        .join("electron-react-java.jar");

    // Verifica si el archivo existe
    if !jar_path.exists() {
        return Err(format!("El archivo no existe: {}", jar_path.display()));
    }

    // Ejecuta el comando para correr el archivo .jar
    let output = Command::new("java")
        .args(&["-jar", jar_path.to_str().unwrap()])
        .output()
        .map_err(|e| format!("Error al ejecutar el proceso: {}", e))?;

    // Verifica si el comando se ejecutó correctamente
    if output.status.success() {
        println!("Java ejecutado con éxito.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Error al ejecutar Java: {}", stderr))
    }
}
