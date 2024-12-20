use aoc_tools::{Grid, IterMoreTools, Neighbours2D, ResultExt};
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

    //draw_picture_p2(&parsed, 101, 103);

    let result1 = calculate_p1(&parsed, 101, 103)?;
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed, 101, 103)?;
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {

    let robot_rx = Regex::new(r"p=(\d+),(\d+) v=(\-?\d+),(\-?\d+)").unwrap();

    input.read_lines()?
    .into_iter()
    .map(|line|{
            let (_, [px, py, vx, vy]) = robot_rx.captures(&line)
                .map_err_to_invalid_input(&line)?
                .extract();
            Ok(Robot {
                px: px.parse().map_err_to_invalid_input(px)?,
                py: py.parse().map_err_to_invalid_input(py)?,
                vx: vx.parse().map_err_to_invalid_input(vx)?,
                vy: vy.parse().map_err_to_invalid_input(vy)?,
            })
    })
    .try_collect_vec()

}

fn calculate_p1(input: &ParsedInput, width: usize, height: usize) -> anyhow::Result<u64> {
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

    Ok(q1 * q2 * q3 * q4)
}

impl Robot {
    fn position_after(&self, time: usize, width: usize, height: usize) -> (usize, usize) {
        let width = width as i32;
        let height = height as i32;
        let time = time as i32;
        (
            ((((self.px + self.vx * time) % width) + width) % width) as usize,
            ((((self.py + self.vy * time) % height) + height) % height) as usize,
        )
    }
}

fn calculate_p2(input: &ParsedInput, width: usize, height: usize) -> anyhow::Result<usize> {
    for time in 0..(width * height) {
        let mut positions: Grid<bool> = Grid::new(false, width, height);
        
        for robot in input.iter() {
            positions[robot.position_after(time, width, height)] = true;
        }

        let mut nmatches = 0;
        for (v, pos) in positions.enumerate() {
            if *v {
                for neigh in Neighbours2D::new_only_valid(pos.into(), (width as usize, height as usize), aoc_tools::NeighbourMap::Plus) {
                    if positions[neigh] {
                        nmatches += 1;
                    }
                }
            }
        }

        if nmatches > input.len() {
            print_map_after(input, time, width, height);
            return Ok(time);
        }
    }

    Err(anyhow::anyhow!("Failed to calculate result"))
}

fn draw_picture_p2(input: &ParsedInput, width: usize, height: usize) {

    /* Generate a huge 10201x10609 image (101^2 x 103^2), where every possible
       101x103 image is arranged in 101x103 grid.

       Open the image in a viewer that allows to zoom in and shows current coordinates.

       Then:
            col = x / 101
            row = y / 103
            answer = row * 101 + col
     */

    let mut picture: Vec<Vec<FormattedCell>> = 
        (0..(height * height))
            .into_iter()
            .map(|_| vec![FormattedCell('0'); width * width])
            .collect();
    for row in 0..height {
        for col in 0..width {
            let time = col + row * width;

            for robot in input.iter() {
                let (rx, ry) = robot.position_after(time, width, height);

                let px = col * width + rx;
                let py = row * height + ry;

                *picture.get_mut(py).unwrap().get_mut(px).unwrap() = FormattedCell('1');
            }
        }
    }

    println!("P1");
    println!("{} {}", width * width, height * height);

    for row in picture.into_iter() {
        let line: String = row
                .into_iter()
                .map(|v|v.to_string())
                .collect();
            println!("{}", line);
    }
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


#[derive(Default, Clone, Copy)]
struct FormattedCell(char);

impl std::fmt::Display for FormattedCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.0)
    }
}


fn print_map_after(robots: &ParsedInput, time: usize, width: usize, height: usize) {
    // output the grid in BPM format
    // it consists of "magic" P1
    // then size of the image
    // and finally a grid of 1 and 0 (separated by spaces)

    let mut map: Grid<FormattedCell> = Grid::new(FormattedCell('0'), width, height);

    for robot in robots.iter() {
        let pos = robot.position_after(time, width, height);
        map[pos] = FormattedCell('1');
    }

    println!("P1");
    println!("{} {}", width, height);

    map.print();
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
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>), #[case] width: usize, #[case] height: usize) -> anyhow::Result<()> {
        let result1 = calculate_p1(&parsed, width, height)?;

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("input.txt")?, 101, 103)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>), #[case] width: usize, #[case] height: usize) -> anyhow::Result<()> {
        let result2 = calculate_p2(&parsed, width, height)?;

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
