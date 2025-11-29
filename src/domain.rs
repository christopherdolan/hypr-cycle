use std::cmp::Ordering;
use std::str::FromStr;

use clap::ValueEnum;
use hyprrust::data::{Monitor, Workspace, WorkspaceBrief};

/// Represents the direction argument when invoked from the command line.
#[derive(Debug, Clone, ValueEnum)]
pub enum Direction {
    Next,
    Previous,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_ascii_lowercase();

        match s.as_str() {
            "next" => Ok(Direction::Next),
            "previous" => Ok(Direction::Previous),
            _ => Err("Unrecognized direction"),
        }
    }
}

/// Wraps the hyprrust::Monitor type, which describes a physical monitor
/// reported by Hyprland.
///
/// Each monitor has a unique name and ID. Only one monitor is marked 'focused'
/// at any time.
/// This type is returned by 'HyprlandClient::get_monitors()'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedMonitor {
    name: String,
    id: i64,
    focused: bool,
    active_workspace: OwnedWorkspace,
}

impl OwnedMonitor {
    pub fn new(
        name: String,
        id: i64,
        focused: bool,
        active_workspace: OwnedWorkspace,
    ) -> OwnedMonitor {
        OwnedMonitor {
            name,
            id,
            focused,
            active_workspace,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn active_workspace(&self) -> OwnedWorkspace {
        self.active_workspace.clone()
    }
}

impl From<&Monitor> for OwnedMonitor {
    fn from(m: &Monitor) -> Self {
        OwnedMonitor {
            name: m.name.clone(),
            id: m.id,
            focused: m.focused,
            active_workspace: OwnedWorkspace {
                id: m.active_workspace.id,
                monitor_name: m.name.clone(),
            },
        }
    }
}

/// Wraps the hyprrust::Workspace type, which describes a workspace reported
/// by Hyprland.
///
/// Each workspace has a unique ID and is associated with exactly one monitor.
/// This type is returned by 'HyprlandClient::get_workspaces()'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedWorkspace {
    id: i64,
    monitor_name: String,
}

impl OwnedWorkspace {
    pub fn new(id: i64, monitor_name: String) -> OwnedWorkspace {
        OwnedWorkspace { id, monitor_name }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn monitor_name(&self) -> String {
        self.monitor_name.clone()
    }

    pub fn visible(&self) -> bool {
        self.id > 0
    }
}

impl From<&Workspace> for OwnedWorkspace {
    fn from(w: &Workspace) -> Self {
        OwnedWorkspace {
            id: w.id,
            monitor_name: w.monitor.clone(),
        }
    }
}

impl From<&WorkspaceBrief> for OwnedWorkspace {
    fn from(w: &WorkspaceBrief) -> Self {
        OwnedWorkspace {
            id: w.id,
            monitor_name: String::new(),
        }
    }
}

impl Ord for OwnedWorkspace {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for OwnedWorkspace {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
