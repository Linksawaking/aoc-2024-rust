use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn common(input: &str) -> (Vec<isize>, Vec<isize>) {
    let input_parsed: Vec<(isize, isize)> = input
        .lines()
        .map(|s| {
            s.split_once("   ")
                .map(|(x1, x2)| (x1.parse().unwrap(), x2.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let mut list1: Vec<_> = input_parsed.iter().map(|(x1, _)| *x1).collect();
    let mut list2: Vec<_> = input_parsed.iter().map(|(_, x2)| *x2).collect();

    list1.sort();
    list2.sort();

    (list1, list2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (list1, list2) = common(input);
    let total_distance = list1
        .iter()
        .zip(list2.iter())
        .map(|(x1, x2)| isize::abs_diff(*x1, *x2))
        .fold(0, |acc, x| acc + x);

    println!("total_distance: {total_distance}");
    Some(total_distance as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = common(input);

    let mut occurrences = HashMap::new();

    for x1 in list1 {
        let count = list2.iter().filter(|x| **x == x1).count();
        occurrences
            .entry(x1)
            .and_modify(|x| *x += count)
            .or_insert(count);
    }

    let similarity_score = occurrences
        .iter()
        .fold(0, |acc, (key, value)| acc + *key * (*value as isize));

    println!("similarity_score: {similarity_score}");
    Some(similarity_score as u32)
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
