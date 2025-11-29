use hypr_cycle::Args;
use hypr_cycle::HyprCycle;

fn main() -> anyhow::Result<()> {
    let mut svc = HyprCycle::real();
    let args = Args::parse_args();
    let direction = args.direction;
    let target = svc.get_target_workspace(direction)?;

    svc.switch_to_workspace(&target)
}
