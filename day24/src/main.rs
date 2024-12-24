use aoc_tools::{InvalidInput, IterMoreTools, NameRegistry, ResultExt};
use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;

type ParsedInput = (Box<[String]>, Vec<Node>);

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed)?;
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed)?;
    println!("Result p2: {}", result2);

    Ok(())
}

#[derive(Debug)]
enum NodeOp {
    Const(bool),
    And(usize, usize),
    Or(usize, usize),
    Xor(usize, usize),
}

#[derive(Debug)]
struct Node {
    id: usize,
    op: NodeOp,
}


fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    let lines = input.read_lines()?;
    let in_rx = Regex::new(r"(\w+): ([01])").unwrap();
    let op_rx = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();

    let mut name_reg: NameRegistry = NameRegistry::new();
    let mut nodes: Vec<Node> = Vec::new();

    for line in lines.into_iter() {
        if let Some(in_wire) = in_rx.captures(&line) {
            let (_, [name, val]) = in_wire.extract();

            nodes.push(Node {
                id: name_reg.add_or_lookup(name),
                op: NodeOp::Const(val == "1"),
            });

        } else if let Some(in_op) = op_rx.captures(&line) {
            let (_, [arg0, op_name, arg1, target]) = in_op.extract();
            let a0 = name_reg.add_or_lookup(arg0);
            let a1 = name_reg.add_or_lookup(arg1);
            let t = name_reg.add_or_lookup(target);

            nodes.push(Node {
                id: t,
                op: match op_name {
                    "AND" => NodeOp::And(a0, a1),
                    "OR" => NodeOp::Or(a0, a1),
                    "XOR" => NodeOp::Xor(a0, a1),
                    _ => panic!("Invalid op")
                }
            });
        } else if line != "" {
            panic!("Invalid input line")
        }
    }

    let names_vec: Vec<String> = name_reg.into();
    Ok((names_vec.into_boxed_slice(), nodes))
}

fn calculate_p1(input: &ParsedInput) -> anyhow::Result<u64> {
    let (names, nodes) = input;

    //println!("{}, {}", names.len(), nodes.len());

    let mut val_vec = vec![None; names.len()];
    let values: &mut [Option<bool>] = val_vec.as_mut_slice();

    let mut new_values = true;
    while new_values {
        new_values = false;
        for node in nodes.iter() {
            if values[node.id].is_none() {
                values[node.id] = node.op.eval(|v| values[v]);
                new_values |= values[node.id].is_some();
            }
        }
    }

    let znames: Vec<_> = names
        .iter()
        .enumerate()
        .filter(|(_, n)| n.starts_with("z"))
        .sorted_by(|(_, a), (_, b)| b.cmp(a))
        .map(|(i, _)|i)
        .collect();

    let mut result: u64 = 0;

    for zi in znames.iter() {
        result <<= 1;
        if values[*zi] == Some(true) {
            result |= 1;
        }
    }

    //Err(anyhow!("Not implemented"))
    Ok(result)
}

impl NodeOp {
    fn eval<F>(&self, fetch: F) -> Option<bool> 
        where F: Fn(usize) -> Option<bool>
    {
        match *self {
            NodeOp::Const(val) => Some(val),
            NodeOp::And(a0, a1) => Some(fetch(a0)? && fetch(a1)?),
            NodeOp::Or(a0, a1) => Some(fetch(a0)? || fetch(a1)?),
            NodeOp::Xor(a0, a1) => Some(fetch(a0)? ^ fetch(a1)?),
        }
    }
}


fn calculate_p2(_input: &ParsedInput) -> anyhow::Result<u64> {
    Ok(0)
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
    #[case(load_sample("sample_0.txt")?)]
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
}
