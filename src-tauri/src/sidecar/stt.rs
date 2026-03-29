use std::sync::Mutex;
use tauri_plugin_shell::process::CommandChild as Child;

pub struct SttSidecar {
    process: Mutex<Option<Child>>,
    pub model_path: String,
}

impl SttSidecar {
    pub fn new(model_path: String) -> Self {
        Self {
            process: Mutex::new(None),
            model_path,
        }
    }

    /// Transcribe an audio file using Whisper sidecar
    pub async fn transcribe(&self, app: &tauri::AppHandle, binaries_dir: &std::path::Path, audio_path: &str) -> Result<String, String> {
        use tauri_plugin_shell::ShellExt;
        
        let whisper_dll_dir = binaries_dir.join("whisper");
        
        let mut sidecar = app.shell().sidecar("binaries/whisper-stt")
            .map_err(|e| format!("Failed to create whisper-stt sidecar: {}", e))?
            .args([
                "-m",
                &self.model_path,
                "-f",
                audio_path,
                "--language",
                "auto",
                "--no-timestamps",
                "--output-txt",
            ]);

        // Add DLL directories to PATH so the sidecar can find its dependencies
        #[cfg(target_os = "windows")]
        {
            let current_path = std::env::var("PATH").unwrap_or_default();
            let new_path = format!("{};{};{}", 
                binaries_dir.display(), 
                whisper_dll_dir.display(),
                current_path
            );
            sidecar = sidecar.env("PATH", new_path);
        }
        
        log::info!("STT: Running whisper-stt sidecar for audio: {}", audio_path);
        
        let output = sidecar.output()
            .await
            .map_err(|e| {
                log::error!("STT: Failed to execute: {}", e);
                format!("STT failed: {}", e)
            })?;

        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            log::info!("STT: Transcription successful: {}", text);
            Ok(text)
        } else {
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            log::error!("STT Error (stderr): {}", err);
            Err(format!("STT error: {}", err))
        }
    }
}

impl Drop for SttSidecar {
    fn drop(&mut self) {
        if let Ok(mut proc) = self.process.lock() {
            if let Some(child) = proc.take() {
                let _ = child.kill();
            }
        }
    }
}
