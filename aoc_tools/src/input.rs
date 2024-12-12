use core::fmt;
use std::{env, io::{self, BufRead, Read}, fs::File};
use crate::Grid;


pub struct Input {
    filename: String,
}

impl Input {
    pub fn from_cmd() -> Result<Self, NoInputFileArg> {
        Ok(Self {
            filename: env::args()
                .skip(1)
                .next()
                .ok_or(NoInputFileArg)?,
        })
    }

    pub fn from_filename(filename: &str) -> Result<Self, NoInputFileArg> {
        Ok(Self {
            filename: filename.to_owned(),
        })
    }

    pub fn read_single_line(&self) -> io::Result<String> {
        let mut reader = self.open_file()?;

        let mut contents = String::new();
        reader.read_line(&mut contents)?;

        while contents.ends_with(|p| p == '\n' || p == '\r') {
            contents.pop();
        }

        Ok(contents)
    }

    pub fn read_lines(&self) -> io::Result<Vec<String>> {
        let reader = self.open_file()?;

        reader
            .lines()
            .collect()
    }

    pub fn read_all(&self) -> io::Result<String> {
        let mut reader = self.open_file()?;

        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        Ok(contents)
    }

    pub fn read_grid(&self) -> io::Result<Grid<char>> {
        let mut reader = self.open_file()?;

        Grid::<char>::try_from_reader(&mut reader)
    }

    fn open_file(&self) -> io::Result<io::BufReader<File>> {
        let input = File::open(&self.filename)?;
        Ok(io::BufReader::new(input))
    }
}

#[derive(PartialEq)]
pub struct NoInputFileArg;

impl fmt::Display for NoInputFileArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No input file specified!")
    }
}

impl fmt::Debug for NoInputFileArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for NoInputFileArg { }


#[derive(Debug, PartialEq)]
pub struct InvalidInput(pub String);

impl fmt::Display for InvalidInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid input '{}'", self.0)
    }
}

impl std::error::Error for InvalidInput { }

impl InvalidInput {
    pub fn err<T>(text: &str) -> Result<T, InvalidInput> {
        Err(Self(text.to_owned()))
    }

    pub fn err_char<T>(chr: char) -> Result<T, InvalidInput> {
        Err(Self(chr.to_string()))
    }
}
