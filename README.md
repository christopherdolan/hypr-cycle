# hypr-cycle

A fast, minimal Rust utility to cycle through workspaces on the **currently focused monitor** in [Hyprland](https://github.com/hyprwm/Hyprland). Unlike `hyprctl dispatch workspace e+1`, this tool avoids switching workspaces on the wrong monitor in multi-monitor setups.

---

## Features

- Cycles only the workspaces **bound to the focused monitor**
- Wraps around when reaching the end or beginning
- Fast, clean, and dependency-free
- Written in Rust for performance and reliability

---

## Installation

1. Clone the repo:
   ```bash
   git clone https://github.com/christopherdolan/hypr-cycle
   cd hypr-cycle
   ```

2. Build and install:
   ```bash
   cargo build --release
   sudo install -Dm755 target/release/hypr-cycle /usr/local/bin/hypr-cycle
   ```

---

## Usage

```bash
hypr-cycle next
hypr-cycle prev
```

This will cycle to the next or previous workspace **on the monitor that currently has keyboard focus**.

---

## Example Hyprland Config

If you want to changge workspaces using Mod+Tab and Mod+Shift+Tab, add these bindings to your `~/.config/hypr/hyprland.conf`:

```ini
bind = $mod, Tab, exec, hypr-cycle next
bind = $mod SHIFT, Tab, exec, hypr-cycle prev
```

---

## Example Waybar Config

If you want to change workspaces on waybar using your mouse wheel, add these (particularly `on-scroll-up` and `on-scroll-down`) to your `~/.config/waybar/config.jsonc`:

```jsonc
        "hyprland/window": {
            "icon": true,
            "format": "{title}",
            "on-scroll-up":"hypr-cycle prev",
            "on-scroll-down":"hypr-cycle next",
            "tooltip": true,
        },
```

---

## Dependencies

- [Hyprland](https://github.com/hyprwm/Hyprland)
- Rust (for building only)

---

## License

MIT license. See `LICENSE` file.

---

## Contributions

PRs and suggestions welcome! This is a minimal utility, but if you have an idea to make it more powerful or efficient, feel free to open an issue or PR.
