pub mod connection;
pub mod domain;

pub use domain::{OwnedMonitor,OwnedWorkspace};
pub use connection::HyprlandClient;

pub fn get_focused_monitor(conn: &mut dyn HyprlandClient) -> anyhow::Result<OwnedMonitor> {
    let monitors = conn.get_monitors()?;
    let monitor = monitors
        .iter()
        .find(|m| m.focused)
        .ok_or_else(|| anyhow::anyhow!("No focused monitor found"))?;
    Ok(monitor.clone())
}

pub fn get_workspaces_for_monitor(conn: &mut dyn HyprlandClient, monitor: &OwnedMonitor) -> anyhow::Result<Vec<OwnedWorkspace>> {
    let workspaces = conn.get_workspaces()?;
    let mut workspaces_for_monitor : Vec<OwnedWorkspace> = workspaces
        .iter()
        .filter(|w| w.monitor_name.eq(&monitor.name) && w.id > 0)
        .map(|w| w.to_owned())
        .collect();
    if workspaces_for_monitor.is_empty() {
        return Err(anyhow::anyhow!("No workspaces found for monitor: {}", &monitor.name));
    }
    workspaces_for_monitor.sort();
    Ok(workspaces_for_monitor.to_owned())
}

pub fn get_current_workspace(conn: &mut dyn HyprlandClient) -> anyhow::Result<OwnedWorkspace> {
    let focused_monitor = get_focused_monitor(conn)?;
    let active_workspace = focused_monitor.active_workspace;
    Ok(active_workspace)
}

pub fn switch_to_workspace(conn: &mut dyn HyprlandClient, target: &OwnedWorkspace) -> anyhow::Result<()> {
    conn.go_to_workspace(target.id)?;
    Ok(())
}

#[cfg(test)]
pub mod fixtures {
    use crate::domain::{OwnedMonitor, OwnedWorkspace};

    pub fn ws(id: i64, mon: &str) -> OwnedWorkspace {
        OwnedWorkspace {
            id,
            monitor_name: mon.to_string(),
        }
    }

    pub fn mon(name: &str, id: i64, focused: bool, active_id: i64) -> OwnedMonitor {
        OwnedMonitor {
            name: name.to_string(),
            id,
            focused,
            active_workspace: ws(active_id, name),
        }
    }

    pub fn monitors() -> Vec<OwnedMonitor> {
        vec!(
            mon("eDP-1", 1, true, 1),
            mon("HDMI-1", 2, false, 3),
        )
    }

    pub fn workspaces() -> Vec<OwnedWorkspace> {
        vec!(
            ws(1, "eDP-1"),
            ws(2, "eDP-1"),
            ws(3, "HDMI-1")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::{MockHyprlandClient};

    pub fn create_mock(conn: &mut MockHyprlandClient) {
        conn.expect_get_monitors()
            .returning(move || Ok(fixtures::monitors()));
        conn.expect_get_workspaces()
            .returning(move || Ok(fixtures::workspaces()));
    }

    #[test]
    fn test_get_focused_monitor() {
        let conn = &mut MockHyprlandClient::new();
        create_mock(conn);

        assert_eq!(get_focused_monitor(conn).unwrap(), fixtures::monitors()[0]);
    }

    #[test]
    fn test_get_workspaces_for_monitor() {
        let conn = &mut MockHyprlandClient::new();
        create_mock(conn);

        assert!(get_workspaces_for_monitor(conn, &fixtures::monitors()[0])
            .unwrap()
            .iter()
            .any(|m| m == &fixtures::workspaces()[0])
        );
    }

    #[test]
    fn test_get_current_workspace() {
        let conn = &mut MockHyprlandClient::new();
        create_mock(conn);

        assert_eq!(crate::get_current_workspace(conn).unwrap().id, fixtures::workspaces()[0].id);
    }

    #[test]
    fn test_switch_to_workspace() {
        let conn = &mut MockHyprlandClient::new();
        create_mock(conn);

        conn.expect_go_to_workspace()
            .times(1)
            .returning(move |_| Ok(()));
        let _ = crate::switch_to_workspace(conn, &fixtures::workspaces()[0]);
    }
}
