# Pototogho

A simple GNOME application to compare cryptographic hash strings, built in pure Rust using GTK4 and Libadwaita.

## Features

- Compare two hash strings (MD5, SHA-1, SHA-256, SHA-512, etc.)
- Case-insensitive matching with automatic whitespace trimming
- Native GNOME interface with keyboard shortcuts

## Build & Run

### GNOME Builder
1. Open the project directory in **GNOME Builder**.
2. Press `Ctrl + F5` to build and run, or select **Export** from the build menu to package as a `.flatpak`.

### Terminal (Meson)
```bash
meson setup _build
ninja -C _build
```

## License

- **Source Code:** [GPL-3.0-or-later](COPYING)
- **Application Icons:** [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) © 2026 labambangpunto
