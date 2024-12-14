use itertools::{Itertools};
use regex::Regex;

#[derive(Debug, Clone)]
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

    let result2 = calculate_p2(&parsed, 101, 103);
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
        let (target_x, target_y) = robot.position_after(time, width, height);

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

impl Robot {
    fn position_after(&self, time: i32, width: i32, height: i32) -> (i32, i32) {
        (
            (((self.px + self.vx * time) % width) + width) % width,
            (((self.py + self.vy * time) % height) + height) % height,
        )
    }
}

fn is_next_to((p1x, p1y): (i32, i32), (p2x, p2y): (i32, i32)) -> bool {
    let dx = (p2x - p1x).abs();
    let dy = (p2y - p1y).abs();

    (dx == 1 && dy == 0) || (dx == 0 && dy == 1)
}

fn calculate_p2(input: &ParsedInput, width: i32, height: i32) -> i32 {
    for time in 0..(width * height) {
        let mut nmatches = 0;
        let pairs = input.iter().tuple_combinations::<(_, _)>();
        for (r1, r2) in pairs {
            let loc1 = r1.position_after(time, width, height);
            let loc2 = r2.position_after(time, width, height);

            if is_next_to(loc1, loc2) {
                nmatches += 1;
            }
        }

        if nmatches > input.len() {
            print_map_after(input, time, width, height);
            return time;
        }
    }

    0
}

// fn cycle_length(robot: &Robot, width: i32, height: i32) -> usize {
//     let mut r1px = robot.px;
//     let mut r1py = robot.py;

//     for t in 1..usize::MAX {
        
//         r1px = (r1px + width + robot.vx) % width;
//         r1py = (r1py + height + robot.vy) % height;

//         if r1px == robot.px && r1py == robot.py {
//             return t;
//         }
//     }

//     usize::MAX
// }

fn print_map_after(robots: &ParsedInput, time: i32, width: i32, height: i32) {
    let mut map: Vec<Vec<char>> = 
        (0..height)
            .into_iter()
            .map(|_| {
                (0..width)
                    .map(|_| '.')
                    .collect()
            })
            .collect();

    for robot in robots.iter() {
        let (x, y) = robot.position_after(time, width, height);
        *map.get_mut(y as usize).unwrap().get_mut(x as usize).unwrap() = '#';
    }

    for row in map.iter() {
        let line: String = row.into_iter().collect();
        println!("{}", line);
    }
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
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>), #[case] width: i32, #[case] height: i32) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed, width, height);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("input.txt")?, 101, 103)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>), #[case] width: i32, #[case] height: i32) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed, width, height);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
