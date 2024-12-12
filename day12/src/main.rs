use aoc_tools::{IterMoreTools, InvalidInput, ResultExt, Grid};
use std::collections::HashSet;
use std::collections::HashMap;
use aoc_tools::Neighbours2D;

type ParsedInput = (Box<[[Plot; Grid::MAX_WIDTH]]>, (usize, usize));

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
    let grid = input.read_grid()?;

    let plv: Vec<[Plot; Grid::MAX_WIDTH]> = vec![[Plot::default(); Grid::MAX_WIDTH]; grid.height()];

    let mut parsed = plv.into_boxed_slice();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            parsed[y][x].plant = grid[(x, y)];
        }
    }

    Ok((parsed, grid.size()))
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct Plot {
    plant: char,
    id: Option<usize>,
}

#[derive(Default, Debug)]
struct Totals {
    area: usize,
    perimeter: usize,
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let (plots, (width, height)) = input;
    let mut plots = plots.clone();
    // 0 1 2
    // 3   4
    // 5 6 7
    let side_indices = HashSet::from([1, 3, 4, 6]);

    let mut next_id = 0;

    for y in 0..*height {
        for x in 0..*width {

            if plots[y][x].id.is_none() {
                plots[y][x].id = Some(next_id);

                //println!("Plot: {:?}", (x, y));
                fill_neighbours(&mut plots, x, y, *width, *height, next_id);
                next_id += 1;
            }
        }
    }

    print_plots(&plots, *width, *height);

    let mut totals: HashMap<usize, Totals> = HashMap::new();

    for y in 0..*height {
        for x in 0..*width {
            if let Some(plot_id) = plots[y][x].id {

                let total = totals.entry(plot_id).or_default();

                total.area += 1;

                let neigh = get_neighbours(x, y, *width, *height);

                for n_pos in neigh {
                    if let Some((nx, ny)) = n_pos {
                        if plots[ny][nx].id != Some(plot_id) {
                            total.perimeter += 1;
                        }
                    } else {
                        total.perimeter += 1;
                    }
                }

                println!("{:?} {} {:?}", (x, y), plot_id, total);

            } else {
                panic!("Not filled");
            }
        }
    }

    println!("{:?}", totals);

    totals
        .into_iter()
        .map(|(_, t)| t.area * t.perimeter)
        .sum()
}


fn fill_neighbours(plots: &mut [[Plot; Grid::MAX_WIDTH]], x: usize, y: usize, width: usize, height: usize, next_id: usize) {
    let neigh = get_neighbours(x, y, width, height);

    for n_pos in neigh {
        //println!("Neigh: {:?}", n_pos);
        if let Some((nx, ny)) = n_pos {
            if plots[ny][nx].plant == plots[y][x].plant && plots[ny][nx].id.is_none() {
                plots[ny][nx].id = Some(next_id);
                fill_neighbours(plots, nx, ny, width, height, next_id);
            }
        }
    }
}

fn get_neighbours(x: usize, y: usize, width: usize, height: usize) -> impl Iterator<Item = Option<(usize, usize)>> {
    let side_indices = HashSet::from([1, 3, 4, 6]);

    Neighbours2D::new((x, y), (width, height))
        .enumerate()
        .filter(move |(i, _)| side_indices.contains(i))
        .map(|(_, v)| v)
}

fn print_plots(plots: &[[Plot; Grid::MAX_WIDTH]], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            print!("{:?}, ", plots[y][x]);
        }
        println!();
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
    #[case(load_sample("sample_1.txt")?)]
    #[case(load_sample("sample_2.txt")?)]
    //#[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed);

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
