use std::usize;

use crate::{Direction, NumExt};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {

    pub fn advance(&self, direction: Direction, bounds: (usize, usize)) -> Option<Self> {
        self.advance_with_distance(direction, bounds, 1)
    }

    pub fn advance_with_distance(&self, direction: Direction, (bound_x, bound_y): (usize, usize), distance: usize) -> Option<Self> {
        match direction {
            Direction::Up => Some(Self {
                x: self.x,
                y: self.y.clamped_add_signed(-1 * distance as isize, bound_y)?,
            }),
            Direction::Down => Some(Self {
                x: self.x,
                y: self.y.clamped_add_signed(distance as isize, bound_y)?,
            }),
            Direction::Left => Some(Self {
                x: self.x.clamped_add_signed(-1 * distance as isize, bound_x)?,
                y: self.y,
            }),
            Direction::Right => Some(Self {
                x: self.x.clamped_add_signed(distance as isize, bound_x)?,
                y: self.y,
            }),
         }
    }

    pub fn middle(&self, other: &Self) -> Self {
        Self {
            x: (other.x + self.x) / 2,
            y: (other.y + self.y) / 2,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}