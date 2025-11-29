use clap::Parser;

use crate::args::Args;
use crate::connection::HyprlandClient;
use crate::domain::{Direction,OwnedMonitor,OwnedWorkspace};

/// Represents the total functionality of the program.
/// It can inspect the connected monitors, the extant workspaces,
/// and can switch between workspaces.
pub struct HyprCycle {
    connection: Box<dyn HyprlandClient>,
}

impl HyprCycle {

    pub fn parse_args() -> Args {
        Args::parse()
    }

    /// The connection can be real or a mock object, as seen in the tests
    /// in `src/service.rs`.
    pub fn new(connection: Box<dyn HyprlandClient>) -> HyprCycle {
        HyprCycle {
            connection
        }
    }

    /// This function builds a version of the service backed by a real
    /// HyprlandConnection. It's just for convenience to keep main() clean.
    pub fn real() -> HyprCycle {
        let conn = hyprrust::HyprlandConnection::current().unwrap();
        let client = crate::connection::RealHyprlandClient::new(conn);
        HyprCycle::new(Box::new(client))
    }

    /// In Hyprland, only one monitor can be in focus at a time.
    /// This function returns that monitor.
    pub fn get_focused_monitor(&mut self)
        -> anyhow::Result<OwnedMonitor> {

        let monitors = self.connection.get_monitors()?;
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
        &mut self,
        monitor: &OwnedMonitor
    ) -> anyhow::Result<Vec<OwnedWorkspace>> {

        let workspaces = self.connection.get_workspaces()?;
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
    pub fn get_current_workspace(&mut self) -> anyhow::Result<OwnedWorkspace> {

        let focused_monitor = self.get_focused_monitor()?;
        let active_workspace = focused_monitor.active_workspace();
        Ok(active_workspace)
    }

    /// The index of the sorted list of workspaces tells us where to
    /// target the upcoming workspace switch.
    pub fn get_target_workspace(&mut self, direction: Direction)
    -> anyhow::Result<OwnedWorkspace> {

        let monitor = &self.get_focused_monitor()?;
        let workspaces = &self.get_workspaces_for_monitor(monitor)?;
        let current_ws = &self.get_current_workspace()?;

        let idx = workspaces
            .iter()
            .position(|w| w == current_ws)
            .ok_or_else(|| anyhow::anyhow!("Current workspace not found"))?;
        let len = workspaces.len();

        let next_idx = match direction {
            Direction::Next => (idx + 1) % len,
            Direction::Previous => (idx + len - 1) % len,
        };
        Ok(workspaces[next_idx].clone())
    }

    pub fn switch_to_workspace(
        &mut self,
        target: &OwnedWorkspace
    ) -> anyhow::Result<()> {
        self.connection.go_to_workspace(target.id())?;
        Ok(())
    }

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

    pub fn create_mock_service() -> HyprCycle {
        let mut conn = MockHyprlandClient::new();
        conn.expect_get_monitors()
            .returning(move || Ok(fixtures::monitors()));
        conn.expect_get_workspaces()
            .returning(move || Ok(fixtures::workspaces()));
        HyprCycle::new(Box::new(conn))
    }

    pub fn compose_mock_service(conn: MockHyprlandClient) -> HyprCycle {
        HyprCycle::new(Box::new(conn))
    }

    /// There are two monitors in the fixture. One is marked active.
    /// This test ensures that the active monitor is returned by the function.
    #[test]
    fn test_get_focused_monitor() {
        let mut hs = create_mock_service();

        assert_eq!(
            hs.get_focused_monitor().unwrap(),
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
        let mut hs = create_mock_service();

        let target_monitor = &fixtures::monitors()[0];
        let workspaces = hs.get_workspaces_for_monitor(target_monitor)
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
        let mut hs = create_mock_service();

        assert_eq!(
            hs.get_current_workspace().unwrap().id(),
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
        let mut conn = MockHyprlandClient::new();
        conn.expect_go_to_workspace()
            .times(1)
            .returning(move |_| Ok(()));
        let mut hs = compose_mock_service(conn);

        let _ = hs.switch_to_workspace(&fixtures::workspaces()[0]);
    }
}
