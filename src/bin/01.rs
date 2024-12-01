use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn parse_inputs(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lines = input.lines();
    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        let mut nums_as_str = line.split_whitespace();
        if let Some(num_str) = nums_as_str.next() {
            let num = num_str.parse::<u32>().unwrap();
            left.push(num);
        }
        if let Some(num_str) = nums_as_str.next() {
            let num = num_str.parse::<u32>().unwrap();
            right.push(num);
        }
    }
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_inputs(input);
    left.sort();
    right.sort();
    if left.len() != right.len() {
        panic!("Not correct length of string!");
    }
    let mut sum = 0u32;
    for i in 0..left.len() {
        if left[i] > right[i] {
            sum += left[i] - right[i];
        } else {
            sum += right[i] - left[i];
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_inputs(input);
    let mut count_right: HashMap<u32, usize> = HashMap::new();
    let mut sum = 0u32;
    for i in right {
        count_right.entry(i).and_modify(|n| *n += 1).or_insert(1);
    }
    for i in left {
        if let Some(&num) = count_right.get(&i) {
            sum += i * num as u32;
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
