use aoc_tools::{Grid, InvalidInput, Neighbours2D, NeighbourMap};

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?.read_grid()?;

    let result1 = find_words_1(&input)?;
    println!("Result p1: {}", result1);

    let result2 = find_x_2(&input)?;
    println!("Result p2: {}", result2);

    Ok(())
}


fn find_words_1(input: &Grid) -> Result<usize, InvalidInput> {
    let search = vec![Some('M'), Some('A'), Some('S')];

    let mut total = 0;

    for (chr, pos) in input.enumerate() {
        if *chr == 'X' {

            let neighbours: Vec<Vec<Option<char>>> =
                (1..4)
                    .into_iter()
                    .map(|distance|
                            Neighbours2D::new_with_distance(pos, input.size(), distance, NeighbourMap::All)
                                .map(|xy| get_char_xy(input, xy))
                                .collect()
                    )
                    .collect();


            let words = transpose(neighbours);

            total += words.into_iter().filter(|w| *w == search).count();
        }
    }

    Ok(total)
}

fn find_x_2(input: &Grid) -> Result<usize, InvalidInput> {
    let search = vec![
        vec![Some('M'), Some('S'), Some('M'), Some('S')],
        vec![Some('M'), Some('M'), Some('S'), Some('S')],
        vec![Some('S'), Some('S'), Some('M'), Some('M')],
        vec![Some('S'), Some('M'), Some('S'), Some('M')],
    ];

    let mut total = 0;

    for (chr, pos) in input.enumerate() {
        if *chr == 'A' {

            let cross: Vec<_> = Neighbours2D::new(pos, input.size(), NeighbourMap::X)
                .map(|xy| get_char_xy(input, xy))
                .collect();

            if search.contains(&cross) {
                total += 1;
            }
        }
    }

    Ok(total)
}


fn get_char_xy(input: &Grid, coords: Option<(usize, usize)>) -> Option<char> {
    Some(input[coords?])
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
    fn test_sample_1() -> anyhow::Result<()> {

        let input = aoc_tools::Input::from_filename("sample.txt")?.read_grid()?;

        let result1 = find_words_1(&input)?;

        assert_eq!(18, result1);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {

        let input = aoc_tools::Input::from_filename("sample.txt")?.read_grid()?;

        let result2 = find_x_2(&input)?;

        assert_eq!(9, result2);
        Ok(())
    }
}
