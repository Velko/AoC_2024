use aoc_tools::{IterMoreTools, ResultExt};
use std::collections::HashMap;

type ParsedInput = Vec<u64>;

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
    let line = input.read_single_line()?;

    let parsed = line
        .split_ascii_whitespace()
        .map(str::parse::<u64>)
        .try_collect_vec()
        .map_err_to_invalid_input(line.as_str())?;

    Ok(parsed)
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let mut pebbles = input.clone();

    for _ in 0..25 {
        let mut new_row: Vec<u64> = Vec::with_capacity(pebbles.len() * 2);

        for p in pebbles.into_iter() {
            if p == 0 {
                new_row.push(1);
            } else if let Some((np1, np2)) = split_pebble(p) {
                new_row.push(np1);
                new_row.push(np2);
            } else {
                new_row.push(p * 2024);
            }
        }

        pebbles = new_row;
    }

    pebbles.len()
}

fn split_pebble(p: u64) -> Option<(u64, u64)> {
    let s = format!("{}", p);
    let l = s.len();

    if (l & 1) == 0 {
        let p1 = s[..l/2].parse().unwrap();
        let p2 = s[l/2..].parse().unwrap();

        Some((p1, p2))
    } else {
        None
    }
}

fn calculate_p2(input: &ParsedInput) -> usize {

    let mut pebbles: HashMap<u64, usize> = input
        .into_iter()
        .map(|p| (*p, 1))
        .collect();

    for _ in 0..75 {
        let mut new_row: HashMap<u64, usize> = HashMap::new();

        for (p, count) in pebbles.into_iter() {
            if p == 0 {
                *new_row.entry(1).or_insert(0) += count;
            } else if let Some((np1, np2)) = split_pebble(p) {
                *new_row.entry(np1).or_insert(0) += count;
                *new_row.entry(np2).or_insert(0) += count;
            } else {
                let np = p * 2024;
                *new_row.entry(np).or_insert(0) += count;
            }
        }

        pebbles = new_row;
    }

    pebbles
        .into_iter()
        .map(|(_, count)|count)
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;
    use aoc_tools::TestSamples;

    fn load_sample(index: usize) -> anyhow::Result<(ParsedInput, Option<u64>, Option<u64>)> {
        let samples = TestSamples::try_new()?;
        let (input, expected1, expected2) = samples.get_sample(index)?;
        let parsed = parse_input(input)?;
        Ok((parsed, expected1, expected2))
    }

    #[rstest]
    #[case(load_sample(0)?)]
    #[case(load_sample(1)?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample(0)?)]
    #[case(load_sample(1)?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
