use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use itertools::Itertools;
use std::cmp::Ordering;

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?.read_lines()?;

    let rules = input
        .iter()
        .take_while(|l| !l.is_empty())
        .map(parse_rules)
        .try_collect_vec()?;

    let updates = input
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(1)
         .map(parse_update)
         .try_collect_vec()?;

    let safe_updates: Vec<_> = updates
        .iter()
        .filter(|u|is_update_safe(u, &rules))
        .collect();

    let result1: u32 = safe_updates
        .iter()
        .map(|&mp| extract_middle_page(mp))
        .sum();

    println!("Result p1: {}", result1);

    let fixed_updates: Vec<_> = updates
        .iter()
        .filter(|u|!is_update_safe(u, &rules))
        .map(|u| fix_unsafe_update(u, &rules))
        .collect();

    let result2: u32 = fixed_updates
        .iter()
        .map(|mp| extract_middle_page(mp))
        .sum();

    println!("Result p2: {}", result2);

    Ok(())
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

fn is_update_safe(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
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


fn fix_unsafe_update(bad_one: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut fixed = bad_one.clone();

    fixed.sort_by(|a, b| cmp_by_rules(a, b, rules));

    fixed
}

fn cmp_by_rules(a: &u32, b: &u32, rules: &Vec<(u32, u32)>) -> Ordering {
    let mut page_rules = rules
        .iter()
        .filter(|(p, _)| p == a)
        .map(|(_, p)| p );

    if page_rules.contains(b) {
        Ordering::Less
    } else if a == b {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {

        assert_eq!(1, 1);
    }
}
