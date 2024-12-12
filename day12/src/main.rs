use aoc_tools::{Grid, NumExt, Neighbours2D, NeighbourMap};
use std::collections::HashMap;

type ParsedInput = Grid<Plot>;

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
    let in_grid = input.read_grid()?;


    let parsed = in_grid.map(|p| Plot { plant: p, id: None, sides: 0 });

    Ok(parsed)
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct Plot {
    plant: char,
    id: Option<usize>,
    sides: usize,
}

#[derive(Default, Debug)]
struct Totals {
    area: usize,
    perimeter: usize,
    sides: usize,
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let mut plots = input.clone();

    fill_plots(&mut plots);

    let mut totals: HashMap<usize, Totals> = HashMap::new();

    for y in 0..plots.height() {
        for x in 0..plots.width() {
            if let Some(plot_id) = plots[(x, y)].id {

                let total = totals.entry(plot_id).or_default();

                total.area += 1;

                let neigh = Neighbours2D::new((x, y), plots.size(), NeighbourMap::Plus);
                for n_pos in neigh {
                    if let Some((nx, ny)) = n_pos {
                        if plots[(nx, ny)].id != Some(plot_id) {
                            total.perimeter += 1;
                        }
                    } else {
                        total.perimeter += 1;
                    }
                }
            } else {
                panic!("Not filled");
            }
        }
    }

    totals
        .into_iter()
        .map(|(_, t)| t.area * t.perimeter)
        .sum()
}

fn fill_plots(plots: &mut Grid<Plot>) {
    let mut next_id = 0;

    for y in 0..plots.height() {
        for x in 0..plots.width() {

            if plots[(x, y)].id.is_none() {
                plots[(x, y)].id = Some(next_id);

                fill_neighbours(plots, x, y, next_id);
                next_id += 1;
            }
        }
    }
}

fn fill_neighbours(plots: &mut Grid<Plot>, x: usize, y: usize, next_id: usize) {
    let neigh = Neighbours2D::new((x, y), plots.size(), NeighbourMap::Plus);

    for n_pos in neigh {
        if let Some((nx, ny)) = n_pos {
            if plots[(nx, ny)].plant == plots[(x, y)].plant && plots[(nx, ny)].id.is_none() {
                plots[(nx, ny)].id = Some(next_id);
                fill_neighbours(plots, nx, ny, next_id);
            }
        }
    }
}

// fn print_plots(plots: &[[Plot; Grid::<char>::MAX_WIDTH]], width: usize, height: usize) {
//     for y in 0..height {
//         for x in 0..width {
//             if plots[y][x].plant == 'E' {
//                 print!("{:?}", plots[y][x].sides);
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }

fn calculate_p2(input: &ParsedInput) -> usize {
    let mut plots = input.clone();

    fill_plots(&mut plots);

    for y in 0..plots.height() {
        let mut current_id: Option<usize> = None;
        for x in 0..plots.width() {
            let up = y.clamped_add_signed(-1, plots.height());
            let mut is_border = true;
            if let Some(up_y) = up {
                if plots[(x, up_y)].id == plots[(x, y)].id {
                    is_border = false;
                }
            }

            if is_border && plots[(x, y)].id != current_id {
                plots[(x, y)].sides += 1;
                current_id = plots[(x, y)].id;
            }

            if !is_border {
                current_id = None
            }
        }
    }

    for y in 0..plots.height() {
        let mut current_id: Option<usize> = None;
        for x in 0..plots.width() {
            let down = y.clamped_add_signed(1, plots.height());
            let mut is_border = true;
            if let Some(down_y) = down {
                if plots[(x, down_y)].id == plots[(x, y)].id {
                    is_border = false;
                }
            }

            if is_border && plots[(x, y)].id != current_id {
                plots[(x, y)].sides += 1;
                current_id = plots[(x, y)].id;
            }

            if !is_border {
                current_id = None
            }
        }
    }

    for x in 0..plots.width() {
        let mut current_id: Option<usize> = None;
        for y in 0..plots.height() {
            let left = x.clamped_add_signed(-1, plots.width());
            let mut is_border = true;
            if let Some(left_x) = left {
                if plots[(left_x, y)].id == plots[(x, y)].id {
                    is_border = false;
                }
            }

            if is_border && plots[(x, y)].id != current_id {
                plots[(x, y)].sides += 1;
                current_id = plots[(x, y)].id;
            }

            if !is_border {
                current_id = None
            }
        }
    }

    for x in 0..plots.width() {
        let mut current_id: Option<usize> = None;
        for y in 0..plots.height() {
            let right = x.clamped_add_signed(1, plots.width());
            let mut is_border = true;
            if let Some(right_x) = right {
                if plots[(right_x, y)].id == plots[(x, y)].id {
                    is_border = false;
                }
            }

            if is_border && plots[(x, y)].id != current_id {
                plots[(x, y)].sides += 1;
                current_id = plots[(x, y)].id;
            }

            if !is_border {
                current_id = None
            }
        }
    }

    let mut totals: HashMap<usize, Totals> = HashMap::new();

    for y in 0..plots.height() {
        for x in 0..plots.width() {
            if let Some(plot_id) = plots[(x, y)].id {

                let total = totals.entry(plot_id).or_default();

                total.area += 1;
                total.sides += plots[(x, y)].sides;
            } else {
                panic!("Not filled");
            }
        }
    }

    totals
        .into_iter()
        .map(|(_, t)| t.area * t.sides)
        .sum()
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
    #[case(load_sample("input.txt")?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample("sample_3.txt")?)]
    #[case(load_sample("input.txt")?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
