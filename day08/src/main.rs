use aoc_tools::{NumExt};
use itertools::Itertools;
use std::collections::HashSet;

type ParsedInput = (Vec<(char, Vec<(usize, usize)>)>, (usize, usize));

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

    let height = grid.len();
    let width = grid.get(0).unwrap().len();

    let mut locations: Vec<(char, (usize, usize))> = Vec::new();

    for (y, row) in grid.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if cell != '.' {
                locations.push((cell, (x, y)));
            }
        }
    }

    let grouping = locations
        .into_iter()
        .sorted_by_key(|(k, _)| *k)
        .chunk_by(|(k, _)| *k);

    let loc_groups: Vec<_> = grouping
        .into_iter()
        .map(|(key, val)| (key,
            val
                .map(|(_, v)| v)
                .collect::<Vec<(usize, usize)>>()
        ))
        .collect();

    Ok((loc_groups, (width, height)))
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let (loc_groups, (width, height)) = input;

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, loc) in loc_groups.into_iter() {
        let pairs = loc.into_iter().tuple_combinations::<(_, _)>();
        for ((ax, ay), (bx, by)) in pairs {

            let diffx = *bx as isize - *ax as isize;
            let diffy = *by as isize - *ay as isize;

            if let Some(anta) = calc_ant(*ax, *ay, -diffx, -diffy, *width, *height) {
                antinodes.insert(anta);
            }

            if let Some(antb) = calc_ant(*bx, *by, diffx, diffy, *width, *height) {
                antinodes.insert(antb);
            }
        }
    }

    antinodes.len()
}

fn calc_ant(x: usize, y: usize, dx: isize, dy: isize, width: usize, height: usize) -> Option<(usize, usize)> {
    let antx = x.clamped_add_signed(dx, width)?;
    let anty = y.clamped_add_signed(dy, height)?;

    Some((antx, anty))
}


fn calculate_p2(input: &ParsedInput) -> usize {
    let (loc_groups, (width, height)) = input;

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, loc) in loc_groups.into_iter() {
        let pairs = loc.into_iter().tuple_combinations::<(_, _)>();
        for ((ax, ay), (bx, by)) in pairs {
            let diffx = *bx as isize - *ax as isize;
            let diffy = *by as isize - *ay as isize;

            let mut multiplier = 0;
            while let Some(anta) = calc_ant(*ax, *ay, -diffx * multiplier, -diffy * multiplier, *width, *height) {
                antinodes.insert(anta);
                multiplier += 1;
            }

            multiplier = 0;
            while let Some(antb) = calc_ant(*bx, *by, diffx * multiplier, diffy * multiplier, *width, *height) {
                antinodes.insert(antb);
                multiplier += 1;
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_tools::TestSamples;
    use aoc_tools::ResultExt;

    fn load_sample(index: usize) -> anyhow::Result<(ParsedInput, u64)> {
        let samples = TestSamples::try_new()?;
        let (input, expected, _) = samples.get_sample(index)?;
        let parsed = parse_input(input)?;
        Ok((parsed, expected.map_err_to_invalid_input("Expected value missing")?))
    }

    #[test]
    fn test_sample_p1() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(0)?;

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, result1 as u64);
        Ok(())
    }

    #[test]
    fn test_sample_2_p2() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(2)?;

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, result2 as u64);
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
