use crate::cmd::Direction;
use hyprland::keyword::Keyword;

pub const MATRIX_SIZE: i32 = 8;
pub const MAX_SCREENS: i32 = 10;

pub fn x_value(n: i32) -> i32 {
    n % MATRIX_SIZE
}

pub fn y_value(n: i32) -> i32 {
    n / MATRIX_SIZE
}

pub fn set_animation(direction: Direction) -> hyprland::Result<()> {
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

pub fn move_workspace(direction: Direction, screen: i32, workspace_id: i32) -> hyprland::Result<i32> {
    let workspace = (workspace_id - 1) / MAX_SCREENS;
    let mut x_index = x_value(workspace);
    let mut y_index = y_value(workspace);

    set_animation(direction)?;

    match direction {
        Direction::Left | Direction::MoveLeft => {
            x_index = (x_index + MATRIX_SIZE - 1) % MATRIX_SIZE;
        }
        Direction::Right | Direction::MoveRight => {
            x_index = (x_index + 1) % MATRIX_SIZE;
        }
        Direction::Up | Direction::MoveUp => {
            y_index = (y_index + MATRIX_SIZE - 1) % MATRIX_SIZE;
        }
        Direction::Down | Direction::MoveDown => {
            y_index = (y_index + 1) % MATRIX_SIZE;
        }
    }

    let target_workspace = MAX_SCREENS * (y_index * MATRIX_SIZE + x_index) + screen + 1;

    let workspace_action = if direction.is_move() {
        format!("dispatch movetoworkspace {}", target_workspace)
    } else {
        format!("dispatch workspace {}", target_workspace)
    };

    let batch_cmd = format!(
        "dispatch focusmonitor {} ; {} ; dispatch moveworkspacetomonitor {} {}",
        screen, workspace_action, target_workspace, screen
    );

    std::process::Command::new("hyprctl")
        .arg("--batch")
        .arg(&batch_cmd)
        .output()
        .map_err(|e| hyprland::error::HyprError::CommandFailed(format!("Failed to execute hyprctl: {}", e)))?;

    Ok(target_workspace)
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
