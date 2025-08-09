use clap::Parser;
use hypr_cycle::*;

#[derive(Parser)]
struct Args {
    #[arg(default_value = "next")]
    direction: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let monitor = get_focused_monitor()?;
    let workspaces = get_workspaces_for_monitor(&monitor.name)?;
    let current_ws = get_current_workspace()?;
    let idx = workspaces
        .iter()
        .position(|w| w.id == current_ws.id)
        .ok_or_else(|| anyhow::anyhow!("Current workspace not found"))?;

    let next_idx = if args.direction == "next" {
        (idx + 1) % workspaces.len()
    } else {
        (idx + workspaces.len() - 1) % workspaces.len()
    };

    let target = workspaces[next_idx].id;
    switch_to_workspace(target)?;
    Ok(())
}
