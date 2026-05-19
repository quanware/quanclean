# QuanClean — Claude Instructions

## Project Overview

QuanClean is an open source, cross-platform disk cleaner for Windows and macOS.
- **GUI:** Tauri 2 + React + TypeScript
- **CLI:** Rust (shares core library with GUI)
- **Core logic:** Rust crate (`crates/core`)
- **i18n:** i18next (frontend), source language is English (`locales/en/`)

## Goals

1. Build a fast, trustworthy, modern disk cleaner
2. Support Windows and macOS equally
3. Provide both GUI and CLI interfaces from the same codebase
4. Be fully open source (MIT) with no telemetry, no ads, no upsells

## Architecture

```
crates/core      → platform-agnostic cleaning logic (Rust)
crates/cli       → CLI binary, calls core (Rust)
src-tauri        → Tauri backend, calls core via Rust (Rust)
src/             → React frontend (TypeScript)
locales/         → i18n strings (en = source)
```

## Coding Standards

- All source code, comments, and variable names in **English**
- Translations live in `locales/<lang>/`, never hardcode UI strings
- Prefer `Result<T, E>` over panics in Rust
- Frontend: functional React components, TypeScript strict mode
- Test coverage for all `crates/core` cleaning logic

## Key Commands

```bash
# Dev
pnpm tauri dev          # GUI dev mode
cargo run -p quanclean-cli -- scan   # CLI dev

# Test
cargo test
pnpm test

# Build
pnpm tauri build        # GUI release
cargo build -p quanclean-cli --release  # CLI release
```

## Current Status

- [ ] Project scaffold (Tauri + Rust workspace)
- [ ] Core: temp file scanner (Windows)
- [ ] Core: temp file scanner (macOS)
- [ ] Core: cache scanner
- [ ] Core: duplicate file detector
- [ ] CLI: scan command
- [ ] CLI: clean command
- [ ] GUI: dashboard
- [ ] GUI: treemap visualization
- [ ] i18n: zh-CN (in progress)
