use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use regex::Regex;

fn main() -> anyhow::Result<()> {

    let input = aoc_tools::Input::from_cmd()?.read_all()?;

    let result1 = extract_and_multiply_1(&input)?;
    println!("Result p1: {}", result1);

    let result2 = 0;
    println!("Result p2: {}", result2);

    Ok(())
}


fn extract_and_multiply_1(input: &str) -> Result<u32, InvalidInput> {
    let re = Regex::new(r"mul\s*\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();

    let mut total: u32 = 0;

    for (_, [n1, n2]) in re.captures_iter(input).map(|c| c.extract()) {

        let arg1 = str::parse::<u32>(n1).map_err_to_invalid_input(n1)?;
        let arg2 = str::parse::<u32>(n2).map_err_to_invalid_input(n2)?;

        total += arg1 * arg2;
    }

    Ok(total)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {

        assert_eq!(1, 1);
    }
}
