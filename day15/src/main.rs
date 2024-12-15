use std::{self, collections::HashSet, io::{self, BufRead}};

use aoc_tools::{Grid, NeighbourMap, Neighbours2D};

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

    grid[start.unwrap()] = '.';

    let mut rpos = start.unwrap();

    for cmd in commands.chars() {
        let dir = command_to_direction(cmd);

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

        if points.is_empty() {
            continue; // at the wall already
        }

        let first_empty = points.iter().position(|s| grid[*s] == '.');

        if let Some(p_empty) = first_empty {
            grid[*points.get(p_empty).unwrap()] = 'O';
            
            rpos = *points.get(0).unwrap();
            grid[rpos] = '.';
        } else {
            continue; // no spaces between robot and wall
        }
    }

    grid
        .enumerate()
        .filter(|(o, _)| **o == 'O')
        .map(|(_, (x, y))| y * 100 + x)
        .sum()
}

fn command_to_direction(cmd: char) -> NeighbourMap {
    match cmd {
        '^' => NeighbourMap::Top,
        '>' => NeighbourMap::Right,
        'v' => NeighbourMap::Bottom,
        '<' => NeighbourMap::Left,
        _ => panic!("Unexpected command")
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct WhBox([(usize, usize); 2]);

impl WhBox {
    fn is_located_here(&self, pos: (usize, usize)) -> bool {
        self.0[0] == pos || self.0[1] == pos
    }
}


fn calculate_p2(input: &ParsedInput) -> usize {

    let (orig_grid, commands) = input;
    let mut grid: Grid<char>  = Grid::new('.', orig_grid.width() * 2, orig_grid.height());

    let mut all_boxes: Vec<WhBox> = Vec::new();
    let mut start: Option<(usize, usize)> = None;

    for (c, (x, y)) in orig_grid.enumerate() {
        match c {
            '#' => {
                grid[(x * 2, y)] = '#';
                grid[(x * 2 + 1, y)] = '#';
            },
            'O' => {
                grid[(x * 2, y)] = '[';
                grid[(x * 2 + 1, y)] = ']';
                all_boxes.push(WhBox([(x * 2, y), (x * 2 + 1, y)]));
            },
            '@' => {
                start = Some((x * 2, y));
            }
            _ => {},
        }
    }

    let mut rpos = start.unwrap();

    for cmd in commands.chars() {
        let dir = command_to_direction(cmd);

        let new_pos = move_to(rpos, grid.size(), dir);

        match grid[new_pos] {
            '[' | ']' => {
                // a box, collect boxes in a way and decide if can move
                let mut boxes_to_move: HashSet<WhBox> = HashSet::new();
                if collect_boxes(&mut boxes_to_move, new_pos, dir, &all_boxes, &grid) {
                    move_boxes(&boxes_to_move, dir, &mut all_boxes, &mut grid);
                    rpos = new_pos;
                }
            },
            '.' => {
                // free space, just move
                rpos = new_pos;
            },
            '#' => {
                // wall, don't move
            },
            _ => panic!("Something unexpected on the map"),
        }
    }

    grid
        .enumerate()
        .filter(|(o, _)| **o == '[')
        .map(|(_, (x, y))| y * 100 + x)
        .sum()
}

fn move_to(pos: (usize, usize), bounds: (usize, usize), dir: NeighbourMap) -> (usize, usize) {
    Neighbours2D::new(pos, bounds, dir).filter_map(|f|f).next().unwrap()
}

fn collect_boxes(boxes_to_move: &mut HashSet<WhBox>, pos: (usize, usize), dir: NeighbourMap, all_boxes: &[WhBox], grid: &Grid<char>) -> bool {

    let b_to_m = all_boxes.iter().find(|p|p.is_located_here(pos)).unwrap();  // we have already checked in grid if something is here

    let inserted = boxes_to_move.insert(*b_to_m);

    if inserted {
        for box_pos in b_to_m.0.iter() {
            let new_pos1 = move_to(*box_pos, grid.size(), dir);

            match grid[new_pos1] {
                '#' => {
                    // a wall, abandon whole thing
                    return false;
                },
                '.' => {
                    // free space, good to go
                },
                '[' | ']' => {
                    // another box
                    let can_move = collect_boxes(boxes_to_move, new_pos1, dir, all_boxes, grid);
                    if !can_move {
                        // another box down the line can not move
                        return false;
                    }
                },
                _ => panic!("Something unexpected on the map"),
            }
        }
    }

    // got this far without early returns, wer're good to go
    true
}

fn move_boxes(boxes_to_move: &HashSet<WhBox>, dir: NeighbourMap, all_boxes: &mut [WhBox], grid: &mut Grid<char>) {

    // erase old boxes
    for b_to_m in boxes_to_move.iter() {
        grid[b_to_m.0[0]] = '.';
        grid[b_to_m.0[1]] = '.';
    }

    for old_box in boxes_to_move.iter() {
        let new_box = all_boxes.iter_mut().find(|p|*p == old_box).unwrap();

        new_box.0[0] = move_to(old_box.0[0], grid.size(), dir);
        new_box.0[1] = move_to(old_box.0[1], grid.size(), dir);

        grid[new_box.0[0]] = '[';
        grid[new_box.0[1]] = ']';
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
    #[case(load_sample("sample.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
