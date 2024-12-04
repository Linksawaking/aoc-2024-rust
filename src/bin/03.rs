use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    Some(
        regex
            .captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [x, y])| x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\))").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for c in regex.captures_iter(input) {
        if c.get(1).unwrap().as_str() == "do()" {
            enabled = true;
        } else if c.get(1).unwrap().as_str() == "don't()" {
            enabled = false;
        } else {
            if enabled {
                let x = c.get(2).unwrap().as_str();
                let y = c.get(3).unwrap().as_str();
                sum += x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap()
            }
        }
    }

    Some(sum)
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
