use anyhow::anyhow;
use aoc_tools::{Direction, Input, InvalidInput, IterMoreTools, Point, ResultExt};
use itertools::Itertools;
use std::{collections::HashMap, io::{self, Write}};

type ParsedInput = Vec<String>;

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed)?;
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed)?;
    println!("Result p2: {}", result2);

    Ok(())
}

const NUMERIC_A: Point = Point { x: 2, y: 3 };
const NUMERIC_F: Point = Point { x: 0, y: 3 };

const DIRECTIONAL_A: Point = Point { x: 2, y: 0 };
const DIRECTIONAL_F: Point = Point { x: 0, y: 0 };

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    Ok(input.read_lines()?)
}

fn calculate_p1(input: &ParsedInput) -> anyhow::Result<usize> {

    let transitions = prepare_numpad_transitions();

    let mut totals = 0;

    for digits in input.iter() {
        let code: usize = digits[..3].parse().unwrap();
        let mut n_steps = 0;

        for (start, end) in Some('A').into_iter().chain(digits.chars()).tuple_windows() {
            let steps = transitions.get(&(start, end)).unwrap();
            n_steps += steps;
        }
        println!("{} -> {}", code, n_steps);
        totals += n_steps * code;
    }

    Ok(totals)
}


fn prepare_numpad_transitions() -> HashMap<(char, char), usize>{
    let all_digits = ['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut transitions: HashMap<(char, char), usize> = HashMap::new();

    for start in all_digits.iter() {
        for end in all_digits.iter() {
            let start_point = digit_to_numeric_key(*start);
            let end_point = digits_to_numeric_keys(&end.to_string());

            let num_dist = distances_between_points(&start_point, &end_point);

            let commands = eval_key_distances(&num_dist, start_point).unwrap();

            transitions.insert((*start, *end), commands.len());
        }
    }

    transitions
}

fn calculate_p2(_input: &ParsedInput) -> anyhow::Result<u64> {
    Ok(0)
}

fn find_and_check_commands(digits: &str) -> anyhow::Result<String> {
    let command = digits_to_commands(digits);

    command
        .ok_or(anyhow!("No valid command found"))
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
    Move(Direction),
    Activate,
}

fn digits_to_commands(digits: &str) -> Option<String> {
    println!("Digits: {}", digits);
    let keys = digits_to_numeric_keys(digits);
    let distances = distances_between_points(&NUMERIC_A, &keys);
    eval_key_distances(&distances, NUMERIC_A)
}

fn eval_key_distances(distances: &[(isize, isize)], initial_pos: Point) -> Option<String> {
    let mut result: Option<String> = None;
    let mut shortest = usize::MAX;

    for cmds in all_commands_from_distances(&distances).into_iter() {
        //println!("Cmds: {:?}", commands_to_string(&cmds));
        if commands_on_numeric_pad(&cmds, initial_pos).is_err() { continue;}
        let keys2 = commands_to_directional_keys(&cmds);
        let distances2 = distances_between_points(&DIRECTIONAL_A, &keys2);
        for cmds2 in all_commands_from_distances(&distances2) {
            if commands_on_directional_pad(&cmds2).is_err() { continue;}
            //print!("."); io::stdout().flush().unwrap();
            let keys3 = commands_to_directional_keys(&cmds2);
            let distances3 = distances_between_points(&DIRECTIONAL_A, &keys3);
            for cmds3 in all_commands_from_distances(&distances3) {
                if commands_on_directional_pad(&cmds3).is_err() { continue;}
                if cmds3.len() < shortest {
                    shortest = cmds3.len();
                    //println!("New shortest: {}", shortest);
                    result = Some(commands_to_string(&cmds3));
                }
            }
        }
        //println!();
    }

    result
}


fn commands_to_digits(commands: &str) -> anyhow::Result<String> {
    let cmds = commands_from_string(commands)?;

    let step1_output = commands_on_directional_pad(&cmds)?;

    let cmds2 = directional_keys_to_commands(&step1_output);
    //println!("St 2: {:?}", commands_to_string(&cmds2));
    let step2_output = commands_on_directional_pad(&cmds2)?;

    let cmds3 = directional_keys_to_commands(&step2_output);
    //println!("St 3: {:?}", commands_to_string(&cmds3));
    let step3_output = commands_on_numeric_pad(&cmds3, NUMERIC_A)?;

    Ok(numeric_keys_to_digits(&step3_output))
}

fn commands_on_numeric_pad(cmds: &[Command], initial_pos: Point) -> anyhow::Result<Vec<Point>> {
    interpret_commands(cmds, initial_pos, (3, 4), NUMERIC_F)
}

fn commands_on_directional_pad(cmds: &[Command]) -> anyhow::Result<Vec<Point>> {
    interpret_commands(cmds, DIRECTIONAL_A, (3, 2), DIRECTIONAL_F)
}

fn commands_from_string(s: &str) -> anyhow::Result<Vec<Command>> {
    Ok(s.chars()
        .map(|c| match c {
            '^' => Ok(Command::Move(Direction::Up)),
            '>' => Ok(Command::Move(Direction::Right)),
            'v' => Ok(Command::Move(Direction::Down)),
            '<' => Ok(Command::Move(Direction::Left)),
            'A' => Ok(Command::Activate),
            _ => Err(InvalidInput(format!("Invalid command: {}", c))),
        })
        .try_collect_vec()?)
}

fn commands_to_string(cmds: &[Command]) -> String {
    cmds.iter().map(|cmd| match cmd {
        Command::Move(Direction::Up) => '^',
        Command::Move(Direction::Right) => '>',
        Command::Move(Direction::Down) => 'v',
        Command::Move(Direction::Left) => '<',
        Command::Activate => 'A',
    }).collect()
}

fn interpret_commands(cmds: &[Command], initial_pos: Point, bounds: (usize, usize), forbidden: Point) -> anyhow::Result<Vec<Point>> {

    let mut pos = initial_pos;
    let mut output: Vec<Point> = Vec::new();

    for cmd in cmds {
        match cmd {
            Command::Move(dir) => {
                pos = pos.advance(*dir, bounds).map_err_to_invalid_input(&format!("Invalid position for move : {:?} {:?}", pos, dir))?;
                if pos == forbidden {
                    return Err(InvalidInput(format!("Forbidden position reached: {:?}", pos)))?;
                }
            }
            Command::Activate => {
                output.push(pos);
            }
        }
    }

    Ok(output)
}

fn directional_keys_to_commands(keys: &[Point]) -> Vec<Command> {
    keys.iter().map(|key| {
        match key {
            Point { x: 1, y: 0 } => Command::Move(Direction::Up),
            Point { x: 2, y: 0 } => Command::Activate,
            Point { x: 0, y: 1 } => Command::Move(Direction::Left),
            Point { x: 1, y: 1 } => Command::Move(Direction::Down),
            Point { x: 2, y: 1 } => Command::Move(Direction::Right),
            _ => panic!("Invalid key: {:?}", key),
        }
    }).collect()
}

fn commands_to_directional_keys(commands: &[Command]) -> Vec<Point> {
    commands.iter().map(|cmd| {
        match cmd {
            Command::Move(Direction::Up) => Point { x: 1, y: 0 },
            Command::Move(Direction::Right) => Point { x: 2, y: 1 },
            Command::Move(Direction::Down) => Point { x: 1, y: 1 },
            Command::Move(Direction::Left) => Point { x: 0, y: 1 },
            Command::Activate => Point { x: 2, y: 0 },
        }
    }).collect()
}

fn numeric_keys_to_digits(keys: &[Point]) -> String {
    keys.iter().map(|key| {
        match key {
            Point { x: 0, y: 2 } => '1',
            Point { x: 1, y: 2 } => '2',
            Point { x: 2, y: 2 } => '3',
            Point { x: 0, y: 1 } => '4',
            Point { x: 1, y: 1 } => '5',
            Point { x: 2, y: 1 } => '6',
            Point { x: 0, y: 0 } => '7',
            Point { x: 1, y: 0 } => '8',
            Point { x: 2, y: 0 } => '9',
            Point { x: 1, y: 3 } => '0',
            Point { x: 2, y: 3 } => 'A',
            _ => panic!("Invalid key: {:?}", key),
        }
    }).collect()
}

fn digits_to_numeric_keys(digits: &str) -> Vec<Point> {
    digits.chars().map(digit_to_numeric_key).collect()
}

fn digit_to_numeric_key(digit: char) -> Point {
    match digit {
        '1' => Point { x: 0, y: 2 },
        '2' => Point { x: 1, y: 2 },
        '3' => Point { x: 2, y: 2 },
        '4' => Point { x: 0, y: 1 },
        '5' => Point { x: 1, y: 1 },
        '6' => Point { x: 2, y: 1 },
        '7' => Point { x: 0, y: 0 },
        '8' => Point { x: 1, y: 0 },
        '9' => Point { x: 2, y: 0 },
        '0' => Point { x: 1, y: 3 },
        'A' => Point { x: 2, y: 3 },
        _ => panic!("Invalid digit: {}", digit),
    }
}

fn distances_between_points(start: &Point, points: &[Point]) -> Vec<(isize, isize)> {
    Some(start)
        .into_iter()
        .chain(points.into_iter())
        .tuple_windows()
        .map(|(p1, p2)| (p2.x as isize - p1.x as isize, p2.y as isize - p1.y as isize))
        .collect()
}

fn all_commands_from_distances(distances: &[(isize, isize)]) -> Vec<Vec<Command>> {
    let mut output = Vec::new();

    recursive_add_all_commands(distances, Vec::new(), &mut output, 0);

    fn recursive_add_all_commands(distances: &[(isize, isize)], mut current: Vec<Command>, output: &mut Vec<Vec<Command>>, depth: usize) {
        if depth == distances.len() {
            output.push(current);
            return;
        }

        for commands in all_commands_from_distance(distances[depth]) {
            let mut specific = current.clone();
            specific.extend(commands);
            recursive_add_all_commands(distances, specific, output, depth + 1);
        }
    }

    output
}

#[allow(dead_code)]
fn all_commands_from_distance(distance: (isize, isize)) -> Vec<Vec<Command>> {
    let mut output = Vec::new();

    fn generate_commands(distance: (isize, isize), current: Vec<Command>, output: &mut Vec<Vec<Command>>) {
        if distance == (0, 0) {
            let mut new_current = current.clone();
            new_current.push(Command::Activate);
            output.push(new_current);
            return;
        }

        if distance.0 > 0 {
            let mut new_current = current.clone();
            new_current.push(Command::Move(Direction::Right));
            generate_commands((distance.0 - 1, distance.1), new_current, output);
        }

        if distance.0 < 0 {
            let mut new_current = current.clone();
            new_current.push(Command::Move(Direction::Left));
            generate_commands((distance.0 + 1, distance.1), new_current, output);
        }

        if distance.1 > 0 {
            let mut new_current = current.clone();
            new_current.push(Command::Move(Direction::Down));
            generate_commands((distance.0, distance.1 - 1), new_current, output);
        }

        if distance.1 < 0 {
            let mut new_current = current.clone();
            new_current.push(Command::Move(Direction::Up));
            generate_commands((distance.0, distance.1 + 1), new_current, output);
        }
    }

    generate_commands(distance, Vec::new(), &mut output);

    output
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
    //#[ignore]
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

    #[rstest]
    #[case("256A", "<vA<AA>>^AvA<^A>AvA^A<v<A>>^AvA^A<vA>^A<A>A<v<A>A>^AAvA<^A>A")]
    #[case("512A", "<vA<AA>>^AvA<^A>AAvA^A<vA<AA>>^AvA^AvA<^A>A<vA>^A<A>A<v<A>A>^AvA^A<A>A")]
    fn test_commands_to_digits(#[case] expected: &str, #[case] input: &str) -> anyhow::Result<()> {
        let result = commands_to_digits(input)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_all_commands_from_distance_1_1() {
        let result = all_commands_from_distance((1, 1));
        
        assert_eq!(vec![vec![Command::Move(Direction::Right), Command::Move(Direction::Down), Command::Activate],
                        vec![Command::Move(Direction::Down), Command::Move(Direction::Right), Command::Activate]], result);
    }

    #[test]
    fn test_all_commands_from_distance_0_2() {
        let result = all_commands_from_distance((0, 2));
        
        assert_eq!(vec![vec![Command::Move(Direction::Down), Command::Move(Direction::Down), Command::Activate]], result);
    }

    #[test]
    fn test_all_commands_from_distance_2_2() {
        let result = all_commands_from_distance((2, 2));
        
        assert_eq!(vec![
            vec![Command::Move(Direction::Right), Command::Move(Direction::Right), Command::Move(Direction::Down), Command::Move(Direction::Down), Command::Activate],
            vec![Command::Move(Direction::Right), Command::Move(Direction::Down), Command::Move(Direction::Right), Command::Move(Direction::Down), Command::Activate],
            vec![Command::Move(Direction::Right), Command::Move(Direction::Down), Command::Move(Direction::Down), Command::Move(Direction::Right), Command::Activate],
            vec![Command::Move(Direction::Down), Command::Move(Direction::Right), Command::Move(Direction::Right), Command::Move(Direction::Down), Command::Activate],
            vec![Command::Move(Direction::Down), Command::Move(Direction::Right), Command::Move(Direction::Down), Command::Move(Direction::Right), Command::Activate],
            vec![Command::Move(Direction::Down), Command::Move(Direction::Down), Command::Move(Direction::Right), Command::Move(Direction::Right), Command::Activate],
            ], result);
    }

    #[test]
    fn test_all_commands_from_distances_1_1() {
        let result = all_commands_from_distances(&[(1, 0), (1, 1), (0, 1)]);
        
        assert_eq!(vec![
            vec![Command::Move(Direction::Right), Command::Activate, Command::Move(Direction::Right), Command::Move(Direction::Down), Command::Activate, Command::Move(Direction::Down), Command::Activate],
            vec![Command::Move(Direction::Right), Command::Activate, Command::Move(Direction::Down), Command::Move(Direction::Right), Command::Activate, Command::Move(Direction::Down), Command::Activate],
            ], result);
    }
}