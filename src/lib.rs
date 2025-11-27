pub mod domain;

pub use domain::{OwnedMonitor,OwnedWorkspace};

use hyprrust::HyprlandConnection;
use hyprrust::data::{Monitors,Workspaces};
use hyprrust::commands::prelude::*;

pub fn get_focused_monitor(conn: &HyprlandConnection) -> anyhow::Result<OwnedMonitor> {
    let monitors = conn.get_sync::<Monitors>()?;
    let monitor = monitors
        .iter()
        .find(|m| m.focused)
        .ok_or_else(|| anyhow::anyhow!("No focused monitor found"))?;
    Ok(monitor.into())
}

pub fn get_workspaces_for_monitor(conn: &HyprlandConnection, monitor: &OwnedMonitor) -> anyhow::Result<Vec<OwnedWorkspace>> {
    let workspaces = conn.get_sync::<Workspaces>()?;
    let mut workspaces_for_monitor : Vec<OwnedWorkspace> = workspaces
        .iter()
        .filter(|w| w.monitor.eq(&monitor.name) && w.id > 0)
        .map(|w| w.into())
        .collect::<Vec<OwnedWorkspace>>();
    if workspaces_for_monitor.is_empty() {
        return Err(anyhow::anyhow!("No workspaces found for monitor: {}", &monitor.name));
    }
    workspaces_for_monitor.sort();
    Ok(workspaces_for_monitor.to_owned())
}

pub fn get_current_workspace_id(conn: &HyprlandConnection) -> anyhow::Result<OwnedWorkspace> {
    let focused_monitor = get_focused_monitor(conn)?;
    let active_workspace = focused_monitor.active_workspace;
    Ok(active_workspace)
}

pub fn switch_to_workspace(conn: &HyprlandConnection, target: &OwnedWorkspace) -> anyhow::Result<()> {
    conn.send_command_sync(&go_to_work_space(WorkspaceArgument::ID(target.id)))?;
    Ok(())
}
