use std::collections::HashMap;

use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use itertools::Itertools;

type ParsedInput = Vec<u32>;

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

    let parsed = lines
        .into_iter()
        .map(|line| {
            line.parse()
                .map_err_to_invalid_input(&line)
        })
        .try_collect_vec()?;

    Ok(parsed)
}

fn calculate_p1(input: &ParsedInput) -> anyhow::Result<u64> {
    
    Ok(input
        .into_iter()
        .map(|seed|generate_secret(*seed))
        .sum()
    )
}

fn calculate_p2(input: &ParsedInput) -> anyhow::Result<u32> {
    
    let buyers: Vec<_> = input
        .into_iter()
        .map(|seed|BuyerPrices::new(*seed))
        .collect();
    
    let mut total_price_index: HashMap<[i8; 4], u32> = HashMap::new();

    for buyer in buyers {

        let mut price_index: HashMap<[i8; 4], u32> = HashMap::new();
        //println!("{:?}\n   {:?}", &buyer.prices[..10], &buyer.changes[..9]);
        for i in 4..buyer.prices.len() {
            let changes: [i8; 4] = buyer.changes[i-4..i].try_into().unwrap();
            let _ = price_index.entry(changes).or_insert(*buyer.prices.get(i).unwrap() as u32);
        }

        //println!("Found: {:?}", price_index.get(&[-2,1,-1,3]).unwrap_or(&0));

        for (changes, price) in price_index {
            let total_price = total_price_index.entry(changes).or_insert(0);
            *total_price += price;
        }
    }

    let max_price = total_price_index.values().max().unwrap();


    Ok(*max_price)
}

struct BuyerPrices {
    prices: Vec<u8>,
    changes: Vec<i8>,
}

impl BuyerPrices {
    fn new(seed: u32) -> Self {
        let prices: Vec<u8> = SecretGenerator::new(seed)
            .take(2000)
            .map(|secret| (secret % 10) as u8)
            .collect();
        let changes = prices.iter().tuple_windows::<(_, _)>()
            .map(|(a, b)| *b as i8 - *a as i8)
            .collect();

        Self {
            prices,
            changes,
        }
    }
}

struct SecretGenerator {
    value: u64,
}

impl SecretGenerator {
    fn new(seed: u32) -> Self {
        Self {
            value: seed as u64,
        }
    }
}

impl Iterator for SecretGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.value;
        self.value ^= self.value * 64;
        self.value %= 16777216;
        
        self.value ^= self.value / 32;
        self.value %= 16777216;
        
        self.value ^= self.value * 2048;
        self.value %= 16777216;
        
        Some(result)
    }
}

fn generate_secret(seed: u32) -> u64 {
    SecretGenerator::new(seed).nth(2000).unwrap()
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
    #[case(load_sample("sample_1.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed)?;

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }

    #[test]
    fn test_prices() {
        let buyer = BuyerPrices::new(486);

        assert_eq!(vec![6, 9, 0, 1, 4, 0, 7, 4, 1, 7].as_slice(), &buyer.prices[..10]);
    }

    #[test]
    fn test_changes() {
        let buyer = BuyerPrices::new(486);

        assert_eq!(vec![3, -9, 1, 3, -4, 7, -3, -3, 6].as_slice(), &buyer.changes[..9]);
    }
}
