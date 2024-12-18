use regex::Regex;
use itertools::Itertools;

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
    let reg_re = Regex::new(r"Register\s+(.)\s*:\s*(\d+)").unwrap();
    let prog_re = Regex::new(r"Program:\s*([0-7,\s]+)").unwrap();

    let text = input.read_lines()?;

    let mut computer = Computer {
        registers: Registers {
            a: 0,
            b: 0,
            c: 0,
            pc: 0,
        },
        progmem: Vec::new(),
    };

    for line in text.into_iter() {
        if let Some((_, [reg_m, val])) = reg_re.captures(&line).map(|c|c.extract()) {
            match reg_m {
                "A" => computer.registers.a = val.parse().unwrap(),
                "B" => computer.registers.b = val.parse().unwrap(),
                "C" => computer.registers.c = val.parse().unwrap(),
                _ => panic!("Unexpected register"),
            }
        }

        if let Some((_, [prog])) = prog_re.captures(&line).map(|c|c.extract()) {
            computer.progmem =
                prog
                    .split(',')
                    .map(|p|p.trim().parse().unwrap())
                    .collect();
        }
    }

    Ok(computer)
}

type RegVal = u64;

#[derive(Debug, Clone)]
struct Computer {
    registers: Registers,
    progmem: Vec<u8>,
}


#[derive(Debug, Clone)]
struct Registers {
    a: RegVal,
    b: RegVal,
    c: RegVal,
    pc: usize,
}


#[derive(Debug)]
enum Instruction {
    Adv(Combo),
    Bxl(u8),
    Bst(Combo),
    Jnz(u8),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

impl From<&[u8]> for Instruction {
    fn from(ibytes: &[u8]) -> Self {
        match ibytes[0] {
            0 => Self::Adv(Combo::from(ibytes[1])),
            1 => Self::Bxl(ibytes[1]),
            2 => Self::Bst(Combo::from(ibytes[1])),
            3 => Self::Jnz(ibytes[1]),
            4 => Self::Bxc,
            5 => Self::Out(Combo::from(ibytes[1])),
            6 => Self::Bdv(Combo::from(ibytes[1])),
            7 => Self::Cdv(Combo::from(ibytes[1])),
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Combo {
    Const(u8),
    A,
    B,
    C,
}

impl From<u8> for Combo {
    fn from(u: u8) -> Self {
        match u {
            0|1|2|3 => Self::Const(u),
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            _ => panic!("Invalid combo arg"),
        }
    }
}

impl Registers {
    fn get_combo(&self, arg: Combo) -> RegVal {
        match arg {
            Combo::Const(val) => val as RegVal,
            Combo::A => self.a,
            Combo::B => self.b,
            Combo::C => self.c,
        }
    }
}

fn calculate_p1(input: &ParsedInput) -> Vec<u8> {

    let program = input.progmem
        .as_slice()
        .chunks_exact(2)
        .map(Instruction::from)
        .collect_vec();

    run_program(&mut input.registers.clone(), &program)
}

fn run_program(registers: &mut Registers, program: &[Instruction]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    while registers.pc < program.len() {

        match program[registers.pc] {
            Instruction::Adv(c_val) => {
                registers.a >>= registers.get_combo(c_val);
                //println!("A >>= {:?}\t{}", c_val, computer.a);
            },
            Instruction::Bxl(arg) => {
                registers.b ^= arg as RegVal;
                //println!("B ^= {:?}\t{}", arg, computer.b);
            },
            Instruction::Bst(c_val) => {
                registers.b = registers.get_combo(c_val) % 8;
                //println!("B = {:?} % 8\t{}", c_val, computer.b);
            },
            Instruction::Jnz(arg) => {
                // jnz
                if registers.a != 0 {
                    registers.pc = (arg / 2) as usize; //divide by 2, because program is decoded
                    //println!("Jnz {}\n", arg);
                    continue;
                }
            },
            Instruction::Bxc => {
                registers.b ^= registers.c;
                //println!("B ^= C\t{}", computer.b);
            },
            Instruction::Out(c_val) => {
                output.push((registers.get_combo(c_val) % 8) as u8);
                //println!("Out {:?} % 8\t{}", c_val, (computer.combo(c_val) % 8));
            },
            Instruction::Bdv(c_val) => {
                registers.b = registers.a >> registers.get_combo(c_val);
                //println!("B = A >> {:?}\t{}", c_val, computer.b);
            },
            Instruction::Cdv(c_val) => {
                registers.c = registers.a >> registers.get_combo(c_val);
                //println!("C = A >> {:?}; / {} \t{}", c_val, computer.combo(c_val), computer.c);
            },
        }

        registers.pc += 1;
    }

    output
}

fn calculate_p2(input: &ParsedInput) -> RegVal {

    let program = input.progmem
        .as_slice()
        .chunks_exact(2)
        .map(Instruction::from)
        .collect_vec();

    let result = search_n_digits(input, &program, 0, input.progmem.len()-1);

    result.unwrap()
}

fn search_n_digits(input: &ParsedInput, program: &[Instruction], mut search_a: RegVal, n: usize) -> Option<RegVal> {

    search_a <<= 3;

    for i in 0..8 {
        let mut registers = input.registers.clone();
        registers.a = search_a | i;

        let res = run_program(&mut registers, program);

        if res == &input.progmem[n..] {

            if n == 0 {
                // a solution at the deepest level - got our answer
                return Some(search_a | i);
            }

            // dive deeper
            let inner_res = search_n_digits(input, program, search_a | i, n - 1);

            // and exit if the deeper level produced an answer
            // if not, try another digit
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
    #[case(load_sample("challenging.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = res2num(&calculate_p1(&parsed));

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample_1.txt")?)]
    #[case(load_sample("input.txt")?)]
    #[case(load_sample("challenging.txt")?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
