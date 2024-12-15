use std::{self, io::{self, BufRead}};

use aoc_tools::{Grid, InvalidInput, IterMoreTools, NeighbourMap, Neighbours2D, ResultExt};

type ParsedInput = (Grid<char>, String);

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
    let reader = input.open_file()?;

    let grid_srs = reader
            .lines()
            .take_while(|f| f.is_ok() && f.as_ref().unwrap() != "");

    let grid = Grid::try_from_lines(grid_srs)?;

    let reader = input.open_file()?;

    let commands_srs = reader
            .lines()
            .skip_while(|f| f.is_ok() && f.as_ref().unwrap() != "")
            .skip(1);

    let commands: io::Result<String> = 
        commands_srs.collect();



    Ok((grid, commands?))
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let (grid, commands) = input;
    let mut grid = grid.clone();


    let mut start: Option<(usize, usize)> = None;

    for (c, pos) in grid.enumerate() {
        if *c == '@' {
            start = Some(pos);
            break;
        }
    }

    //println!("{:?}", start);


    grid[start.unwrap()] = '.';


    // grid.print();
    // println!("{}", commands);


    let mut rpos = start.unwrap();

    for cmd in commands.chars() {
        let dir = match cmd {
            '^' => NeighbourMap::Top,
            '>' => NeighbourMap::Right,
            'v' => NeighbourMap::Bottom,
            '<' => NeighbourMap::Left,
            _ => panic!("Unexpected command")
        };

        let mut points: Vec<(usize, usize)> = Vec::new();

        for dist in 1..usize::MAX {
            let np = Neighbours2D::new_with_distance(rpos, grid.size(), dist, dir).filter_map(|f|f).next();

            if let Some(point) = np {
                if grid[point] != '#' {
                    points.push(point);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        //print!("{:?}->", points);

        if points.is_empty() {
            continue; // at the wall already
        }

        // for p in points.iter() {
        //     print!("{}", grid[*p]);
        // }

        let first_empty = points.iter().position(|s| grid[*s] == '.');
        //println!("{:?}", first_empty);


        if let Some(p_empty) = first_empty {
            grid[*points.get(p_empty).unwrap()] = 'O';
            
            rpos = *points.get(0).unwrap();
            grid[rpos] = '.';
        } else {
            continue; // no spaces between robot and wall
        }
    }

    //grid.print();

    grid
        .enumerate()
        .filter(|(o, _)| **o == 'O')
        .map(|(_, (x, y))| y * 100 + x)
        .sum()
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
    #[case(load_sample("input.txt")?)]
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
