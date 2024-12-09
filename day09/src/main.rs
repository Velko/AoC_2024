use std::iter::repeat_n;

type ParsedInput = Vec<(Option<usize>, usize)>;

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
    let in_str = input.read_single_line()?;

    // 2333133121414131402
    let mut in_c = in_str.chars().into_iter();
    let mut file_id = 0;

    let mut file_desc: Vec<(Option<usize>, usize)> = Vec::new();

    while let Some(file_len) = in_c.next() {
        file_desc.push((Some(file_id), parse_char(file_len)));

        file_id += 1;
        if let Some(space_len) = in_c.next() {
            file_desc.push((None, parse_char(space_len)));
        } else {
            break;
        }
    }

    Ok(file_desc)
}

fn parse_char(c: char) -> usize {
    format!("{}", c).parse().unwrap()
}

fn calculate_p1(input: &ParsedInput) -> u64 {
    let mut disk_map = expand_disk_map(input);

    let mut start_idx = 0;
    let mut end_idx = disk_map.len() - 1;

    while start_idx < end_idx {
        while disk_map.get(start_idx).unwrap().0.is_some() {
            start_idx += 1;
        }

        while disk_map.get(end_idx).unwrap().0.is_none() {
            end_idx -= 1;
        }

        *disk_map.get_mut(start_idx).unwrap()
            = *disk_map.get(end_idx).unwrap();
        *disk_map.get_mut(end_idx).unwrap() = (None, 0);

        start_idx += 1;
        end_idx -= 1;
    }

    calculate_disk_checksum(&disk_map)
}

fn expand_disk_map(input: &ParsedInput) -> Vec<(Option<usize>, usize)> {
    let mut disk_map: Vec<(Option<usize>, usize)> = Vec::new();

    for (val, count) in input.into_iter() {
        disk_map.extend(repeat_n((*val, *count), *count));
    }

    disk_map
}

fn calculate_disk_checksum(disk_map: &Vec<(Option<usize>, usize)>) -> u64 {
    disk_map
        .into_iter()
        .enumerate()
        .map(|(pos, (file_id, _))| pos as u64 * file_id.unwrap_or(0) as u64 )
        .sum()
}

fn calculate_p2(input: &ParsedInput) -> u64 {
    let mut disk_map = expand_disk_map(input);

    let mut block_idx = disk_map.len() - 1;

    loop {
        while disk_map.get(block_idx).unwrap().0.is_none() {
            block_idx -= 1;
        }

        let block_size = disk_map.get(block_idx).unwrap().1;
        block_idx -= block_size - 1;

        //println!("{:?}", &disk_map.as_slice()[end_idx..end_idx + block_size]);

        let mut free_idx = 0;

        while free_idx < disk_map.len() && not_fit(disk_map.get(free_idx).unwrap(), block_size) {
            free_idx += 1;
        }


        if free_idx < disk_map.len() && block_idx > free_idx {
            let free_size = disk_map.get(free_idx).unwrap().1;

            //println!("{:?}", &disk_map.as_slice()[free_idx..free_idx + free_size]);


            move_block(&mut disk_map, free_idx, block_idx, block_size);
            resize_remaining_free(&mut disk_map, free_idx + block_size, free_size - block_size);

            //println!("{:?}\n", disk_map);
            //print_map(&disk_map);
        }
        if block_idx == 0 {
            break;
        }
        block_idx -= 1;

        //println!("{}", 100 * (disk_map.len() - block_idx) / disk_map.len());
    }

    //print_map(&disk_map);

    calculate_disk_checksum(&disk_map)
}

fn not_fit(block: &(Option<usize>, usize), wanted: usize) -> bool {
    block.0.is_some() || block.1 < wanted
}

fn move_block(disk_map: &mut Vec<(Option<usize>, usize)>, free_idx: usize, block_idx: usize, block_size: usize) {
    for i in 0..block_size {
        *disk_map.get_mut(free_idx + i).unwrap()
            = *disk_map.get(block_idx + i).unwrap();

        *disk_map.get_mut(block_idx + i).unwrap() = (None, block_size);
    }
}

fn resize_remaining_free(disk_map: &mut Vec<(Option<usize>, usize)>, free_idx: usize, free_size: usize) {
    for i in 0..free_size {
        *disk_map.get_mut(free_idx + i).unwrap() = (None, free_size);
    }
}


// fn print_map(disk_map: &Vec<(Option<usize>, usize)>) {
//     for (p, _) in disk_map.into_iter() {
//         print!("{}", match p{
//             Some(d) => format!("{}", d),
//             None => ".".to_string(),
//         });
//     }

//     println!();
// }

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_tools::TestSamples;

    fn load_sample(index: usize) -> anyhow::Result<(ParsedInput, u64)> {
        let samples = TestSamples::try_new()?;
        let (input, expected) = samples.get_sample(index)?;
        let parsed = parse_input(input)?;
        Ok((parsed, expected))
    }

    #[test]
    fn test_sample_p1() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(0)?;

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, result1 as u64);
        Ok(())
    }

    #[test]
    fn test_sample_p2() -> anyhow::Result<()> {
        let (parsed, expected) = load_sample(1)?;

        let result2 = calculate_p2(&parsed);

        assert_eq!(expected, result2 as u64);
        Ok(())
    }
}
