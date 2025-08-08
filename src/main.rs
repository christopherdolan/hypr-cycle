use clap::Parser;
use serde::Deserialize;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    direction: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Next,
    Prev,
}

impl clap::ValueEnum for Direction {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Next, Self::Prev]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Direction::Next => clap::builder::PossibleValue::new("next"),
            Direction::Prev => clap::builder::PossibleValue::new("prev"),
        })
    }
}

#[derive(Deserialize)]
struct Monitor {
    name: String,
    focused: bool,
}

#[derive(Deserialize)]
struct Workspace {
    id: i32,
    monitor: String,
}

#[derive(Deserialize)]
struct ActiveWorkspace {
    id: i32,
}

fn main() {
    let args = Args::parse();

    // Get the focused monitor
    let monitors_json = run("hyprctl monitors -j");
    let monitors: Vec<Monitor> = serde_json::from_str(&monitors_json).unwrap();
    let focused_monitor = monitors
        .into_iter()
        .find(|m| m.focused)
        .expect("No focused monitor")
        .name;

    // Get all workspaces
    let workspaces_json = run("hyprctl workspaces -j");
    let workspaces: Vec<Workspace> = serde_json::from_str(&workspaces_json).unwrap();

    // Filter workspaces on the focused monitor
    let mut monitor_workspaces: Vec<i32> = workspaces
        .into_iter()
        .filter(|w| w.monitor == focused_monitor)
        .map(|w| w.id)
        .collect();
    monitor_workspaces.sort();

    // Get current workspace
    let current_json = run("hyprctl activeworkspace -j");
    let current_ws: ActiveWorkspace = serde_json::from_str(&current_json).unwrap();

    let current_index = monitor_workspaces
        .iter()
        .position(|&id| id == current_ws.id)
        .expect("Current workspace not found");

    let next_index = match args.direction {
        Direction::Next => (current_index + 1) % monitor_workspaces.len(),
        Direction::Prev => {
            (current_index + monitor_workspaces.len() - 1) % monitor_workspaces.len()
        }
    };

    let target_workspace = monitor_workspaces[next_index];
    run(&format!("hyprctl dispatch workspace {}", target_workspace));
}

fn run(cmd: &str) -> String {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let output = Command::new(parts[0])
        .args(&parts[1..])
        .output()
        .expect("Failed to run command");

    String::from_utf8(output.stdout).expect("Invalid UTF-8")
}
