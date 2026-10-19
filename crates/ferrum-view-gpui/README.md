# ferrum-view-gpui

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![Documentation](https://docs.rs/ferrum-view-gpui/badge.svg)](https://docs.rs/ferrum-view-gpui)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE-APACHE)

> **GPU-accelerated desktop window and UI adapter for Ferrum powered by GPUI.**  
> Provides rich native rendering on macOS (Metal), Linux (Vulkan/Wayland/X11), and Windows (DirectX).

---

## Overview

`ferrum-view-gpui` connects the abstract, functional editor engine of Ferrum (`ferrum-state`, `ferrum-commands`, `ferrum-view`) to native GPU windows using [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui).

It operates as a hermetically isolated rendering bridge:
- Core Ferrum crates maintain **zero dependencies on GPUI**.
- `ferrum-view-gpui` handles window lifecycles, font rasterization, mouse dragging, keyboard event parsing, and clipboard synchronization.

## Key Features

### 1. High-Performance GPU Rendering
- Native 60/120Hz frame rates with sub-millisecond input-to-photon latency.
- Direct hardware text rendering via GPUI's text layout engine and glyph atlas.
- Multi-cursor rendering with smooth time-based blinking animation.

### 2. Full Mouse & Selection Gestures
- **Left Click**: Places primary cursor at clicked line and column.
- **Mouse Drag**: Dynamically extends selection range from anchor to pointer.
- **Double Click**: Automatically detects and selects the word boundary under the cursor.

### 3. Native Clipboard & Keystrokes
- Integrated clipboard handling: `Mod-c` (copy), `Mod-x` (cut), and `Mod-v` (paste).
- Full keystroke parsing supporting modifier combinations (`Mod`, `Shift`, `Alt`) across macOS and Linux/Windows conventions.
- Dispatches shortcuts through `ferrum-commands`'s `KEYMAP` facet.

### 4. Custom Themes & Visual Styling (`EditorTheme`)
- Dark and light theme presets.
- Granular control over background, gutter, line numbers, cursor color, selection highlights, and text styles.

---

## Running the Interactive Desktop Example

Launch the interactive GPU editor window:
```bash
cargo run --example poc_editor
```

---

## Testing

Includes unit tests for keystroke parsing, clipboard operations, double-click word selection, and mouse drag selection extension:
```bash
cargo test -p ferrum-view-gpui --lib
```

---

## Clean-Room & Licensing Notice

- **License**: Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/your-org/ferrum/blob/main/LICENSE-APACHE)), matching GPUI.
- **Quarantine Guarantee**: Developed strictly under Clean-Room rules without importing or copying any GPL/AGPL-licensed Zed editor logic.
