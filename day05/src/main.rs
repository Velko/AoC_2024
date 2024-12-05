use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashSet, HashMap};

type RuleMap = HashMap<u32, HashSet<u32>>;
type Updates = Vec<Vec<u32>>;

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;

    let (updates, rule_map) = parse_input(input)?;

    let result1 = calculate_p1(&updates, &rule_map);
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&updates, &rule_map);
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<(Updates, RuleMap)> {

    let lines = input.read_lines()?;

    let rules = lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(parse_rules)
        .try_collect_vec()?;

    let updates: Updates = lines
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(1)
         .map(parse_update)
         .try_collect_vec()?;

    let rule_map: RuleMap = rules
        .iter()
        .sorted_by_key(|(k, _)| k)
        .chunk_by(|(k, _)| k)
        .into_iter()
        .map(|(key, val)| (*key,
            val
                .map(|(_, v)| *v)
                .collect()
        ))
        .collect();
    Ok((updates, rule_map))
}

fn parse_rules<S: AsRef<str>>(dim: S) -> Result<(u32, u32), InvalidInput>
    where S: Into<String>{
    let r = dim.as_ref();
    let parsed =
        r
            .split('|')
            .map(str::parse::<u32>)
            .try_collect_vec()
            .map_err_to_invalid_input(r)?;

    let res: (u32, u32) = parsed
        .into_iter()
        .collect_tuple()
        .map_err_to_invalid_input(r)?;

    Ok(res)
}

fn parse_update<S: AsRef<str>>(dim: S) -> Result<Vec<u32>, InvalidInput>
    where S: Into<String>{
    let r = dim.as_ref();
    let parsed =
        r
            .split(',')
            .map(str::parse::<u32>)
            .try_collect_vec()
            .map_err_to_invalid_input(r)?;

    Ok(parsed)
}


fn calculate_p1(updates: &Updates, rule_map: &RuleMap) -> u32 {

    let safe_updates: Vec<_> = updates
    .iter()
    .filter(|u|is_update_safe(u, rule_map))
    .collect();

    let result1: u32 = safe_updates
        .iter()
        .map(|&mp| extract_middle_page(mp))
        .sum();

    result1
}

fn calculate_p2(updates: &Updates, rule_map: &RuleMap) -> u32 {

    let fixed_updates: Vec<_> = updates
        .iter()
        .filter(|u|!is_update_safe(u, &rule_map))
        .map(|u| fix_unsafe_update(u, &rule_map))
        .collect();

    let result2: u32 = fixed_updates
        .iter()
        .map(|mp| extract_middle_page(mp))
        .sum();

    result2
}


fn is_update_safe(update: &Vec<u32>, rules: &RuleMap) -> bool {
    update
        .iter()
        .is_sorted_by(|a, b| cmp_by_rules(a, b, rules) != Ordering::Greater)
}

fn extract_middle_page(update: &Vec<u32>) -> u32 {
    assert!(update.len() & 1 == 1);
    let mid_idx = update.len() / 2;

    let val = update.get(mid_idx).unwrap();

    *val
}


fn fix_unsafe_update(bad_one: &Vec<u32>, rules: &RuleMap) -> Vec<u32> {
    let mut fixed = bad_one.clone();

    fixed.sort_by(|a, b| cmp_by_rules(a, b, rules));

    fixed
}

fn cmp_by_rules(a: &u32, b: &u32, page_rules: &RuleMap) -> Ordering {
    if let Some(rule) = page_rules.get(a) {
        if rule.contains(b) {
            return Ordering::Less
        }
    }

    if a == b {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use aoc_tools::TestSamples;

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        let samples = TestSamples::try_new()?;
        let (input, expected) = samples.get_sample(0)?;

        let (updates, rule_map) = parse_input(input)?;

        let result1 = calculate_p1(&updates, &rule_map).into();

        assert_eq!(expected, result1);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        let samples = TestSamples::try_new()?;
        let (input, expected) = samples.get_sample(1)?;

        let (updates, rule_map) = parse_input(input)?;

        let result2 = calculate_p2(&updates, &rule_map).into();

        assert_eq!(expected, result2);
        Ok(())
    }
}
