# QuanClean

> Open source, cross-platform disk cleaner for Windows and macOS — GUI + CLI, built with Tauri and Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS-blue)]()
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)]()

---

## Why QuanClean?

| | BleachBit | CCleaner | CleanMyMac | **QuanClean** |
|---|:---:|:---:|:---:|:---:|
| Open Source | ✅ | ❌ | ❌ | ✅ |
| Free | ✅ | Freemium | ❌ | ✅ |
| Windows | ✅ | ✅ | ❌ | ✅ |
| macOS | ❌ | ✅ | ✅ | ✅ |
| Modern UI | ❌ | ✅ | ✅ | ✅ |
| CLI Support | ✅ | ❌ | ❌ | ✅ |
| i18n | Partial | ✅ | ✅ | ✅ |

---

## Features

- 🗑️ **Temp & cache cleaner** — system temp files, browser caches, app caches
- 📦 **App residue cleaner** — leftover files after uninstalling apps
- 🔍 **Large file finder** — find space hogs with treemap visualization
- 👯 **Duplicate file detector** — find and remove duplicate files
- 🖥️ **GUI mode** — modern cross-platform desktop app (Tauri + React)
- ⌨️ **CLI mode** — scriptable, CI-friendly, power-user ready
- 🌍 **Multilingual** — English (source), 中文, 日本語, Español, and more

---

## Installation

### Download (GUI)

> Coming soon — releases will be available for Windows (.msi) and macOS (.dmg)

### CLI (via cargo)

```bash
cargo install quanclean
```

### Build from source

```bash
# Prerequisites: Rust, Node.js, pnpm
git clone https://github.com/quanware/quanclean.git
cd quanclean
pnpm install
pnpm tauri build
```

---

## CLI Usage

```bash
# Scan and report (dry run)
quanclean scan

# Clean with confirmation
quanclean clean

# Clean specific targets
quanclean clean --temp --cache --duplicates

# Output report as JSON
quanclean scan --output json

# Non-interactive (for scripts/CI)
quanclean clean --yes
```

---

## Project Structure

```
quanclean/
├── crates/
│   ├── core/           # Core cleaning logic (cross-platform Rust)
│   └── cli/            # CLI entry point
├── src/                # React + TypeScript frontend (GUI)
├── src-tauri/          # Tauri config + Rust backend bridge
├── locales/            # i18n resources
│   ├── en/             # Source language (English)
│   ├── zh-CN/
│   ├── ja/
│   └── es/
└── docs/               # Documentation
```

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a PR.

- **Source language:** English (all code comments, strings source in `locales/en/`)
- **New language?** Add a folder under `locales/` and translate `locales/en/common.json`
- **Bug reports:** Open an issue with your OS, version, and steps to reproduce

---

## Roadmap

- [ ] MVP: temp/cache cleaner (Windows + macOS)
- [ ] CLI: `scan`, `clean`, `report` commands
- [ ] GUI: dashboard + treemap visualization
- [ ] Duplicate file detection
- [ ] App residue cleaner
- [ ] Plugin system for custom clean rules
- [ ] i18n: zh-CN, ja, es

---

## License

[MIT](LICENSE) © [Quanware](https://github.com/quanware)
