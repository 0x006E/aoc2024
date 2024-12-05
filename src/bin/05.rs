use std::{
    collections::HashMap,
    vec,
};
use topological_sort::TopologicalSort;
advent_of_code::solution!(5);

pub fn parse_num(input: &str) -> (u32, u32) {
    let split = input.split("|").collect::<Vec<&str>>();
    let left_num_str = split.get(0).unwrap();
    let right_num_str = split.get(1).unwrap();
    (
        left_num_str.parse::<u32>().unwrap(),
        right_num_str.parse::<u32>().unwrap(),
    )
}

pub fn parse_vec(input: &str) -> Vec<u32> {
    input.split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()

}

pub fn follow_rules(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> u32 {
    let mut hash_map = HashMap::new();
    for (idx, &page) in update.iter().enumerate() {
        hash_map.insert(page, idx);
    }
    for (a, b) in rules {
        if hash_map.contains_key(a) && hash_map.contains_key(b) && !(hash_map.get(a) < hash_map.get(b)) {
            return 0
        }
    }
    let mid = update.len();
    update[mid / 2]
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut page_ordering_rules= vec![];
    let mut start = false;
    let mut count: u32 = 0;
    for line in input.lines() {
        if line == "" {
            start = true;
            continue;
        }
        if start {
            let pages_vec = parse_vec(line);
            let val = follow_rules(&pages_vec, &page_ordering_rules);
            count += val;
        } else {
            let v = parse_num(line);
            page_ordering_rules.push(v);

        }
    }
    Some(count)
}

pub fn get_unsafe_order_middle(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> u32 {
    let mut ts = TopologicalSort::<u32>::new();
    let middle = update.len() / 2;
    for (left,right) in rules {
        if update.contains(left) && update.contains(right) {
            ts.add_dependency(*left, *right);
        }
    }
    ts.collect::<Vec<u32>>()[middle]
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut page_ordering_rules= vec![];
    let mut start = false;
    let mut count: u32 = 0;
    for line in input.lines() {
        if line == "" {
            start = true;
            continue;
        }
        if start {
            let pages_vec = parse_vec(line);
            let val = follow_rules(&pages_vec, &page_ordering_rules);
            if val == 0 {
                count += get_unsafe_order_middle(&pages_vec, &page_ordering_rules);
            }
        } else {
            let v = parse_num(line);
            page_ordering_rules.push(v);

        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
