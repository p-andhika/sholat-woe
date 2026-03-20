# Sholat Woe

A beautiful, minimalist desktop application for Islamic prayer times. Built with Tauri, Svelte, and Rust. Supports Indonesian cities and desktop notifications.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey.svg)
![Tauri](https://img.shields.io/badge/built%20with-Tauri-FFC131.svg)

## ✨ Features

- **Accurate Prayer Times**: Fetches precise prayer times from Aladhan API using multiple calculation methods
- **Indonesian Cities Support**: 98+ Indonesian cities pre-configured (Kota/Kabupaten)
- **Desktop Notifications**: Get notified before prayer time and at the exact moment
- **Beautiful UI**: Glass-morphism design with smooth animations
- **Lightweight**: Native desktop app powered by Rust
- **System Tray**: Run quietly in the background
- **Hijri Date Display**: Shows current Islamic date alongside Gregorian date
- **Customizable Settings**:
  - Select your city
  - Configure notification advance time
- **Real-time Countdown**: See time remaining until next prayer

## 📸 Screenshots

_Screenshots coming soon_

## 🚀 Installation

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Build from Source

````bash
# Clone the repository
git clone https://github.com/p-andhika/sholat-woe.git
cd sholat-woe

# Install dependencies
npm install

# Run development server
npm run tauri dev

# Build for current platform
npm run tauri build
````

Output will be in `src-tauri/target/release/bundle/`:

| Platform | Output                      |
| -------- | --------------------------- |
| macOS    | `.dmg`, `.app`              |
| Windows  | `.msi`, `.exe` (setup)      |
| Linux    | `.AppImage`, `.deb`, `.rpm` |

### Build for All Platforms (Cross-compilation)

**Option A: GitHub Actions (Recommended)**

Automated builds run on every tag push via the [release workflow](.github/workflows/release.yml). It also auto-generates changelog entries from commit history.

```bash
# Release a new version (bumps package.json, tauri.conf.json, Cargo.toml)
npm run release:patch   # 1.0.2 → 1.0.3
npm run release:minor   # 1.0.2 → 1.1.0
npm run release:major   # 1.0.2 → 2.0.0
```

This bumps the version in all files, commits, tags, and pushes — triggering the release workflow automatically.

**Option B: Manual Cross-compilation**

| Target                | Command                                                    | Requirements   |
| --------------------- | ---------------------------------------------------------- | -------------- |
| macOS (Apple Silicon) | `npm run tauri build -- --target aarch64-apple-darwin`     | macOS + Xcode  |
| macOS (Intel)         | `npm run tauri build -- --target x86_64-apple-darwin`      | macOS + Xcode  |
| Windows               | `npm run tauri build -- --target x86_64-pc-windows-msvc`   | Windows + MSVC |
| Linux                 | `npm run tauri build -- --target x86_64-unknown-linux-gnu` | Linux + GCC    |

For complete cross-platform building, see [Tauri's distribution guide](https://tauri.app/v1/guides/building/cross-platform/).

### Download Pre-built Binaries

Download pre-compiled binaries from [GitHub Releases](https://github.com/p-andhika/sholat-woe/releases) — or [build from source](#build-from-source) below.

| Platform              | Download                              |
| --------------------- | ------------------------------------- |
| macOS (Apple Silicon) | `Sholat-Woe_<version>_aarch64.dmg`    |
| macOS (Intel)         | `Sholat-Woe_<version>_x64.dmg`        |
| Windows               | `Sholat-Woe_<version>_x64-setup.exe`  |
| Linux                 | `sholat-woe_<version>_amd64.AppImage` |

## 🖥️ Supported Platforms

- macOS 10.13+ (Intel & Apple Silicon)
- Windows 10/11
- Linux (major distributions)

## 🏙️ Supported Cities

98+ Indonesian cities are pre-configured, including:

- Jakarta, Surabaya, Bandung, Medan
- Semarang, Makassar, Palembang
- Yogyakarta, Malang, Denpasar
- Padang, Pekanbaru, Pontianak
- and many more...

_Full list in [src/lib/indonesian-cities.ts](src/lib/indonesian-cities.ts)_

## 🛠️ Development

### Tech Stack

- **Frontend**: [Svelte 5](https://svelte.dev/) + [Vite](https://vitejs.dev/)
- **Backend**: [Rust](https://www.rust-lang.org/) + [Tauri 2.0](https://tauri.app/)
- **Icons**: [Phosphor Icons](https://phosphoricons.com/)
- **Prayer Times API**: [Aladhan API](https://aladhan.com/prayer-times-api)

### Project Structure

```
sholat-woe/
├── src/                    # Frontend (Svelte)
│   ├── lib/               # Components & utilities
│   ├── routes/            # SvelteKit routes
│   └── main.ts            # Entry point
├── src-tauri/             # Backend (Rust)
│   ├── src/              # Rust source code
│   │   ├── main.rs       # Tauri commands & app setup
│   │   ├── prayer.rs     # Prayer time fetching logic
│   │   └── tray.rs       # System tray implementation
│   ├── icons/            # App icons
│   └── Cargo.toml        # Rust dependencies
├── package.json          # Node dependencies
└── README.md
```

### Available Scripts

```bash
npm run dev            # Start Vite dev server
npm run build          # Build frontend
npm run preview        # Preview built frontend
npm run check          # Run Svelte type checking
npm run tauri dev      # Run Tauri dev (hot reload)
npm run tauri build    # Build desktop app
npm run release:patch  # Bump patch version & release
npm run release:minor  # Bump minor version & release
npm run release:major  # Bump major version & release
```

## 📦 Attribution & Credits

### Core Technologies

This project is built on the shoulders of amazing open-source projects:

#### Frontend

| Library                                                                                    | License        | Purpose                       |
| ------------------------------------------------------------------------------------------ | -------------- | ----------------------------- |
| [Svelte](https://svelte.dev/)                                                              | MIT            | Frontend framework            |
| [SvelteKit](https://kit.svelte.dev/)                                                       | MIT            | Application framework         |
| [Vite](https://vitejs.dev/)                                                                | MIT            | Build tool                    |
| [Tauri](https://tauri.app/)                                                                | Apache-2.0/MIT | Desktop application framework |
| [@tauri-apps/api](https://github.com/tauri-apps/tauri)                                     | Apache-2.0/MIT | Tauri JavaScript API          |
| [@tauri-apps/plugin-notification](https://github.com/tauri-apps/tauri-plugin-notification) | MIT/Apache-2.0 | Desktop notifications         |
| [@tauri-apps/plugin-opener](https://github.com/tauri-apps/tauri-plugin-opener)             | MIT/Apache-2.0 | Open URLs externally          |

#### Backend (Rust)

| Crate                                                                           | License        | Purpose                |
| ------------------------------------------------------------------------------- | -------------- | ---------------------- |
| [tauri](https://crates.io/crates/tauri)                                         | Apache-2.0/MIT | Desktop app framework  |
| [tauri-plugin-opener](https://crates.io/crates/tauri-plugin-opener)             | MIT/Apache-2.0 | URL opening            |
| [tauri-plugin-notification](https://crates.io/crates/tauri-plugin-notification) | MIT/Apache-2.0 | Native notifications   |
| [serde](https://crates.io/crates/serde)                                         | MIT/Apache-2.0 | Serialization          |
| [serde_json](https://crates.io/crates/serde_json)                               | MIT/Apache-2.0 | JSON handling          |
| [reqwest](https://crates.io/crates/reqwest)                                     | MIT            | HTTP client            |
| [tokio](https://crates.io/crates/tokio)                                         | MIT            | Async runtime          |
| [chrono](https://crates.io/crates/chrono)                                       | MIT/Apache-2.0 | Date/time handling     |
| [window-vibrancy](https://crates.io/crates/window-vibrancy)                     | MIT            | macOS vibrancy effects |
| [cocoa](https://crates.io/crates/cocoa)                                         | MIT            | macOS platform APIs    |
| [objc](https://crates.io/crates/objc)                                           | MIT            | Objective-C runtime    |

#### Icons & Assets

| Asset                                        | License | Source                                            |
| -------------------------------------------- | ------- | ------------------------------------------------- |
| [Phosphor Icons](https://phosphoricons.com/) | MIT     | Icons for prayer times (moon, sun, sunrise, etc.) |
| App Icons                                    | Custom  | Generated via Tauri icon tooling                  |

### External APIs & Data

| Resource                                            | Provider                                 | Usage                    |
| --------------------------------------------------- | ---------------------------------------- | ------------------------ |
| [Aladhan API](https://aladhan.com/prayer-times-api) | Islamic Network                          | Prayer times calculation |
| Indonesian Cities Data                              | Compiled from various government sources | City selection dropdown  |

### Calculation Methods

Prayer time calculations are powered by the Aladhan API, which implements established Islamic prayer calculation methods including:

- Kementerian Agama Republik Indonesia (KEMENAG)

See [Aladhan API Documentation](https://aladhan.com/calculation-methods) for complete details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) team for the amazing Rust-based desktop framework
- [Svelte](https://svelte.dev/) team for the beautiful reactive UI framework
- [Aladhan](https://aladhan.com/) for providing free prayer times API
- [Phosphor Icons](https://phosphoricons.com/) for the beautiful icon set
- Indonesian Ministry of Religion (KEMENAG) for official calculation guidelines

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## 📧 Contact

Created by [Andhika Prakasiwi](https://github.com/p-andhika)

---

<div align="center">

**Sholat Woe** — _Stay connected with your prayers_

</div>
