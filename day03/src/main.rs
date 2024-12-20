use aoc_tools::{InvalidInput, ResultExt};
use regex::Regex;

fn main() -> anyhow::Result<()> {

    let input = aoc_tools::Input::from_cmd()?.read_all()?;

    let result1 = extract_and_multiply_1(&input)?;
    println!("Result p1: {}", result1);

    let result2 = extract_and_multiply_2(&input)?;
    println!("Result p2: {}", result2);

    Ok(())
}


fn extract_and_multiply_1(input: &str) -> Result<u32, InvalidInput> {
    let re = Regex::new(r"mul\s*\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();

    let mut total: u32 = 0;

    for (_, [n1, n2]) in re.captures_iter(input).map(|c| c.extract()) {

        let arg1: u32 = n1.parse().map_err_to_invalid_input(n1)?;
        let arg2: u32 = n2.parse().map_err_to_invalid_input(n2)?;

        total += arg1 * arg2;
    }

    Ok(total)
}


fn extract_and_multiply_2(input: &str) -> Result<u32, InvalidInput> {
    let re = Regex::new(r"(?:(mul)\s*\(\s*(\d+)\s*,\s*(\d+)\s*\))|(?:(do(?:n't)?)\s*\(\s*\))").unwrap();

    let mut total: u32 = 0;

    let mut enabled = true;

    for caps in re.captures_iter(input) {

        enabled = match caps.get(4).map(|m|m.as_str()) {
            Some("do") => true,
            Some("don't") => false,
            _ => enabled,
        };

        if enabled && caps.get(1).is_some() {
            let n1 = caps.get(2).map(|m| m.as_str()).map_err_to_invalid_input(input)?;
            let n2 = caps.get(3).map(|m| m.as_str()).map_err_to_invalid_input(input)?;
            let arg1 = str::parse::<u32>(n1).map_err_to_invalid_input(n1)?;
            let arg2 = str::parse::<u32>(n2).map_err_to_invalid_input(n2)?;
            total += arg1 * arg2;
        }
    }

    Ok(total)
}
