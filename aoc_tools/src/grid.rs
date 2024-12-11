use std::{io::{self, BufRead}, ops::Index};

pub struct Grid {
    content: Box<[[char; Self::MAX_WIDTH]]>,
    _width: usize,
    _height: usize,
}

impl Grid {
    const MAX_WIDTH: usize = 256;
    const MAX_HEIGHT: usize = 256;

    pub fn new() -> Self {
        Grid {
            content: vec![[char::default(); Self::MAX_WIDTH]; Self::MAX_HEIGHT].into_boxed_slice(),
            _width: 0,
            _height: 0,
        }
    }

    pub fn try_from_reader(input: &mut impl io::BufRead) -> io::Result<Self>
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

    pub fn enumerate(&self) -> GridEnumerator {
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
}

pub struct GridEnumerator<'a> {
    grid: &'a Grid,
    col: usize,
    row: usize,
}

impl<'a> Iterator for GridEnumerator<'a> {
    type Item = (&'a char, (usize, usize));

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

impl Index<(usize, usize)> for Grid {
    type Output = char;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.content[row][col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_empty() {
        let grid = Grid::new();

        assert_eq!(0, grid.width());
        assert_eq!(0, grid.height());
    }

    #[test]
    fn test_grid_from_reader() {

        let mut buffer = "123\n456\n789\n".as_bytes();


        let grid = Grid::try_from_reader(&mut buffer).unwrap();

        assert_eq!(3, grid.width());
        assert_eq!(3, grid.height());
    }
}