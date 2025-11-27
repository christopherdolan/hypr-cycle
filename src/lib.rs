use hyprrust::commands::prelude::*;
use hyprrust::HyprlandConnection;
use hyprrust::data::{Monitors,Workspaces};

pub fn get_focused_monitor(conn: &HyprlandConnection) -> anyhow::Result<String> {
    let monitors = conn.get_sync::<Monitors>()?;
    let name = &monitors
        .iter()
        .find(|m| m.focused)
        .ok_or("No focused monitor found")
        .unwrap()
        .name;
    Ok(name.to_owned())
}

pub fn get_workspaces_for_monitor(conn: &HyprlandConnection, monitor: &String) -> anyhow::Result<Vec<i64>> {
    let workspaces = conn.get_sync::<Workspaces>()?;
    let mut workspace_ids_for_monitor : Vec<i64> = workspaces
        .iter()
        .filter(|w| w.monitor.eq(monitor) && w.id > 0)
        .map(|w| w.id)
        .collect::<Vec<i64>>();
    if workspace_ids_for_monitor.is_empty() {
        return Err(anyhow::anyhow!("No workspaces found for monitor: {}", monitor));
    }
    workspace_ids_for_monitor.sort();
    Ok(workspace_ids_for_monitor.to_owned())
}

pub fn get_current_workspace(conn: &HyprlandConnection) -> anyhow::Result<i64> {
    let focused_monitor_name = get_focused_monitor(conn)?;
    let monitors = conn.get_sync::<Monitors>()?;
    let focused_monitor = monitors
        .iter()
        .find(|m| m.name == focused_monitor_name)
        .ok_or("Focused monitor not found")
        .unwrap();
    let active_workspace_id = focused_monitor.active_workspace.id;
    Ok(active_workspace_id)
}

pub fn switch_to_workspace(conn: &HyprlandConnection, id: i64) -> anyhow::Result<()> {
    conn.send_command_sync(&go_to_work_space(WorkspaceArgument::ID(id)))?;
    Ok(())
}
