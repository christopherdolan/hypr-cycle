# hypr-cycle

A fast, minimal Rust utility to cycle through workspaces on the **currently focused monitor** in [Hyprland](https://github.com/hyprwm/Hyprland). Unlike `hyprctl dispatch workspace e+1`, this tool avoids switching workspaces on the wrong monitor in multi-monitor setups.

Written entirely by ChatGPT.

---

## 🔧 Features

- Cycles only the workspaces **bound to the focused monitor**
- Wraps around when reaching the end or beginning
- Fast, clean, and dependency-free (uses `hyprctl` under the hood)
- Written in Rust for performance and reliability

---

## 🚀 Installation

1. Clone the repo:
   ```bash
   git clone https://github.com/your-username/hypr-cycle
   cd hypr-cycle
   ```

2. Build and install:
   ```bash
   cargo build --release
   sudo install -Dm755 target/release/hypr-cycle /usr/local/bin/hypr-cycle
   ```

---

## 🎮 Usage

```bash
hypr-cycle --direction next
hypr-cycle --direction prev
```

This will cycle to the next or previous workspace **on the monitor that currently has keyboard focus**.

---

## 🖥️ Example Hyprland Config

Add these bindings to your `~/.config/hypr/hyprland.conf`:

```ini
bind = $mod, Tab, exec, hypr-cycle --direction next
bind = $mod SHIFT, Tab, exec, hypr-cycle --direction prev
```

---

## 🛠️ Dependencies

- [Hyprland](https://github.com/hyprwm/Hyprland)
- `hyprctl` in your `PATH`
- Rust (for building only)

---

## 📜 License

BSD 0-Clause license. See `LICENSE` file.

---

## 🤝 Contributions

PRs and suggestions welcome! This is a minimal utility, but if you have an idea to make it more powerful or efficient, feel free to open an issue or PR.
