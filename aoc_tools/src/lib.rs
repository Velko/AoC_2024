use core::fmt;
use std::{env, io::{self, BufRead, Read}, fs::File, collections::HashMap};

use itertools::Itertools;

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

    pub fn read_grid(&self) -> io::Result<Vec<Vec<char>>> {
        let reader = self.open_file()?;

        reader
            .lines()
            .map(|l| Ok(l?.chars().collect()))
            .collect()
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

pub trait ResultExt<T> {
    fn map_err_to_invalid_input(self, input: &str) -> Result<T, InvalidInput>;
}

impl<T, E> ResultExt<T> for Result<T, E> {
    fn map_err_to_invalid_input(self, input: &str) -> Result<T, InvalidInput> {
        self.map_err(|_| InvalidInput(input.to_owned()))
    }
}

impl<T> ResultExt<T> for Option<T> {
    fn map_err_to_invalid_input(self, input: &str) -> Result<T, InvalidInput> {
        self.ok_or(InvalidInput(input.to_owned()))
    }
}

pub trait IterMoreTools: Iterator {
    fn try_collect_vec<T, E>(self) -> Result<Vec<T>, E>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
    {
        self.collect()
    }

    fn try_collect_map<K, V, E>(self) -> Result<HashMap<K, V>, E>
    where
        Self: Sized + Iterator<Item = Result<(K, V), E>>,
        K: std::hash::Hash + Eq,
    {
        self.collect()
    }

    fn stateful_map<B, F, S>(self, initial_state: S, f: F) -> StatefulMap<Self, F, S>
    where
        Self: Sized,
        F: FnMut(&mut S, Self::Item) -> B,
    {
        StatefulMap::new(self, f, initial_state)
    }

    fn aggregate<B, F>(self, aggregator: F) -> Option<B>
        where
            Self: Sized,
            F: Fn(B, Self::Item) -> B,
            B: From<Self::Item> {
        self.fold(None, |acc, x| {
            Some(match acc {
                Some(acc_val) => aggregator(acc_val, x),
                None => x.into(),
            })
        })
    }
}

pub struct StatefulMap<I, F, S> {
    iter: I,
    f: F,
    state: S,
}

impl<I, F, S> StatefulMap<I, F, S> {
    fn new(iter: I, f: F, state: S) -> StatefulMap<I, F, S> {
        StatefulMap { iter, f, state }
    }
}

impl<B, I: Iterator, F, S> Iterator for StatefulMap<I, F, S>
where
    F: FnMut(&mut S, I::Item) -> B,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        Some((self.f)(&mut self.state, self.iter.next()?))
    }
}

impl<T> IterMoreTools for T where T: Iterator + ?Sized {}


pub struct NameRegistry {
    names: HashMap<String, usize>,
}

impl NameRegistry {
    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
        }
    }

    pub fn add_or_lookup<S: AsRef<str>>(&mut self, name: S) -> usize {
        let next_id = self.names.len();
        *self.names.entry(name.as_ref().to_owned()).or_insert(next_id)
    }
}

impl From<NameRegistry> for Vec<String> {
    fn from(value: NameRegistry) -> Self {
        value.names
            .into_iter()
            .sorted_by_key(|(_, i)| *i)
            .map(|(s, _)| s)
            .collect()
    }
}


pub struct Neigbours2D {
    col: usize,
    row: usize,
    width: usize,
    height: usize,
    offsets: std::array::IntoIter<(isize, isize), 8>,
}


impl Neigbours2D {

    const OFFSETS: [(isize, isize); 8] =
        [(-1, -1), (-1, 0), (-1, 1),
         ( 0, -1),          ( 0, 1),
         ( 1, -1), ( 1, 0), ( 1, 1)];

    pub fn new(col: usize, row: usize, width: usize, height: usize) -> Self {
        Self {
            col,
            row,
            width,
            height,
            offsets: Self::OFFSETS.into_iter(),
        }
    }
}

impl Iterator for Neigbours2D {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((off_row, off_col)) = self.offsets.next()  {
            if let Some(n_col) = self.col.checked_add_signed(off_col)
            {
               if let Some(n_row) = self.row.checked_add_signed(off_row) {
                    if n_col < self.width && n_row < self.height {
                        return Some((n_col, n_row));
                    }
               }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stateful_map() {
        let source = vec![1, 2, 3, 4, 5];

        let dest: Vec<_> = source
            .into_iter()
            .stateful_map(0, |a, f| { *a += f; *a} )
            .collect();


        // 0+1=1, 1+2=3, 3+3=6, 6+4=10, 10+5=15
        assert_eq!(dest, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_neighbours_0_0() {
        let mut neigh = Neigbours2D::new(0, 0, 5, 5);

        assert_eq!(Some((1, 0)), neigh.next());
        assert_eq!(Some((0, 1)), neigh.next());
        assert_eq!(Some((1, 1)), neigh.next());
        assert_eq!(None, neigh.next());
    }

    #[test]
    fn test_neighbours_4_4() {
        let mut neigh = Neigbours2D::new(4, 4, 5, 5);

        assert_eq!(Some((3, 3)), neigh.next());
        assert_eq!(Some((4, 3)), neigh.next());
        assert_eq!(Some((3, 4)), neigh.next());
        assert_eq!(None, neigh.next());
    }

    #[test]
    fn test_neighbours_2_2() {
        let mut neigh = Neigbours2D::new(2, 2, 5, 5);

        assert_eq!(Some((1, 1)), neigh.next());
        assert_eq!(Some((2, 1)), neigh.next());
        assert_eq!(Some((3, 1)), neigh.next());

        assert_eq!(Some((1, 2)), neigh.next());
        assert_eq!(Some((3, 2)), neigh.next());

        assert_eq!(Some((1, 3)), neigh.next());
        assert_eq!(Some((2, 3)), neigh.next());
        assert_eq!(Some((3, 3)), neigh.next());

        assert_eq!(None, neigh.next());
    }
}
