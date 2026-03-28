pub mod db;
pub mod sidecar;
pub mod commands;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex as AsyncMutex;
use commands::focus_commands::FocusState;
use commands::chat_commands::LlmState;
use commands::system_commands::SystemState;
use tauri_plugin_shell::ShellExt;
use sysinfo::System;
use std::sync::Mutex;
use std::path::PathBuf;
use crate::sidecar::stt::SttSidecar;
use crate::sidecar::tts::TtsSidecar;

pub struct VoiceSidecarState {
    pub stt: Mutex<SttSidecar>,
    pub tts: Mutex<TtsSidecar>,
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
        .plugin(tauri_plugin_shell::init());

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
            let models_dir = resource_dir.join("models");
            let binaries_dir = resource_dir.join("binaries");

            app.manage(VoiceSidecarState {
                stt: Mutex::new(SttSidecar::new(models_dir.join("ggml-tiny.bin").to_str().unwrap().to_string())),
                tts: Mutex::new(TtsSidecar::new(models_dir.join("hi_IN-pratham-medium.onnx").to_str().unwrap().to_string())),
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

            // Start LLM Sidecar (llama-server)
            let models_dir = app.path().resource_dir()
                .unwrap_or_else(|_| std::env::current_dir().unwrap())
                .join("models");
            let model_path = models_dir.join("Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf");
            
            log::info!("Model path resolved to: {:?}", model_path);
            if !model_path.exists() {
                log::error!("CRITICAL: Model file NOT FOUND at {:?}", model_path);
            }

            match app.shell().sidecar("llama-server") {
                Ok(s) => {
                    let binaries_dir = app.path().resource_dir()
                        .unwrap_or_else(|_| std::env::current_dir().unwrap())
                        .join("binaries").join("llama");
                    
                    log::info!("Binaries (DYLD/CWD) path: {:?}", binaries_dir);
                    
                    #[cfg(target_os = "macos")]
                    let env_key = "DYLD_LIBRARY_PATH";
                    #[cfg(not(target_os = "macos"))]
                    let env_key = "LD_LIBRARY_PATH";

                    let mut sidecar_cmd = s.args([
                        "--model", model_path.to_str().unwrap(),
                        "--port", "8081",
                        "--ctx-size", "4096",
                        "--no-mmap",
                        "--n-gpu-layers", "0",
                        "--threads", "6",
                    ])
                    .current_dir(binaries_dir.clone());

                    #[cfg(target_os = "windows")]
                    {
                        let current_path = std::env::var("PATH").unwrap_or_default();
                        let new_path = format!("{};{}", binaries_dir.to_str().unwrap(), current_path);
                        sidecar_cmd = sidecar_cmd.env("PATH", new_path);
                    }
                    #[cfg(not(target_os = "windows"))]
                    {
                        sidecar_cmd = sidecar_cmd.env(env_key, binaries_dir.to_str().unwrap());
                    }

                    let sidecar_handle = sidecar_cmd.spawn();

                    match sidecar_handle {
                        Ok((mut rx, _child)) => {
                            log::info!("LLM Sidecar spawned successfully");
                            
                            // Pipe sidecar output to terminal for debugging
                            tauri::async_runtime::spawn(async move {
                                while let Some(event) = rx.recv().await {
                                    if let tauri_plugin_shell::process::CommandEvent::Stdout(line) = event {
                                        log::info!("LLM STDOUT: {}", String::from_utf8_lossy(&line));
                                    } else if let tauri_plugin_shell::process::CommandEvent::Stderr(line) = event {
                                        log::error!("LLM STDERR: {}", String::from_utf8_lossy(&line));
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            log::error!("Failed to spawn sidecar: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to create sidecar: {}", e);
                }
            }


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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
