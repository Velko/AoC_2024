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
    // Wrong Result p2: 1847, 2120

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
    let (g, (x, y)) = input;
    let mut grid = g.clone();

    let height = grid.len();
    let width = grid.get(0).unwrap().len();

    let mut guard = GuardState::new(*x, *y);

    let mut steps: usize = 0;

    while let Some(new_pos) = guard.step(width, height) {

        let cell_val = grid.get(new_pos.posy).unwrap().get(new_pos.posx).unwrap();

        if *cell_val == '#' {
            guard = guard.turn();
            continue;
        }

        guard = new_pos;

        if *cell_val != 'X' {
            steps += 1;
            *grid.get_mut(guard.posy).unwrap().get_mut(guard.posx).unwrap() = 'X';
        }
    }

    steps
}

type VisitedGrid = Vec<Vec<HashSet<Direction>>>;

fn calculate_p2(input: &ParsedInput) -> usize {
    let (grid, (x, y)) = input;

    let height = grid.len();
    let width = grid.get(0).unwrap().len();

    let mut visited: VisitedGrid =
        (0..height)
            .map(|_| (0..width)
                .map(|_| HashSet::new())
                .collect())
            .collect();

    let mut guard = GuardState::new(*x, *y);

    let mut num_obstacles: usize = 0;

    if check_if_obstacle_makes_loop(&guard, &visited, &grid) {
        num_obstacles += 1;
    }

    while let Some(new_pos) = guard.step(width, height) {

        let cell_val = grid.get(new_pos.posy).unwrap().get(new_pos.posx).unwrap();

        if *cell_val == '#' {
            guard = guard.turn();
            continue;
        }

        guard = new_pos;

        visited.get_mut(guard.posy).unwrap().get_mut(guard.posx).unwrap().insert(guard.dir);

        //print_grid(&grid);



        if check_if_obstacle_makes_loop(&guard, &visited, &grid) {
            num_obstacles += 1;
        }
    }

    num_obstacles
}

fn check_if_obstacle_makes_loop(initial_state: &GuardState, visited_template: &VisitedGrid, grid: &Grid) -> bool {
    let height = grid.len();
    let width = grid.get(0).unwrap().len();

    let mut visited = visited_template.clone();
    let mut what_if_turned = initial_state.turn();

    while let Some(nextstep) = what_if_turned.step(width, height) {

        if *grid.get(nextstep.posy).unwrap().get(nextstep.posx).unwrap() == '#' {
            what_if_turned = what_if_turned.turn();
            continue;
        }

        what_if_turned = nextstep;

        let next_val = visited.get(what_if_turned.posy).unwrap().get(what_if_turned.posx).unwrap();
        if next_val.contains(&what_if_turned.dir) {
            //println!("Loop if turned");
            return true;
        }
        visited.get_mut(what_if_turned.posy).unwrap().get_mut(what_if_turned.posx).unwrap().insert(what_if_turned.dir);
    }
    false
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for r in grid.iter() {
        let line: String = r.into_iter().collect();
        println!("{}", line);
    }

    println!();
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
