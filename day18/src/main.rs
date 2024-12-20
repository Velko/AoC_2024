use aoc_tools::{ResultExt, Point, Grid, Neighbours2D, NeighbourMap};
use std::collections::{HashSet, BinaryHeap};

type ParsedInput = Vec<Point>;

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed, 70, 70, 1024)?;
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed, 70, 70)?;
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    let lines = input.read_lines()?;

    let mut points: Vec<Point> = Vec::new();

    for line in lines.into_iter() {
        let (x, y) = line.split_once(',').map_err_to_invalid_input(&line)?;

        points.push((
            x.parse().map_err_to_invalid_input(x)?,
            y.parse().map_err_to_invalid_input(x)?,
        ).into());
    }

    Ok(points)
}

fn calculate_p1(input: &ParsedInput, width: usize, height: usize, nbytes: usize) -> anyhow::Result<usize> {
    traverse_grid(input, width, height, nbytes).ok_or_else(|| anyhow::anyhow!("Failed to traverse grid"))
}

fn traverse_grid(input: &ParsedInput, width: usize, height: usize, nbytes: usize) -> Option<usize> {

    let mut grid: Grid<char> = Grid::new('.', width+1, height+1);

    for p in input.into_iter().take(nbytes) {
        grid[*p] = '#';
    }

    let start_state = BfsState {
        pos: (0, 0).into(),
        score: 0,
    };

    let mut queue: BinaryHeap<BfsState> = BinaryHeap::new();
    let mut queue_dedup: HashSet<BfsState> = HashSet::new();
    let mut visited: HashSet<Point> = HashSet::new();
    queue.push(start_state);

    let mut best_score: Option<usize> = None;

    while let Some(state) = queue.pop() {

        visited.insert(state.pos);

        if state.pos == (width, height).into() {
            best_score = Some(state.score);
            break;
        }

        let neigh = Neighbours2D::new_only_valid(state.pos.into(), grid.size(), NeighbourMap::Plus);

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

    best_score
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

fn calculate_p2(input: &ParsedInput, width: usize, height: usize) -> anyhow::Result<String> {
    let mut lo = 0;
    let mut hi = input.len();

    while lo < hi {
        let i = (lo + hi) / 2;

        let steps = traverse_grid(input, width, height, i);

        if steps.is_some() {
            lo = i + 1;
        } else {
            hi = i;
        }
    }

    let answer = input.get(lo - 1).ok_or_else(|| anyhow::anyhow!("Failed to get answer"))?;

    Ok(format!("{},{}", answer.x, answer.y))
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use rstest::rstest;
    use super::*;
    use aoc_tools::TestSamples;

    fn load_sample(filename: &str) -> anyhow::Result<(ParsedInput, Option<u64>, Option<u64>)> {
        let samples = TestSamples::try_new()?;
        let (input, expected1, expected2) = samples.get_sample(filename)?;
        let parsed = parse_input(input)?;
        Ok((parsed, expected1, expected2))
    }

    fn encode_coordinates(s: &str) -> Result<u64, ParseIntError> {
        s
            .chars()
            .filter(|c|c.is_digit(10))
            .collect::<String>()
            .parse()
    }

    #[rstest]
    #[case(load_sample("sample.txt")?, 6, 12)]
    #[case(load_sample("input.txt")?, 70, 1024)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>), #[case] size: usize, #[case] nbytes: usize) -> anyhow::Result<()> {
        let result1 = calculate_p1(&parsed, size, size, nbytes)?;

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?, 6)]
    #[case(load_sample("input.txt")?, 70)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>), #[case] size: usize) -> anyhow::Result<()> {
        let result2 = encode_coordinates(&calculate_p2(&parsed, size, size)?)?;

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
