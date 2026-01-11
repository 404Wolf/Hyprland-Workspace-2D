mod cmd;
mod r#impl;

use clap::Parser;
use cmd::{Args, Direction};
use hyprland::data::{Monitor, Monitors};
use hyprland::shared::{HyprData, HyprDataActive};
use r#impl::{move_workspace, reload_waybar};

fn main() -> hyprland::Result<()> {
    let args = Args::parse();

    let monitors = Monitors::get()?;
    let active_monitor = Monitor::get_active()?;
    let original_monitor_id = active_monitor.id as i32;
    let original_workspace_id = active_monitor.active_workspace.id;

    move_workspace(args.direction, original_monitor_id, original_workspace_id)?;

    if args.all {
        let direction_all = args.direction.normalize();

        for monitor in monitors.iter() {
            if monitor.id != original_monitor_id {
                let workspace_to_use = if args.sync {
                    original_workspace_id
                } else {
                    monitor.active_workspace.id
                };

                move_workspace(direction_all, monitor.id, workspace_to_use)?;
            }
        }

        move_workspace(direction_all, original_monitor_id, original_workspace_id)?;
    }

    reload_waybar();

    Ok(())
}
