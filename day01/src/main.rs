use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use itertools::Itertools;

fn main() -> anyhow::Result<()> {

    let input = aoc_tools::Input::from_cmd()?.read_lines()?;
    
    let locations =
        input
            .iter()
            .map(parse_locations)
            .try_collect_vec()?;

    let result1 = match_lists_1(&locations);
    println!("Result p1: {}", result1);

    let result2 = match_lists_2(&locations);
    println!("Result p2: {}", result2);

    Ok(())
}


fn parse_locations<S: AsRef<str>>(dim: S) -> Result<(i32, i32), InvalidInput>
    where S: Into<String>{
    let r = dim.as_ref();
    
    let parsed =
        r
            .split_ascii_whitespace()
            .map(str::parse::<i32>)
            .try_collect_vec()
            .map_err_to_invalid_input(r)?;

    let res: (i32, i32) = parsed
            .into_iter()
            .collect_tuple()
            .map_err_to_invalid_input(r)?;

    Ok(res)
}

fn match_lists_1(locations: &Vec<(i32, i32)>) -> i32 {

    let first_list = 
        locations
            .iter()
            .map(|(first, _)| first)
            .sorted();

    let second_list = 
        locations
            .iter()
            .map(|(_, second)| second)
            .sorted();

    first_list
        .zip(second_list)
        .map(|(first, second)| (first-second).abs())
        .sum()
}

fn match_lists_2(locations: &Vec<(i32, i32)>) -> i32 {

    let first_list = 
        locations
            .iter()
            .map(|(first, _)| first);

    let second_list = 
        locations
            .iter()
            .map(|(_, second)| second)
            .collect_vec();

    first_list
        .map(|&item| second_list
                        .iter()
                        .filter(|&&m| *m == item)
                        .count() as i32 * item)
        .sum()
}

