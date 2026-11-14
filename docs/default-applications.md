# Asari-OS Default Application Suite

This document defines the recommended out-of-the-box applications for Asari-OS.

The goal is a polished, capable desktop experience comparable to CachyOS, Nobara, Bazzite, Omarchy, or macOS — while remaining true to the **Spectre Netrunner** visual identity (Mass Effect holographic clarity + Cyberpunk rainy neon elegance).

All primary recommendations are open-source, actively maintained, and highly themeable (dark / holographic blue / biotic violet-cyan).

**Distribution preference:** Flatpak (Flathub) for sandboxed GUI applications. Native packages only where Flatpak is impractical or performance-critical (terminal, system tools).

---

## Core Applications (Pre-installed / First-class)

### Web Browser
**Primary: Zen Browser**  
- Firefox (Gecko) based, modern Arc-inspired interface  
- Vertical tabs, workspaces, split view, excellent theming via Mods  
- Strong privacy posture and active development (2026)  
- Feels premium, calm, and futuristic — ideal match for holographic panels  
- Official Flatpak available: `app.zen_browser.zen`

**Strong alternatives:**  
- LibreWolf — privacy-hardened Firefox fork (uBlock Origin pre-installed, telemetry stripped)  
- Floorp — heavy UI customization + sidebars  
- Mullvad Browser — maximum anti-fingerprinting

### Digital Painting / Concept Art
**Primary: Krita**  
- Best-in-class open-source digital painting and illustration application  
- Professional brush engines, animation timeline, excellent tablet support  
- Perfect for concept art, character design, and creative work inside a netrunner aesthetic  
- Flatpak: `org.krita.Krita`

**Alternatives:**  
- MyPaint — lighter, distraction-free painting  
- Pinta — simple Paint.NET-style editor  
- GIMP — general raster editing when needed

### Terminal Emulator
**Primary: Kitty**  
- GPU-accelerated, extremely themeable  
- Excellent for biotic/holographic color schemes and ligatures  
- Supports image display and advanced layouts

**Strong alternatives:**  
- Ghostty — modern, fast, beautiful defaults  
- Alacritty — minimal and fast  
- WezTerm — feature-rich with Lua config

### Code / Text Editor
**Primary:** VSCodium (telemetry-free VS Code) + Neovim (LazyVim / NvChad style config)  
Clean modern GUI editor paired with a powerful terminal-based editor for true netrunner workflows.

**Alternatives:** Zed, Helix, Lapce

### File Manager
**Primary:** Custom Asari File Explorer (designed)  
**Fallback / companion:** Dolphin (KDE) or Nautilus (GNOME) — both highly themeable.

### Media
| Role              | Primary                          | Notes / Alternatives                  |
|-------------------|----------------------------------|---------------------------------------|
| Video Player      | Celluloid (mpv frontend)         | Haruna, VLC                           |
| Music Player      | Amberol                          | Strawberry, Rhythmbox                 |
| Image Viewer      | Loupe                            | Gwenview, Eye of GNOME                |
| Screenshot        | Flameshot                        | Spectacle, Grim + Slurp               |
| Screen Recording  | OBS Studio (Flatpak)             | SimpleScreenRecorder                  |

### Documents & Productivity
| Role              | Primary                          | Notes                                 |
|-------------------|----------------------------------|---------------------------------------|
| Office Suite      | LibreOffice                      | Pre-themed dark; OnlyOffice as alt    |
| PDF Viewer        | Papers (or Okular)               | Modern, clean                         |
| Notes / PKM       | Logseq or Notesnook              | Local-first preferred                 |
| Password Manager  | KeePassXC                        | Bitwarden / Proton Pass optional      |
| Calculator        | Qalculate! or GNOME Calculator   | Scientific + programmer modes         |

### System & Utilities
| Role              | Primary                          | Notes                                 |
|-------------------|----------------------------------|---------------------------------------|
| System Monitor    | Mission Center                   | Modern, clean, Flatpak-friendly       |
| Archive Manager   | File Roller / Ark                |                                       |
| Software Center   | Flatpak + Discover / Bazaar      | Custom Asari Software Center planned  |
| Flatpak Permissions | Flatseal                      | Essential for sandbox control         |
| System Info       | Fastfetch                        | Stylish, themeable                    |
| Terminal Prompt   | Starship                         | Cross-shell, highly configurable      |
| Audio             | PipeWire + EasyEffects           | High-quality audio stack              |

### Optional / Gaming-Oriented (Bazzite / Nobara style)
Enabled via a first-boot or Settings toggle rather than forced:
- Steam + Proton
- Lutris / Heroic Games Launcher
- MangoHud + GameMode
- Bottles (Wine prefixes)
- Gamescope (for exclusive fullscreen)

---

## Design & Theme Philosophy

Applications must support (or be easily adapted to):
1. Deep dark themes with holographic blue panels
2. Biotic violet / cyan energy accents
3. Clean geometry and subtle neon highlights
4. High contrast without visual noise
5. Preference for privacy, local-first, and no forced telemetry

A shared **Asari / Spectre theme package** (GTK + Qt + icon theme + cursor) will be provided so that Zen, Krita, Kitty, LibreOffice, and system dialogs all feel cohesive.

---

## Implementation Notes

- Prefer Flatpak for all major GUI applications (sandbox + easy updates + consistent runtime).
- Terminal, Starship, Fastfetch, and core CLI tools remain native for performance and integration.
- The custom Asari File Explorer, Terminal, Settings, and Login screens (already designed) take visual priority; third-party apps should not clash.
- First-boot experience should offer a short “Application Profile” chooser: Creator, Developer, Gamer, Minimal.
- All choices above are intentionally curated to avoid the “barren” feeling of a minimalist research OS while keeping the aesthetic tight and intentional.

---

## Package Manifest (Conceptual)

See `config/default-apps.toml` and `manifests/flatpak-apps.txt` for the concrete list used by image builders and the software center.
