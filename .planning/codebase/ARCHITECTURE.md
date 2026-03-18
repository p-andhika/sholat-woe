# Architecture

**Analysis Date:** 2026-03-18

## Pattern Overview

**Overall:** Hybrid desktop application using Tauri framework with SvelteKit frontend

**Key Characteristics:**
- Desktop application with native capabilities via Tauri
- Separation of concerns between frontend (Svelte) and backend (Rust)
- Event-driven architecture with inter-process communication via Tauri commands
- Tray-based application design for constant accessibility

## Layers

**Frontend (Svelte Layer):**
- Purpose: User interface rendering and interaction
- Location: `src/`
- Contains: Components, stores, CSS, routing logic
- Depends on: Tauri API, SvelteKit runtime
- Used by: End users for prayer time visualization and settings

**Tauri Bridge Layer:**
- Purpose: Communication between frontend and backend
- Location: Tauri API commands in `src/App.svelte`
- Contains: Invoke methods to call Rust backend functions
- Depends on: Tauri API, plugin-notification
- Used by: Frontend to access system-level features

**Backend (Rust Layer):**
- Purpose: Core business logic, system integration, and external API communication
- Location: `src-tauri/src/`
- Contains: Prayer calculation logic, configuration management, tray functionality
- Depends on: External APIs (aladhan.com), system libraries (notification, file system)
- Used by: Frontend via Tauri commands

## Data Flow

**Prayer Time Fetching:**

1. User opens app or refreshes prayer times
2. Frontend calls `invoke("get_prayer_times")` via Tauri API
3. Rust backend executes `get_prayer_times()` command handler
4. Backend calls external Aladhan API to fetch prayer times
5. Rust processes and formats the response
6. Response sent back to frontend via Tauri IPC
7. Svelte store updates and UI re-renders

**Configuration Management:**

1. Frontend initializes by calling `invoke("get_config")`
2. Rust loads config from persistent storage (`app_data_dir/config.json`)
3. Config returned to frontend
4. When settings change, frontend calls `invoke("update_config")`
5. Rust saves config to persistent storage and applies changes

**State Management:**
- Frontend: Svelte reactive state and stores in `src/lib/stores.ts`
- Backend: Rust structs managed by Tauri's state system in `AppState`

## Key Abstractions

**Tauri Commands:**
- Purpose: Secure communication channel between frontend and backend
- Examples: `get_prayer_times`, `update_config`, `get_config`, `quit_app`, `set_tray_title`
- Pattern: Async command pattern with TypeScript interfaces for type safety

**Persistent Configuration:**
- Purpose: Store user preferences across application restarts
- Examples: `config_path()`, `load_config()`, `save_config()` in `src-tauri/src/lib.rs`
- Pattern: JSON file stored in platform-specific app data directory

**Tray Window System:**
- Purpose: Provides accessible prayer times through system tray
- Examples: `create_tray()` in `src-tauri/src/tray.rs`
- Pattern: Hidden webview window that appears near tray icon on click

## Entry Points

**Frontend Entry Point:**
- Location: `src/main.ts`
- Triggers: Application startup in browser/webview environment
- Responsibilities: Mount Svelte app to DOM, apply global styles

**Backend Entry Point:**
- Location: `src-tauri/src/main.rs` (main function)
- Triggers: Application launch
- Responsibilities: Initialize Tauri application, set up state, create tray

## Error Handling

**Strategy:** Comprehensive error handling with user feedback at all layers

**Patterns:**
- Frontend: Try/catch blocks with Svelte stores to propagate error states
- Backend: Rust Result types with string conversion for frontend consumption
- UI: Error messages displayed in the interface with retry capability

## Cross-Cutting Concerns

**Logging:** Console logging via browser dev tools and Rust println macros
**Validation:** Performed at API boundaries with Rust type system and external API validation
**Authentication:** Not applicable - application is locally focused without user accounts

---

*Architecture analysis: 2026-03-18*