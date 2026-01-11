use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

impl Direction {
    pub fn is_move(&self) -> bool {
        matches!(
            self,
            Direction::MoveLeft | Direction::MoveRight | Direction::MoveUp | Direction::MoveDown
        )
    }

    pub fn normalize(&self) -> Direction {
        match self {
            Direction::MoveLeft => Direction::Left,
            Direction::MoveRight => Direction::Right,
            Direction::MoveUp => Direction::Up,
            Direction::MoveDown => Direction::Down,
            _ => *self,
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "workspace2d")]
#[command(about = "2D workspace navigation for Hyprland", long_about = None)]
pub struct Args {
    /// Direction to move (left, right, up, down, move_left, move_right, move_up, move_down)
    pub direction: Direction,

    /// Move all monitors together
    #[arg(long)]
    pub all: bool,

    /// Synchronize all monitors to the same workspace
    #[arg(long)]
    pub sync: bool,
}
