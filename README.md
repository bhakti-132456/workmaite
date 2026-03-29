# WorkmAIte Pro: Local Agentic AI for Creative Workflows

**WorkmAIte Pro** is a 100% offline, privacy-focused productivity companion built for creatives. It integrates deeply with DaVinci Resolve and Studio One to provide AI-driven task management, pc control, and creative automation.

## 🚀 Strategic Reorientation (2026)
We are currently pivoting toward a "Living Loop" architecture using the **Sarvam AI Ecosystem**:
- **Sarvam-1 (LLM)**: For localized, high-performance thinking.
- **Bulbul v3 (TTS)**: High-quality localized speech synthesis.
- **Sarvam Edge (STT)**: Efficient background voice listening.
- **Proactive Agency**: The app now features "The Captain" (productivity checks) and "The Mirror" (somatic/wellness checks) via an automated background ticker.

## 🛠 Features
- **The Deck**: A kanban-style Command Center for task decomposition.
- **The Brainstem**: Local SQLite state management with vector-search RAG stubs.
- **Zero-External Connectivity**: All models (Llama, Qwen, Sarvam) run strictly on local hardware.
- **App Control**: Automated launching and monitoring for professional creative suites.

## 📦 Tech Stack
- **Frontend**: Svelte 5 / Vite
- **Desktop Wrapper**: Tauri V2 (Rust)
- **AI Core**: llama-server / whisper-stt / piper-tts sidecars
- **Database**: SQLite (via rusqlite)

## 🏗 Setup & Installation
1.  Initialize the project: `npm install`
2.  Launch dev mode: `npm run tauri dev`
3.  Binary isolated models go in `src-tauri/models/`.

---
*Created by the Advanced Agentic Coding team at DeepMind.*

## 🤖 AI Agent Instructions

If you are an AI coding assistant working on this repository, please note the following architectural requirements:

### **1. AI Sidecar Architecture (Tauri v2)**
- **LLM Engine**: Running on `llama-server` (port **8081**). The identifier is `binaries/llama-server`.
- **STT Engine**: Running on `whisper-stt`. The identifier is `binaries/whisper-stt`.
- **TTS Engine**: Running on `piper-tts`. Matches `src-tauri/binaries` directly.
- **Model Path requirement**: Sidecars MUST be spawned using **absolute paths** to their model files (located in `src-tauri/models/`) to ensure stability across different launch environments.

### **2. Directory Structure**
- `src-tauri/binaries/`: Essential binary sidecars and their respective DLL dependencies.
- `src-tauri/models/`: GGUF/ONNX model files (Llama-3.1 8B, Sarvam, etc.).
- `core/` & `deck/`: Auxiliary Python services. The `workspace_root` must be correctly resolved in `lib.rs` to find these scripts.

### **3. Debugging Spawning Errors**
- If the AI Chat shows a "Connection Error", check if port **8081** is listening (`netstat -ano | findstr :8081`).
- Sidecar logs are piped to the main Tauri console with the prefix `LLM_SIDECAR:`. Check these for model-loading failures or RAM issues.
- Do NOT remove the `binaries/` prefix from sidecar identifiers in `tauri.conf.json`, as this is required for Tauri 2.0 sidecar resolution.

