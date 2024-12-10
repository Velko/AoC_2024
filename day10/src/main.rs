use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use aoc_tools::NumExt;
use std::collections::HashSet;

type ParsedInput = Vec<Vec<char>>;

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
    Ok(input.read_grid()?)
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let height = input.len();
    let width = input.get(0).unwrap().len();

    let mut starts: Vec<(usize, usize)> = Vec::new();

    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if *cell == '0' {
                starts.push((x, y));
            }
        }
    }

    //println!("{:?}", starts);


    let mut total = 0;

    for pos in starts.into_iter() {
        let mut reached: HashSet<(usize, usize)> = HashSet::new();
        walk_path_1(&input, pos, width, height, '0', &mut reached);

        total += reached.len()
    }

    total
}


fn walk_path_1(grid: &ParsedInput, (posx, posy): (usize, usize), width: usize, height: usize, current_step: char, reached: &mut HashSet<(usize, usize)>) {

    //print!("{:?}, {}", (posx, posy), current_step);

    if *grid.get(posy).unwrap().get(posx).unwrap() != current_step {
        //println!("Exit");
        return;
    }

    if current_step == '9' {
        //println!("Score");
        reached.insert((posx, posy));
        return;
    }

    //println!();

    let next_step = (current_step as u8 + 1) as char;

    if let Some(next_x) = posx.clamped_add_signed(1, width) {
        walk_path_1(grid, (next_x, posy), width, height, next_step, reached);
    }
    if let Some(next_x) = posx.clamped_add_signed(-1, width) {
        walk_path_1(grid, (next_x, posy), width, height, next_step, reached);
    }
    if let Some(next_y) = posy.clamped_add_signed(1, height) {
        walk_path_1(grid, (posx, next_y), width, height, next_step, reached);
    }
    if let Some(next_y) = posy.clamped_add_signed(-1, height) {
        walk_path_1(grid, (posx, next_y), width, height, next_step, reached);
    }
}



fn calculate_p2(_input: &ParsedInput) -> u64 {
    0
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
    fn test_sample_1_p1() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(2)?;

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, result1 as u64);
        Ok(())
    }

    #[test]
    fn test_sample_2_p1() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(3)?;

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, result1 as u64);
        Ok(())
    }


    #[test]
    #[ignore]
    fn test_sample_p2() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(1)?;

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, result2 as u64);
        Ok(())
    }
}
