// TODO: Have RealHyprlandClient build the connection itself
use clap::Parser;
use hyprrust::HyprlandConnection;

use hypr_cycle::*;
use connection::RealHyprlandClient;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(default_value = "next")]
    /// Direction to switch workspace ('next' or 'prev[ious]')
    direction: Direction,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let direction = args.direction;

    let conn = &mut RealHyprlandClient::new(HyprlandConnection::current()?);
    let monitor = get_focused_monitor(conn)?;
    let workspaces = get_workspaces_for_monitor(conn,&monitor)?;
    let current_ws = get_current_workspace(conn)?;

    let target = get_target_workspace(workspaces, current_ws, direction)?;
    switch_to_workspace(conn, &target)
}
