use aoc_tools::{IterMoreTools, InvalidInput, ResultExt, Grid, Point, Direction, Rotation, Neighbours2D, NeighbourMap};
use std::collections::{BinaryHeap, HashMap, HashSet};

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
    let (score, _) = bfs_search(input);

    score
}


fn bfs_search(input: &ParsedInput) -> (usize, HashSet<Point>) {
    let (grid, start) = input;

    let start_state = BfsState {
        pos: *start,
        dir: Direction::Right,
        score: 0,
    };

    let mut queue: BinaryHeap<BfsState> = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut vis_scores: Vec<(Point, usize)> = Vec::new();
    queue.push(start_state);

    let mut best_score: Option<usize> = None;
    let mut end_point: Option<Point> = None;

    while !queue.is_empty() {

        let state = queue.pop().unwrap();
        visited.insert(state.pos);
        vis_scores.push((state.pos, state.score));

        if grid[state.pos] == 'E' {
            best_score = Some(state.score);
            end_point = Some(state.pos);
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


    let mut path: HashSet<Point> = HashSet::new();

    let mut tail = (end_point.unwrap(), best_score.unwrap());

    while tail.0 != *start {

        path.insert(tail.0);

        let neigh = Neighbours2D::new(tail.0.into(), grid.size(), NeighbourMap::Plus).filter_map(|f|f);

        for n in neigh {
            //println!("{:?}", n);
            if let Some(prev) = vis_scores.iter().filter(|(p, s)| *p == n.into() && (s + 1 == tail.1 || s + 1001 == tail.1)).next() {
                tail = *prev;
            }
        }
    }

    path.insert(*start);



    (best_score.unwrap_or(usize::MAX), path)
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
    let (score, visited) = bfs_search(input);
    let (grid, start) = input;

    let start_state = BfsState {
        pos: *start,
        dir: Direction::Right,
        score: 0,
    };

    bfs_wide_search(input)

    //let all_paths = dfs_search(start_state, grid, HashSet::new(), score);

    // let mut grid = grid.clone();

    // for p in all_paths.unwrap() {
    //     grid[p] = 'O';
    // }

    // grid.print();
}

fn bfs_wide_search(input: &ParsedInput) -> usize {
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

    let mut grid = grid.clone();

    for p in all_visited.iter() {
        grid[*p] = 'O';
    }

    grid.print();

    all_visited.len()
    //(best_score.unwrap_or(usize::MAX), path)
}

fn follow_back(end_pos: Point, vis_scores: &HashSet<BfsState>, score: usize, bounds: (usize, usize), all_visited: &mut HashSet<Point>) {
    if score > 0 {
        all_visited.insert(end_pos);
    }
    let ended: Vec<_> = vis_scores
        .iter()
        .filter(|p| p.pos == end_pos && (p.score + 1 == score || p.score + 1001 == score))
        .collect();
    //println!("{:?}", ended);

    for e in ended.into_iter() {
        let from_dir = e.dir.turn(Rotation::Flip);
        let from_pos = e.pos.advance(from_dir, bounds).unwrap();
        
        //println!("From {:?}, {:?}", from_dir, from_pos);
        follow_back(from_pos, vis_scores, e.score, bounds, all_visited);
    }
}

fn dfs_search(state: BfsState, grid: &Grid<char>, mut visited: HashSet<Point>, max_score: usize) -> Option<HashSet<Point>> {

    if grid[state.pos] == 'E' && state.score == max_score {
        visited.insert(state.pos);
        return Some(visited);
    }

    if state.score > max_score {
        return None;
    }

    visited.insert(state.pos);

    let mut forward_res: Option<HashSet<Point>> = None;
    let mut left_res: Option<HashSet<Point>> = None;
    let mut right_res: Option<HashSet<Point>> = None;

    let forward = state.pos.advance(state.dir, grid.size()).unwrap();
    if !visited.contains(&forward) && grid[forward] != '#' {
        let new_state = BfsState {
            pos: forward,
            dir: state.dir,
            score: state.score + 1,
        };
        forward_res = dfs_search(new_state, grid, visited.clone(), max_score);
    }

    let dir_left = state.dir.turn(Rotation::AntiClockwise);
    let left = state.pos.advance(dir_left, grid.size()).unwrap();
    if !visited.contains(&left) && grid[left] != '#' {
        let new_state = BfsState {
            pos: left,
            dir: dir_left,
            score: state.score + 1001,
        };
        left_res = dfs_search(new_state, grid, visited.clone(), max_score);
    }

    let dir_right = state.dir.turn(Rotation::Clockwise);
    let right = state.pos.advance(dir_right, grid.size()).unwrap();
    if !visited.contains(&right) && grid[right] != '#' {
        let new_state = BfsState {
            pos: right,
            dir: dir_right,
            score: state.score + 1001,
        };
        right_res = dfs_search(new_state, grid, visited.clone(), max_score);
    }

    let mut result =HashSet::new();

    if let Some(f) = forward_res {
        result.extend(f);
    }

    if let Some(l) = left_res {
        result.extend(l);
    }

    if let Some(r) = right_res {
        result.extend(r);
    }

    Some(result)
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
    //#[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample.txt")?)]
    //#[case(load_sample("sample_1.txt")?)]
    //#[case(load_sample("input.txt")?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
