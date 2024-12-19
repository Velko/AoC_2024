use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use std::collections::HashMap;

type ParsedInput = (Vec<String>, Vec<String>);

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed);
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed);
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    let mut lines = input.read_lines()?;

    let avail = lines.remove(0);
    lines.remove(0);

    let towels = avail
        .split(',')
        .map(|s|s.trim().to_owned())
        .collect();

    Ok((towels, lines))
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let (towels, designs) = input;
    //println!("{:?}", towels);
    //println!("{:?}", designs);

    designs
        .into_iter()
//        .skip(1)
//        .take(1)
        .filter(|design| can_build_design(design, "", &towels, &mut HashMap::new())> 0)
        .count()
}

fn can_build_design(design: &str, base_str: &str, towels: &[String], memo: &mut HashMap<String, usize>) -> usize {

    if memo.contains_key(base_str) {
        return *memo.get(base_str).unwrap();
    }

    let mut n_arrangements = 0;
    //println!("{}", design);

    for towel in towels.iter() {
        let check_des = format!("{}{}", base_str, towel);
        //println!("Checking: {}", check_des);
        if check_des.len() < design.len() {
            if check_des == design[..check_des.len()] {
                //println!("Level");
                n_arrangements += can_build_design(design, &check_des, towels, memo);

            }
        } else {
            //println!("Final: {:?}", check_des == design);
            if check_des == design {
                n_arrangements += 1;
            }
        }
    }

    memo.insert(base_str.to_owned(), n_arrangements);
    n_arrangements
}



fn calculate_p2(input: &ParsedInput) -> usize {
    let (towels, designs) = input;

    designs
        .into_iter()
//        .skip(1)
//        .take(1)
        .map(|design| can_build_design(design, "", towels, &mut HashMap::new()))
        .sum()

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
    //#[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    //#[case(load_sample("input.txt")?)]
    //#[ignore]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
