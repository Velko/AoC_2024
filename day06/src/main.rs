use aoc_tools::{Grid, InvalidInput, NumExt};
use ndarray::{Array3, ShapeBuilder};
use std::collections::HashSet;
use rayon::prelude::*;

type ParsedInput = (Grid, (usize, usize));

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed);
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed);
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    let grid = input.read_grid()?;

    for (c, pos) in grid.enumerate() {
        if *c == '^' {
            return Ok((grid, pos));
        }
    }
    Err(InvalidInput("Guard not found".to_owned()).into())
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let (grid, (x, y)) = input;

    let visited = walk_unobstructed(&grid, *x, *y);

    visited.len()
}

fn walk_unobstructed(grid: &Grid, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut guard = GuardState::new(x, y);

    visited.insert((x, y));

    while let Some(new_pos) = guard.step(grid.width(), grid.height()) {

        let cell_val = grid[(new_pos.posx, new_pos.posy)];

        if cell_val == '#' {
            guard = guard.turn();
            continue;
        }

        guard = new_pos;

        visited.insert((guard.posx, guard.posy));

    }

    visited
}

fn calculate_p2(input: &ParsedInput) -> usize {
    let (grid, (x, y)) = input;

    let mut base_path = walk_unobstructed(&grid, *x, *y);

    base_path.remove(&(*x, *y));

    let obstacles = base_path
        .par_iter()
        .map(|(obx, oby)|walk_detect_loop(&grid, *x, *y, *obx, *oby))
        .sum();

    obstacles
}

fn walk_detect_loop(grid: &Grid, x: usize, y: usize, obx: usize, oby: usize) -> usize {

    let mut visited: Array3<bool> = Array3::from_elem((grid.width(), grid.height(), 4).f(), false);

    let mut guard = GuardState::new(x, y);

    *visited.get_mut(guard.as_index()).unwrap() = true;

    while let Some(new_pos) = guard.step(grid.width(), grid.height()) {

        let cell_val = grid[(new_pos.posx, new_pos.posy)];

        if cell_val == '#' || (new_pos.posx == obx && new_pos.posy == oby) {
            guard = guard.turn();
            continue;
        }

        guard = new_pos;

        if *visited.get(guard.as_index()).unwrap() {
            return 1;
        }
        *visited.get_mut(guard.as_index()).unwrap() = true;
    }

    0
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn(&self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}


struct GuardState {
    posx: usize,
    posy: usize,
    dir: Direction,
}

impl GuardState {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            posx: x,
            posy: y,
            dir: Direction::Up,
        }
    }

    pub fn turn(&self) -> GuardState {
        Self {
            posx: self.posx,
            posy: self.posy,
            dir: self.dir.turn(),
        }
    }

    pub fn step(&self, width: usize, height: usize) -> Option<Self> {
        let newx = match self.dir {
            Direction::Right => self.posx.clamped_add_signed(1, width)?,
            Direction::Left => self.posx.clamped_add_signed(-1, width)?,
            _ => self.posx,
        };

        let newy = match self.dir {
            Direction::Down => self.posy.clamped_add_signed(1, height)?,
            Direction::Up => self.posy.clamped_add_signed(-1, height)?,
            _ => self.posy,
        };

        Some(Self {
            posx: newx,
            posy: newy,
            dir: self.dir,
        })
    }

    pub fn as_index(&self) -> (usize, usize, usize) {
        (self.posx, self.posy, self.dir as usize)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use aoc_tools::TestSamples;
    use aoc_tools::ResultExt;

    fn load_sample(filename: &str) -> anyhow::Result<(ParsedInput, Option<u64>, Option<u64>)> {
        let samples = TestSamples::try_new()?;
        let (input, expected1, expected2) = samples.get_sample(filename)?;
        let parsed = parse_input(input)?;
        Ok((parsed, expected1, expected2))
    }

    #[test]
    fn test_sample_p1() -> anyhow::Result<()> {
        let (parsed, expected, _) = load_sample("sample.txt")?;

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[test]
    fn test_locate_start() -> anyhow::Result<()> {
        let ((_, position), _, _) = load_sample("sample.txt")?;

        assert_eq!((4, 6), position);
        Ok(())
    }

    #[test]
    fn test_sample_p2() -> anyhow::Result<()> {
        let (parsed, _, expected) = load_sample("sample.txt")?;

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
