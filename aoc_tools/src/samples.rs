use std::{io::{self, BufRead}, fs::File};
use itertools::Itertools;
use crate::IterMoreTools;
use crate::ResultExt;
use crate::InvalidInput;
use crate::Input;

pub struct TestSamples {
    samples: Vec<(String, u64)>,
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
                .try_collect_vec()?;
        Ok(TestSamples {
            samples
        })
    }

    pub fn get_sample(&self, index: usize) -> anyhow::Result<(Input, u64)> {

        let (filename, exp_result) = self.samples.get(index).map_err_to_invalid_input("Invalid sample index")?;

        Ok((crate::Input::from_filename(filename)?, *exp_result))
    }
}

fn parse_sample_line(s: String) -> Result<(String, u64), InvalidInput> {
    let (filename, expected) = s
        .split('=')
        .map(|p| p.trim())
        .collect_tuple()
        .map_err_to_invalid_input(s.as_str())?;

    let val = expected
        .parse::<u64>()
        .map_err_to_invalid_input(expected)?;

    Ok((filename.to_owned(), val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_samples() -> anyhow::Result<()> {
        let samples = TestSamples::try_new()?;

        assert_eq!(vec!
            [("sample.txt".to_owned(), 421),
             ("sample2.txt".to_owned(), 1)],
            samples.samples);

        Ok(())
    }

    #[test]
    fn test_create_input() -> anyhow::Result<()> {

        let samples = TestSamples::try_new()?;

        let (input, result) = samples.get_sample(0)?;


        let input_text = input.read_all()?;

        assert_eq!(result, 421);
        assert_eq!("Here there be tigers!\n", input_text);

        Ok(())
    }
}