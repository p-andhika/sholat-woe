# External Integrations

**Analysis Date:** 2026-03-18

## APIs & External Services

**Prayer Times API:**
- Aladhan API - Fetches prayer times data for Islamic prayers
  - SDK/Client: reqwest (Rust HTTP client)
  - Endpoint: https://api.aladhan.com/v1/calendarByCity/{year}?city={city}&country={country}&method={method}&school={school}
  - Auth: No authentication required (public API)

## Data Storage

**Databases:**
- None - No traditional database used

**File Storage:**
- Local filesystem only - User configuration stored in app data directory as config.json

**Caching:**
- None - Real-time API calls made for prayer times

## Authentication & Identity

**Auth Provider:**
- None - No user authentication required

## Monitoring & Observability

**Error Tracking:**
- None - No dedicated error tracking service

**Logs:**
- Console logging only

## CI/CD & Deployment

**Hosting:**
- Desktop application - Distributed as native executable for Windows, macOS, Linux

**CI Pipeline:**
- None - No CI configuration detected

## Environment Configuration

**Required env vars:**
- None - No environment variables required

**Secrets location:**
- Not applicable - No secrets required for this application

## Webhooks & Callbacks

**Incoming:**
- None - No webhook endpoints

**Outgoing:**
- None - No webhooks sent

## Additional External Services

**Notification System:**
- Native OS notifications via Tauri plugin
  - Plugin: @tauri-apps/plugin-notification
  - Platform-specific: Uses native notification systems (Windows, macOS, Linux)

**Sound System:**
- Native OS sounds for notifications
  - Platform-specific: Uses afplay on macOS for system sounds (Ping, Glass)

**System Tray Integration:**
- Native OS tray integration via Tauri
  - Platform-specific: System tray with context menu options

---

*Integration audit: 2026-03-18*