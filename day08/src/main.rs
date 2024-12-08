use aoc_tools::{NumExt};
use itertools::Itertools;
use std::collections::HashSet;

type ParsedInput = (Vec<(char, (usize, usize))>, (usize, usize));

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

    Ok((locations, (width, height)))
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let (locations, (width, height)) = input;

    let grouping = locations
        .into_iter()
        .sorted_by_key(|(k, _)| k)
        .chunk_by(|(k, _)| k);

    let loc_groups: Vec<_> = grouping
        .into_iter()
        .map(|(key, val)| (*key,
            val
                .map(|(_, v)| *v)
                .collect::<Vec<(usize, usize)>>()
        ))
        .collect();

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, loc) in loc_groups.into_iter() {
        let pairs: Vec<_> = loc.into_iter().combinations(2).collect();
        for ab in pairs.into_iter() {
            if let [(ax, ay), (bx, by)] = ab.as_slice() {

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
    }

    antinodes.len()
}

fn calc_ant(x: usize, y: usize, dx: isize, dy: isize, width: usize, height: usize) -> Option<(usize, usize)> {
    let antx = x.clamped_add_signed(dx, width)?;
    let anty = y.clamped_add_signed(dy, height)?;

    Some((antx, anty))
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
    #[ignore]
    fn test_sample_p2() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(1)?;

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, result2 as u64);
        Ok(())
    }
}
