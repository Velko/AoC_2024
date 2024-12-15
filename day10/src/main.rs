use aoc_tools::{Direction, Grid, Point};
use std::collections::HashSet;

type ParsedInput = (Grid<char>, Vec<Point>);

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

    let mut starts: Vec<Point> = Vec::new();

    for (cell, pos) in grid.enumerate() {
        if *cell == '0' {
            starts.push(pos);
        }
    }

    Ok((grid, starts))
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let (grid, starts) = input;

    let mut total = 0;

    for pos in starts.into_iter() {
        let mut reached: HashSet<Point> = HashSet::new();
        walk_path(&grid, *pos, '0', &mut reached);

        total += reached.len()
    }

    total
}


fn walk_path(grid: &Grid<char>, pos: Point, current_step: char, reached: &mut HashSet<Point>) -> usize {

    if grid[pos] != current_step {
        return 0;
    }

    if current_step == '9' {
        reached.insert(pos);
        return 1;
    }

    let mut score = 0;

    let next_step = (current_step as u8 + 1) as char;

    for dir in Direction::all() {
        if let Some(next) = pos.advance(dir, grid.size()) {
            score += walk_path(grid, next, next_step, reached);
        }
    }

    score
}



fn calculate_p2(input: &ParsedInput) -> usize {
    let (grid, starts) = input;

    let mut reached: HashSet<Point> = HashSet::new();

    starts
        .into_iter()
        .map(|pos| walk_path(&grid, *pos, '0', &mut reached))
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;
    use aoc_tools::TestSamples;

    fn load_sample(filename: &str) -> anyhow::Result<(ParsedInput, Option<u64>, Option<u64>)> {
        let samples = TestSamples::try_new()?;
        let (input, expected1, expected2) = samples.get_sample(filename)?;
        let parsed = parse_input(input)?;
        Ok((parsed, expected1, expected2))
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    #[case(load_sample("sample_1.txt")?)]
    #[case(load_sample("sample_2.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
