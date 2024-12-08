use aoc_tools::{IterMoreTools, ResultExt};
use rayon::prelude::*;

type ParsedInput = Vec<(u64, Vec<u64>)>;

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed);
    println!("Result p1: {}", result1);

    let result2 = calculate_p2_v2(&parsed);
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

fn calculate_p2_v2(input: &ParsedInput) -> u64 {
    input
        .into_par_iter()
        .filter(|(expected, args)| calc_exp_value_2_v2(*expected, args))
        .map(|(expected, _)| expected)
        .sum()
}

fn calc_exp_value_2_v2(expected: u64, args: &Vec<u64>) -> bool {

    let mut args = args.into_iter().cloned();

    if let Some(value) = args.next() {
        let m = apply_op_and_check(expected, value, args);
        m
    } else {
        false
    }
}

fn apply_op_and_check<I>(expected: u64, calculated: u64, mut args: I) -> bool
    where I: Iterator<Item = u64> + Clone
{
    if calculated > expected {
        return false;
    }

    if let Some(arg) = args.next() {
        let add_res = calculated + arg;
        if apply_op_and_check(expected, add_res, args.clone()) {
            return true;
        }

        let mul_res = calculated * arg;
        if apply_op_and_check(expected, mul_res, args.clone()) {
            return true;
        }

        let concat_res = concat_numbers(calculated, arg);
        return apply_op_and_check(expected, concat_res, args.clone());
    } else {
        return expected == calculated;
    }
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
    fn test_sample_p2_v2() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(1)?;

        let result2 = calculate_p2_v2(&parsed);

        assert_eq!(expected, result2);
        Ok(())
    }
}
