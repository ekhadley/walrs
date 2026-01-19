# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

walrs is a fast, minimalist CLI tool for generating color schemes from wallpapers, written in Rust. It's a drop-in replacement for pywal - users can replace the `wal` command with `walrs`.

## Build & Run Commands

```bash
cargo build           # Debug build
cargo build --release # Release build
cargo run -- -i /path/to/image.jpg  # Run with an image
cargo run -- --help   # Show help
```

## Architecture

The codebase is organized into 6 modules:

- **main.rs** - CLI argument parsing via `argh`, orchestrates the workflow
- **get_colors.rs** - Color extraction using three methods (kmeans_colors, palette_extract, color_thief), then sorts by luminance and adjusts saturation/brightness
- **create_templates.rs** - Template engine that processes files from `~/.config/walrs/templates/`, replacing placeholders like `{color0}`, `{background}`, `{foreground}` with generated colors
- **reload.rs** - Applies colors to open terminals via escape sequences (`/dev/pts/*`), sets wallpaper, and runs user scripts from `~/.config/walrs/scripts/`
- **wallpaper.rs** - Desktop environment detection and wallpaper setting (supports Hyprland, Sway, GNOME, KDE, i3, bspwm, etc.)
- **utils.rs** - Path helpers for `~/.config`, `~/.cache`, `/usr/share/walrs`

## Key Paths

- **Templates**: `~/.config/walrs/templates/` (user) or `/usr/share/walrs/templates/` (system fallback)
- **Colorschemes**: `~/.config/walrs/colorschemes/{dark,light}/`
- **Scripts**: `~/.config/walrs/scripts/` - bash scripts run after color generation
- **Output**: `~/.cache/wal/` - generated color files (colors, colors.json, colors.css, etc.)

## Template Variables

Templates support these placeholders:
- `{color0}` through `{color15}` - 16 terminal colors (with variants: `.strip`, `.rgb`, `.rgba`, `.alpha`)
- `{background}`, `{foreground}`, `{cursor}` - special colors (same variant support)
- `{wallpaper}` - path to current wallpaper
- `{checksum}` - concatenated hex colors for cache invalidation

## CLI Flags

Key flags: `-i <image>` (generate from image), `-r` (reload), `-t <theme>` (use preset), `-g <name>` (save current as theme), `-s/-b` (saturation/brightness -128 to 127), `-W` (skip wallpaper), `-S` (skip scripts), `-q` (quiet)
