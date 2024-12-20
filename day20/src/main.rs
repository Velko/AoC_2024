use aoc_tools::{ResultExt, Grid, Point, Neighbours2D, NeighbourMap};
use std::collections::{BinaryHeap, HashSet};
use itertools::Itertools;

type ParsedInput = Grid<char>;

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed, 100)?;
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed)?;
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    Ok(input.read_grid()?)
}

fn calculate_p1(grid: &ParsedInput, limit: usize) -> anyhow::Result<usize> {
    let track = fill_track(grid)?;

    let mut cheats: HashSet<Cheat> = HashSet::new();

    for (c, pos) in track.enumerate() {
        if let Some(cell) = c {
            let neighbours = Neighbours2D::new_with_distance(pos.into(), track.size(), 2, NeighbourMap::Plus).filter_map(|f|f);
            for n in neighbours {
                if let Some(neihbour) = track[n] {
                    if neihbour.distance > cell.distance + 2 {
                        cheats.insert(Cheat {
                            start: pos,
                            end: n.into(),
                            gain: neihbour.distance - cell.distance - 2,
                        });
                    }
                }
            }
        }
    }


    for c in cheats.iter().sorted_by_key(|s|s.gain) {
        println!("{:?}", c);
    }

    Ok(cheats
        .into_iter()
        .filter(|c|c.gain >= limit)
        .count())
}

fn calculate_p2(_input: &ParsedInput) -> anyhow::Result<u64> {
    Ok(0)
}


fn fill_track(grid: &ParsedInput) -> anyhow::Result<Grid<Option<TrackCell>>> {
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
        distance: 0,
    });

    while let Some(state) = queue.pop() {
        let new_distance = state.distance + 1;

        if grid[state.pos] == 'E' {
            break;
        }

        let neighbours = Neighbours2D::new(state.pos.into(), grid.size(), NeighbourMap::Plus).filter_map(|f|f);

        for neihbour in neighbours {
            if grid[neihbour] != '#' {
                if let Some(reached) = track[neihbour] {
                    if reached.distance > state.distance + 1 {
                        track[neihbour] = Some(TrackCell {
                            distance: new_distance,
                        });
                        queue.push(BfsState {
                            pos: neihbour.into(),
                            distance: new_distance,
                        })
                    }
                } else {
                    track[neihbour] = Some(TrackCell {
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

    Ok(track)
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
    #[case(load_sample("sample.txt")?, 2)]
    #[case(load_sample("input.txt")?, 100)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>), #[case] limit: usize) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed, limit)?;

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
