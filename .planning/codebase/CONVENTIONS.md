# Coding Conventions

**Analysis Date:** 2026-03-18

## Naming Patterns

**Files:**
- Svelte components use PascalCase: `PrayerCard.svelte`, `Settings.svelte`
- TypeScript modules use camelCase: `stores.ts`, `indonesian-cities.ts`
- Route files use SvelteKit convention: `+layout.svelte`, `+page.svelte`

**Functions:**
- Use camelCase: `loadPrayerTimes()`, `formatTime()`, `checkAndNotify()`, `isTimePassed()`
- Async functions typically have descriptive names indicating their purpose

**Variables:**
- Component props use camelCase: `{ prayer }`
- Local variables use camelCase: `now`, `localConfig`, `filteredCities`
- State variables often prefixed with `$`: `$config`, `$prayerData` (when accessing Svelte stores)
- Store references are typically camelCase: `prayerData`, `config`, `loading`, `error`

**Types:**
- Interfaces use PascalCase: `PrayerTime`, `PrayerTimesResult`, `PrayerConfig`
- Type properties use camelCase: `name`, `time`, `passed`, `is_next`

## Code Style

**Formatting:**
- Use 2-space indentation throughout the codebase
- Semicolons are used consistently
- Double quotes for strings in Svelte templates, single quotes in TypeScript
- Trailing commas are used in object/array literals
- Arrow function spacing follows: `(params) => { }`

**Linting:**
- Based on TypeScript compiler options in `tsconfig.json` with strict: true
- Includes checks for JavaScript files (allowJs: true, checkJs: true)
- Uses Svelte's built-in type checking capabilities

## Import Organization

**Order:**
1. Built-in libraries: `svelte`, `svelte/store`, `@tauri-apps/api`
2. Third-party packages: `phosphor-svelte`
3. Relative imports: `./lib/stores`, `../app.css`

**Path Aliases:**
- Standard relative imports are used (`./` and `../`)
- No special path aliases detected in the codebase

## Error Handling

**Patterns:**
- Try-catch blocks with explicit error handling: `try { ... } catch (e: any) { $error = e.toString(); }`
- Errors are stored in Svelte stores for UI display
- Silent catches use `_` as error parameter: `catch (_) {}`
- API calls from Tauri backend wrapped in try-catch blocks
- User-facing error messages are displayed through Svelte store bindings

## Logging

**Framework:** console logging not extensively used; relies on Svelte store-based error display

**Patterns:**
- Minimal logging in favor of UI feedback via stores
- Error states are captured in Svelte stores and displayed to users

## Comments

**When to Comment:**
- Section dividers using ASCII-style comments: `// ─── Fetch prayer times from Rust backend ──────────────────`
- Brief explanatory comments for non-obvious logic
- File headers documenting purpose (in settings file)

**JSDoc/TSDoc:**
- Not extensively used in the current codebase
- Basic JSDoc used for configuration: `/** @type {import('@sveltejs/kit').Config} */`

## Function Design

**Size:** Functions are kept reasonably small, typically 10-30 lines
**Parameters:**
- Svelte components use destructured props: `let { prayer }: { prayer: PrayerTime } = $props();`
- Event handlers follow standard patterns with typed event parameters
**Return Values:** Most functions either return Promise for async operations or void for side effects

## Module Design

**Exports:**
- Named exports for constants and interfaces
- Default exports for Svelte components
- Constants and interfaces exported from utility modules like `stores.ts`

**Barrel Files:**
- Not implemented in this codebase; each module has its own file
- Components and utilities are imported directly by file path

## Svelte-Specific Conventions

**Reactivity:**
- Uses Svelte 5 `$state` and `$derived` for reactive variables
- Svelte stores accessed with `$` prefix: `$config`, `$prayerData`
- Component props defined with `$props()` in Svelte 5

**Event Handling:**
- Inline event handlers for simple actions: `onclick={() => ($showSettings = true)}`
- Named functions for complex logic: `onMount`, lifecycle callbacks

**Conditional Rendering:**
- Uses Svelte syntax: `{#if}`, `{:else if}`, `{:else}`
- Each blocks for lists: `{#each $prayerData.prayers as prayer (prayer.name)}`

---

*Convention analysis: 2026-03-18*