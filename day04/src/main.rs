use aoc_tools::{IterMoreTools, InvalidInput, ResultExt, Neighbours2D};

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?.read_grid()?;

    let result1 = find_words_1(&input)?;
    println!("Result p1: {}", result1);

    let result2 = 0;
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
