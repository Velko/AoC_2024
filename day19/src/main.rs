use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};

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
    println!("{:?}", towels);
    println!("{:?}", designs);

    let towel_it = towels.clone().into_iter();

    designs
        .into_iter()
//        .skip(1)
//        .take(1)
        .filter(|design| can_build_design(design, "", towel_it.clone()))
        .count()
}

fn can_build_design<I, S>(design: &str, base_str: &str, towels: I) -> bool
    where I: Iterator<Item = S> + Clone,
        S: AsRef<str> + std::fmt::Display

{
    //println!("\nBase: {} ? '{}'", design, base_str);
    let tow_i = towels.clone();

    for towel in towels {
        let check_des = format!("{}{}", base_str, towel);
        //println!("Checking: {}", check_des);
        if check_des.len() < design.len() {
            if check_des == design[..check_des.len()] {
                //println!("Level");
                let inner_res = can_build_design(design, &check_des, tow_i.clone());

                if inner_res {
                    return true;
                }
            }
        } else {
            //println!("Final: {:?}", check_des == design);
            if check_des == design {
                return true;
            }
        }
    }
    false
}



fn calculate_p2(_input: &ParsedInput) -> u64 {
    0
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
    #[ignore]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
