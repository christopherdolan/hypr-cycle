use crate::domain::{OwnedMonitor, OwnedWorkspace};

use hyprrust::HyprlandConnection;
use hyprrust::data::{Monitors, Workspaces};
use hyprrust::commands::prelude::*;

/// Small trait representing the subset of Hyprland functionality your app needs.
/// Methods take `&mut self` so implementations can mutate without Mutex/RefCell.
/// `cfg_attr(test, mockall::automock)` generates a `MockHyprlandClient` for tests.
#[cfg_attr(test, mockall::automock)]
pub trait HyprlandClient {
    fn get_monitors(& mut self) -> anyhow::Result<Vec<OwnedMonitor>>;
    fn get_workspaces(&mut self) -> anyhow::Result<Vec<OwnedWorkspace>>;
    fn go_to_workspace(&mut self, id: i64) -> anyhow::Result<()>;
}

/// Real adapter around hyprrust::HyprlandConnection.
/// Converts hyprrust types into your Owned domain types at the boundary.
pub struct RealHyprlandClient {
    conn: HyprlandConnection,
}

impl RealHyprlandClient {
    pub fn new(conn: HyprlandConnection) -> Self {
        Self { conn }
    }
}

impl HyprlandClient for RealHyprlandClient {
    fn get_monitors(&mut self) -> anyhow::Result<Vec<OwnedMonitor>> {
        let monitors = self.conn.get_sync::<Monitors>()?;
        Ok(monitors.iter().map(OwnedMonitor::from).collect())
    }

    fn get_workspaces(&mut self) -> anyhow::Result<Vec<OwnedWorkspace>> {
        let workspaces = self.conn.get_sync::<Workspaces>()?;
        Ok(workspaces.iter().map(OwnedWorkspace::from).collect())
    }

    fn go_to_workspace(&mut self, id: i64) -> anyhow::Result<()> {
        self.conn
            .send_command_sync(&go_to_work_space(WorkspaceArgument::ID(id)))?;
        Ok(())
    }
}
