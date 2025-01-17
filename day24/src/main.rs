use std::collections::HashMap;

use aoc_tools::NameRegistry;
use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;

type ParsedInput = (Box<[String]>, HashMap<usize, Node>);

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed)?;
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed)?;
    println!("Result p2: {}", result2);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum NodeOp {
    Const(bool),
    And(usize, usize),
    Or(usize, usize),
    Xor(usize, usize),
}

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    op: NodeOp,
}


fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    let lines = input.read_lines()?;
    let in_rx = Regex::new(r"(\w+): ([01])").unwrap();
    let op_rx = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();

    let mut name_reg: NameRegistry = NameRegistry::new();
    let mut nodes: HashMap<usize, Node> = HashMap::new();

    for line in lines.into_iter() {
        if let Some(in_wire) = in_rx.captures(&line) {
            let (_, [name, val]) = in_wire.extract();

            let node = Node {
                id: name_reg.add_or_lookup(name),
                op: NodeOp::Const(val == "1"),
            };

            nodes.insert(node.id, node);

        } else if let Some(in_op) = op_rx.captures(&line) {
            let (_, [arg0, op_name, arg1, target]) = in_op.extract();
            let a0 = name_reg.add_or_lookup(arg0);
            let a1 = name_reg.add_or_lookup(arg1);
            let t = name_reg.add_or_lookup(target);

            let node = Node {
                id: t,
                op: match op_name {
                    "AND" => NodeOp::And(a0, a1),
                    "OR" => NodeOp::Or(a0, a1),
                    "XOR" => NodeOp::Xor(a0, a1),
                    _ => panic!("Invalid op")
                }
            };
            nodes.insert(node.id, node);
        } else if line != "" {
            panic!("Invalid input line")
        }
    }

    let names_vec: Vec<String> = name_reg.into();
    Ok((names_vec.into_boxed_slice(), nodes))
}

fn calculate_p1(input: &ParsedInput) -> anyhow::Result<u64> {
    let (names, nodes) = input;

    let mut values = vec![None; nodes.len()];

    let znames = find_nodes(names, "z");
    process_adder(nodes, &mut values);

    Ok(wires_to_int(&values, &znames))
}

fn calculate_p2(input: &ParsedInput) -> anyhow::Result<String> {
    let (names, nodes) = input;

    let mut nodes = nodes.clone();

    let xnames = find_nodes(names, "x");
    let ynames = find_nodes(names, "y");
    let znames = find_nodes(names, "z");

    let mut swaps = Vec::new();

    let defective = find_defective_bits(&nodes, &xnames, &ynames, &znames);

    for d in defective.iter() {
        let x = xnames[*d];
        let z = znames[*d];

        repair_out_xor(&mut nodes, x, z, &mut swaps);
    }

    let verify = find_defective_bits(&nodes, &xnames, &ynames, &znames);
    if verify.len() > 0 {
        Err(anyhow!("Could not repair the circuit"))?
    }

    let result = swaps
        .iter()
        .map(|s|&names[*s])
        .sorted()
        .join(",");

    Ok(result)
}


fn repair_out_xor(nodes: &mut HashMap<usize, Node>, x: usize, z: usize, swaps: &mut Vec<usize>) {
    let out_node = nodes.get(&z).unwrap();
    match out_node.op {
        NodeOp::And(_, _)
        | NodeOp::Or(_, _)
        | NodeOp::Const(_) => {
            // the output stage of an adder must be XOR gate
            // if it is not, traverse the gate chain from input
            // site through 2 XORs to get the node
            if let Some(xor2) = find_z_from_x(&nodes, x) {
                swap_nodes(nodes, z, xor2);
                swaps.push(z);
                swaps.push(xor2);
            }
        },
        NodeOp::Xor(arg0, arg1) => {
            repair_out_xor_input(nodes, x, arg0, swaps);
            repair_out_xor_input(nodes, x, arg1, swaps);
        }
    };
}

