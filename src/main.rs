// TODO: Have RealHyprlandClient build the connection itself
use hyprrust::HyprlandConnection;
use connection::RealHyprlandClient;
use hypr_cycle::*;

fn main() -> anyhow::Result<()> {
    let conn = &mut RealHyprlandClient::new(HyprlandConnection::current()?);
    let monitor = get_focused_monitor(conn)?;
    let workspaces = get_workspaces_for_monitor(conn,&monitor)?;
    let current_ws = get_current_workspace(conn)?;

    let args = Args::parse();
    let direction = args.direction;
    let target = get_target_workspace(workspaces, current_ws, direction)?;

    switch_to_workspace(conn, &target)
}
