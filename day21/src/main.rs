use aoc_tools::{Direction, Point};
use itertools::Itertools;
use std::{collections::HashMap, usize};

mod c2d;
mod commands;

use commands::{commands_to_string, Command, DIRECTIONAL_A};

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

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    Ok(input.read_lines()?)
}

fn calculate_p1(input: &ParsedInput) -> anyhow::Result<usize> {
    calculate_p_x(input, 2)
}


fn prepare_numpad_transitions() -> HashMap<(char, char), String> {
    let all_digits = ['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut transitions: HashMap<(char, char), String> = HashMap::new();

    for start in all_digits.iter() {
        for end in all_digits.iter() {
            let start_point = digit_to_numeric_key(*start);
            let end_point = digits_to_numeric_keys(&end.to_string());

            let num_dist = distances_between_points(&start_point, &end_point);

            let lencmd = eval_key_distances(&num_dist, |c| c2d::commands_on_numeric_pad(c, start_point).is_ok());

            transitions.insert((*start, *end), lencmd);
        }
    }

    transitions
}

fn prepare_directional_transitions() -> HashMap<(char, char), String>{
    let all_buttons = ['A', '>', '<', '^', 'v'];

    let mut transitions: HashMap<(char, char), String> = HashMap::new();

    for start in all_buttons.iter() {
        for end in all_buttons.iter() {
            let start_point = c2d::directional_button_to_command(*start);
            let end_point = c2d::commands_from_string(&end.to_string());
            let start_key = command_to_directional_key(&start_point);
            let end_key = commands_to_directional_keys(&end_point);

            let dir_dist = distances_between_points(&start_key, &end_key);

            let lencmd = eval_key_distances(&dir_dist, |c|c2d::commands_on_directional_pad(c, start_key).is_ok());

            transitions.insert((*start, *end), lencmd);
        }
    }

    transitions
}

fn calculate_p2(input: &ParsedInput) -> anyhow::Result<usize> {
    calculate_p_x(input, 25)
}

fn calculate_p_x(input: &ParsedInput, middle_bots: usize) -> anyhow::Result<usize> {

    let transitions = prepare_numpad_transitions();
    let dir_transitions = prepare_directional_transitions();

    let mut totals = 0;

    for digits in input.iter() {
        let code: usize = digits[..3].parse().unwrap();
        let n_steps = calculate_cmd_len_v2(&transitions, &dir_transitions, digits, middle_bots);

        totals += n_steps * code;
    }

    Ok(totals)
}

fn calculate_cmd_len_v2(transitions: &HashMap<(char, char), String>, dir_transitions: &HashMap<(char, char), String>, digits: &str, middle_bots: usize) -> usize {
    let mut all_commands: HashMap<(char, char), usize> = HashMap::new();

    for (nf, nt) in Some('A').into_iter().chain(digits.chars()).tuple_windows() {
        let num_level = transitions.get(&(nf, nt)).unwrap();
        let mut level: HashMap<(char, char), usize> = HashMap::new();

        for ft in Some('A').into_iter().chain(num_level.chars()).tuple_windows() {
            *level.entry(ft).or_insert(0) += 1;
        }

        for _ in 0..middle_bots {
            let mut next_level: HashMap<(char, char), usize> = HashMap::new();
            for (ft, cnt) in level.into_iter() {
                let dir_level = dir_transitions.get(&ft).unwrap();
                for nft in Some('A').into_iter().chain(dir_level.chars()).tuple_windows() {
                    *next_level.entry(nft).or_insert(0) += cnt;
                }
            }
            level = next_level;
        }

        for (ft, cnt) in level.into_iter() {
            *all_commands.entry(ft).or_insert(0) += cnt;
        }
    }

    all_commands.values().sum()
}

fn eval_key_distances<F>(distances: &[(isize, isize)], validator: F) -> String
    where F: Fn(&[Command]) -> bool
{
    let mut result: Option<String> = None;
    let mut shortest = usize::MAX;

    for cmds in all_commands_from_distances(&distances).into_iter() {
        if !validator(&cmds) { continue;}
        let keys2 = commands_to_directional_keys(&cmds);
        let distances2 = distances_between_points(&DIRECTIONAL_A, &keys2);

        for cmds2 in all_commands_from_distances(&distances2) {
                if c2d::commands_on_directional_pad(&cmds2, DIRECTIONAL_A).is_err() { continue;}
                let keys3 = commands_to_directional_keys(&cmds2);
                let distances3 = distances_between_points(&DIRECTIONAL_A, &keys3);
                for cmds3 in all_commands_from_distances(&distances3) {
                    if c2d::commands_on_directional_pad(&cmds3, DIRECTIONAL_A).is_err() { continue;}
                    if cmds3.len() < shortest {
                        shortest = cmds3.len();
                        result = Some(commands_to_string(&cmds));
                    }
                }
        }
    }
    result.unwrap()
}


fn commands_to_directional_keys(commands: &[Command]) -> Vec<Point> {
    commands.iter().map(command_to_directional_key).collect()
}

fn command_to_directional_key(command: &Command) -> Point {
    match command {
        Command::Move(Direction::Up) => Point { x: 1, y: 0 },
        Command::Move(Direction::Right) => Point { x: 2, y: 1 },
        Command::Move(Direction::Down) => Point { x: 1, y: 1 },
        Command::Move(Direction::Left) => Point { x: 0, y: 1 },
        Command::Activate => Point { x: 2, y: 0 },
    }
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

    fn recursive_add_all_commands(distances: &[(isize, isize)], current: Vec<Command>, output: &mut Vec<Vec<Command>>, depth: usize) {
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

        if distance.1 < 0 {
            let mut new_current = current.clone();
            new_current.push(Command::Move(Direction::Up));
            generate_commands((distance.0, distance.1 + 1), new_current, output);
        }


        if distance.0 > 0 {
            let mut new_current = current.clone();
            new_current.push(Command::Move(Direction::Right));
            generate_commands((distance.0 - 1, distance.1), new_current, output);
        }

        if distance.1 > 0 {
            let mut new_current = current.clone();
            new_current.push(Command::Move(Direction::Down));
            generate_commands((distance.0, distance.1 - 1), new_current, output);
        }



        if distance.0 < 0 {
            let mut new_current = current.clone();
            new_current.push(Command::Move(Direction::Left));
            generate_commands((distance.0 + 1, distance.1), new_current, output);
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
    #[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed)?;

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed)?;

        assert_eq!(expected, Some(result2 as u64));
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