fn repair_out_xor_input(nodes: &mut HashMap<usize, Node>, x: usize, arg: usize, swaps: &mut Vec<usize>) {
    let xor_in_node = nodes.get(&arg).unwrap();
    match xor_in_node.op {
        NodeOp::Or(_, _) => {
            // if input comes from OR gate, most likely it is Carry from
            // previous stage. It is fine, leaving it as-is
        },
        NodeOp::Xor(_, _) => {
            // input comes from XOR gate, most likely that came from 1st stage
            // half-adder XOR, It is fine, leave as-is
        },
        NodeOp::And(_, _)
        | NodeOp::Const(_) => {
            // unexpected input
            // assuming that carry from previous bit is fine, let's adjust this one to be from
            // 1st stage half-adder's output
            if let Some(xor1) = node_xor_with_input(nodes, x) {
                swap_nodes(nodes, arg, xor1);
                swaps.push(arg);
                swaps.push(xor1);
            }
        },
    }
}

fn find_z_from_x(nodes: &HashMap<usize, Node>, x: usize) -> Option<usize> {
    let xor1 = node_xor_with_input(nodes, x)?;
    node_xor_with_input(nodes, xor1)
}

fn node_xor_with_input(nodes: &HashMap<usize, Node>, input: usize) -> Option<usize> {
    for node in nodes.values() {
        match node.op {
            NodeOp::Xor(arg0, arg1) if arg0 == input || arg1 == input => {
                return Some(node.id);
            },
            _ => {},
        }
    }

    None
}

fn swap_nodes(nodes: &mut HashMap<usize, Node>, n1: usize, n2: usize) {

    let t = nodes.get(&n1).unwrap().op;
    nodes.get_mut(&n1).unwrap().op = nodes.get(&n2).unwrap().op;
    nodes.get_mut(&n2).unwrap().op = t;
}

fn find_defective_bits(nodes: &HashMap<usize, Node>, xnames: &[usize], ynames: &[usize], znames: &[usize]) -> Vec<usize> {

    let mut defective = Vec::new();

    for bit in 0..xnames.len() {

        let calc_result = add_numbers(1 << bit, 0, &nodes, &xnames, &ynames, &znames);
        if calc_result != 1 << bit {
            defective.push(bit);
        }
    }

    defective
}

fn add_numbers(x: u64, y: u64, nodes: &HashMap<usize, Node>, xnames: &[usize], ynames: &[usize], znames: &[usize]) -> u64 {
    let mut wires = vec![None; nodes.len()];

    int_to_wires(&mut wires, xnames, x);
    int_to_wires(&mut wires, ynames, y);

    process_adder(nodes, &mut wires);

    wires_to_int(&wires, znames)
}

fn process_adder(nodes: &HashMap<usize, Node>, values: &mut [Option<bool>]) {
    let mut new_values = true;
    while new_values {
        new_values = false;
        for node in nodes.values() {
            if values[node.id].is_none() {
                values[node.id] = node.op.eval(|v| values[v]);
                new_values |= values[node.id].is_some();
            }
        }
    }
}

fn find_nodes(names: &[String], prefix: &str) -> Vec<usize> {
    names
        .iter()
        .enumerate()
        .filter(|(_, n)| n.starts_with(prefix))
        .sorted_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(i, _)|i)
        .collect()
}

fn wires_to_int(values: &[Option<bool>], znames: &[usize]) -> u64 {
    let mut result: u64 = 0;

    for zi in znames.iter().rev() {
        result <<= 1;
        if values[*zi] == Some(true) {
            result |= 1;
        }
    }
    result
}

fn int_to_wires(wires: &mut [Option<bool>], names: &[usize], mut value: u64) {
    for name in names {
        wires[*name] = Some((value & 1) != 0);
        value >>= 1;
    }
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


#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hasher};
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

        let mut hasher = DefaultHasher::new();
        hasher.write(result2.as_bytes());

        assert_eq!(expected, Some(hasher.finish()));
        Ok(())
    }
}
