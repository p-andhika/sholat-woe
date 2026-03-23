# Changelog

All notable changes to Sholat Woe will be documented in this file.
## [1.3.1] - 2026-03-23

### Fixed

- remove redundant condition in prayerData check

## [1.3.0] - 2026-03-23

### Added

- persist notified prayers in localStorage

### Fixed

- update git config and push process for release

## [1.2.0] - 2026-03-22

### Added

- add caching for prayer times API responses

### Fixed

- ensure CHANGELOG updates are fast-forwardable
- ensure changelog updates are rebased before push

## [1.1.0] - 2026-03-20

### Added

- add automated version bump and release scripts
- display app version in UI for better visibility

### Fixed

- rewrite changelog generation to avoid YAML parsing errors


## [1.0.2] - 2026-03-18

### Added

- Improved prayer notifications with advance and exact-time alerts
- Auto-refresh prayer data when a prayer time passes
- Sound notifications (Ping for advance, Glass for exact time)

### Fixed

- Notification timing logic for more reliable alerts

## [1.0.1] - 2026-03-18

### Added

- Desktop notifications when prayer time arrives
- Auto-refresh prayer times on prayer pass

## [1.0.0] - 2026-03-18

### Added

- Initial release of Sholat Woe - Islamic Prayer Times Desktop App
- Prayer times display with grid layout
- Hijri date display
- Next prayer countdown with tray menu title
- Settings for city, country, and calculation method
- macOS, Windows, and Linux support

### Fixed

- macOS BOOL type compatibility for x86_64 and aarch64
- Windows icon format
- Linux notification plugin dependencies
- macOS Intel (x86_64) build target
- CI/CD release workflow
