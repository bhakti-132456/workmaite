# WorkmAIte Pro (Windows Build) 🛠🚀

WorkmAIte is a "Zero-External" AI OS layer for your computer, designed to handle tasks, creative app control, and system automation on your local hardware. No cloud, 100% privacy.

## 🛠 Prerequisites

To get this project functional after cloning, your machine must have the following installed:

### ⚙️ Software Core
1. **Node.js**: (Version 20+) [Download](https://nodejs.org/)
2. **Rust**: (Version 1.75+) [Install via Rustup](https://rustup.rs/)
3. **Python**: (Version 3.10+) [Download](https://python.org/)
4. **Visual Studio 2022**: Ensure you check the "Desktop development with C++" workload during installation.

---

## 🏗 Setup: External Dependencies

Since models and sidecar binaries are over 10GB, they are excluded from Git. You must populate the following folders manually:

### 1. Model Files (`src-tauri/models/`)
Place these `.gguf` files in the directory:
- **Main Brain**: `Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf`
- **Calculator Mode**: `Qwen2.5-Coder-0.5B-Instruct-Q8_0.gguf`
- **Vision Scan**: `moondream2-q8.gguf` & `moondream2-mmproj.gguf`

### 2. Sidecar Binaries (`src-tauri/binaries/`)
The project expects the following Windows executables organized into isolated folders to prevent `DLL Hell` (ABI conflicts):

#### **llama-server** (`/binaries/llama/`)
- Binary: `llama-server-x86_64-pc-windows-msvc.exe`
- Dependencies: `ggml.dll`, `llama.dll` (standard Windows build files from llama.cpp)

#### **whisper-stt** (`/binaries/whisper/`)
- Binary: `whisper-stt-x86_64-pc-windows-msvc.exe`
- Dependencies: Ensure the Whisper-specific `ggml.dll` is in this separate folder.

---

## 🚀 Launching the System

Double-click the following files from the root directory:

- **Option A (Visible)**: `WorkmAIte_PRO_Launcher.bat`
- **Option B (Silent)**: `WorkmAIte_SILENT.vbs`

**Manual Start Commands:**
```powershell
# 1. Start the main AI Engine
npm run tauri dev

# 2. Start the State-to-Task sync
python core/state_manager.py

# 3. Start the Command Center UI
python deck/server.py
```

## 🔋 Hardware Targets (Optimization)
- **RAM**: 16GB Minimum (Model allocations ~5-6GB).
- **VRAM**: 4GB Minimum (RTX Ti/AMD Mobile compatible).
- **Backend**: Currently using CPU (OpenBLAS/AVX2). GPU acceleration can be unlocked by updating the `--n-gpu-layers` flag in `src-tauri/src/lib.rs`.

---
*Powered by Local AI & Zero-External Architecture.*
