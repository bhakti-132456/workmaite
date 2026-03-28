import subprocess
import os
import signal
import time
import sqlite3
import json

# Setup
LLAMA_SERVER_EXE = "src-tauri/binaries/llama-server-x86_64-pc-windows-msvc.exe"
MODEL_DIR = "src-tauri/models"
LLM_PORT = 8081

MODELS = {
    "brain": "Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf",
    "calculator": "Qwen2.5-Coder-0.5B-Instruct-Q8_0.gguf",
    "vision": "moondream2-q8.gguf"
}

MMPROJ = {
    "vision": "moondream2-mmproj.gguf"
}

def get_llama_pid():
    """Find the llama-server PID on Windows."""
    try:
        output = subprocess.check_output('tasklist /FI "IMAGENAME eq llama-server-x86_64-pc-windows-msvc.exe" /FO CSV /NH', shell=True).decode()
        if "llama-server" in output:
            parts = output.split(',')
            if len(parts) > 1:
                return int(parts[1].replace('"', ''))
    except:
        pass
    return None

def kill_llama():
    """Kill any running llama-server instances."""
    pid = get_llama_pid()
    if pid:
        print(f"Terminating llama-server (PID: {pid})...")
        try:
            os.kill(pid, signal.SIGTERM)
            time.sleep(2)
        except:
            subprocess.run(f"taskkill /F /PID {pid}", shell=True)

def start_llama(model_name):
    """Start llama-server with the specified model."""
    kill_llama()
    
    model_file = MODELS.get(model_name, MODELS["calculator"])
    model_path = os.path.join(MODEL_DIR, model_file)
    bin_dir = os.path.dirname(os.path.abspath(LLAMA_SERVER_EXE))
    dll_dir = os.path.join(bin_dir, "llama")
    
    # Environment with DLL path
    env = os.environ.copy()
    env["PATH"] = f"{dll_dir};{env.get('PATH', '')}"
    
    cmd = [
        os.path.abspath(LLAMA_SERVER_EXE),
        "--model", os.path.abspath(model_path),
        "--port", str(LLM_PORT),
        "--ctx-size", "4096",
        "--n-gpu-layers", "0",
        "--threads", "6"
    ]
    
    if model_name in MMPROJ:
        proj_path = os.path.join(MODEL_DIR, MMPROJ[model_name])
        cmd.extend(["--mmproj", os.path.abspath(proj_path)])
    
    print(f"Launching {model_name} mode...")
    # Spawn and detach
    subprocess.Popen(
        cmd,
        cwd=dll_dir,
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        creationflags=subprocess.CREATE_NEW_PROCESS_GROUP | subprocess.DETACHED_PROCESS
    )
    print(f"Model {model_name} is loading in the background.")

def snapshot_database():
    """Create a JSON snapshot of the state for fast recovery."""
    try:
        conn = sqlite3.connect("workmaite_core.db")
        conn.row_factory = sqlite3.Row
        c = conn.cursor()
        
        # Snapshot tasks
        c.execute("SELECT * FROM tasks")
        tasks = [dict(row) for row in c.fetchall()]
        
        snapshot = {
            "timestamp": time.time(),
            "tasks": tasks
        }
        
        with open("state_snapshot.json", "w") as f:
            json.dump(snapshot, f, indent=2)
        print("State snapshot saved.")
        conn.close()
    except Exception as e:
        print(f"Snapshot failed: {e}")

if __name__ == "__main__":
    # Example usage:
    # start_llama("calculator")
    pass
