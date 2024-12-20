pub struct Neighbours2D {
    col: usize,
    row: usize,
    width: usize,
    height: usize,
    nmap: u8,
    distance: usize,
    offset_idx: usize,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NeighbourMap {
    TopLeft     = 1 << 0,
    Top         = 1 << 1,
    TopRight    = 1 << 2,
    Left        = 1 << 3,
    Right       = 1 << 4,
    BottomLeft  = 1 << 5,
    Bottom      = 1 << 6,
    BottomRight = 1 << 7,
    Plus = (1 << 1) | (1 << 3) | (1 << 4) | (1 << 6),
    X    = (1 << 0) | (1 << 2) | (1 << 5) | (1 << 7),
    All         = 0xFF,
}

impl Neighbours2D {

    const OFFSETS: [(isize, isize); 8] =
        [(-1, -1), (-1, 0), (-1, 1),
         ( 0, -1),          ( 0, 1),
         ( 1, -1), ( 1, 0), ( 1, 1)];

    pub fn new(position: (usize, usize), size: (usize, usize), nmap: NeighbourMap) -> Self {
        Self::new_with_distance(position, size, 1, nmap)
    }

    pub fn new_with_distance((col, row): (usize, usize), (width, height): (usize, usize), distance: usize, nmap: NeighbourMap) -> Self {
        Self {
            col,
            row,
            width,
            height,
            nmap: nmap as u8,
            distance,
            offset_idx: 0,
        }
    }

    pub fn new_only_valid(position: (usize, usize), size: (usize, usize), nmap: NeighbourMap) -> impl Iterator<Item=(usize, usize)> {
        Self::new(position, size, nmap).filter_map(|n|n)
    }

    fn get_neighbour(&self, off_row: isize, off_col: isize) -> Option<(usize, usize)> {
        let n_col = self.col.checked_add_signed(off_col * self.distance as isize)?;
        let n_row = self.row.checked_add_signed(off_row * self.distance as isize)?;

        if n_col < self.width && n_row < self.height {
            Some((n_col, n_row))
        } else {
            None
        }
    }

}

impl Iterator for Neighbours2D {
    type Item = Option<(usize, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.offset_idx < Self::OFFSETS.len() {
            if self.nmap & (1 << self.offset_idx) != 0 {
                let (off_row, off_col) = Self::OFFSETS[self.offset_idx];
                self.offset_idx += 1;
                return Some(self.get_neighbour(off_row, off_col));
            }
            self.offset_idx += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[test]
    fn test_neighbours_0_0() {
        let mut neigh = Neighbours2D::new((0, 0), (5, 5), NeighbourMap::All);

        assert_eq!(Some(None), neigh.next());
        assert_eq!(Some(None), neigh.next());
        assert_eq!(Some(None), neigh.next());
        assert_eq!(Some(None), neigh.next());
        assert_eq!(Some(Some((1, 0))), neigh.next());
        assert_eq!(Some(None), neigh.next());
        assert_eq!(Some(Some((0, 1))), neigh.next());
        assert_eq!(Some(Some((1, 1))), neigh.next());
        assert_eq!(None, neigh.next());
    }

    #[test]
    fn test_neighbours_4_4() {
        let mut neigh = Neighbours2D::new((4, 4), (5, 5), NeighbourMap::All);

        assert_eq!(Some(Some((3, 3))), neigh.next());
        assert_eq!(Some(Some((4, 3))), neigh.next());
        assert_eq!(Some(None), neigh.next());
        assert_eq!(Some(Some((3, 4))), neigh.next());
        assert_eq!(Some(None), neigh.next());
        assert_eq!(Some(None), neigh.next());
        assert_eq!(Some(None), neigh.next());
        assert_eq!(Some(None), neigh.next());
        assert_eq!(None, neigh.next());
    }

    #[test]
    fn test_neighbours_2_2() {
        let mut neigh = Neighbours2D::new((2, 2), (5, 5), NeighbourMap::All);

        assert_eq!(Some(Some((1, 1))), neigh.next());
        assert_eq!(Some(Some((2, 1))), neigh.next());
        assert_eq!(Some(Some((3, 1))), neigh.next());

        assert_eq!(Some(Some((1, 2))), neigh.next());
        assert_eq!(Some(Some((3, 2))), neigh.next());

        assert_eq!(Some(Some((1, 3))), neigh.next());
        assert_eq!(Some(Some((2, 3))), neigh.next());
        assert_eq!(Some(Some((3, 3))), neigh.next());

        assert_eq!(None, neigh.next());
    }

    #[test]
    fn test_neighbours_0_0_valid_only() {
        let mut neigh = Neighbours2D::new_only_valid((0, 0), (5, 5), NeighbourMap::All);

        assert_eq!(Some((1, 0)), neigh.next());
        assert_eq!(Some((0, 1)), neigh.next());
        assert_eq!(Some((1, 1)), neigh.next());
        assert_eq!(None, neigh.next());
    }

    #[test]
    fn test_neigbours_1_1_x() {
        let mut neigh = Neighbours2D::new((1, 1), (3, 3), NeighbourMap::X);

        assert_eq!(Some(Some((0, 0))), neigh.next());
        assert_eq!(Some(Some((2, 0))), neigh.next());
        assert_eq!(Some(Some((0, 2))), neigh.next());
        assert_eq!(Some(Some((2, 2))), neigh.next());
        assert_eq!(None, neigh.next());
    }

    #[test]
    fn test_neigbours_1_1_plus() {
        let mut neigh = Neighbours2D::new((1, 1), (3, 3), NeighbourMap::Plus);

        assert_eq!(Some(Some((1, 0))), neigh.next());
        assert_eq!(Some(Some((0, 1))), neigh.next());
        assert_eq!(Some(Some((2, 1))), neigh.next());
        assert_eq!(Some(Some((1, 2))), neigh.next());
        assert_eq!(None, neigh.next());
    }

    #[rstest]
    #[case(NeighbourMap::TopLeft, (0, 0))]
    #[case(NeighbourMap::Top, (1, 0))]
    #[case(NeighbourMap::TopRight, (2, 0))]
    #[case(NeighbourMap::Left, (0, 1))]
    #[case(NeighbourMap::Right, (2, 1))]
    #[case(NeighbourMap::BottomLeft, (0, 2))]
    #[case(NeighbourMap::Bottom, (1, 2))]
    #[case(NeighbourMap::BottomRight, (2, 2))]
    fn test_neigbours_1_1_single(#[case] nmap: NeighbourMap, #[case] expected: (usize, usize)) {
        let mut neigh = Neighbours2D::new((1, 1), (3, 3), nmap);

        assert_eq!(Some(Some(expected)), neigh.next());
        assert_eq!(None, neigh.next());
    }

}
