use aoc_tools::{IterMoreTools, InvalidInput, ResultExt, Grid, Point, Neighbours2D, NeighbourMap};
use std::collections::{BinaryHeap, HashSet};

type ParsedInput = Grid<char>;

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
    Ok(input.read_grid()?)
}

fn calculate_p1(grid: &ParsedInput) -> anyhow::Result<usize> {
    let mut track: Grid<Option<TrackCell>> = Grid::new(None, grid.width(), grid.height());

    let mut start:Option<Point> = None;

    for (cell, point) in grid.enumerate() {
        if *cell == 'S' {
            start = Some(point);
            break;
        }
    }

    let mut queue: BinaryHeap<BfsState> = BinaryHeap::new();
    queue.push(BfsState {
        pos: start.map_err_to_invalid_input("Start not found")?,
        distance: 0,
    });
    track[start.unwrap()] = Some(TrackCell {
        came_from: None,
        distance: 0,
    });

    let mut end_point: Option<Point> = None;

    while let Some(state) = queue.pop() {
        let new_distance = state.distance + 1;

        if grid[state.pos] == 'E' {
            end_point = Some(state.pos);
            break;
        }

        let neighbours = Neighbours2D::new(state.pos.into(), grid.size(), NeighbourMap::Plus).filter_map(|f|f);

        for neihbour in neighbours {
            if grid[neihbour] != '#' {
                if let Some(reached) = track[neihbour] {
                    if reached.distance > state.distance + 1 {
                        track[neihbour] = Some(TrackCell {
                            came_from: Some(state.pos),
                            distance: new_distance,
                        });
                        queue.push(BfsState {
                            pos: neihbour.into(),
                            distance: new_distance,
                        })
                    }
                } else {
                    track[neihbour] = Some(TrackCell {
                        came_from: Some(state.pos),
                        distance: new_distance,
                    });
                    queue.push(BfsState {
                        pos: neihbour.into(),
                        distance: new_distance,
                    })
                }
            }
        }
    }

    let mut grid = grid.clone();
    let mut p: Option<Point> = end_point;

    while let Some(pos) = p {
        grid[pos] = 'O';
        p = track[pos].unwrap().came_from;
    }

    let mut cheats: HashSet<Cheat> = HashSet::new();

    for (c, pos) in track.enumerate() {
        if let Some(cell) = c {
            let neighbours = Neighbours2D::new_with_distance(pos.into(), track.size(), 2, NeighbourMap::Plus).filter_map(|f|f);
            for n in neighbours {
                if let Some(neihbour) = track[n] {
                    if neihbour.distance > cell.distance && track[pos.middle(&n.into())].is_none() {
                        cheats.insert(Cheat {
                            start: pos,
                            end: n.into(),
                            gain: neihbour.distance - cell.distance,
                        });
                    }
                }
            }
        }
    }


    // println!("{:?}", cheats);
    // println!("{:?}", cheats.len());

    Ok(cheats
        .into_iter()
        //.filter(|c|c.gain >= 100)
        .count())

    // grid.print();


    //Err(anyhow::anyhow!("Result not yet calculated"))
}

fn calculate_p2(_input: &ParsedInput) -> anyhow::Result<u64> {
    Ok(0)
}

#[derive(Debug, Eq, PartialEq)]
struct BfsState {
    pos: Point,
    distance: usize,
}

impl Ord for BfsState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for BfsState {
    fn partial_cmp(&self, other: &BfsState) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Clone, Copy)]
struct TrackCell {
    came_from: Option<Point>,
    distance: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cheat {
    start: Point,
    end: Point,
    gain: usize,
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
    //#[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed)?;

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    //#[case(load_sample("input.txt")?)]
    #[ignore]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed)?;

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
