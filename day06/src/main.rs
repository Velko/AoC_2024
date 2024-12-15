use aoc_tools::{Direction, Grid, InvalidInput, Point, Rotation};
use ndarray::{Array3, ShapeBuilder};
use std::collections::HashSet;
use rayon::prelude::*;

type ParsedInput = (Grid<char>, Point);

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
    let (grid, pos) = input;

    let visited = walk_unobstructed(&grid, pos);

    visited.len()
}

fn walk_unobstructed(grid: &Grid<char>, pos: &Point) -> HashSet<Point> {
    let mut visited: HashSet<Point> = HashSet::new();

    let mut guard = GuardState::new(*pos);

    visited.insert(*pos);

    while let Some(new_pos) = guard.step(grid.size()) {

        let cell_val = grid[new_pos.pos];

        if cell_val == '#' {
            guard = guard.turn();
            continue;
        }

        guard = new_pos;

        visited.insert(guard.pos);

    }

    visited
}

fn calculate_p2(input: &ParsedInput) -> usize {
    let (grid, pos) = input;

    let mut base_path = walk_unobstructed(&grid, pos);

    base_path.remove(pos);

    let obstacles = base_path
        .par_iter()
        .map(|ob|walk_detect_loop(&grid, *pos, *ob))
        .sum();

    obstacles
}

fn walk_detect_loop(grid: &Grid<char>, pos: Point, ob: Point) -> usize {

    let (width, height) = grid.size();
    let mut visited: Array3<bool> = Array3::from_elem((width, height, 4).f(), false);

    let mut guard = GuardState::new(pos);

    *visited.get_mut(guard.as_index()).unwrap() = true;

    while let Some(new_pos) = guard.step(grid.size()) {

        let cell_val = grid[new_pos.pos];

        if cell_val == '#' || (new_pos.pos == ob) {
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

struct GuardState {
    pos: Point,
    dir: Direction,
}

impl GuardState {
    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            dir: Direction::Up,
        }
    }

    pub fn turn(&self) -> GuardState {
        Self {
            pos: self.pos,
            dir: self.dir.turn(Rotation::Clockwise),
        }
    }

    pub fn step(&self, bounds: (usize, usize)) -> Option<Self> {
        Some(Self {
            pos: self.pos.advance(self.dir, bounds)?,
            dir: self.dir,
        })
    }

    pub fn as_index(&self) -> (usize, usize, usize) {
        (self.pos.x, self.pos.y, self.dir as usize)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use aoc_tools::TestSamples;

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

        assert_eq!(Point { x: 4, y: 6}, position);
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
