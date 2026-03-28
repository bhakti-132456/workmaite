use tauri::{State, Manager, AppHandle};
use base64::{Engine as _, engine::general_purpose};
use std::io::Write;
use crate::VoiceSidecarState;
use uuid::Uuid;

#[tauri::command]
pub async fn transcribe_audio(
    app: tauri::AppHandle,
    state: State<'_, VoiceSidecarState>,
    audio_base64: String,
) -> Result<String, String> {
    let audio_data = general_purpose::STANDARD
        .decode(audio_base64)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join(format!("{}.wav", Uuid::new_v4()));
    
    let mut file = std::fs::File::create(&temp_path).map_err(|e| e.to_string())?;
    file.write_all(&audio_data).map_err(|e| e.to_string())?;

    let stt = state.stt.lock().unwrap();
    
    // Resolve sidecar path correctly using the AppHandle
    // Note: Tauri 2 uses shell().sidecar() or just find the file in resources/binaries
    let binary_name = if cfg!(target_os = "windows") { "whisper-stt" } else { "whisper-stt" };
    
    // In dev, binaries are in src-tauri/binaries. In prod, they are in resources/binaries
    let resource_dir = app.path().resource_dir().map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "windows")]
    let suffix = "-x86_64-pc-windows-msvc.exe";
    #[cfg(target_os = "macos")]
    let suffix = "-x86_64-apple-darwin"; // Adjust for M1 if needed
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let suffix = "";

    let binary_path = resource_dir.join("binaries").join(format!("{}{}", binary_name, suffix));

    let result = stt.transcribe(binary_path.to_str().unwrap(), temp_path.to_str().unwrap());
    
    // Cleanup
    std::fs::remove_file(temp_path).ok();

    result
}

#[tauri::command]
pub async fn speak_text(
    app: tauri::AppHandle,
    state: State<'_, VoiceSidecarState>,
    text: String,
) -> Result<String, String> {
    let temp_dir = std::env::temp_dir();
    let output_path = temp_dir.join(format!("{}.wav", Uuid::new_v4()));
    
    let tts = state.tts.lock().unwrap();
    
    let binary_name = if cfg!(target_os = "windows") { "piper-tts" } else { "piper-tts" };
    let resource_dir = app.path().resource_dir().map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "windows")]
    let suffix = "-x86_64-pc-windows-msvc.exe";
    #[cfg(target_os = "macos")]
    let suffix = "-x86_64-apple-darwin";
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let suffix = "";

    let binary_path = resource_dir.join("binaries").join(format!("{}{}", binary_name, suffix));

    let result_path = tts.speak(
        binary_path.to_str().unwrap(),
        &text,
        output_path.to_str().unwrap(),
    )?;

    let audio_data = std::fs::read(&result_path).map_err(|e| e.to_string())?;
    let b64 = general_purpose::STANDARD.encode(audio_data);
    
    std::fs::remove_file(result_path).ok();
    
    Ok(b64)
}
