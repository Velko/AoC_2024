use aoc_tools::{Direction, Point};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    Move(Direction),
    Activate,
}

pub const NUMERIC_A: Point = Point { x: 2, y: 3 };
pub const NUMERIC_F: Point = Point { x: 0, y: 3 };

pub const DIRECTIONAL_A: Point = Point { x: 2, y: 0 };
pub const DIRECTIONAL_F: Point = Point { x: 0, y: 0 };

pub fn commands_to_string(cmds: &[Command]) -> String {
    cmds.iter().map(|cmd| match cmd {
        Command::Move(Direction::Up) => '^',
        Command::Move(Direction::Right) => '>',
        Command::Move(Direction::Down) => 'v',
        Command::Move(Direction::Left) => '<',
        Command::Activate => 'A',
    }).collect()
}
