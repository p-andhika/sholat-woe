# Codebase Concerns

**Analysis Date:** 2026-03-18

## Tech Debt

**Duplicate prayer time logic:**
- Files: `src/App.svelte` and `src/routes/+page.svelte`
- Issue: Prayer time display and notification logic is duplicated in both files. The same core functionality exists in both locations with slight variations
- Impact: Maintenance becomes difficult as changes need to be made in both places, leading to potential inconsistencies
- Fix approach: Consolidate prayer time handling into a shared service or component, and remove the duplicate implementation

**Hardcoded API endpoint:**
- Files: `src-tauri/src/prayer.rs` (line 151)
- Issue: The API URL is hardcoded and doesn't support environment-specific configurations
- Impact: Limited ability to switch between development/production APIs or alternative prayer time providers
- Fix approach: Make API endpoint configurable through environment variables or configuration settings

## Known Bugs

**Time calculation edge case:**
- Files: `src-tauri/src/prayer.rs` (lines 226-235)
- Issue: When no next prayer is found (after all daily prayers are passed), the app defaults to showing Fajr from the next day but displays "tomorrow" as the remaining time, which is confusing
- Impact: Users may be confused by the time remaining display
- Trigger: Using the app late at night after all daily prayers
- Workaround: None available to users

**Memory leaks in interval timers:**
- Files: `src/App.svelte` (lines 94-107) and `src/routes/+page.svelte` (lines 144-155)
- Issue: Multiple intervals are created in onMount but not properly cleared if component unmounts unexpectedly
- Impact: Potential memory leaks and multiple callback executions in SPA environments
- Fix approach: Ensure all intervals are properly cleaned up and add defensive checks

## Security Considerations

**Missing Content Security Policy:**
- Files: `src-tauri/tauri.conf.json` (line 15)
- Issue: CSP is set to null, providing no protection against XSS attacks
- Impact: Increased vulnerability to XSS if any content injection vulnerabilities exist
- Current mitigation: None implemented
- Recommendations: Implement strict CSP rules limiting allowed sources for scripts, styles, and other content

**API key exposure risk:**
- Files: `src-tauri/src/prayer.rs` (line 151)
- Issue: External API calls to aladhan.com are made without authentication, relying on rate limits only
- Risk: Potential service disruption if the API provider changes rate limits or requires authentication
- Recommendations: Consider caching strategies and fallback mechanisms

## Performance Bottlenecks

**Aggressive polling intervals:**
- Files: `src/App.svelte` (line 99) and `src/routes/+page.svelte` (line 148)
- Problem: Notification checks occur every 30 seconds (30,000ms) which is excessive for prayer time notifications
- Files: `src/App.svelte` (line 102) and `src/routes/+page.svelte` (line 149)
- Problem: Prayer times refresh every hour (3,600,000ms), which may be too frequent for an API with daily data
- Impact: Unnecessary network requests and CPU usage
- Improvement path: Calculate exact time until next prayer and schedule notifications more efficiently, reduce refresh interval to daily

**Large file loading on startup:**
- Files: `src/lib/indonesian-cities.ts`
- Problem: Contains all Indonesian cities loaded into memory on initial load, though only one is typically used
- Impact: Increased initial bundle size
- Improvement path: Lazy load city data or implement dynamic loading

## Fragile Areas

**Date/time parsing:**
- Files: `src-tauri/src/prayer.rs` (lines 127-130)
- Why fragile: Relies on specific time string formats from external API, which could change
- Safe modification: Add robust error handling and format fallbacks
- Test coverage: Currently lacks proper error handling tests for malformed time strings

**Tauri command interfaces:**
- Files: `src-tauri/src/lib.rs` (lines 38-88) and `src/lib/stores.ts` (used in Svelte components)
- Why fragile: Tightly coupled between frontend and backend with no clear error handling contracts
- Test coverage: Limited unit testing for error scenarios in IPC calls

## Scaling Limits

**API rate limiting:**
- Current capacity: Relies on free aladhan.com API which has unknown rate limits
- Limit: No protection against API rate limiting during high usage
- Scaling path: Implement caching layer with longer TTLs and local fallback

**Single API provider dependency:**
- Resource/System: Prayer time data retrieval
- Current capacity: Dependent on third-party API availability
- Limit: Service disruption if aladhan.com API goes down
- Scaling path: Add multiple prayer time providers with failover capabilities

## Dependencies at Risk

**Svelte 5 migration:**
- Risk: Project uses Svelte 5 features but Svelte 5 is still evolving rapidly
- Impact: Potential breaking changes as Svelte 5 matures
- Migration plan: Monitor Svelte 5 releases and maintain compatibility

**Outdated dependency versions:**
- Package: Various packages in `package.json` and `Cargo.toml` may have newer stable versions
- Risk: Security vulnerabilities in older versions
- Impact: Potential security issues and missed performance improvements

## Missing Critical Features

**Offline functionality:**
- Problem: App completely relies on network connection to fetch prayer times
- What's missing: Caching of last fetched prayer times for offline use
- Blocks: Users cannot access prayer times during network outages

**Comprehensive error handling:**
- Problem: Network errors and API failures are not thoroughly handled
- What's missing: Graceful degradation when API is unavailable
- Blocks: Poor user experience during temporary connectivity issues

## Test Coverage Gaps

**No automated tests:**
- Untested area: Core business logic for prayer time calculations and notifications
- Files: All frontend and backend logic
- What's not tested: Time-based calculations, notification scheduling, error recovery
- Risk: Changes may introduce regressions undetected
- Priority: High - tests should be implemented before any major feature additions

**Configuration validation:**
- Untested area: User configuration validation and error handling
- Files: `src-tauri/src/prayer.rs` and `src/lib/Settings.svelte`
- What's not tested: Invalid city selection, malformed dates, API errors
- Risk: App may crash or behave unexpectedly with invalid user inputs
- Priority: High - essential for production stability