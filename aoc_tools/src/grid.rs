use std::{default::Default, fmt::{Display, Result}, io::{self, BufRead}, ops::{Index, IndexMut}};
use std::io::Lines;

#[derive(Clone)]
pub struct Grid<T>
    where T: Sized
{
    content: Box<[[T; GRID_MAX_WIDTH]]>,
    _width: usize,
    _height: usize,
}

const GRID_MAX_WIDTH: usize = 256;
const GRID_MAX_HEIGHT: usize = 256;


impl<T> Grid<T>
    where T: Sized + Default + Copy
{
    pub const MAX_WIDTH: usize = GRID_MAX_WIDTH;
    pub const MAX_HEIGHT: usize = GRID_MAX_HEIGHT;

    pub fn new(value: T, width: usize, height: usize) -> Self {
        Grid {
            content: vec![[value; GRID_MAX_WIDTH]; GRID_MAX_HEIGHT].into_boxed_slice(),
            _width: width,
            _height: height,
        }
    }

    pub fn enumerate(&self) -> GridEnumerator<T> {
        GridEnumerator {
            grid: self,
            col: 0,
            row: 0,
        }
    }

    pub fn width(&self) -> usize {
        self._width
    }

    pub fn height(&self) -> usize {
        self._height
    }

    pub fn size(&self) -> (usize, usize) {
        (self._width, self._height)
    }

    pub fn map<U, F>(&self, f: F) -> Grid<U>
    where
        U: Sized + Default + Copy,
        F: Fn(T) -> U,
    {
        let mut content = Vec::with_capacity(self._height);

        for row in self.content.iter() {
            let mut row_content = [U::default(); GRID_MAX_WIDTH];
            for (col, val) in row.iter().enumerate() {
                row_content[col] = f(*val);
            }

            content.push(row_content);
        }

        Grid {
            content: content.into_boxed_slice(),
            _width: self._width,
            _height: self._height,
        }
    }

}

impl<T> Grid<T>
    where T: Display
{
    pub fn print(&self) {
        for row in self.content.iter().take(self._height) {
            let line: String = row
                .into_iter()
                .take(self._width)
                .map(|v|v.to_string())
                .collect();
            println!("{}", line);
        }
    }
}

impl Grid<char> {
    pub fn try_from_reader<U>(input: U) -> io::Result<Self>
        where U: BufRead
    {
        Self::try_from_lines(input.lines())
    }

    pub fn try_from_lines<I>(lines: I) -> io::Result<Self>
        where I: Iterator<Item=io::Result<String>>
    {
        let mut content = Vec::with_capacity(Self::MAX_HEIGHT);
        let mut _width = 0;

        for row in lines {
            let mut col_idx = 0;

            let mut row_content = [char::default(); Self::MAX_WIDTH];
            for c in row?.chars() {
                row_content[col_idx] = c;
                col_idx += 1;
            }

            if content.len() == 0 {
                _width = col_idx;
            } else {
                //TODO: better error handling
                assert_eq!(_width, col_idx);
            }
            content.push(row_content);
        }

        let _height = content.len();

        Ok(Self {
            content: content.into_boxed_slice(),
            _width,
            _height,
        })
    }
}


pub struct GridEnumerator<'a, T> {
    grid: &'a Grid<T>,
    col: usize,
    row: usize,
}

impl<'a, T> Iterator for GridEnumerator<'a, T> {
    type Item = (&'a T, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.grid._height {
            let item = (&self.grid.content[self.row][self.col], (self.col, self.row));
            self.col += 1;
            if self.col >= self.grid._width {
                self.col = 0;
                self.row += 1;
            }
            Some(item)
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.content[row][col]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {

    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        &mut self.content[row][col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_default_val() {
        let grid = Grid::<char>::new('.', 10, 10);

        assert_eq!((10, 10), grid.size());
    }

    #[test]
    fn test_grid_from_reader() {

        let buffer = "123\n456\n789\n".as_bytes();


        let grid = Grid::try_from_reader(buffer).unwrap();

        assert_eq!((3, 3), grid.size());
    }
}