# hypr-cycle

<p align="center">
  <video src="https://github.com/user-attachments/assets/df5e5d89-56ec-418e-9d30-19ff901f6c6d" autoplay loop muted playsinline width="720"></video>
</p>

A fast, minimal Rust utility to cycle through workspaces on the **currently focused monitor** in [Hyprland](https://github.com/hyprwm/Hyprland). Unlike `hyprctl dispatch workspace e+1`, this tool avoids switching workspaces to a different monitor in multi-monitor setups. Unlike `hyprctl dispatch focusworkspaceoncurrentmonitor next`, this will only switch to workspaces with windows on them.

![Build](https://github.com/christopherdolan/hypr-cycle/actions/workflows/rust.yml/badge.svg)

---

## Features

- Cycles only the workspaces **bound to the focused monitor**
- Wraps around when reaching the end or beginning
- Fast, clean, and dependency-free
- Written in Rust for performance and reliability

---

## Installation

### Cargo

If you've got the [Rust tools](https://rust-lang.org/tools/install/) installed, this is probably the easiest way to get it.

```bash
  cargo install hypr-cycle
```

### Arch Linux (via the AUR)

- Requires [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git).

1. Clone the repo:
  ``` bash
  git clone https://aur.archlinux.org/hypr-cycle.git
  ```

2. Make and install

  ``` bash
  cd hypr-cycle
  makepkg -si
  ```

... or just install `hypr-cycle` via your favorite AUR helper.
(This is the only method that doesn't require you to have Rust tools installed.)

- Example requires [yay](https://github.com/Jguer/yay?tab=readme-ov-file#installation), but any AUR helper will do.

```bash
yay hypr-cycle
```

### From Source

- Requires [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git).
- Requires [Rust](https://rust-lang.org/tools/install/).

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

**If you want to package this for other distributions, be my guest!**

---

## Usage

```bash
hypr-cycle next
hypr-cycle prev
```

This will cycle to the next or previous numbered workspace **on the monitor that currently has keyboard focus**.

---

## Example Hyprland Config

If you want to change workspaces using Mod+Tab and Mod+Shift+Tab, add these bindings to your `~/.config/hypr/hyprland.conf`:

```ini
bind = $mod, Tab, exec, hypr-cycle next
bind = $mod SHIFT, Tab, exec, hypr-cycle prev
```

---

## Example Waybar Config

If you want to change workspaces on Waybar using your mouse wheel, add these to your `~/.config/waybar/config.jsonc`:

```jsonc
        "hyprland/workspaces": {
            // ...
            "on-scroll-up":"hypr-cycle prev",
            "on-scroll-down":"hypr-cycle next",
            // ...
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
