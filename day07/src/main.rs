use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};

type ParsedInput = Vec<(u64, Vec<u64>)>;

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
    let lines = input.read_lines()?;

    lines
        .into_iter()
        .map(|l| {
            let splat = l.split_once(':')
                .map_err_to_invalid_input(l.as_str())?;
            Ok((
                str::parse::<u64>(splat.0)
                    .map_err_to_invalid_input(splat.0)?,
                splat.1.split_ascii_whitespace()
                    .map(str::parse::<u64>)
                    .try_collect_vec()
                    .map_err_to_invalid_input(splat.1)?
            ))
        })
        .collect()
}

fn calculate_p1(input: &ParsedInput) -> u64 {
    input
        .into_iter()
        .filter(|(expected, args)| calc_exp_value_1(*expected, args))
        .map(|(expected, _)| expected)
        .sum()
}

fn calc_exp_value_1(expected: u64, args: &Vec<u64>) -> bool {

    (0..(1 << args.len()-1))
        .into_iter()
        .any(|pattern| {
            let (_, result) = args
                .into_iter()
                .cloned()
                .enumerate()
                .reduce(|(_, total), (idx, item)| {
                    if pattern & (1 << (idx - 1)) == 0 {
                        (idx, total + item)
                    } else {
                        (idx, total * item)
                    }
                })
                .unwrap();

            result == expected
        })
}

fn calculate_p2(input: &ParsedInput) -> u64 {
    input
        .into_iter()
        .filter(|(expected, args)| calc_exp_value_2(*expected, args))
        .map(|(expected, _)| expected)
        .sum()
}

fn calc_exp_value_2(expected: u64, args: &Vec<u64>) -> bool {

    (0..(1 << (args.len()-1) * 2))
        .into_iter()
        .any(|pattern| {
            let (_, result) = args
                .into_iter()
                .cloned()
                .enumerate()
                .reduce(|(_, total), (idx, item)| {
                    match (pattern & (3 << ((idx - 1)*2))) >> ((idx - 1)*2) {
                        0 | 1 => (idx, total + item),
                        2 => (idx, total * item),
                        3 => (idx, concat_numbers(total, item)),
                        _ => panic!(),
                    }
                })
                .unwrap();

            result == expected
        })
}

fn concat_numbers(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
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

        assert_eq!(expected, result1);
        Ok(())
    }

    #[test]
    fn test_sample_p2() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(1)?;

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, result2);
        Ok(())
    }
}
