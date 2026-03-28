# WorkmAIte Project Progress (Strategic Reorientation 2026) 🛠🚀

## Status Overview
- **Windows Port (Legacy baseline)**: ✅ 100% STABLE (macOS transition complete)
- **Engine Pivot (Sarvam AI)**: 🚧 IN PROGRESS (Transitioning from Llama-3/Qwen to Sarvam-1 2B)
- **Voice Stack**: 🚧 IN PROGRESS (Whisper-Edge / Piper-Bulbul implementation)
- **Proactive Agency**: ✅ CORE LOGIC LIVE (`LivingLoop` background ticker active)
- **Memory Vault (RAG)**: 📅 PLANNED (Phase 2 - `sqlite-vec` integration)

## Current Phase: Strategic Reorientation

### 1. Legacy Windows Baseline (Completed Mar 2026)
- **Zero-External Core**: Stable Python-driven state and app control (`DaVinci Resolve` & `Studio One`).
- **Sidecar Isolation**: Solved `DLL Hell` via isolated binary subdirectories for `llama-server` and `whisper`.
- **Resource Management**: Implemented context pruning and automated KV cache clearing for 16GB RAM constraints.

### 2. Proactive "Living Loop" (Current Achievement)
- **Implemented**: `src-tauri/src/sidecar/living_loop.rs` background ticker.
- **Personality Prompts**: Defined system roles for **The Captain** (25m Pomodoro), **The Mirror** (2h Somatic), **The Guide** (Encouragement), and **The Scholar** (RAG/Facts).
- **Communication Flow**: Established backend-to-frontend event emitters (`proactive_check`, `loop_status_update`).

### 3. Engine & Voice Upgrades (Work In Progress)
- **LLM Replacement**: Swapping to `sarvam-1-Q4_K_M.gguf` for better localized (Indian region) performance and efficiency.
- **Voice Integration**: `stt.rs` and `tts.rs` stubs created for `Bulbul v3` and `Sarvam Edge` derivatives.

---

## Current Errors & Blockers
1. **Sqlite-vec Integration**: Loadable extension is defined but currently commented out in `src-tauri/src/db/mod.rs`.
2. **MCP Server Stub**: Rust-based Model Context Protocol server for local file interaction is still in the "Planning" state.

---

## Next Steps (Phase 1 & 2)
1. [ ] Finalize the `sarvam-1` GGUF sidecar integration in `LlmSidecar`.
2. [ ] Enable `sqlite-vec` virtual tables for 384-dim embeddings (`all-MiniLM-L6-v2`).
3. [ ] Build the somatic check-in "The Mirror" UI overlay in Svelte 5.
4. [ ] Implement "Vibe-Check" approval workflow for automatic file renaming/organization.

---
*Last Updated: 28 Mar 2026*
