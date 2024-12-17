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
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
    progmem: Vec<u8>,
}

#[repr(u8)]
#[derive(Debug)]
enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Combo {
    Const(u8),
    A,
    B,
    C,
}

impl From<u64> for Combo {
    fn from(u : u64) -> Self {
        match u {
            0|1|2|3 => Self::Const(u as u8),
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            _ => panic!("Invalid combo arg"),
        }
    }
}

impl Computer {
    fn fetch(&self) -> Option<(Instruction, u64)> {
        Some((
            unsafe { std::mem::transmute(*self.progmem.get(self.pc)?) },
            *self.progmem.get(self.pc + 1)? as u64,
        ))
    }

    fn combo(&self, arg: Combo) -> u64 {
        match arg {
            Combo::Const(val) => val as u64,
            Combo::A => self.a,
            Combo::B => self.b,
            Combo::C => self.c,
           _ => panic!("Invalid combo arg"),
        }
    }

    fn reset(&mut self, orig: &Computer) {
        self.a = orig.a;
        self.b = orig.b;
        self.c = orig.c;
        self.pc = 0;
    }
}

fn calculate_p1(input: &ParsedInput) -> Vec<u8> {
    run_program(&mut input.clone())
}

fn run_program(computer: &mut Computer) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    loop {

        if let Some((opcode, arg)) = computer.fetch() {
            //println!("{:?} {}", opcode, arg);
            match opcode {
                Instruction::Adv => {
                    // adv
                    let c_val = arg.into();
                    computer.a >>= computer.combo(c_val);
                    //println!("A >>= {:?}\t{}", c_val, computer.a);
                },
                Instruction::Bxl => {
                    // bxl
                    computer.b ^= arg;
                    //println!("B ^= {:?}\t{}", arg, computer.b);
                },
                Instruction::Bst => {
                    // bst
                    let c_val = arg.into();
                    computer.b = computer.combo(c_val) % 8;
                    //println!("B = {:?} % 8\t{}", c_val, computer.b);
                },
                Instruction::Jnz => {
                    // jnz
                    if computer.a != 0 {
                        computer.pc = arg as usize;
                        //println!("Jnz {}\n", arg);
                        continue;
                    }
                },
                Instruction::Bxc => {
                    // bxc
                    computer.b ^= computer.c;
                    //println!("B ^= C\t{}", computer.b);
                },
                Instruction::Out => {
                    // out
                    let c_val = arg.into();
                    output.push((computer.combo(c_val) % 8) as u8);
                    //println!("Out {:?} % 8\t{}", c_val, (computer.combo(c_val) % 8));
                },
                Instruction::Bdv => {
                    // bdv
                    let c_val = arg.into();
                    computer.b = computer.a >> computer.combo(c_val);
                    //println!("B = A >> {:?}\t{}", c_val, computer.b);
                },
                Instruction::Cdv => {
                    // cdv
                    let c_val = arg.into();
                    computer.c = computer.a >> computer.combo(c_val);
                    //println!("C = A >> {:?}; / {} \t{}", c_val, computer.combo(c_val), computer.c);
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


fn vec_to_str(v: &[u8]) -> String {
    v
        .into_iter()
        .map(|d|d.to_string())
        .collect()
}


fn calculate_p2(input: &ParsedInput) -> u64 {

    let mut computer = input.clone();

    let orig_prog = vec_to_str(&input.progmem);

    let result = search_n_digits(input, 0, input.progmem.len()-1);

    result.unwrap()
}

fn search_n_digits(input: &ParsedInput, mut search_a: u64, n: usize) -> Option<u64> {

    let mut computer = input.clone();
    search_a <<= 3;

    for i in 0..8 {
        computer.reset(input);
        computer.a = search_a | i;

        let res = run_program(&mut computer);

        if res == &input.progmem[n..] {
            println!("{:o}", i);
            println!("{:?}", &input.progmem[n..]);

            if n == 0 {
                return Some(search_a | i);
            }

            let inner_res = search_n_digits(input, search_a | i, n - 1);

            if inner_res.is_some() {
                return inner_res;
            }
        }
    }

    None
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
        vec_to_str(res)
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
