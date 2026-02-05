// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::process::Command;
use std::io::Write;
use std::path::PathBuf;
use std::fs;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]

#[tauri::command]
fn tts(app_handle: tauri::AppHandle, text: String) -> Result<String, String> {
    // Use temp directory for output to avoid triggering rebuilds
    let temp_dir = std::env::temp_dir();
    let output_path = temp_dir.join("tts_output.wav");
    
    println!("Output path: {:?}", output_path);
    
    // In development, piper.exe is in src-tauri/bin
    // In production, it will be in the resources directory
    let piper_path = if cfg!(debug_assertions) {
        PathBuf::from("bin/piper.exe")
    } else {
        app_handle
            .path()
            .resource_dir()
            .map_err(|e| format!("Failed to get resource dir: {}", e))?
            .join("bin")
            .join("piper.exe")
    };
    
    let model_path = if cfg!(debug_assertions) {
        PathBuf::from("models/id_ID-news_tts-medium.onnx")
    } else {
        app_handle
            .path()
            .resource_dir()
            .map_err(|e| format!("Failed to get resource dir: {}", e))?
            .join("models")
            .join("id_ID-news_tts-medium.onnx")
    };
    
    // Verify files exist
    if !piper_path.exists() {
        return Err(format!("Piper executable not found at: {:?}", piper_path));
    }
    if !model_path.exists() {
        return Err(format!("Model file not found at: {:?}", model_path));
    }
    
    println!("Piper path: {:?}", piper_path);
    println!("Model path: {:?}", model_path);

    let mut child = Command::new(&piper_path)
        .args([
            "--model",
            model_path.to_str().ok_or("Invalid model path")?,
            "--output_file",
            output_path.to_str().ok_or("Invalid output path")?,
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn piper: {}", e))?;

    child
        .stdin
        .as_mut()
        .ok_or("Failed to open stdin")?
        .write_all(text.as_bytes())
        .map_err(|e| format!("Failed to write to stdin: {}", e))?;

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for piper: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "TTS failed. Status: {}\nStderr: {}\nStdout: {}",
            output.status, stderr, stdout
        ));
    }
    
    // Verify output file was created
    if !output_path.exists() {
        return Err("Output file was not created".to_string());
    }
    
    // Read the file and convert to base64 data URL
    let audio_data = fs::read(&output_path)
        .map_err(|e| format!("Failed to read audio file: {}", e))?;
    
    let base64_data = base64_encode(&audio_data);
    let data_url = format!("data:audio/wav;base64,{}", base64_data);
    
    // Clean up the temp file
    let _ = fs::remove_file(&output_path);
    
    Ok(data_url)
}

fn base64_encode(data: &[u8]) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3f) as usize;
        
        result.push(CHARSET[b1] as char);
        result.push(CHARSET[b2] as char);
        result.push(if chunk.len() > 1 { CHARSET[b3] as char } else { '=' });
        result.push(if chunk.len() > 2 { CHARSET[b4] as char } else { '=' });
    }
    
    result
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, tts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
