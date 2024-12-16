#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rotation {
    Clockwise,
    AntiClockwise,
    Flip,
}

impl Direction {
    pub fn turn(&self, rotation: Rotation) -> Direction {
        match rotation {
            Rotation::Clockwise =>
                match self {
                    Self::Up => Self::Right,
                    Self::Right => Self::Down,
                    Self::Down => Self::Left,
                    Self::Left => Self::Up,
                },
            Rotation::AntiClockwise =>
                match self {
                    Self::Up => Self::Left,
                    Self::Left => Self::Down,
                    Self::Down => Self::Right,
                    Self::Right => Self::Up,
                },
            Rotation::Flip =>
                match self {
                    Self::Up => Self::Down,
                    Self::Left => Self::Right,
                    Self::Down => Self::Up,
                    Self::Right => Self::Left,
                },
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Up,
         Self::Right,
         Self::Down,
         Self::Left]
            .into_iter()
    }
}