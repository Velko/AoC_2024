use std::iter::repeat_n;

struct InputItem {
    file_id: Option<usize>,
    size: usize,
}

#[derive(Clone, Copy)]
struct FileSysItem {
    file_id: Option<usize>,
    size: usize,
}

type ParsedInput = Vec<InputItem>;

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

    let mut file_desc = Vec::new();

    while let Some(file_len) = in_c.next() {
        file_desc.push(InputItem {
            file_id: Some(file_id),
            size: parse_char(file_len),
        });
        file_id += 1;

        if let Some(space_len) = in_c.next() {
            file_desc.push(InputItem {
                file_id: None,
                size: parse_char(space_len),
            });
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
    let mut dm = expand_disk_map(input);
    let disk_map = dm.as_mut_slice();

    let mut start_idx = 0;
    let mut end_idx = disk_map.len() - 1;

    while start_idx < end_idx {
        while disk_map[start_idx].file_id.is_some() {
            start_idx += 1;
        }

        while disk_map[end_idx].file_id.is_none() {
            end_idx -= 1;
        }

        if start_idx >= end_idx { break; }

        disk_map[start_idx] = disk_map[end_idx];
        disk_map[end_idx] = FileSysItem {
            file_id: None,
            size: 0,
        };

        start_idx += 1;
        end_idx -= 1;
    }

    calculate_disk_checksum(&disk_map)
}

fn expand_disk_map(input: &ParsedInput) -> Vec<FileSysItem> {
    let mut disk_map = Vec::new();

    for item in input.into_iter() {
        disk_map.extend(repeat_n(
            FileSysItem {
                file_id: item.file_id,
                size: item.size
            },
            item.size)
        );
    }

    disk_map
}

fn calculate_disk_checksum(disk_map: &[FileSysItem]) -> u64 {
    disk_map
        .into_iter()
        .enumerate()
        .map(|(pos, item)| pos as u64 * item.file_id.unwrap_or(0) as u64 )
        .sum()
}

fn calculate_p2(input: &ParsedInput) -> u64 {
    let mut dm = expand_disk_map(input);
    let mut disk_map = dm.as_mut_slice();

    let mut block_idx = disk_map.len() - 1;

    loop {
        while disk_map[block_idx].file_id.is_none() {
            block_idx -= 1;
        }

        let block_size = disk_map[block_idx].size;
        block_idx -= block_size - 1;

        //println!("{:?}", &disk_map.as_slice()[end_idx..end_idx + block_size]);

        let mut free_idx = 0;

        while free_idx < disk_map.len() && not_fit(&disk_map[free_idx], block_size) {
            free_idx += 1;
        }


        if free_idx < disk_map.len() && block_idx > free_idx {
            let free_size = disk_map[free_idx].size;

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

fn not_fit(block: &FileSysItem, wanted: usize) -> bool {
    block.file_id.is_some() || block.size < wanted
}

fn move_block(disk_map: &mut [FileSysItem], free_idx: usize, block_idx: usize, block_size: usize) {
    for i in 0..block_size {
        disk_map[free_idx + i] = disk_map[block_idx + i];

        disk_map[block_idx + i] = FileSysItem {
            file_id: None,
            size: block_size,
        };
    }
}

fn resize_remaining_free(disk_map: &mut [FileSysItem], free_idx: usize, free_size: usize) {
    for i in 0..free_size {
        disk_map[free_idx + i] = FileSysItem {
            file_id: None,
            size: free_size,
        };
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
    use rstest::rstest;
    use super::*;
    use aoc_tools::TestSamples;

    fn load_sample(index: usize) -> anyhow::Result<(ParsedInput, Option<u64>, Option<u64>)> {
        let samples = TestSamples::try_new()?;
        let (input, expected1, expected2) = samples.get_sample(index)?;
        let parsed = parse_input(input)?;
        Ok((parsed, expected1, expected2))
    }

    #[rstest]
    #[case(load_sample(0)?)]
    #[case(load_sample(1)?)]
    fn test_sample_p1(#[case] (parsed, expected, _): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result1 = calculate_p1(&parsed);

        assert_eq!(expected, Some(result1 as u64));
        Ok(())
    }

    #[rstest]
    #[case(load_sample(0)?)]
    #[case(load_sample(1)?)]
    fn test_sample_p2(#[case] (parsed, _, expected): (ParsedInput, Option<u64>, Option<u64>)) -> anyhow::Result<()> {

        let result2 = calculate_p2(&parsed);

        println!("{:?}", expected);

        assert_eq!(expected, Some(result2 as u64));
        Ok(())
    }
}
