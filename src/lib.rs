pub mod connection;
pub mod domain;

pub use clap::{Parser,ValueEnum};

use std::str::FromStr;
use domain::{OwnedMonitor,OwnedWorkspace};
use connection::HyprlandClient;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(default_value = "next")]
    /// Direction to switch workspace ('next' or 'prev[ious]')
    pub direction: Direction,
}

/// Represents the direction argument when invoked from the command line.
#[derive(Debug, Clone, ValueEnum)]
pub enum Direction {
    Next,
    Previous,
}

impl FromStr for Direction {
    type Err  = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_ascii_lowercase();

        match s.as_str() {
            "next" => Ok(Direction::Next),
            x if ["prev","previous"].contains(&x) => Ok(Direction::Previous),
            _ => Err("Unrecognized direction"),
        }
    }
}

/// In Hyprland, only one monitor can be in focus at a time.
/// This function returns that monitor.
pub fn get_focused_monitor(conn: &mut dyn HyprlandClient)
    -> anyhow::Result<OwnedMonitor> {

    let monitors = conn.get_monitors()?;
    let monitor = monitors
        .iter()
        .find(|m| m.focused())
        .ok_or_else(|| anyhow::anyhow!("No focused monitor found"))?;
    Ok(monitor.clone())
}

/// Returns a sorted list of the workspaces bound to the provided monitor.
/// Throws an error if the provided monitor doesn't have any workspaces
/// bound to it.
pub fn get_workspaces_for_monitor(
    conn: &mut dyn HyprlandClient,
    monitor: &OwnedMonitor
) -> anyhow::Result<Vec<OwnedWorkspace>> {

    let workspaces = conn.get_workspaces()?;
    let mut workspaces_for_monitor : Vec<OwnedWorkspace> = workspaces
        .into_iter()
        .filter(|w| w.monitor_name().eq(&monitor.name()) && w.id() > 0)
        .collect();
    if workspaces_for_monitor.is_empty() {
        return Err(
            anyhow::anyhow!(
                "No workspaces found for monitor: {}",
                &monitor.name()
            )
        );
    }
    workspaces_for_monitor.sort();
    Ok(workspaces_for_monitor.to_owned())
}

/// Returns the workspace that's active on the monitor that's in focus
pub fn get_current_workspace(
    conn: &mut dyn HyprlandClient
) -> anyhow::Result<OwnedWorkspace> {

    let focused_monitor = get_focused_monitor(conn)?;
    let active_workspace = focused_monitor.active_workspace();
    Ok(active_workspace)
}

/// The index of the sorted list of workspaces tells us where to
/// target the upcoming workspace switch.
pub fn get_target_workspace(
    all_workspaces: Vec<OwnedWorkspace>,
    active_workspace: OwnedWorkspace,
    direction: Direction)
-> anyhow::Result<OwnedWorkspace> {
    let idx = all_workspaces
        .iter()
        .position(|w| w == &active_workspace)
        .ok_or_else(|| anyhow::anyhow!("Current workspace not found"))?;
    let len = all_workspaces.len();

    let next_idx = match direction {
        Direction::Next => (idx + 1) % len,
        Direction::Previous => (idx + len - 1) % len,
    };
    Ok(all_workspaces[next_idx].clone())
}

pub fn switch_to_workspace(
    conn: &mut dyn HyprlandClient,
    target: &OwnedWorkspace
) -> anyhow::Result<()> {
    conn.go_to_workspace(target.id())?;
    Ok(())
}

#[cfg(test)]
pub mod fixtures {
    use crate::domain::{OwnedMonitor, OwnedWorkspace};

    pub fn ws(id: i64, mon: &str) -> OwnedWorkspace {
        OwnedWorkspace::new(id, mon.to_string())
    }

    pub fn mon(name: &str, id: i64, focused: bool, active_id: i64
    ) -> OwnedMonitor {
        OwnedMonitor::new(name.to_string(), id, focused, ws(active_id, name))
    }

    pub fn monitors() -> Vec<OwnedMonitor> {
        vec!(
            mon("eDP-1", 1, true, 1), //active monitor
            mon("HDMI-1", 2, false, 3),
        )
    }

    pub fn workspaces() -> Vec<OwnedWorkspace> {
        vec!(
            ws(-97, "eDP-1"), //hidden workspace ("scratch")
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

    /// There are two monitors in the fixture. One is marked active.
    /// This test ensures that the active monitor is returned by the function.
    #[test]
    fn test_get_focused_monitor() {
        let conn = &mut MockHyprlandClient::new();
        create_mock(conn);

        assert_eq!(
            get_focused_monitor(conn).unwrap(),
            fixtures::monitors()
                .iter()
                .find(|m| m.focused())
                .unwrap()
                .clone()
        );
    }

    /// The first monitor has three workspaces, but only two are visible.
    /// This test ensures that only the visible workspaces are returned
    /// by the function.
    #[test]
    fn test_get_workspaces_for_monitor() {
        let conn = &mut MockHyprlandClient::new();
        create_mock(conn);

        let target_monitor = &fixtures::monitors()[0];
        let workspaces = get_workspaces_for_monitor(conn, target_monitor)
            .unwrap();

        // None of the returned workspaces is invisible
        assert_eq!(None, workspaces.iter().find(|w| w.id() < 0));

        let expected_workspaces: Vec<OwnedWorkspace> = 
            fixtures::workspaces().into_iter()
                .filter(|w|
                    w.id() > 0
                    && w.monitor_name() == target_monitor.name()
                ).collect();
       
        // All of the expected workspaces are visible
        assert!(expected_workspaces.iter().all(|w| workspaces.contains(w)));
    }

    /// Monitors each keep track of their active workspace.
    /// Of the two monitors in the fixture, one is marked focused.
    /// This test ensures that the function returns the focused monitor's active workspace.
    #[test]
    fn test_get_current_workspace() {
        let conn = &mut MockHyprlandClient::new();
        create_mock(conn);

        assert_eq!(
            get_current_workspace(conn).unwrap().id(),
            fixtures::monitors()
                .iter()
                .find(|m| m.focused())
                .unwrap()
                    .active_workspace().id()
        );
    }

    /// Hard to test this function's behavior. We can only really ensure that the right underlying
    /// function call is made.
    #[test]
    fn test_switch_to_workspace() {
        let conn = &mut MockHyprlandClient::new();
        create_mock(conn);

        conn.expect_go_to_workspace()
            .times(1)
            .returning(move |_| Ok(()));
        let _ = crate::switch_to_workspace(
            conn,
            &fixtures::workspaces()[0]
        );
    }
}
