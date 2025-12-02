use crate::domain::{OwnedMonitor, OwnedWorkspace};

use hyprrust::commands::prelude::*;
use hyprrust::data::{Monitors, Workspaces};
use hyprrust::HyprlandConnection;

use anyhow::Result;

/// Exposes the necessary fraction of the hyprrust::HyprlandConnection struct.
#[cfg_attr(test, mockall::automock)]
pub trait HyprlandClient {
    fn get_monitors(&self) -> Result<Vec<OwnedMonitor>>;
    fn get_workspaces(&self) -> Result<Vec<OwnedWorkspace>>;
    fn go_to_workspace(&self, id: i64) -> Result<()>;
}

/// Real adapter around hyprrust::HyprlandConnection.
/// Converts hyprrust types into Owned domain types at the boundary.
pub struct RealHyprlandClient {
    conn: HyprlandConnection,
}

impl RealHyprlandClient {
    pub fn new(conn: HyprlandConnection) -> Self {
        Self { conn }
    }
}

impl HyprlandClient for RealHyprlandClient {
    fn get_monitors(&self) -> Result<Vec<OwnedMonitor>> {
        let monitors = self.conn.get_sync::<Monitors>()?;
        Ok(monitors.iter().map(OwnedMonitor::from).collect())
    }

    fn get_workspaces(&self) -> Result<Vec<OwnedWorkspace>> {
        let workspaces = self.conn.get_sync::<Workspaces>()?;
        Ok(workspaces.iter().map(OwnedWorkspace::from).collect())
    }

    fn go_to_workspace(&self, id: i64) -> Result<()> {
        self.conn
            .send_command_sync(&go_to_work_space(WorkspaceArgument::ID(id)))?;
        Ok(())
    }
}
