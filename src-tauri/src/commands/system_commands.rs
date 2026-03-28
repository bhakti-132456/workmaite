use serde::{Serialize, Deserialize};
use sysinfo::{System};
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize, Deserialize, Default)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub mem_usage: f32,
    pub gpu_usage: Option<f32>,
    pub cpu_temp: Option<f32>,
    pub gpu_temp: Option<f32>,
}

pub struct SystemState {
    pub sys: Mutex<System>,
}

#[tauri::command]
pub async fn get_system_stats(
    state: State<'_, SystemState>,
) -> Result<SystemStats, String> {
    let mut sys = state.sys.lock().unwrap();
    
    // Refresh only what's needed
    sys.refresh_cpu();
    sys.refresh_memory();
    
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let total_mem = sys.total_memory() as f32;
    let used_mem = sys.used_memory() as f32;
    let mem_usage = (used_mem / total_mem) * 100.0;
    
    // Try to get GPU usage via ioreg on Mac Intel/Radeon
    let gpu_usage = get_mac_gpu_usage().ok();
    
    // Try to get CPU Temp (SMC) via ioreg
    let cpu_temp = get_mac_cpu_temp().ok();

    Ok(SystemStats {
        cpu_usage,
        mem_usage,
        gpu_usage,
        cpu_temp,
        gpu_temp: None, // Harder to get for specific GPU without SMCHK
    })
}

/// Hacky way to get GPU usage on Mac via ioreg
fn get_mac_gpu_usage() -> Result<f32, String> {
    use std::process::Command;
    let output = Command::new("ioreg")
        .args(["-l", "-c", "IOAccelerator"])
        .output()
        .map_err(|e| e.to_string())?;
        
    let out_str = String::from_utf8_lossy(&output.stdout);
    
    // Check various common keys for GPU load on Intel/Radeon
    for key in ["\"Device Utilization %\"=", "\"GPU Activity(%)\"="] {
        if let Some(idx) = out_str.find(key) {
            let sub = &out_str[idx + key.len()..];
            if let Some(end) = sub.find(|c: char| !c.is_numeric() && c != '.') {
                if let Ok(val) = sub[..end].parse::<f32>() {
                    return Ok(val);
                }
            }
        }
    }
    
    Err("Not found".to_string())
}

/// Get GPU/System temp via ioreg on Intel Mac
fn get_mac_cpu_temp() -> Result<f32, String> {
    use std::process::Command;
    let output = Command::new("ioreg")
        .args(["-l", "-c", "IOAccelerator"])
        .output()
        .map_err(|e| e.to_string())?;
        
    let out_str = String::from_utf8_lossy(&output.stdout);
    
    if let Some(idx) = out_str.find("\"Temperature(C)\"=") {
        let sub = &out_str[idx + 17..];
        if let Some(end) = sub.find(|c: char| !c.is_numeric() && c != '.') {
            if let Ok(val) = sub[..end].parse::<f32>() {
                return Ok(val);
            }
        }
    }
    
    Err("Not found".to_string())
}
