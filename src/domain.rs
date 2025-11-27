use std::cmp::Ordering;
use hyprrust::data::{Monitor, Workspace, WorkspaceBrief};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedMonitor {
    pub name: String,
    pub id: i64,
    pub focused: bool,
    pub active_workspace: OwnedWorkspace,
}

impl From<&Monitor> for OwnedMonitor {
    fn from(m: &Monitor) -> Self {
        OwnedMonitor {
            name: m.name.clone(),
            id: m.id,
            focused: m.focused,
            active_workspace: OwnedWorkspace { id: m.active_workspace.id, monitor_name: m.name.clone() },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedWorkspace {
    pub id: i64,
    pub monitor_name: String
}

impl From<&Workspace> for OwnedWorkspace {
    fn from(w: &Workspace) -> Self {
        OwnedWorkspace { id: w.id, monitor_name: w.monitor.clone() }
    }
}

impl From<&WorkspaceBrief> for OwnedWorkspace {
    fn from(w: &WorkspaceBrief) -> Self {
        OwnedWorkspace { id: w.id, monitor_name: String::new() }
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
