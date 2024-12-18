use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use aoc_tools::{Point, Grid, Neighbours2D, NeighbourMap};
use std::collections::{HashSet, BinaryHeap};

type ParsedInput = Vec<Point>;

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed, 70, 70, 1024);
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed);
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    let lines = input.read_lines()?;

    let mut points: Vec<Point> = Vec::new();

    for line in lines.into_iter() {
        let mut p = line.split(',');

        let x: usize = p.next().expect("X").parse().unwrap();
        let y: usize = p.next().expect("Y").parse().unwrap();

        points.push((x, y).into());
    }

    //println!("{:?}", points);

    Ok(points)
}

fn calculate_p1(input: &ParsedInput, width: usize, height: usize, nbytes: usize) -> usize {
    let mut grid: Grid<char> = Grid::new('.', width+1, height+1);

    for p in input.into_iter().take(nbytes) {
        grid[*p] = '#';
    }

    grid.print();

    let start_state = BfsState {
        pos: (0, 0).into(),
        score: 0,
    };

    let mut queue: BinaryHeap<BfsState> = BinaryHeap::new();
    let mut queue_dedup: HashSet<BfsState> = HashSet::new();
    let mut visited: HashSet<Point> = HashSet::new();
    queue.push(start_state);

    let mut best_score: Option<usize> = None;

    while !queue.is_empty() {

        let state = queue.pop().unwrap();
        visited.insert(state.pos);

        println!("V: {} H:{}", visited.len(), queue.len());

        if state.pos == (width, height).into() {
            best_score = Some(state.score);
            break;
        }

        let neigh = Neighbours2D::new(state.pos.into(), grid.size(), NeighbourMap::Plus).filter_map(|f|f);

        for n in neigh {
            let new_state = BfsState {
                pos: n.into(),
                score: state.score + 1,
            };

            if !visited.contains(&new_state.pos) && grid[new_state.pos] != '#' && !queue_dedup.contains(&new_state) {
                queue.push(new_state.clone());
                queue_dedup.insert(new_state);
            }
        }
    }

    best_score.unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct BfsState {
    pos: Point,
    score: usize,
}

impl Ord for BfsState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for BfsState {
    fn partial_cmp(&self, other: &BfsState) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_p2(_input: &ParsedInput) -> u64 {
    0
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

        let result1 = calculate_p1(&parsed, 6, 6, 12);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    //#[case(load_sample("input.txt")?)]
    #[ignore]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
