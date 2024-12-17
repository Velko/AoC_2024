use regex::Regex;
use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};

type ParsedInput = Computer;

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed);
    println!("Result p1: {:?}", result1);

    let result2 = calculate_p2(&parsed);
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    let reg_re = Regex::new(r"Register (.): (\d+)").unwrap();
    //let prog_re = Regex::new(r"(?<=Program: )\d+(?=,|\b)").unwrap();

    let text = input.read_lines()?;

    let mut computer = Computer {
        a: 0,
        b: 0,
        c: 0,
        pc: 0,
        progmem: Vec::new(),
    };

    for line in text.into_iter() {
        if let Some((_, [reg_m, val])) = reg_re.captures(&line).map(|c|c.extract()) {
            match reg_m {
                "A" => computer.a = val.parse().unwrap(),
                "B" => computer.b = val.parse().unwrap(),
                "C" => computer.b = val.parse().unwrap(),
                _ => panic!("Unexpected register"),
            }
        }

        if line.starts_with("Program: ") {
            computer.progmem =
                (&line[9..])
                .split(',')
                .map(|p|p.parse().unwrap())
                .collect();

        }

        // if let Some(prog_m) = prog_re.captures(&line) {
        //
        // }
    }

//    println!("{:?}", computer);

    Ok(computer)
}

#[derive(Debug, Clone)]
struct Computer {
    a: u32,
    b: u32,
    c: u32,
    pc: usize,
    progmem: Vec<u8>,
}

impl Computer {
    fn fetch(&self) -> Option<(u8, u32)> {
        Some((
            *self.progmem.get(self.pc)?,
            *self.progmem.get(self.pc + 1)? as u32,
        ))
    }

    fn combo(&self, arg: u32) -> u32 {
        match arg {
            0|1|2|3 => arg,
            4 => self.a,
            5 => self.b,
            6 => self.c,
           _ => panic!("Invalid combo arg"),
        }
    }
}

fn calculate_p1(input: &ParsedInput) -> Vec<u8> {
    run_program(input)
}

fn run_program(computer: &Computer) -> Vec<u8> {
    let mut computer = computer.clone();

    let mut output: Vec<u8> = Vec::new();

    loop {

        if let Some((opcode, arg)) = computer.fetch() {

            match opcode {
                0 => {
                    // adv
                    computer.a /= 1 << computer.combo(arg);
                },
                1 => {
                    // bxl
                    computer.b ^= arg;
                },
                2 => {
                    // bst
                    computer.b = computer.combo(arg) % 8;
                },
                3 => {
                    // jnz
                    if computer.a != 0 {
                        computer.pc = arg as usize;
                        continue;
                    }
                },
                4 => {
                    // bxc
                    computer.b ^= computer.c;
                },
                5 => {
                    // out
                    output.push((computer.combo(arg) % 8) as u8);
                },
                6 => {
                    // bdv
                    computer.b = computer.a / (1 << computer.combo(arg));
                },
                7 => {
                    // cdv
                    computer.c = computer.a / (1 << computer.combo(arg));
                },
                _ => panic!("Invalid opcode"),
            }

            computer.pc += 2;
        } else {
            break
        }
    }

    output
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

    fn res2num(res: &[u8]) -> u64 {
        res
            .into_iter()
            .map(|d|d.to_string())
            .collect::<String>()
            .parse()
            .unwrap()
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = res2num(&calculate_p1(&parsed));

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
