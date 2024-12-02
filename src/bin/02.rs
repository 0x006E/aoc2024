advent_of_code::solution!(2);

pub fn parse_inputs(input: &str) -> Vec<Vec<i32>> {
    let lines = input.lines();
    let mut levels = vec![];

    for line in lines {
        let mut level: Vec<i32> = vec![];
        let nums_as_str = line.split_whitespace();
        for num_str in nums_as_str {
            let num = num_str.parse::<i32>().unwrap();
            level.push(num);
        }
        levels.push(level);
    }
    levels
}

pub fn part_one(input: &str) -> Option<u32> {
    let test_cases = parse_inputs(input);
    let mut safe_cases = 0;
    for levels in test_cases {
        if is_sequence_safe(&levels) {
            safe_cases += 1;
        }
    }

    Some(safe_cases)
}

pub fn is_sequence_safe(input: &[i32]) -> bool {
    if input.len() < 2 {
        return true;
    }

    let increasing = input[0] < input[1];

    for i in 1..input.len() {
        let diff = input[i] - input[i - 1];

        match (increasing, diff) {
            (true, diff) if diff <= 0 || diff > 3 => return false,
            (false, diff) if !(-3..0).contains(&diff) => return false,
            _ => continue,
        }
    }
    true
}

pub fn check_safe_with_removal(levels: &[i32]) -> bool {
    if is_sequence_safe(levels) {
        return true;
    }

    for idx in 0..levels.len() {
        let mut test_sequence = levels.to_owned();
        test_sequence.remove(idx);

        if is_sequence_safe(&test_sequence) {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let test_cases = parse_inputs(input);
    let mut safe_cases = 0;

    for levels in test_cases {
        if check_safe_with_removal(&levels) {
            safe_cases += 1;
        }
    }

    Some(safe_cases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
