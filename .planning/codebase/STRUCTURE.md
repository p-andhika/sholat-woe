# Codebase Structure

**Analysis Date:** 2026-03-18

## Directory Layout

```
prayer-times/
├── src/                  # Frontend source code (SvelteKit)
│   ├── lib/             # Shared utilities and stores
│   │   ├── PrayerCard.svelte  # Prayer time display component
│   │   ├── Settings.svelte    # Settings UI component
│   │   ├── indonesian-cities.ts  # Indonesian cities data
│   │   └── stores.ts    # Svelte stores for app state
│   ├── routes/          # SvelteKit routing
│   │   ├── +layout.svelte  # App layout
│   │   └── +page.svelte    # Main page component
│   ├── App.svelte       # Root application component
│   ├── app.css          # Global styles
│   ├── app.html         # HTML template
│   └── main.ts          # Application entry point
├── src-tauri/           # Tauri backend source code (Rust)
│   ├── capabilities/    # Tauri security capabilities
│   ├── gen/             # Generated Tauri code
│   ├── icons/           # Application icons
│   ├── sounds/          # Audio assets
│   ├── src/             # Rust source code
│   │   ├── lib.rs       # Main application logic and Tauri commands
│   │   ├── main.rs      # Application entry point
│   │   ├── prayer.rs    # Prayer time calculation and API handling
│   │   └── tray.rs      # System tray functionality
│   ├── target/          # Compiled Rust binaries
│   ├── Cargo.toml       # Rust dependencies and configuration
│   ├── build.rs         # Rust build script
│   └── tauri.conf.json  # Tauri configuration
├── static/              # Static assets
├── node_modules/        # Node.js dependencies
├── build/               # Compiled frontend output
├── .svelte-kit/         # SvelteKit build artifacts
├── .clauude/            # Claude-specific configurations
├── .planning/           # Planning documents
└── package.json         # JavaScript dependencies and scripts
```

## Directory Purposes

**src/:**
- Purpose: Frontend application code using SvelteKit framework
- Contains: Components, stores, routes, styles, and application logic
- Key files: `App.svelte`, `main.ts`, `lib/stores.ts`

**src-tauri/:**
- Purpose: Backend code and configuration for Tauri desktop application
- Contains: Rust code, icons, sounds, build configuration
- Key files: `Cargo.toml`, `tauri.conf.json`, `src/lib.rs`, `src/prayer.rs`

**src/lib/:**
- Purpose: Shared frontend utilities and state management
- Contains: Reusable components and store definitions
- Key files: `stores.ts`, `Settings.svelte`, `PrayerCard.svelte`

**src/routes/:**
- Purpose: SvelteKit routing and page components
- Contains: Page-level components and layout files
- Key files: `+page.svelte`, `+layout.svelte`

## Key File Locations

**Entry Points:**
- `src/main.ts`: Frontend application initialization
- `src-tauri/src/main.rs`: Backend application initialization

**Configuration:**
- `package.json`: JavaScript dependencies and scripts
- `Cargo.toml`: Rust dependencies and configuration
- `tauri.conf.json`: Tauri desktop application settings
- `svelte.config.js`: SvelteKit configuration
- `vite.config.js`: Vite build configuration

**Core Logic:**
- `src-tauri/src/prayer.rs`: Prayer time calculation and API integration
- `src-tauri/src/tray.rs`: System tray functionality
- `src/lib/stores.ts`: Frontend state management
- `src/App.svelte`: Main application logic and lifecycle management

**Testing:**
- Not present: No dedicated test files detected in current structure

## Naming Conventions

**Files:**
- Frontend components: PascalCase with .svelte extension (e.g., `PrayerCard.svelte`)
- TypeScript files: camelCase with .ts extension (e.g., `stores.ts`)
- Rust files: snake_case with .rs extension (e.g., `prayer.rs`)

**Directories:**
- Source directories: Lowercase with hyphens allowed (e.g., `src-tauri`)
- Rust modules: snake_case (e.g., `src-tauri/src/`)

## Where to Add New Code

**New Feature:**
- Primary code: `src/lib/` for shared components/utilities, `src/routes/` for new pages
- Tests: Currently no test directory structure established

**New Component/Module:**
- Implementation: `src/lib/` for reusable components, `src/routes/` for route-specific components

**Utilities:**
- Shared helpers: `src/lib/` for frontend utilities, `src-tauri/src/` for backend utilities

## Special Directories

**src-tauri/target/:**
- Purpose: Compiled Rust binaries and intermediate build artifacts
- Generated: Yes
- Committed: No (in .gitignore)

**node_modules/:**
- Purpose: Installed npm packages
- Generated: Yes
- Committed: No (in .gitignore)

**build/:**
- Purpose: Compiled frontend application output
- Generated: Yes
- Committed: No (in .gitignore)

**.svelte-kit/:**
- Purpose: SvelteKit build artifacts and generated code
- Generated: Yes
- Committed: No (in .gitignore)

---

*Structure analysis: 2026-03-18*