use aoc_tools::{Grid, Neighbours2D, NeighbourMap};
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

    for (plot, xy) in plots.enumerate() {
        if let Some(plot_id) = plot.id {

            let total = totals.entry(plot_id).or_default();

            total.area += 1;

            let neigh = Neighbours2D::new(xy.into(), plots.size(), NeighbourMap::Plus);
            for n_pos in neigh {
                if let Some(nxy) = n_pos {
                    if plots[nxy].id != Some(plot_id) {
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

                fill_neighbours(plots, (x, y), next_id);
                next_id += 1;
            }
        }
    }
}

fn fill_neighbours(plots: &mut Grid<Plot>, xy: (usize, usize), next_id: usize) {
    let neigh = Neighbours2D::new(xy, plots.size(), NeighbourMap::Plus);

    for n_pos in neigh {
        if let Some(nxy) = n_pos {
            if plots[nxy].plant == plots[xy].plant && plots[nxy].id.is_none() {
                plots[nxy].id = Some(next_id);
                fill_neighbours(plots, nxy, next_id);
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
            mark_plot_side(&mut plots, &mut current_id, (x, y), NeighbourMap::Top);
        }
        current_id = None;
        for x in 0..plots.width() {
            mark_plot_side(&mut plots, &mut current_id, (x, y), NeighbourMap::Bottom);
        }
    }

    for x in 0..plots.width() {
        let mut current_id: Option<usize> = None;
        for y in 0..plots.height() {
            mark_plot_side(&mut plots, &mut current_id, (x, y), NeighbourMap::Left);
        }

        current_id = None;
        for y in 0..plots.height() {
            mark_plot_side(&mut plots, &mut current_id, (x, y), NeighbourMap::Right);
        }
    }

    let mut totals: HashMap<usize, Totals> = HashMap::new();

    for (plot, _) in plots.enumerate() {
        if let Some(plot_id) = plot.id {

            let total = totals.entry(plot_id).or_default();

            total.area += 1;
            total.sides += plot.sides;
        } else {
            panic!("Not filled");
        }
    }

    totals
        .into_iter()
        .map(|(_, t)| t.area * t.sides)
        .sum()
}

fn mark_plot_side(plots: &mut Grid<Plot>, current_id: &mut Option<usize>, pos: (usize, usize), chk_side: NeighbourMap) {
    let neigh = Neighbours2D::new(pos, plots.size(), chk_side);
    let mut is_border = true;
    if let Some(n_pos) = neigh.filter_map(|f|f).next() {
        if plots[n_pos].id == plots[pos].id {
            is_border = false;
        }
    }

    if is_border && plots[pos].id != *current_id {
        plots[pos].sides += 1;
        *current_id = plots[pos].id;
    }

    if !is_border {
        *current_id = None
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
