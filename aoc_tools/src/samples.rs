use std::{io::{self, BufRead}, fs::File};
use std::collections::HashMap;
use itertools::Itertools;
use crate::IterMoreTools;
use crate::ResultExt;
use crate::InvalidInput;
use crate::Input;

pub struct TestSamples {
    samples: HashMap<String, (Option<u64>, Option<u64>)>,
}

impl TestSamples {
    pub fn try_new() -> anyhow::Result<Self> {
        let input = File::open("tests.txt")?;
        let reader = io::BufReader::new(input);

        let rows =
            reader
                .lines()
                .try_collect_vec()?;

        let samples =
            rows
                .into_iter()
                .map(|l| parse_sample_line(l))
                .try_collect_map()?;
        Ok(TestSamples {
            samples
        })
    }

    pub fn get_sample(&self, filename: &str) -> anyhow::Result<(Input, Option<u64>, Option<u64>)> {

        let (exp_result1, exp_result2) = self.samples.get(filename).map_err_to_invalid_input("Invalid sample index")?;

        Ok((crate::Input::from_filename(filename)?, *exp_result1, *exp_result2))
    }
}

fn parse_sample_line(s: String) -> Result<(String, (Option<u64>, Option<u64>)), InvalidInput> {
    let (filename, expected) = s
        .split('=')
        .map(|p| p.trim())
        .collect_tuple()
        .map_err_to_invalid_input(s.as_str())?;

    let (first, second) = expected
        .split(',')
        .map(|p| p.trim().parse::<u64>().ok())
        .chain(Some(None).into_iter())
        .take(2)
        .collect_tuple()
        .map_err_to_invalid_input(expected)?;

    Ok((filename.to_owned(), (first, second)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_samples() -> anyhow::Result<()> {
        let samples = TestSamples::try_new()?;

        assert_eq!(HashMap::from(
            [("sample.txt".to_owned(), (Some(421), None)),
             ("sample2.txt".to_owned(), (None, Some(1))),
             ("both.txt".to_owned(), (Some(3), Some(4)))]),
            samples.samples);

        Ok(())
    }

    #[test]
    fn test_create_input() -> anyhow::Result<()> {

        let samples = TestSamples::try_new()?;

        let (input, result1, _) = samples.get_sample("sample.txt")?;


        let input_text = input.read_all()?;

        assert_eq!(result1, Some(421));
        assert_eq!("Here there be tigers!\n", input_text);

        Ok(())
    }
}