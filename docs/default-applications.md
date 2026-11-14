# Asari-OS Default Application Suite

This document defines the recommended out-of-the-box applications for Asari-OS.

The goal is to ship a polished, capable desktop experience comparable to CachyOS, Nobara, Bazzite, or macOS, while staying true to the **Spectre Netrunner** visual identity (Mass Effect holographic clarity + Cyberpunk rainy neon elegance).

All primary recommendations are open-source and highly themeable.

---

## Core Applications

### Web Browser
**Primary: Zen Browser**  
- Firefox-based, modern Arc-inspired interface  
- Excellent theming support via Mods  
- Strong privacy features  
- Feels premium and futuristic  

**Alternatives:** LibreWolf, Mullvad Browser

### Digital Painting / Creative
**Primary: Krita**  
- Best-in-class open-source digital painting application  
- Professional brush engines, animation support, excellent tablet integration  
- Perfect for concept art and creative work in a netrunner-themed OS  

**Alternatives:** MyPaint (lighter), Pinta (simple)

### Terminal
**Primary: Kitty**  
- GPU-accelerated  
- Extremely themeable  
- Excellent for a holographic / biotic aesthetic  

**Alternatives:** Alacritty, Ghostty

### Code / Text Editor
**Primary:** VSCodium + Neovim (with modern config such as NvChad or LazyVim)  
Clean modern GUI editor paired with a powerful terminal-based editor.

### Media
- **Video Player:** Celluloid (modern mpv frontend)
- **Music Player:** Amberol
- **Image Viewer:** Loupe

### Documents & Productivity
- **PDF Viewer:** Papers (or Okular)
- **Office Suite:** LibreOffice (pre-themed)
- **Notes:** Notesnook or Logseq
- **Password Manager:** KeePassXC

### System Tools
- **System Monitor:** Mission Center
- **Screenshot Tool:** Flameshot
- **Archive Manager:** File Roller / Ark
- **Calculator:** GNOME Calculator or Qalculate
- **Software Center:** Flatpak + Discover (or custom Asari Software Center)

### Polish & Quality-of-Life
- **Fastfetch** — stylish system information
- **Starship** — modern terminal prompt
- **PipeWire + EasyEffects** — high quality audio
- Flatpak + Flathub enabled by default

---

## Design Philosophy

Applications should:
1. Support deep dark theming (holographic blue, biotic violet/cyan, neon accents)
2. Feel modern and premium rather than utilitarian
3. Prefer privacy and local-first where possible
4. Be actively maintained open-source projects

---

## Implementation Notes

- Prefer Flatpak for sandboxed GUI applications where sensible
- Provide a consistent Asari/Spectre theme package for GTK and Qt
- The custom Asari File Explorer, Terminal, and Settings (already designed) should take visual priority
- Gaming-oriented users can optionally enable a Bazzite-style stack (Gamescope, MangoHud, etc.)
