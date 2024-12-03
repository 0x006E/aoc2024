advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<i32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let lines = input.lines();
    let mut sum = 0i32;
    for line in lines {
        for (_, [left_num_str, right_num_str]) in re.captures_iter(line).map(|c| c.extract()) {
            let left_num = left_num_str.parse::<i32>().unwrap_or_default();
            let right_num = right_num_str.parse::<i32>().unwrap_or_default();
            sum += right_num * left_num
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(d)(o)\(\)|(do)(n't)\(\)").unwrap();
    let lines = input.lines();
    let mut sum = 0i32;
    let mut dont = false;
    for line in lines {
        for (_, [left_num_str, right_num_str]) in re.captures_iter(line).map(|c| c.extract()) {
            match (left_num_str, right_num_str) {
                ("do", "n't") => dont = true,
                ("d", "o") => dont = false,
                _ => {
                    if !dont {
                        let left_num = left_num_str.parse::<i32>().unwrap_or_default();
                        let right_num = right_num_str.parse::<i32>().unwrap_or_default();
                        sum += right_num * left_num
                    }
                }
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
