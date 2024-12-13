use itertools::Itertools;
use num_traits::Euclid;
use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let v = input
        .replace("\n", "")
        .split(" ")
        .map(|value| value.parse::<u32>().unwrap())
        .collect_vec();
    let mut count = 0;
    let mut count_map: HashMap<(u128, u32), u64> = HashMap::new();
    for el in v {
        count += blink(el as u128, 25, &mut count_map);
    }
    Some(count)
}

fn blink(num: u128, max_depth: u32, hash_map: &mut HashMap<(u128, u32), u64>) -> u64 {
    let num_digits = num.checked_ilog10().unwrap_or(0) + 1;
    let mut count = 0u64;
    if let Some(v) = hash_map.get(&(num, max_depth)) {
        return *v;
    }
    if max_depth == 0 {
        hash_map.insert((num, max_depth), 1);
        return 1;
    }
    if num == 0 {
        count += blink(1, max_depth - 1, hash_map);
    } else if num_digits % 2 == 0 {
        let (left, right) = num.div_rem_euclid(&(10u128.pow(num_digits / 2)));
        count += blink(left, max_depth - 1, hash_map) + blink(right, max_depth - 1, hash_map);
    } else {
        count += blink(num * 2024, max_depth - 1, hash_map);
    }
    hash_map.insert((num, max_depth), count);
    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let v = input
        .replace("\n", "")
        .split(" ")
        .map(|value| value.parse::<u32>().unwrap())
        .collect_vec();
    let mut count = 0;
    let mut count_map: HashMap<(u128, u32), u64> = HashMap::new();
    for el in v {
        count += blink(el as u128, 75, &mut count_map);
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
