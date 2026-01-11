use crate::cmd::Direction;
use hyprland::dispatch::{
    Dispatch, DispatchType, MonitorIdentifier, WorkspaceIdentifier, WorkspaceIdentifierWithSpecial,
};
use hyprland::keyword::Keyword;
use wrapping_coords2d::WrappingCoords2d;

pub const MATRIX_SIZE: i32 = 8;
pub const MAX_SCREENS: i32 = 10;

pub fn move_workspace(
    direction: Direction,
    screen: i32,
    workspace_id: i32,
) -> hyprland::Result<i32> {
    // Create a wrapping coordinate system for the matrix
    let coords = WrappingCoords2d::new(MATRIX_SIZE, MATRIX_SIZE)
        .expect("Failed to create coordinate system");

    // Calculate which workspace in the matrix (subtract screen offset)
    let workspace = ((workspace_id - 1) / MAX_SCREENS) as usize;

    set_animation(direction)?;

    // Calculate new position based on direction with wrapping
    let new_workspace = match direction {
        Direction::Left | Direction::MoveLeft => coords.shift(workspace, -1, 0),
        Direction::Right | Direction::MoveRight => coords.shift(workspace, 1, 0),
        Direction::Up | Direction::MoveUp => coords.shift(workspace, 0, -1),
        Direction::Down | Direction::MoveDown => coords.shift(workspace, 0, 1),
    };

    // Convert back to workspace ID (add screen offset)
    let target_workspace = (new_workspace as i32) * MAX_SCREENS + screen + 1;

    // Focus the monitor first
    Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Id(
        screen.into(),
    )))?;

    // Move to or move window to the target workspace
    if direction.is_move() {
        Dispatch::call(DispatchType::MoveToWorkspace(
            WorkspaceIdentifierWithSpecial::Id(target_workspace),
            None,
        ))?;
    } else {
        Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
            target_workspace,
        )))?;
    }

    // Move the workspace to the monitor to ensure it's on the correct screen
    Dispatch::call(DispatchType::MoveWorkspaceToMonitor(
        WorkspaceIdentifier::Id(target_workspace),
        MonitorIdentifier::Id(screen.into()),
    ))?;

    Ok(target_workspace)
}

fn set_animation(direction: Direction) -> hyprland::Result<()> {
    let animation = match direction {
        Direction::Left | Direction::Right | Direction::MoveLeft | Direction::MoveRight => {
            "workspaces,1,1,default,slide"
        }
        Direction::Up | Direction::Down | Direction::MoveUp | Direction::MoveDown => {
            "workspaces,1,1,default,slidevert"
        }
    };
    Keyword::set("animation", animation)?;
    Ok(())
}

pub fn reload_waybar() {
    if let Ok(output) = std::process::Command::new("pgrep").arg("waybar").output() {
        if let Ok(pid_str) = String::from_utf8(output.stdout) {
            if let Ok(pid) = pid_str.trim().parse::<i32>() {
                unsafe {
                    libc::kill(pid, libc::SIGRTMIN() + 1);
                }
            }
        }
    }
}
