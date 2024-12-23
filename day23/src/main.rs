use std::collections::{HashMap, HashSet};

use aoc_tools::{NameRegistry, ResultExt};
use itertools::Itertools;

type ParsedInput = (Box<[String]>, HashMap<usize, Vec<usize>>);

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
    let mut index: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in lines.into_iter() {
        let (n1, n2) = line.split_once('-').map_err_to_invalid_input(&line)?;

        let p1 = namereg.add_or_lookup(n1);
        let p2 = namereg.add_or_lookup(n2);

        index.entry(p1).or_insert_with(Vec::new).push(p2);
        index.entry(p2).or_insert_with(Vec::new).push(p1);
    }


    let names_vec: Vec<String> = namereg.into();

    Ok((names_vec.into_boxed_slice(), index))
}

fn calculate_p1(input: &ParsedInput) -> anyhow::Result<usize> {
    let (names, index) = input;

    let null_vec = Vec::new();

    let mut sets_of_3: HashSet<[usize; 3]> = HashSet::new();

    for s0 in 0..names.len() {

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

    Ok(sets_containing_t.len())
}

fn calculate_p2(input: &ParsedInput) -> anyhow::Result<String> {
    let (names, index) = input;

    let mut best_intersection = HashSet::new();

    for start in 0..names.len() {
        let intersection = find_best_intersection(index, start);
        if intersection.len() > best_intersection.len() {
            best_intersection = intersection;
        }
    }

    let best_as_str: Vec<_> = best_intersection.iter().map(|i| names[*i].as_str()).collect();
    let passw = best_as_str.into_iter().sorted().join(",");

    Ok(passw)
}

fn find_best_intersection(index: &HashMap<usize, Vec<usize>>, start: usize) -> HashSet<usize> {
    let sets_0 = collect_node_sets(index, start);

    for picks in (1..=sets_0.len()).rev() {
        for comb in sets_0.iter().combinations(picks) {
            let intersection = intersect_all(comb.into_iter());
            if intersection.len() == picks {
                return intersection;
            }
        }
    }

    HashSet::new()
}

fn collect_node_sets(index: &HashMap<usize, Vec<usize>>, start: usize) -> Vec<HashSet<usize>> {
    let mut sets: Vec<HashSet<usize>> = Vec::new();

    let neighbours = index.get(&start).unwrap();
    let mut set = HashSet::new();
    set.insert(start);
    set.extend(neighbours.iter().copied());
    sets.push(set);

    for n in neighbours.iter() {
        let n_neighbours = index.get(n).unwrap();
        let mut n_set = HashSet::new();
        n_set.insert(*n);
        n_set.extend(n_neighbours.iter().copied());
        sets.push(n_set);
    }

    sets
}

fn intersect_all<'a, I>(mut sets: I) -> HashSet<usize>
    where I: Iterator<Item = &'a HashSet<usize>> {
    let mut result = sets.next().unwrap().clone();

    for set in sets {
        result = result.intersection(set).copied().collect();
    }

    result
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
    #[case(load_sample("sample.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed)?;

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed)?;

        println!("Result: {}", result2);
        let mut hasher = DefaultHasher::new();
        hasher.write(result2.as_bytes());

        assert_eq!(expected, Some(hasher.finish()));
        Ok(())
    }
}
