use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use std::iter::repeat_n;

type ParsedInput = Vec<(Option<usize>, usize)>;

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
    let in_str = input.read_single_line()?;

    // 2333133121414131402
    let mut in_c = in_str.chars().into_iter();
    let mut file_id = 0;

    let mut file_desc: Vec<(Option<usize>, usize)> = Vec::new();

    while let Some(file_len) = in_c.next() {
        file_desc.push((Some(file_id), parse_char(file_len)));

        file_id += 1;
        if let Some(space_len) = in_c.next() {
            file_desc.push((None, parse_char(space_len)));
        } else {
            break;
        }
    }

    Ok(file_desc)
}

fn parse_char(c: char) -> usize {
    format!("{}", c).parse().unwrap()
}

fn calculate_p1(input: &ParsedInput) -> u64 {
    let mut disk_map = expand_disk_map(input);

    let mut start_idx = 0;
    let mut end_idx = disk_map.len() - 1;

    while start_idx < end_idx {
        while disk_map.get(start_idx).unwrap().is_some() {
            start_idx += 1;
        }

        while disk_map.get(end_idx).unwrap().is_none() {
            end_idx -= 1;
        }

        *disk_map.get_mut(start_idx).unwrap()
            = *disk_map.get(end_idx).unwrap();
        *disk_map.get_mut(end_idx).unwrap() = None;

        start_idx += 1;
        end_idx -= 1;
    }

    calculate_disk_checksum(&disk_map)
}

fn expand_disk_map(input: &ParsedInput) -> Vec<Option<usize>> {
    let mut disk_map: Vec<Option<usize>> = Vec::new();

    for (val, count) in input.into_iter() {
        disk_map.extend(repeat_n(val, *count));
    }

    disk_map
}

fn calculate_disk_checksum(disk_map: &Vec<Option<usize>>) -> u64 {
    disk_map
        .into_iter()
        .filter_map(|f|*f)
        .enumerate()
        .map(|(pos, file_id)| pos as u64 * file_id as u64)
        .sum()
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
