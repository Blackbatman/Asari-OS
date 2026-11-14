# Asari-OS

**Elegant. Adaptive. Multi-platform.**

Asari-OS is a research operating system that combines the elegance and long-view thinking of the Asari with a modern multi-platform, agent-driven engineering approach.

Named after the influential Asari species from Mass Effect — natural biotics, millennia-spanning perspective, masters of joining different systems — Asari-OS aims to be a cohesive, honest, and adaptable foundation that runs cleanly across platforms instead of being locked to one.

## Visual Identity
Spectre Netrunner aesthetic: Mass Effect holographic clarity + Cyberpunk 2077 rainy neon density.  
Deep blue holographic panels, biotic energy accents, chrome frames, subtle scanlines.

## Core Principles
- **Multi-platform first** — Platform trait abstraction. QEMU and Linux-native are primary development targets. Other platforms (including experimental ones) are peers.
- **Honest capabilities** — What works is clearly marked. What is research is labeled as such.
- **Agent-directed development** — Human sets architecture, constraints, and acceptance criteria. Agents implement, test, and iterate inside those bounds.
- **Elegant systems joining** — Inspired by Asari melding: clean composition of services rather than monolithic complexity.

## Default Application Suite
Asari-OS ships a curated, theme-aligned application suite so the desktop never feels barren:

| Category          | Primary Choice          |
|-------------------|-------------------------|
| Web Browser       | **Zen Browser**         |
| Digital Painting  | **Krita**               |
| Terminal          | Kitty                   |
| Code Editor       | VSCodium + Neovim       |
| Video / Music     | Celluloid / Amberol     |
| Office / PDF      | LibreOffice / Papers    |
| System Monitor    | Mission Center          |
| Notes / Passwords | Logseq / KeePassXC      |

Full rationale, alternatives, Flatpak IDs, and theming notes live in [`docs/default-applications.md`](docs/default-applications.md).  
Machine-readable manifests: [`config/default-apps.toml`](config/default-apps.toml) and [`manifests/flatpak-apps.txt`](manifests/flatpak-apps.txt).

## Architecture Principles

### Multi-Platform Contract
Asari-OS is built around a strict `Platform` trait (`crates/asari-platform/src/traits.rs`).

Primary development targets:
1. QEMU (fast iteration cycle)
2. Linux-native

Other platforms are peers that must implement the same contract.

### Elegant Composition (“Melding”)
Inspired by Asari nervous-system joining:
- Prefer small, well-defined services that can be composed cleanly.
- Avoid deep monolithic kernels early.
- Services should be able to “join” (communicate and share state) with clear boundaries.

### Agent-Directed Development
- Humans define the architecture, visual identity, and acceptance criteria.
- Agents implement, test, and update the living `feature_list.json`.
- Every change must remain reviewable and reversible.

### Visual Identity
- Primary palette: deep holographic blue, biotic violet, neon magenta/yellow accents.
- Aesthetic: Mass Effect holographic clarity + Cyberpunk rainy neon density.
- Boot sequence: organic biotic energy and crest-inspired geometry rather than hard digital glitch.

## Current Status
Very early research stage. Skeleton + visual identity established. Boot animation concept complete. Core platform abstraction and agent harness in progress. Default application suite fully defined and documented.

## Getting Started
```bash
make setup
make check
make qemu-smoke
```

## License
MIT OR Apache-2.0
