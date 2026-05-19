# Contributing to QuanClean

Thank you for your interest in contributing! 🎉

## Getting Started

1. Fork the repo and clone locally
2. Install prerequisites: [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/), [pnpm](https://pnpm.io/)
3. Run `pnpm install` and `cargo build`

## Development

```bash
# Run GUI in dev mode
pnpm tauri dev

# Run CLI
cargo run -p quanclean-cli -- scan

# Run tests
cargo test
pnpm test
```

## Guidelines

- **Source language is English** — all code comments, variable names, and `locales/en/` strings must be in English
- **Translations** — PRs adding new languages are always welcome; add a folder under `locales/`
- **Commits** — follow [Conventional Commits](https://www.conventionalcommits.org/): `feat:`, `fix:`, `docs:`, `chore:`
- **Tests** — add tests for any new core cleaning logic in `crates/core/`
- **Platform** — test on both Windows and macOS when touching platform-specific code

## Adding a Language

1. Copy `locales/en/` to `locales/<lang>/` (e.g. `locales/fr/`)
2. Translate all values in the JSON files (keep keys in English)
3. Add the language to the list in `src/i18n/config.ts`
4. Submit a PR with title `i18n: add French (fr)`

## Reporting Bugs

Please include:
- OS and version
- QuanClean version
- Steps to reproduce
- Expected vs actual behavior
