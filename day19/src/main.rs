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

    designs
        .into_iter()
        .filter(|design| count_possible_designs(design, "", &towels, &mut HashMap::new())> 0)
        .count()
}

fn calculate_p2(input: &ParsedInput) -> usize {
    let (towels, designs) = input;

    designs
        .into_iter()
        .map(|design| count_possible_designs(design, "", towels, &mut HashMap::new()))
        .sum()
}

fn count_possible_designs(design: &str, base_str: &str, towels: &[String], memo: &mut HashMap<String, usize>) -> usize {

    if let Some(result) = memo.get(base_str) {
        return *result;
    }

    let mut n_arrangements = 0;

    let mut check_des = String::with_capacity(design.len() * 2);
    check_des.push_str(base_str);

    for towel in towels.iter() {
        check_des.push_str(towel);

        if check_des.len() < design.len() {
            if check_des == design[..check_des.len()] {
                n_arrangements += count_possible_designs(design, &check_des, towels, memo);
            }
        } else {
            if check_des == design {
                n_arrangements += 1;
            }
        }
        check_des.truncate(base_str.len())
    }

    memo.insert(base_str.to_owned(), n_arrangements);
    n_arrangements
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

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
