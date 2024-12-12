use std::{io::{self, BufRead}, ops::Index, default::Default};

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

    pub fn new() -> Self {
        Grid {
            content: vec![[T::default(); GRID_MAX_WIDTH]; GRID_MAX_HEIGHT].into_boxed_slice(),
            _width: 0,
            _height: 0,
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
}

impl Grid<char> {
    pub fn try_from_reader<U>(input: U) -> io::Result<Self>
        where U: BufRead
    {
        let mut content = Vec::with_capacity(Self::MAX_HEIGHT);
        let mut _width = 0;

        for row in input.lines() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_empty() {
        let grid = Grid::<char>::new();

        assert_eq!((0, 0), grid.size());
    }

    #[test]
    fn test_grid_from_reader() {

        let buffer = "123\n456\n789\n".as_bytes();


        let grid = Grid::try_from_reader(buffer).unwrap();

        assert_eq!((3, 3), grid.size());
    }
}