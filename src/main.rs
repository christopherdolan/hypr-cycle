use clap::Parser;
use hypr_cycle::*;
use hyprrust::HyprlandConnection;

#[derive(Parser)]
struct Args {
    #[arg(default_value = "next")]
    direction: String,
}

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    let conn: HyprlandConnection = HyprlandConnection::current()?;

    let monitor: String = get_focused_monitor_name(&conn)?;
    let workspaces: Vec<i64> = get_workspace_ids_for_monitor(&conn, &monitor)?;
    let current_ws: i64 = get_current_workspace_id(&conn)?;
    let idx: usize = workspaces
        .iter()
        .position(|w| w == &current_ws)
        .ok_or_else(|| anyhow::anyhow!("Current workspace not found"))?;

    let next_idx: usize = if args.direction == "next" {
        (idx + 1) % workspaces.len()
    } else {
        (idx + workspaces.len() - 1) % workspaces.len()
    };

    let target: i64 = workspaces[next_idx];
    switch_to_workspace(&conn, target)?;
    Ok(())
}
