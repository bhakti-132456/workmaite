use reqwest::Client;
use serde_json::json;
use std::sync::Mutex;
use tauri::State;

// Simple struct to hold the base URL of the running LLM
pub struct LlmState {
    pub base_url: Mutex<String>,
}

#[tauri::command]
pub async fn send_chat(
    llm: State<'_, LlmState>,
    prompt: String,
    personality: String,
) -> Result<String, String> {
    let base_url = llm.base_url.lock().unwrap().clone();
    
    // Get the system prompt based on personality
    let system_prompt = crate::sidecar::llm::get_personality_prompt(&personality.to_lowercase());

    let client = Client::new();
    let req_body = json!({
        "messages": [
            {"role": "system", "content": "You are THE CAPTAIN. You are a professional English productivity assistant. Be direct and concise."},
            {"role": "user", "content": prompt}
        ],
        "temperature": 0.4,
        "max_tokens": 1024,
        "stream": false
    });

    let url = format!("{}/v1/chat/completions", base_url);
    log::info!("DEBUG: Sending request to Chat API: {}", url);
    
    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&req_body)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if res.status().is_success() {
        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        
        // Extract content from OpenAI-style response: choices[0].message.content
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| format!("Invalid response format: {}", json))?;
            
        log::info!("DEBUG: AI Response: {}", content);
        Ok(content.to_string())
    } else {
        let status = res.status();
        let err_text = res.text().await.unwrap_or_default();
        Err(format!("LLM Error ({}): {}", status, err_text))
    }
}
