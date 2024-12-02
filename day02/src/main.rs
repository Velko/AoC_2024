use aoc_tools::{IterMoreTools, InvalidInput, ResultExt};
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = aoc_tools::Input::from_cmd()?.read_lines()?;

    let reports =
        input
            .iter()
            .map(parse_report)
            .try_collect_vec()?;

    let result1 = count_safe_reports_1(&reports);
    println!("Result p1: {}", result1);

    let result2 = count_safe_reports_2(&reports);
    println!("Result p2: {}", result2);

    Ok(())
}

fn parse_report<S: AsRef<str>>(dim: S) -> Result<Vec<i32>, InvalidInput>
    where S: Into<String>{
    let r = dim.as_ref();
    let parsed =
        r
            .split_ascii_whitespace()
            .map(str::parse::<i32>)
            .try_collect_vec()
            .map_err_to_invalid_input(r)?;

    Ok(parsed)
}

fn count_safe_reports_1(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .into_iter()
        .filter(|r|is_report_safe(r))
        .count()
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let diffs = calc_diffs(report.as_slice());

    diffs
        .iter()
        .all(|(v, _)| 1 <= *v && *v <= 3)
    &&
    diffs
        .iter()
        .unique_by(|(_, s)| s)
        .count() == 1
}

fn calc_diffs(report: &[i32]) -> Vec<(i32, i32)> {
    report
        .windows(2)
        .map(|w| ((w[1] - w[0]).abs(), (w[1] - w[0]).signum()))
        .collect()
}

fn count_safe_reports_2(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .into_iter()
        .filter(|r|is_report_safe(r) || is_report_safe_when_item_removed(r))
        .count()
}

fn is_report_safe_when_item_removed(report: &Vec<i32>) -> bool {
    (0..report.len())
        .any(|nth| is_report_safe(&clone_nth_skipped(report, nth)))
}

fn clone_nth_skipped(report: &Vec<i32>, nth: usize) -> Vec<i32> {
    report
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i != nth)
        .map(|(_, &v)| v)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_one() {

        let input = vec![1, 2, 3, 4, 5];

        let skipped = clone_nth_skipped(&input, 1);


        assert_eq!(skipped, vec![1, 3, 4, 5]);
    }
}
