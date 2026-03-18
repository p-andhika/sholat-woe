# Technology Stack

**Analysis Date:** 2026-03-18

## Languages

**Primary:**
- TypeScript ~5.6.2 - Frontend application logic
- Rust 2021 - Backend/core functionality

**Secondary:**
- Svelte 5 - Component framework
- JavaScript ES2022 - Scripting support

## Runtime

**Environment:**
- Node.js (varies) - Development and build environment

**Package Manager:**
- npm - Package management
- Lockfile: package-lock.json present

## Frameworks

**Core:**
- SvelteKit 2.9.0 - Application framework
- Tauri 2 - Desktop application framework

**Testing:**
- Not implemented - No testing frameworks detected

**Build/Dev:**
- Vite 6.0.3 - Build tool and dev server
- TypeScript 5.6.2 - Type checking

## Key Dependencies

**Critical:**
- @tauri-apps/api 2.10.1 - Tauri core functionality
- @tauri-apps/cli 2 - Tauri command line tools
- @sveltejs/kit 2.9.0 - SvelteKit framework
- svelte 5.0.0 - Svelte component framework

**Infrastructure:**
- @sveltejs/adapter-static 3.0.6 - Static site generation
- @sveltejs/vite-plugin-svelte 5.0.0 - Svelte/Vite integration
- phosphor-svelte 3.1.0 - Icon library

## Configuration

**Environment:**
- Configured through Tauri configuration
- User preferences stored in app data directory as config.json

**Build:**
- Vite configuration: vite.config.js
- Svelte configuration: svelte.config.js
- Tauri configuration: src-tauri/tauri.conf.json
- TypeScript configuration: tsconfig.json

## Platform Requirements

**Development:**
- Node.js v18+
- Rust toolchain
- Cargo package manager
- Tauri CLI

**Production:**
- Desktop applications targeting Windows, macOS, and Linux
- Built with Tauri for native desktop distribution

---

*Stack analysis: 2026-03-18*