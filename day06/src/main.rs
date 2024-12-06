use aoc_tools::{InvalidInput};
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

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

    for (rownum, row) in grid.iter().enumerate() {
        if let Some(colnum) = row.iter().position(|c| *c == '^') {
            return Ok((grid, (colnum, rownum)));
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

    let height = grid.len();
    let width = grid.get(0).unwrap().len();

    let mut guard = GuardState::new(x, y);

    visited.insert((x, y));

    while let Some(new_pos) = guard.step(width, height) {

        let cell_val = grid.get(new_pos.posy).unwrap().get(new_pos.posx).unwrap();

        if *cell_val == '#' {
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

    let mut obstacles = 0;

    for (obx, oby) in base_path.iter() {
        let mut check_grid = grid.clone();
        *check_grid.get_mut(*oby).unwrap().get_mut(*obx).unwrap() = 'O';

        if walk_detect_loop(&check_grid, *x, *y) {
            obstacles += 1;
        }
    }

    obstacles
}

fn walk_detect_loop(grid: &Grid, x: usize, y: usize) -> bool {
    let mut visited: HashSet<GuardState> = HashSet::new();

    let height = grid.len();
    let width = grid.get(0).unwrap().len();

    let mut guard = GuardState::new(x, y);

    visited.insert(guard);

    while let Some(new_pos) = guard.step(width, height) {

        let cell_val = grid.get(new_pos.posy).unwrap().get(new_pos.posx).unwrap();

        if *cell_val == '#' || *cell_val == 'O' {
            guard = guard.turn();
            continue;
        }

        guard = new_pos;

        if visited.contains(&guard) {
            return true;
        }
        visited.insert(guard);
    }

    false
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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


#[derive(Eq, Hash, PartialEq, Clone, Copy)]
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
            Direction::Right => self.posx + 1,
            Direction::Left => self.posx.checked_add_signed(-1)?,
            _ => self.posx,
        };

        if newx >= width { return None; }

        let newy = match self.dir {
            Direction::Down => self.posy + 1,
            Direction::Up => self.posy.checked_add_signed(-1)?,
            _ => self.posy,
        };

        if newy >= height { return None; }

        Some(Self {
            posx: newx,
            posy: newy,
            dir: self.dir,
        })
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use aoc_tools::TestSamples;

    fn load_sample(index: usize) -> anyhow::Result<(ParsedInput, u64)> {
        let samples = TestSamples::try_new()?;
        let (input, expected) = samples.get_sample(index)?;
        let parsed = parse_input(input)?;
        Ok((parsed, expected))
    }

    #[test]
    fn test_sample_p1() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(0)?;

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, result1 as u64);
        Ok(())
    }

    #[test]
    fn test_locate_start() -> anyhow::Result<()> {
        let ((_, position), _) = load_sample(0)?;

        assert_eq!((4, 6), position);
        Ok(())
    }

    #[test]
    fn test_sample_p2() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(1)?;

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, result2 as u64);
        Ok(())
    }
}