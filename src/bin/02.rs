use std::iter;

advent_of_code::solution!(2);

pub fn is_report_safe<T>(num_report: &T) -> bool
where
    T: Iterator<Item = i32> + Clone,
{
    let diffs = iter::zip(num_report.clone(), num_report.clone().skip(1)).map(|(l1, l2)| l2 - l1);

    let all_neg = diffs.clone().all(|d| d < 0);
    let all_pos = diffs.clone().all(|d| d > 0);
    let all_low = diffs.clone().all(|d| d.abs() <= 3);

    all_low && (all_neg || all_pos)
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_reports: Vec<_> = input
        .lines()
        .filter(|report| {
            let num_report = report.split(' ').map(|x| x.parse::<i32>().unwrap());

            is_report_safe(&num_report)
        })
        .collect();

    Some(safe_reports.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_reports: Vec<_> = input
        .lines()
        .filter(|report| {
            let num_report = report.split(' ').map(|x| x.parse::<i32>().unwrap());

            if is_report_safe(&num_report) {
                true
            } else {
                let mut found_safe = false;
                for (index, _) in num_report.clone().enumerate() {
                    let new_num_report = num_report
                        .clone()
                        .enumerate()
                        .filter(|(i, _)| *i != index)
                        .map(|(_, l)| l);

                    if is_report_safe(&new_num_report) {
                        found_safe = true;
                    }
                }

                found_safe
            }
        })
        .collect();

    Some(safe_reports.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
