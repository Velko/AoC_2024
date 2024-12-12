use aoc_tools::{Grid, NumExt};
use std::collections::HashSet;

type ParsedInput = (Grid<char>, Vec<(usize, usize)>);

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

    let mut starts: Vec<(usize, usize)> = Vec::new();

    for (cell, (x, y)) in grid.enumerate() {
        if *cell == '0' {
            starts.push((x, y));
        }
    }

    Ok((grid, starts))
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let (grid, starts) = input;

    let mut total = 0;

    for pos in starts.into_iter() {
        let mut reached: HashSet<(usize, usize)> = HashSet::new();
        walk_path(&grid, *pos, '0', &mut reached);

        total += reached.len()
    }

    total
}


fn walk_path(grid: &Grid<char>, (posx, posy): (usize, usize), current_step: char, reached: &mut HashSet<(usize, usize)>) -> usize {

    if grid[(posx, posy)] != current_step {
        return 0;
    }

    if current_step == '9' {
        reached.insert((posx, posy));
        return 1;
    }

    let mut score = 0;

    let next_step = (current_step as u8 + 1) as char;

    if let Some(next_x) = posx.clamped_add_signed(1, grid.width()) {
        score += walk_path(grid, (next_x, posy), next_step, reached);
    }
    if let Some(next_x) = posx.clamped_add_signed(-1, grid.width()) {
        score += walk_path(grid, (next_x, posy), next_step, reached);
    }
    if let Some(next_y) = posy.clamped_add_signed(1, grid.height()) {
        score += walk_path(grid, (posx, next_y), next_step, reached);
    }
    if let Some(next_y) = posy.clamped_add_signed(-1, grid.height()) {
        score += walk_path(grid, (posx, next_y), next_step, reached);
    }

    score
}



fn calculate_p2(input: &ParsedInput) -> usize {
    let (grid, starts) = input;

    let mut reached: HashSet<(usize, usize)> = HashSet::new();

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
