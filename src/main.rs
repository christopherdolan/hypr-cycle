use crate::connection::RealHyprlandClient;
use hypr_cycle::*;

use clap::Parser;
use hyprrust::HyprlandConnection;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value = "next")]
    /// Direction to switch workspace ('next' or 'prev')
    direction: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let conn = &mut RealHyprlandClient::new(HyprlandConnection::current()?);

    let monitor = get_focused_monitor(conn)?;
    let workspaces = get_workspaces_for_monitor(
        conn,
        &monitor
    )?;
    let current_ws = get_current_workspace(conn)?;
    let idx = workspaces
        .iter()
        .position(|w| w == &current_ws)
        .ok_or_else(|| anyhow::anyhow!("Current workspace not found"))?;

    let next_idx = if args.direction == "next" {
        (idx + 1) % workspaces.len()
    } else if args.direction == "prev" {
        (idx + workspaces.len() - 1) % workspaces.len()
    } else {
        return Err(anyhow::anyhow!("Direction must be 'next' or 'prev'"));
    };

    let target = workspaces[next_idx].clone();
    switch_to_workspace(conn, &target)?;
    Ok(())
}
