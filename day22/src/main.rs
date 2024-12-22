use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};

type ParsedInput = Vec<u32>;

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

    let parsed = lines
        .into_iter()
        .map(|line| {
            line.parse()
                .map_err_to_invalid_input(&line)
        })
        .try_collect_vec()?;

    Ok(parsed)
}

fn calculate_p1(input: &ParsedInput) -> anyhow::Result<u64> {
    
    Ok(input
        .into_iter()
        .map(|seed|generate_secret(*seed))
        .sum()
    )
}

fn calculate_p2(_input: &ParsedInput) -> anyhow::Result<u64> {
    Ok(0)
}

fn generate_secret(seed: u32) -> u64 {
    let mut value:u64 = seed as u64;
    for _ in 0..2000 {
        value ^= value * 64;
        value %= 16777216;
        
        value ^= value / 32;
        value %= 16777216;
        
        value ^= value * 2048;
        value %= 16777216;
    }
    value
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
    #[case(load_sample("input.txt")?)]
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
