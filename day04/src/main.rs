use aoc_tools::{InvalidInput, ResultExt, Neighbours2D};
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?.read_grid()?;

    let result1 = find_words_1(&input)?;
    println!("Result p1: {}", result1);

    let result2 = find_x_2(&input)?;
    println!("Result p2: {}", result2);

    Ok(())
}


fn find_words_1(input: &Vec<Vec<char>>) -> Result<usize, InvalidInput> {
    let height = input.len();
    let width = input.get(0).map_err_to_invalid_input("Empty input")?.len();

    let search = vec![Some('M'), Some('A'), Some('S')];

    let mut total = 0;

    for y in 0..height {
        for x in 0..width {

            if get_char_xy(input, Some((x, y))) == Some('X') {

                let neighbours: Vec<Vec<Option<char>>> =
                    (1..4)
                        .into_iter()
                        .map(|distance|
                                Neighbours2D::new_with_distance(x, y, width, height, distance)
                                    .map(|xy| get_char_xy(input, xy))
                                    .collect()
                        )
                        .collect();


                let words = transpose(neighbours);

                total += words.into_iter().filter(|w| *w == search).count();
            }
        }
    }

    Ok(total)
}

fn find_x_2(input: &Vec<Vec<char>>) -> Result<usize, InvalidInput> {
    let height = input.len();
    let width = input.get(0).map_err_to_invalid_input("Empty input")?.len();

    let corner_indices = HashSet::from([0, 2, 5, 7]);

    let search = vec![
        vec![Some('M'), Some('S'), Some('M'), Some('S')],
        vec![Some('M'), Some('M'), Some('S'), Some('S')],
        vec![Some('S'), Some('S'), Some('M'), Some('M')],
        vec![Some('S'), Some('M'), Some('S'), Some('M')],
    ];

    let mut total = 0;

    for y in 1..height-1 {
        for x in 1..width-1 {

            if get_char_xy(input, Some((x, y))) == Some('A') {

                let block: Vec<_> = Neighbours2D::new(x, y, width, height)
                    .map(|xy| get_char_xy(input, xy))
                    .collect();

                // pick the corners
                // 012
                // 3 4
                // 567

                let cross: Vec<_> =
                    block
                        .into_iter()
                        .enumerate()
                        .filter(|(i, _)| corner_indices.contains(i))
                        .map(|(_, v)| v)
                        .collect();

                if search.contains(&cross) {
                    total += 1;
                }
            }
        }
    }

    Ok(total)
}


fn get_char_xy(input: &Vec<Vec<char>>, coords: Option<(usize, usize)>) -> Option<char> {
    if let Some((x, y)) = coords {
        Some(*input
                .get(y)?
                .get(x)?
        )
    } else {
        None
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {

        assert_eq!(1, 1);
    }
}
