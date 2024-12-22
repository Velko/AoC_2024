use aoc_tools::{Direction, InvalidInput, Point, ResultExt};

use crate::commands::{commands_to_string, Command, DIRECTIONAL_A, DIRECTIONAL_F, NUMERIC_A, NUMERIC_F};

pub fn commands_to_digits(commands: &str) -> anyhow::Result<String> {
    let cmds = commands_from_string(commands);

    let step1_output = commands_on_directional_pad(&cmds, DIRECTIONAL_A)?;

    let cmds2 = directional_keys_to_commands(&step1_output);
    println!("St 2: {:?}", commands_to_string(&cmds2));
    let step2_output = commands_on_directional_pad(&cmds2, DIRECTIONAL_A)?;

    let cmds3 = directional_keys_to_commands(&step2_output);
    println!("St 3: {:?}", commands_to_string(&cmds3));
    let step3_output = commands_on_numeric_pad(&cmds3, NUMERIC_A)?;

    Ok(numeric_keys_to_digits(&step3_output))
}

pub fn commands_from_string(s: &str) -> Vec<Command> {
    s.chars()
        .map(directional_button_to_command)
        .collect()
}

pub fn directional_button_to_command(button: char) -> Command {
    match button {
        '^' => Command::Move(Direction::Up),
        '>' => Command::Move(Direction::Right),
        'v' => Command::Move(Direction::Down),
        '<' => Command::Move(Direction::Left),
        'A' => Command::Activate,
        _ => panic!("Invalid key: {:?}", button),
    }
}

pub fn commands_on_numeric_pad(cmds: &[Command], initial_pos: Point) -> anyhow::Result<Vec<Point>> {
    interpret_commands(cmds, initial_pos, (3, 4), NUMERIC_F)
}

pub fn commands_on_directional_pad(cmds: &[Command], initial_pos: Point) -> anyhow::Result<Vec<Point>> {
    interpret_commands(cmds, initial_pos, (3, 2), DIRECTIONAL_F)
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


#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case("256A", "<vA<AA>>^AvA<^A>AvA^A<v<A>>^AvA^A<vA>^A<A>A<v<A>A>^AAvA<^A>A")]
    #[case("512A", "<vA<AA>>^AvA<^A>AAvA^A<vA<AA>>^AvA^AvA<^A>A<vA>^A<A>A<v<A>A>^AvA^A<A>A")]
    #[case("42", "<v<A>>^AA<vA<A>>^AAvAA<^A>A<v<A>A>^AvA^A<A>A")]
    fn test_commands_to_digits(#[case] expected: &str, #[case] input: &str) -> anyhow::Result<()> {
        let result = commands_to_digits(input)?;
        assert_eq!(result, expected);
        Ok(())
    }
}