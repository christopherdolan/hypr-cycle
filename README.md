## hypr-cycle — Rust Project
A fast and monitor-aware workspace cycler for Hyprland, written in Rust.

### License
BSD Zero Clause License

### Features
- Cycles workspaces only on the currently focused monitor.
- Written in Rust for speed and low overhead.
- Uses Hyprland's JSON API via `hyprctl`.
- Ready for AUR packaging.

### Installation
```bash
cargo install --path . --root /usr/local
```

### Usage
```bash
hypr-cycle next
hypr-cycle prev
```
