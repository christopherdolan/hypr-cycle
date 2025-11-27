use std::cmp::Ordering;
use hyprrust::data::{Monitor,Workspace,WorkspaceBrief};

#[derive(Debug, Clone)]
pub struct OwnedMonitor {
    pub name: String,
    pub id: i64,
    pub focused: bool,
    pub active_workspace: OwnedWorkspace
}

impl From<&Monitor> for OwnedMonitor {
    fn from(m: &Monitor) -> Self {
        OwnedMonitor {
            name: m.name.clone(),
            id: m.id,
            focused: m.focused,
            active_workspace: OwnedWorkspace { id: m.active_workspace.id }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedWorkspace {
    pub id: i64
}

impl From<&Workspace> for OwnedWorkspace {
    fn from(w: &Workspace) -> Self {
        OwnedWorkspace {
            id: w.id
        }
    }
}

impl From<&WorkspaceBrief> for OwnedWorkspace {
    fn from(w: &WorkspaceBrief) -> Self {
        OwnedWorkspace {
            id: w.id
        }
    }
}

impl Ord for OwnedWorkspace {
    fn cmp(&self, other:&Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for OwnedWorkspace {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

