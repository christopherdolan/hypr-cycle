use anyhow::Context;
use hypr_cycle::HyprCycle;

fn main() -> anyhow::Result<()> {
    let svc = HyprCycle::real().context("Are you sure you're running Hyprland?")?;

    let args = HyprCycle::parse_args();
    let direction = args.direction;
    let target = svc.get_target_workspace(direction)?;

    svc.switch_to_workspace(&target)
}
