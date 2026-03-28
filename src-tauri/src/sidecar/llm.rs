use serde::{Deserialize, Serialize};
use std::process::{Child, Command};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompletionRequest {
    pub prompt: String,
    pub n_predict: i32,
    pub temperature: f32,
    pub stop: Vec<String>,
    pub stream: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompletionResponse {
    pub content: String,
    pub stop: bool,
}

pub struct LlmSidecar {
    process: Mutex<Option<Child>>,
    pub port: u16,
    pub model_path: String,
}

impl LlmSidecar {
    pub fn new(model_path: String, port: u16) -> Self {
        Self {
            process: Mutex::new(None),
            port,
            model_path,
        }
    }

    /// Start the llama-server sidecar process using Tauri's shell plugin
    pub fn start(&self, app: &tauri::AppHandle) -> Result<(), String> {
        let mut proc = self.process.lock().map_err(|e| e.to_string())?;

        if proc.is_some() {
            return Ok(()); // Already running
        }

        use tauri_plugin_shell::ShellExt;

        let sidecar = app.shell().sidecar("llama-server")
            .map_err(|e| format!("Failed to create llama-server sidecar: {}", e))?
            .args([
                "--model",
                &self.model_path,
                "--ctx-size",
                "2048",
                "--port",
                &self.port.to_string(),
                "--threads",
                "4",
                "--n-gpu-layers",
                "0",
                "--log-disable",
            ]);

        let (mut _rx, _child) = sidecar.spawn()
            .map_err(|e| format!("Failed to spawn LLM sidecar: {}", e))?;

        // We don't store the child in the Mutex here because we want to use Tauri's handle 
        // but for now we'll store a dummy to mark it as started. 
        // In a real implementation we would monitor the stdout for 'ready'.
        
        // *proc = Some(child); // In Tauri 2 sidecar.spawn returns a SidecarChild
        
        Ok(())
    }

    /// Stop the sidecar process
    pub fn stop(&self) -> Result<(), String> {
        let mut proc = self.process.lock().map_err(|e| e.to_string())?;
        if let Some(ref mut child) = *proc {
            child.kill().map_err(|e| e.to_string())?;
            child.wait().map_err(|e| e.to_string())?;
            log::info!("LLM sidecar stopped");
        }
        *proc = None;
        Ok(())
    }

    /// Check if the sidecar is running
    pub fn is_running(&self) -> bool {
        let proc = self.process.lock().unwrap();
        proc.is_some()
    }

    /// Get the base URL for the llama-server API
    pub fn base_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }
}

impl Drop for LlmSidecar {
    fn drop(&mut self) {
        self.stop().ok();
    }
}

/// Personality system prompts
pub fn get_personality_prompt(personality: &str) -> &'static str {
    match personality {
        "captain" => "You are THE CAPTAIN. You are a professional English productivity assistant. Be brief and direct.",
        "guide" => "You are THE GUIDE. You are a helpful English mentor. Be encouraging and concise.",
        "scholar" => "You are THE SCHOLAR. You are a precise English researcher. Give facts only.",
        "mirror" => "You are THE MIRROR. You are a wellness companion. Speak kindly in English.",
        _ => "You are a helpful English AI assistant."
    }
}

/// Build a full prompt with personality, context, and user message
pub fn build_prompt(personality: &str, context: &str, user_message: &str) -> String {
    let system_desc = get_personality_prompt(personality);

    let mut prompt = String::from("<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n");
    prompt.push_str(system_desc);
    prompt.push_str("<|eot_id|>");

    if !context.is_empty() {
        prompt.push_str("<|start_header_id|>context<|end_header_id|>\n\n");
        prompt.push_str(context);
        prompt.push_str("<|eot_id|>");
    }

    prompt.push_str("<|start_header_id|>user<|end_header_id|>\n\n");
    prompt.push_str(user_message);
    prompt.push_str("<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n");
    
    prompt
}
