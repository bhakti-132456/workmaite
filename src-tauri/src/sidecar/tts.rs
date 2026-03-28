use std::process::Command;

pub struct TtsSidecar {
    pub model_path: String,
}

impl TtsSidecar {
    pub fn new(model_path: String) -> Self {
        Self { model_path }
    }

    /// Synthesize speech from text, returns path to generated WAV
    pub fn speak(
        &self,
        binary_path: &str,
        text: &str,
        output_path: &str,
    ) -> Result<String, String> {
        let binary_dir = std::path::Path::new(binary_path).parent().unwrap();
        let output = Command::new(binary_path)
            .current_dir(binary_dir)
            .args([
                "--model",
                &self.model_path,
                "--output_file",
                output_path,
            ])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(ref mut stdin) = child.stdin {
                    stdin.write_all(text.as_bytes()).ok();
                }
                child.wait_with_output()
            })
            .map_err(|e| format!("TTS failed: {}", e))?;

        if output.status.success() {
            Ok(output_path.to_string())
        } else {
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            Err(format!("TTS error: {}", err))
        }
    }
}
