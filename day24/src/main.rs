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

#[derive(Debug, Clone)]
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

    let mut values = vec![None; nodes.len()];

    let znames = find_nodes(names, "z");
    process_adder(nodes, &mut values);
    //Err(anyhow!("Not implemented"))
    
    Ok(wires_to_int(&values, &znames))
}

fn calculate_p2(input: &ParsedInput) -> anyhow::Result<String> {
    let (names, nodes) = input;

    let mut nodes = nodes.clone();

    let xnames = find_nodes(names, "x");
    let ynames = find_nodes(names, "y");
    let znames = find_nodes(names, "z");


    swap_nodes(&mut nodes, 305, 127);
    swap_nodes(&mut nodes, 267, 115);
    swap_nodes(&mut nodes, 173, 179);
    swap_nodes(&mut nodes, 299, 192);

    println!("Xnames: {}", xnames.len());
    for bit in 0..xnames.len() {

        let calc_result = add_numbers(1 << bit, 0, &nodes, &xnames, &ynames, &znames);
        if calc_result != 1 << bit {
            println!("Bit #{}, Expect: {:x} Res: {:x}", bit, 1u64 << bit,  calc_result);
        }
    }

    println!("Ynames: {}", ynames.len());
    for bit in 0..ynames.len() {

        let calc_result = add_numbers(0, 1 << bit, &nodes, &xnames, &ynames, &znames);
        if calc_result != 1 << bit {
            println!("Bit #{}, Expect: {:x} Res: {:x}", bit, 1u64 << bit,  calc_result);
        }
    }

    println!("XYnames: {}", ynames.len());
    for bit in 0..ynames.len() {

        let calc_result = add_numbers(1 << bit, 1 << bit, &nodes, &xnames, &ynames, &znames);
        if calc_result != 1 << (bit + 1) {
            println!("Bit #{}, Expect: {:x} Res: {:x}", bit, 1u64 << (bit + 1),  calc_result);
        }
    }


    // for z in znames.iter() {
    //     println!("{} {}, {:?}", *z, names[*z], nodes.iter().find(|i| i.id == *z));
    // }

    // for x in ynames.iter() {
    //     println!("{} {}, {:?}", *x, names[*x], nodes.iter().find(|i| i.id == *x));
    // }


    // print_node(&nodes, names, 299);
    // print_node(&nodes, names, 37);
    // print_node(&nodes, names, 82);
    // nodes_with_input(&nodes, 37);
    // nodes_with_input(&nodes, 273);

    let swaps = vec![
        305, 127,
        267, 115,
        173, 179,
        299, 192,];

    let result = swaps
        .iter()
        .map(|s|&names[*s])
        .sorted()
        .join(",");

    Ok(result)

    //Err(anyhow!("Not implemented"))
}


fn nodes_with_input(nodes: &[Node], i: usize) {
    for n in nodes.iter() {
        if match n.op {
            NodeOp::And(arg0, arg1) => arg0 == i || arg1 == i,
            NodeOp::Or(arg0, arg1) => arg0 == i || arg1 == i,
            NodeOp::Xor(arg0, arg1) => arg0 == i || arg1 == i,
            _ => false,
        }{
            println!("{:?}", n);
        }
        
    }
}

fn print_node(nodes: &[Node], names: &[String], id: usize) {
    let node = nodes.iter().find(|i| i.id == id).unwrap();

    println!("{} {:?}", names[node.id], node);
}

fn swap_nodes(nodes: &mut [Node], n1: usize, n2: usize) {
    let i2 = nodes.iter().position(|i|i.id == n2).unwrap();
    nodes.iter_mut().find(|i|i.id == n1).as_mut().unwrap().id = n2;
    nodes.get_mut(i2).unwrap().id = n1;
}

fn add_numbers(x: u64, y: u64, nodes: &[Node], xnames: &[usize], ynames: &[usize], znames: &[usize]) -> u64 {
    let mut wires = vec![None; nodes.len()];

    int_to_wires(&mut wires, xnames, x);
    int_to_wires(&mut wires, ynames, y);

    process_adder(nodes, &mut wires);

    wires_to_int(&wires, znames)
}

fn process_adder(nodes: &[Node], values: &mut [Option<bool>]) {
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
    //#[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed)?;

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    //#[case(load_sample("sample.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed)?;

        let mut hasher = DefaultHasher::new();
        hasher.write(result2.as_bytes());

        assert_eq!(expected, Some(hasher.finish()));
        Ok(())
    }
}
