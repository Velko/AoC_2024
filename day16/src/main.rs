use aoc_tools::{Grid, Point, Direction, Rotation};
use std::collections::{BinaryHeap, HashSet};

type ParsedInput = (Grid<char>, Point);

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?;
    let parsed = parse_input(input)?;

    let result1 = calculate_p1(&parsed);
    println!("Result p1: {}", result1);

    let result2 = calculate_p2(&parsed);
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_input(input: aoc_tools::Input) -> anyhow::Result<ParsedInput> {
    let mut grid = input.read_grid()?;

    let start = grid
        .enumerate()
        .filter(|(c, _)| **c == 'S')
        .map(|(_, p)| p)
        .next()
        .unwrap();

    grid[start] = '.';

    Ok((grid, start))
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let (grid, start) = input;

    let start_state = BfsState {
        pos: *start,
        dir: Direction::Right,
        score: 0,
    };

    let mut queue: BinaryHeap<BfsState> = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::new();
    queue.push(start_state);

    let mut best_score: Option<usize> = None;

    while !queue.is_empty() {

        let state = queue.pop().unwrap();
        visited.insert(state.pos);

        if grid[state.pos] == 'E' {
            best_score = Some(state.score);
            break;
        }

        let forward = state.pos.advance(state.dir, grid.size()).unwrap();
        if !visited.contains(&forward) && grid[forward] != '#' {
            queue.push(BfsState {
                pos: forward,
                dir: state.dir,
                score: state.score + 1,
            });
        }

        let dir_left = state.dir.turn(Rotation::AntiClockwise);
        let left = state.pos.advance(dir_left, grid.size()).unwrap();
        if !visited.contains(&left) && grid[left] != '#' {
            queue.push(BfsState {
                pos: left,
                dir: dir_left,
                score: state.score + 1001,
            });
        }

        let dir_right = state.dir.turn(Rotation::Clockwise);
        let right = state.pos.advance(dir_right, grid.size()).unwrap();
        if !visited.contains(&right) && grid[right] != '#' {
            queue.push(BfsState {
                pos: right,
                dir: dir_right,
                score: state.score + 1001,
            });
        }
    }

    best_score.unwrap()
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct BfsState {
    pos: Point,
    dir: Direction,
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


fn calculate_p2(input: &ParsedInput) -> usize {
    let (grid, start) = input;

    let start_state = BfsState {
        pos: *start,
        dir: Direction::Right,
        score: 0,
    };

    let mut queue: BinaryHeap<BfsState> = BinaryHeap::new();
    let mut visited: HashSet<(Point, Direction)> = HashSet::new();
    let mut vis_scores: HashSet<BfsState> = HashSet::new();
    queue.push(start_state);

    let mut best_score: Option<usize> = None;
    let mut end_point: Option<Point> = None;

    while !queue.is_empty() {

        let state = queue.pop().unwrap();

        if let Some(best) = best_score {
            if state.score > best {
                break;
            }
        }

        visited.insert((state.pos, state.dir));
        vis_scores.insert(state.clone());

        if grid[state.pos] == 'E' {
            best_score = Some(state.score);
            end_point = Some(state.pos);
            continue;
        }

        let forward = state.pos.advance(state.dir, grid.size()).unwrap();
        if !visited.contains(&(forward, state.dir)) && grid[forward] != '#' {
            queue.push(BfsState {
                pos: forward,
                dir: state.dir,
                score: state.score + 1,
            });
        }

        let dir_left = state.dir.turn(Rotation::AntiClockwise);
        let left = state.pos.advance(dir_left, grid.size()).unwrap();
        if !visited.contains(&(left, dir_left)) && grid[left] != '#' {
            queue.push(BfsState {
                pos: left,
                dir: dir_left,
                score: state.score + 1001,
            });
        }

        let dir_right = state.dir.turn(Rotation::Clockwise);
        let right = state.pos.advance(dir_right, grid.size()).unwrap();
        if !visited.contains(&(right, dir_right)) && grid[right] != '#' {
            queue.push(BfsState {
                pos: right,
                dir: dir_right,
                score: state.score + 1001,
            });
        }
    }

    let end_pos = end_point.unwrap();

    let mut all_visited: HashSet<Point> = HashSet::new();

    follow_back(end_pos, &vis_scores, 1 + best_score.unwrap(), grid.size(), &mut all_visited);

    all_visited.len()
}

fn follow_back(end_pos: Point, vis_scores: &HashSet<BfsState>, score: usize, bounds: (usize, usize), all_visited: &mut HashSet<Point>) {
    if score > 0 {
        all_visited.insert(end_pos);
    }
    let ended = vis_scores
        .iter()
        .filter(|p| p.pos == end_pos && (p.score + 1 == score || p.score + 1001 == score));

    for e in ended {
        let from_dir = e.dir.turn(Rotation::Flip);
        let from_pos = e.pos.advance(from_dir, bounds).unwrap();
        
        follow_back(from_pos, vis_scores, e.score, bounds, all_visited);
    }
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
    #[case(load_sample("sample_1.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    #[case(load_sample("sample_1.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
