use aoc_tools::{Direction, InvalidInput, IterMoreTools, Point, ResultExt};

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

fn calculate_p1(_input: &ParsedInput) -> anyhow::Result<u64> {

    Ok(0)
}

fn calculate_p2(_input: &ParsedInput) -> anyhow::Result<u64> {
    Ok(0)
}

#[derive(Debug)]
enum Command {
    Move(Direction),
    Activate,
}


fn commands_to_digits(commands: &str) -> anyhow::Result<String> {
    let cmds = commands_from_string(commands)?;

    let step1_output = interpret_commands(&cmds, Point { x: 2, y: 0 }, (3, 2), Point { x: 0, y: 0 })?;

    let cmds2 = directional_keys_to_commands(&step1_output);
    let step2_output = interpret_commands(&cmds2, Point { x: 2, y: 0 }, (3, 2), Point { x: 0, y: 0 })?;

    let cmds3 = directional_keys_to_commands(&step2_output);
    let step3_output = interpret_commands(&cmds3, Point { x: 2, y: 3 }, (3, 4), Point { x: 0, y: 3 })?;

    Ok(numeric_keys_to_digits(&step3_output))
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
    #[case("", "")]
    fn test_commands_to_digits(#[case] expected: &str, #[case] input: &str) -> anyhow::Result<()> {
        let result = commands_to_digits(input)?;
        assert_eq!(result, expected);
        Ok(())
    }
}