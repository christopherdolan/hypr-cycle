use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
pub struct Workspace {
    pub id: i32,
    pub monitor: String,
}

#[derive(Deserialize)]
pub struct Monitor {
    pub name: String,
    pub focused: bool,
}

pub fn get_focused_monitor() -> anyhow::Result<Monitor> {
    let monitors_out = Command::new("hyprctl").args(["monitors", "-j"]).output()?;
    let monitors: Vec<Monitor> = serde_json::from_slice(&monitors_out.stdout)?;
    monitors
        .into_iter()
        .find(|m| m.focused)
        .ok_or_else(|| anyhow::anyhow!("No focused monitor found"))
}

pub fn get_workspaces_for_monitor(monitor: &str) -> anyhow::Result<Vec<Workspace>> {
    let workspaces_out = Command::new("hyprctl")
        .args(["workspaces", "-j"])
        .output()?;
    let mut workspaces: Vec<Workspace> = serde_json::from_slice(&workspaces_out.stdout)?;
    workspaces.retain(|w| w.monitor == monitor && w.id > 0);
    workspaces.sort_by_key(|w| w.id);
    Ok(workspaces)
}

pub fn get_current_workspace() -> anyhow::Result<Workspace> {
    let current_ws_out = Command::new("hyprctl")
        .args(["activeworkspace", "-j"])
        .output()?;
    Ok(serde_json::from_slice(&current_ws_out.stdout)?)
}

pub fn switch_to_workspace(id: i32) -> anyhow::Result<()> {
    Command::new("hyprctl")
        .args(["dispatch", "workspace", &id.to_string()])
        .status()?;
    Ok(())
}
