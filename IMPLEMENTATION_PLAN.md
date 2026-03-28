# Project Implementation Plan: WorkmAIte Pro (Strategic Reorientation)

This document outlines the engineering steps to align **workmAlte Pro** with the *Comprehensive Strategic Reorientation Plan* (2026).

## 🎯 Current Objectives
1.  **Shift to Sarvam AI Ecosystem**: Integrate Sarvam-1 2B (LLM), Sarvam Edge (STT), and Bulbul v3 (TTS).
2.  **Personality-Driven Proactive Loops**:
    *   **THE CAPTAIN**: 25-minute Pomodoro/Vocal Check-ins.
    *   **THE MIRROR**: 2-hour Somatic/Wellness Check-ins.
3.  **Local Memory Vault (RAG)**: Implement vector search using `sqlite-vec` and `all-MiniLM-L6-v2`.
4.  **Hardware Optimization**: Maintain <10% background CPU usage and 8GB RAM compatibility via lazy loading.
5.  **MCP Integration**: Model Context Protocol for local file interactions.

---

## 🛠 Phase 1: Core Engine Upgrades (Local AI)
- [ ] **LLM Replacement**: Swapping `sarvam-1-Q4_K_M.gguf` as the primary engine in `LlmSidecar`.
- [ ] **Voice Integration**:
    - [ ] Deploy `whisper-stt` (Sarvam Edge derivative) for background listening.
    - [ ] Deploy `piper-tts` (Bulbul v3 derivative) for proactive vocal feedback.
- [ ] **Protocol Support**:
    - [ ] Implement an MCP server stub in Rust to allow the LLM to read/write files in the `Memory Vault`.

## 🧠 Phase 2: Memory & Vector DB
- [ ] **Sqlite-vec Integration**:
    - [ ] Add `sqlite-vec` loadable extension to the Tauri bundles.
    - [ ] Update `src-tauri/src/db/mod.rs` with `VIRTUAL TABLE` using `vec0` (512-dim for MiniLM).
- [ ] **Embedding Service**:
    - [ ] Add a sidecar or crate for `all-MiniLM-L6-v2` to generate embeddings from text dumps.

## 🕒 Phase 3: Proactive "Living Loop"
- [ ] **Scheduler System**:
    - [ ] Implement a background ticker in `src-tauri/src/sidecar/living_loop.rs`.
    - [ ] **The Captain (25m)**: Triggers a foreground notification/voice prompt: *"Pomodoro check — are we still on task?"*
    - [ ] **The Mirror (120m)**: Triggers a somatic check: *"Shoulders down. Breath check. Water?"*
- [ ] **Task Decomposition Engine**: Enhance `The Guide`'s prompt to automatically break `user_input` into 5-minute sub-tasks.

## 🎨 Phase 4: UI Refinement (Svelte 5)
- [ ] **Interval Configuration**: Add settings for check-in frequency (defaulting to 25m/2h).
- [ ] **Somatic Prompts**: Specific UI overlays for `The Mirror` check-ins.
- [ ] **RAG Visibility**: Show "Memory Retrieved" indicators when the `Scholar` pulls from the vault.

---

## 📈 Usage Limits & Hardware Guardrails
- **Background CPU < 10%**: Achieved by offloading voice activity detection (VAD) to low-power cycles.
- **8GB RAM Constraint**:
    - MODELS are unloaded if inactive for > 15 minutes.
    - TTS/STT binaries are only spun up when needed.
- **Data Sovereignty**: 100% Offline; no telemetry.

---

## 🚀 Immediate Next Steps (Work In Progress)
1.  **Refine Personality Prompts**: Match exactly the "strategy" document descriptions.
2.  **Update Database Schema**: Prepare for vector search.
3.  **Implement Living Ticker**: The proactive background loop.
