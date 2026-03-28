use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use tauri::Emitter;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoopState {
    Idle,
    Listening,
    Transcribing,
    Retrieving,
    Thinking,
    Speaking,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopStatus {
    pub state: LoopState,
    pub personality: String,
    pub last_user_input: Option<String>,
    pub last_response: Option<String>,
    pub focus_active: bool,
    pub focus_minutes_remaining: Option<u32>,
    pub last_captain_check: DateTime<Utc>,
    pub last_mirror_check: DateTime<Utc>,
}

pub struct LivingLoop {
    pub status: Arc<AsyncMutex<LoopStatus>>,
}

impl LivingLoop {
    pub fn new() -> Self {
        Self {
            status: Arc::new(AsyncMutex::new(LoopStatus {
                state: LoopState::Idle,
                personality: "guide".to_string(),
                last_user_input: None,
                last_response: None,
                focus_active: false,
                focus_minutes_remaining: None,
                last_captain_check: Utc::now(),
                last_mirror_check: Utc::now(),
            })),
        }
    }

    /// The proactive background ticker
    pub async fn run_loop(&self, app_handle: tauri::AppHandle) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        
        loop {
            interval.tick().await;
            let mut status = self.status.lock().await;
            let now = Utc::now();

            // 1. Captain Check (Every 25 minutes)
            if status.personality == "captain" && now - status.last_captain_check > Duration::minutes(25) {
                log::info!("Proactive trigger: THE CAPTAIN (Pomodoro check)");
                app_handle.emit("proactive_check", "captain_sprint_check").ok();
                status.last_captain_check = now;
            }

            // 2. Mirror Check (Every 2 hours)
            if now - status.last_mirror_check > Duration::hours(2) {
                log::info!("Proactive trigger: THE MIRROR (Somatic check)");
                app_handle.emit("proactive_check", "mirror_somatic_check").ok();
                status.last_mirror_check = now;
            }
            
            // Sync status to frontend
            app_handle.emit("loop_status_update", &*status).ok();
        }
    }
}
