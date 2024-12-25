use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use anyhow::anyhow;

type ParsedInput = (Vec<[usize; 5]>, Vec<[usize; 5]>);

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed)?;
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed)?;
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    let lines = input.read_lines()?;

    let mut locks: Vec<[usize; 5]> = Vec::new();
    let mut keys: Vec<[usize; 5]> = Vec::new();


    for block in lines.as_slice().split(|l| l == "") {
        let b_type = block[0].chars().next().unwrap();
        let mut counts = [0; 5];
        for b_line in &block[1..block.len()-1] {
            for (i, c) in b_line.chars().enumerate() {
                if c == '#' {
                    counts[i] += 1;
                }
            }
        }

        if b_type == '#' {
            locks.push(counts);
        } else {
            keys.push(counts);
        }
    }

    Ok((locks, keys))
}

fn calculate_p1(input: &ParsedInput) -> anyhow::Result<usize> {
    let (locks, keys) = input;

    println!("L: {:?}\nK: {:?}", locks, keys);

    let mut matches = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            println!("{:?} vs {:?}", lock, key);
            if lock.iter().zip(key.iter()).all(|(l, k)| l + k < 6) {
                matches += 1;
            }
        }
    }

    Ok(matches)
}

fn calculate_p2(_input: &ParsedInput) -> anyhow::Result<u64> {
    Err(anyhow!("Not implemented"))
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
    //#[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed)?;

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    //#[case(load_sample("input.txt")?)]
    #[ignore]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed)?;

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
