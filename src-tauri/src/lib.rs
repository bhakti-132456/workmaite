pub mod db;
pub mod sidecar;
pub mod commands;

use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use commands::focus_commands::FocusState;
use commands::chat_commands::LlmState;
use commands::system_commands::SystemState;
use sysinfo::System;
use std::sync::Mutex;
use std::path::PathBuf;
use crate::sidecar::stt::SttSidecar;
use crate::sidecar::tts::TtsSidecar;
use tauri_plugin_opener::OpenerExt;
use tauri::Manager;

pub struct VoiceSidecarState {
    pub stt: AsyncMutex<SttSidecar>,
    pub tts: AsyncMutex<TtsSidecar>,
    pub models_dir: PathBuf,
    pub binaries_dir: PathBuf,
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();
    
    // Register plugins
    builder = builder
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init());

    builder
        .setup(|app| {
            // Initialize Database
            let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let db = db::Database::new(app_data_dir).expect("Failed to initialize database");
            app.manage(db);

            // Initialize State
            app.manage(FocusState {
                active: Arc::new(AsyncMutex::new(false)),
            });
            app.manage(SystemState {
                sys: Mutex::new(System::new_all()),
            });

            // Initialize Sidecars and Directories
            let resource_dir = app.path().resource_dir()
                .unwrap_or_else(|_| std::env::current_dir().unwrap());
            
            // Detect dev environment structure (assets inside src-tauri vs root)
            // Robust detection: check for models in resource_dir, then in src-tauri
            let mut actual_assets_dir = resource_dir.clone();
            if !actual_assets_dir.join("models").exists() {
                if actual_assets_dir.join("src-tauri").join("models").exists() {
                   actual_assets_dir = actual_assets_dir.join("src-tauri");
                } else if actual_assets_dir.join("..").join("..").join("src-tauri").join("models").exists() {
                   // When running from target/debug
                   actual_assets_dir = actual_assets_dir.join("..").join("..").join("src-tauri");
                }
            }

            let models_dir = actual_assets_dir.join("models");
            let binaries_dir = actual_assets_dir.join("binaries");

            // Ensure all paths are absolute
            let models_dir = if models_dir.is_relative() {
                std::env::current_dir().unwrap().join(models_dir)
            } else {
                models_dir
            };
            let binaries_dir = if binaries_dir.is_relative() {
                std::env::current_dir().unwrap().join(binaries_dir)
            } else {
                binaries_dir
            };

            // --- [SYS]: Bootstrap Auxiliary Python Services ----------------
            let workspace_root = if actual_assets_dir.ends_with("src-tauri") {
                actual_assets_dir.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from("."))
            } else {
                actual_assets_dir.clone()
            };
            let workspace_root = if workspace_root.is_relative() {
                std::env::current_dir().unwrap().join(workspace_root)
            } else {
                workspace_root
            };

            let state_manager_path = workspace_root.join("core").join("state_manager.py");
            let deck_server_path = workspace_root.join("deck").join("server.py");
            
            log::info!("Starting aux services from absolute root: {:?}", workspace_root);

            // Spawn State Manager
            if state_manager_path.exists() {
                let _ = std::process::Command::new("python")
                    .arg(&state_manager_path)
                    .current_dir(workspace_root.join("core"))
                    .spawn()
                    .map_err(|e| log::error!("Failed to start python state_manager: {}", e));
            }

            // Spawn Deck Server
            if deck_server_path.exists() {
                let _ = std::process::Command::new("python")
                    .arg(&deck_server_path)
                    .current_dir(workspace_root.join("deck"))
                    .spawn()
                    .map_err(|e| log::error!("Failed to start python deck_server: {}", e));
            }

            // ------------------------------------------------------------------

            app.manage(VoiceSidecarState {
                stt: AsyncMutex::new(SttSidecar::new(models_dir.join("ggml-tiny.bin").to_str().unwrap().to_string())),
                tts: AsyncMutex::new(TtsSidecar::new(models_dir.join("hi_IN-pratham-medium.onnx").to_str().unwrap().to_string())),
                models_dir: models_dir.clone(),
                binaries_dir: binaries_dir.clone(),
            });

            // Initialize Living Loop
            let living_loop = sidecar::living_loop::LivingLoop::new();
            let loop_handle = app.handle().clone();
            let living_loop_arc = Arc::new(living_loop);
            
            // Manage state
            app.manage(living_loop_arc.clone());

            // Start the background proactive loop
            tauri::async_runtime::spawn(async move {
                living_loop_arc.run_loop(loop_handle).await;
            });

            // Open Deck URL 5s after startup
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                let _ = handle.opener().open_url("http://localhost:8080", None::<&str>);
            });

            // Initialize LLM Sidecar State
            let llm_sidecar = sidecar::llm::LlmSidecar::new(8081);
            
            let model_name = "abliterated"; // Force Uncensored as default
            let model_path = if model_name == "sarvam" {
                models_dir.join("sarvam-1.Q4_K_M.gguf")
            } else if model_name == "abliterated" {
                models_dir.join("Meta-Llama-3.1-8B-Instruct-abliterated.Q4_K_M.gguf")
            } else {
                models_dir.join("Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf")
            };

            // Ensure we have an absolute path for model
            let model_path = if model_path.is_relative() {
                std::env::current_dir().unwrap().join(model_path)
            } else {
                model_path
            };
            
            log::info!("Checking for LLM model at: {:?}", model_path);
            if model_path.exists() {
                match llm_sidecar.start(&app.handle(), model_path.to_str().unwrap(), &binaries_dir) {
                    Ok(_) => log::info!("Initial LLM Sidecar started with Llama"),
                    Err(e) => log::error!("LLM Sidecar failed to start at boot: {}", e),
                }
            } else {
                log::warn!("Llama model not found at boot. App will start but AI will need manual setup.");
            }

            app.manage(Arc::new(llm_sidecar));

            app.manage(LlmState {
                base_url: std::sync::Mutex::new("http://127.0.0.1:8081".to_string()),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // DB
            commands::db_commands::save_message,
            commands::db_commands::get_messages,
            commands::db_commands::set_context,
            commands::db_commands::get_context,
            
            // Chat & LLM
            commands::chat_commands::send_chat,
            commands::chat_commands::switch_model,
            
            // Focus
            commands::focus_commands::start_focus_session,
            commands::focus_commands::end_focus_session,
            
            // Memory
            commands::memory_commands::ingest_document,
            
            // System
            commands::system_commands::get_system_stats,

            // Voice
            commands::voice_commands::transcribe_audio,
            commands::voice_commands::speak_text,

            // Project
            commands::project_commands::analyze_project_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
