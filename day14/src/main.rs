use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

type ParsedInput = Vec<Robot>;

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed, 101, 103);
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed);
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {

    let robot_rx = Regex::new(r"p=(\d+),(\d+) v=(\-?\d+),(\-?\d+)").unwrap();

    let parsed =
        input.read_lines()?
        .into_iter()
        .map(|line|{
                let (_, [px, py, vx, vy]) = robot_rx.captures(&line).unwrap().extract();
                Robot {
                    px: px.parse().unwrap(),
                    py: py.parse().unwrap(),
                    vx: vx.parse().unwrap(),
                    vy: vy.parse().unwrap(),
                }
        })
        .collect_vec();

    Ok(parsed)
}

fn calculate_p1(input: &ParsedInput, width: i32, height: i32) -> u64 {
    let time = 100;

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for robot in input.into_iter() {
        let target_x = (((robot.px + robot.vx * time) % width) + width) % width;
        let target_y = (((robot.py + robot.vy * time) % height) + height) % height;

        if target_x < width / 2 {
            if target_y < height / 2 {
                q1 += 1;
            } else if target_y > height / 2 {
                q2 += 1;
            }
        } else if target_x > width / 2 {
            if target_y < height / 2 {
                q3 += 1;
            } else if target_y > height / 2 {
                q4 += 1;
            }
        }
    }

    q1 * q2 * q3 * q4
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
    #[case(load_sample("sample.txt")?, 11, 7)]
    #[case(load_sample("input.txt")?, 101, 103)]
    //#[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>), #[case] width: i32, #[case] height: i32) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed, width, height);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?, 11, 7)]
    #[case(load_sample("input.txt")?, 101, 103)]
    #[ignore]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>), #[case] width: i32, #[case] height: i32) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
