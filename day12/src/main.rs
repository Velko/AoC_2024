use aoc_tools::{Grid, NumExt, Neighbours2D, NeighbourMap};
use std::collections::HashMap;

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
    sides: usize,
}

#[derive(Default, Debug)]
struct Totals {
    area: usize,
    perimeter: usize,
    sides: usize,
}

fn calculate_p1(input: &ParsedInput) -> usize {
    let (plots, (width, height)) = input;
    let mut plots = plots.clone();

    fill_plots(&mut plots, *width, *height);

    let mut totals: HashMap<usize, Totals> = HashMap::new();

    for y in 0..*height {
        for x in 0..*width {
            if let Some(plot_id) = plots[y][x].id {

                let total = totals.entry(plot_id).or_default();

                total.area += 1;

                let neigh = Neighbours2D::new((x, y), (*width, *height), NeighbourMap::Plus);
                for n_pos in neigh {
                    if let Some((nx, ny)) = n_pos {
                        if plots[ny][nx].id != Some(plot_id) {
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

fn fill_plots(plots: &mut [[Plot; Grid::MAX_WIDTH]], width: usize, height: usize) {
    let mut next_id = 0;

    for y in 0..height {
        for x in 0..width {

            if plots[y][x].id.is_none() {
                plots[y][x].id = Some(next_id);

                fill_neighbours(plots, x, y, width, height, next_id);
                next_id += 1;
            }
        }
    }
}

fn fill_neighbours(plots: &mut [[Plot; Grid::MAX_WIDTH]], x: usize, y: usize, width: usize, height: usize, next_id: usize) {
    let neigh = Neighbours2D::new((x, y), (width, height), NeighbourMap::Plus);

    for n_pos in neigh {
        if let Some((nx, ny)) = n_pos {
            if plots[ny][nx].plant == plots[y][x].plant && plots[ny][nx].id.is_none() {
                plots[ny][nx].id = Some(next_id);
                fill_neighbours(plots, nx, ny, width, height, next_id);
            }
        }
    }
}

// fn print_plots(plots: &[[Plot; Grid::MAX_WIDTH]], width: usize, height: usize) {
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
    let (plots, (width, height)) = input;
    let mut plots = plots.clone();

    fill_plots(&mut plots, *width, *height);

    for y in 0..*height {
        let mut current_id: Option<usize> = None;
        for x in 0..*width {
            let up = y.clamped_add_signed(-1, *height);
            let mut is_border = true;
            if let Some(up_y) = up {
                if plots[up_y][x].id == plots[y][x].id {
                    is_border = false;
                }
            }

            if is_border && plots[y][x].id != current_id {
                plots[y][x].sides += 1;
                current_id = plots[y][x].id;
            }

            if !is_border {
                current_id = None
            }
        }
    }

    for y in 0..*height {
        let mut current_id: Option<usize> = None;
        for x in 0..*width {
            let down = y.clamped_add_signed(1, *height);
            let mut is_border = true;
            if let Some(down_y) = down {
                if plots[down_y][x].id == plots[y][x].id {
                    is_border = false;
                }
            }

            if is_border && plots[y][x].id != current_id {
                plots[y][x].sides += 1;
                current_id = plots[y][x].id;
            }

            if !is_border {
                current_id = None
            }
        }
    }

    for x in 0..*width {
        let mut current_id: Option<usize> = None;
        for y in 0..*height {
            let left = x.clamped_add_signed(-1, *width);
            let mut is_border = true;
            if let Some(left_x) = left {
                if plots[y][left_x].id == plots[y][x].id {
                    is_border = false;
                }
            }

            if is_border && plots[y][x].id != current_id {
                plots[y][x].sides += 1;
                current_id = plots[y][x].id;
            }

            if !is_border {
                current_id = None
            }
        }
    }

    for x in 0..*width {
        let mut current_id: Option<usize> = None;
        for y in 0..*height {
            let right = x.clamped_add_signed(1, *width);
            let mut is_border = true;
            if let Some(right_x) = right {
                if plots[y][right_x].id == plots[y][x].id {
                    is_border = false;
                }
            }

            if is_border && plots[y][x].id != current_id {
                plots[y][x].sides += 1;
                current_id = plots[y][x].id;
            }

            if !is_border {
                current_id = None
            }
        }
    }

    let mut totals: HashMap<usize, Totals> = HashMap::new();

    for y in 0..*height {
        for x in 0..*width {
            if let Some(plot_id) = plots[y][x].id {

                let total = totals.entry(plot_id).or_default();

                total.area += 1;
                total.sides += plots[y][x].sides;
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
