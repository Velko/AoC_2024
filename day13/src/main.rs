use regex::Regex;
use std::cmp;
use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};

#[derive(Debug)]
struct Machine {
    speed_a: (usize, usize),
    speed_b: (usize, usize),
    prize: (usize, usize),
}

type ParsedInput = Vec<Machine>;

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
    let mut parsed = Vec::new();

    let button_rx = Regex::new(r"Button (.): X\+(\d+), Y\+(\d+)").unwrap();
    let prize_rx = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut speed_a: Option<(usize, usize)> = None;
    let mut speed_b: Option<(usize, usize)> = None;

    for (i, line) in lines.into_iter().enumerate() {
        match i % 4 {
            0 => {
                let (_, [b, x, y]) = button_rx.captures(&line).unwrap().extract();
                assert_eq!("A", b);
                speed_a = Some((
                    x.parse::<usize>().unwrap(),
                    y.parse::<usize>().unwrap(),
                ))
            },
            1 => {
                let (_, [b, x, y]) = button_rx.captures(&line).unwrap().extract();
                assert_eq!("B", b);
                speed_b = Some((
                    x.parse::<usize>().unwrap(),
                    y.parse::<usize>().unwrap(),
                ))
            },
            2 => {
                let (_, [x, y]) = prize_rx.captures(&line).unwrap().extract();

                parsed.push( Machine {
                    speed_a: speed_a.unwrap(),
                    speed_b: speed_b.unwrap(),
                    prize: (
                        x.parse::<usize>().unwrap(),
                        y.parse::<usize>().unwrap(),
                    ),
                });

                speed_a = None;
                speed_b = None;
            },
            3 => {
                assert_eq!("", line);
            },
            _ => unreachable!(),
        }
    }

    Ok(parsed)
}

fn calculate_p1(input: &ParsedInput) -> usize {

    input
        .into_iter()
        .map(|m| find_costs(m))
        .sum()
}


fn find_costs(machine: &Machine) -> usize {
    let (px, py) = machine.prize;
    let (sbx, sby) = machine.speed_b;
    let (sax, say) = machine.speed_a;

    let max_b = cmp::min(px / sbx, py / sby);
    let max_a = cmp::min(px / sax, py / say);

    for a in 0..(max_a+1) {
        for b in 0..(max_b+1) {
            let tx = a * sax + b * sbx;
            let ty = a * say + b * sby;

            if tx == px && ty == py {
                println!("{:?}, {:?}", a, b);
                return a * 3 + b;
            }
        }
    }

    0
}

fn calculate_p2(_input: &ParsedInput) -> u64 {
    0
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

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    //#[case(load_sample("input.txt")?)]
    #[ignore]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
