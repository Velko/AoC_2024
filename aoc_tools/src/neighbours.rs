pub struct Neighbours2D {
    col: usize,
    row: usize,
    width: usize,
    height: usize,
    offsets: std::array::IntoIter<(isize, isize), 8>,
}

impl Neighbours2D {

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

impl Iterator for Neighbours2D {
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
    fn test_neighbours_0_0() {
        let mut neigh = Neighbours2D::new(0, 0, 5, 5);

        assert_eq!(Some((1, 0)), neigh.next());
        assert_eq!(Some((0, 1)), neigh.next());
        assert_eq!(Some((1, 1)), neigh.next());
        assert_eq!(None, neigh.next());
    }

    #[test]
    fn test_neighbours_4_4() {
        let mut neigh = Neighbours2D::new(4, 4, 5, 5);

        assert_eq!(Some((3, 3)), neigh.next());
        assert_eq!(Some((4, 3)), neigh.next());
        assert_eq!(Some((3, 4)), neigh.next());
        assert_eq!(None, neigh.next());
    }

    #[test]
    fn test_neighbours_2_2() {
        let mut neigh = Neighbours2D::new(2, 2, 5, 5);

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
