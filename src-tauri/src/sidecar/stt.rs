use std::process::{Child, Command};
use std::sync::Mutex;

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
    pub fn transcribe(&self, binary_path: &str, audio_path: &str) -> Result<String, String> {
        let binary_dir = std::path::Path::new(binary_path).parent().unwrap();
        log::info!("STT: Running command: {} with audio: {}", binary_path, audio_path);
        
        let output = Command::new(binary_path)
            .current_dir(binary_dir)
            .args([
                "-m",
                &self.model_path,
                "-f",
                audio_path,
                "--language",
                "auto",
                "--no-timestamps",
                "--output-txt",
            ])
            .output()
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
            if let Some(ref mut child) = *proc {
                child.kill().ok();
            }
            *proc = None;
        }
    }
}
