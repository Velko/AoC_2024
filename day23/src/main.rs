use std::collections::{HashMap, HashSet};

use aoc_tools::{InvalidInput, IterMoreTools, NameRegistry, ResultExt};

type ParsedInput = (Vec<String>, Vec<(usize, usize)>);

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
    let lines = input.read_lines()?;

    let mut namereg = NameRegistry::new();
    let mut edges = Vec::new();

    for line in lines.into_iter() {
        let (n1, n2) = line.split_once('-').map_err_to_invalid_input(&line)?;

        let p1 = namereg.add_or_lookup(n1);
        let p2 = namereg.add_or_lookup(n2);

        edges.push((p1, p2));
        edges.push((p2, p1));
    }


    Ok((namereg.into(), edges))
}

fn calculate_p1(input: &ParsedInput) -> anyhow::Result<usize> {
    let (namereg, edges) = input;

    let mut index: HashMap<usize, Vec<usize>> = HashMap::new();

    for (s0, s1) in edges.iter() {
        index.entry(*s0).or_insert_with(Vec::new).push(*s1);
    }

    let null_vec = Vec::new();

    let mut sets_of_3: HashSet<[usize; 3]> = HashSet::new();

    for s0 in 0..namereg.len() {

        let s1items = index.get(&s0).unwrap_or(&null_vec);
        for s1 in s1items {
            let s2items = index.get(&s1).unwrap_or(&null_vec);

            for s2 in s2items.iter().filter(|s| s1items.contains(s)) {
                let mut set = [s0, *s1, *s2];
                set.sort();
                sets_of_3.insert(set);
            }
        }
    }


    let names = namereg.as_slice();

    let with_t: HashSet<_> = names
        .iter()
        .enumerate()
        .filter(|(_, n)| n.starts_with("t"))
        .map(|(i, _)| i)
        .collect();

    let sets_containing_t: Vec<_> = sets_of_3
        .iter()
        .filter(|set| set.iter().any(|i| with_t.contains(i)))
        .collect();

    // for set in sets_of_3.iter() {
    //     println!("{},{},{}", names[set[0]], names[set[1]], names[set[2]]);
    // }

    Ok(sets_containing_t.len())
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
    #[case(load_sample("sample.txt")?)]
    #[case(load_sample("input.txt")?)]
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
