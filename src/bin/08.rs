use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

type NodePositionsWithSize = ((usize, usize), HashMap<char, BTreeSet<(usize, usize)>>);

fn parse_input(input: &str) -> NodePositionsWithSize {
    let mut size = (0, 0);
    let mut node_positons: HashMap<char, BTreeSet<(usize, usize)>> = HashMap::new();
    size.0 = input.lines().count();
    input.lines().enumerate().for_each(|(row, line)| {
        size.1 = size.1.max(line.chars().count());
        line.chars()
            .enumerate()
            .for_each(|(col, i): (usize, char)| {
                match i {
                    '.' => {}
                    v => {
                        node_positons
                            .entry(v)
                            .and_modify(|i| {
                                i.insert((row, col));
                            })
                            .or_insert_with(|| {
                                let mut set = BTreeSet::new();
                                set.insert((row, col));
                                set
                            });
                    }
                };
            });
    });
    (size, node_positons)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (size, nodes) = parse_input(input);
    let mut unique_positions: HashSet<(usize, usize)> = HashSet::new();

    for (_, positions) in nodes {
        for (prev, curr) in positions.iter().tuple_combinations() {
            let diff = (
                2 * (prev.0 as i32 - curr.0 as i32),
                2 * (prev.1 as i32 - curr.1 as i32),
            );
            let positions = [
                (prev.0 as i32 - diff.0, prev.1 as i32 - diff.1),
                (curr.0 as i32 + diff.0, curr.1 as i32 + diff.1),
            ];
            for p in positions {
                if (0..size.0).contains(&(p.0 as usize)) && (0..size.1).contains(&(p.1 as usize)) {
                    unique_positions.insert((p.0 as usize, p.1 as usize));
                }
            }
        }
    }
    Some(unique_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (size, nodes) = parse_input(input);
    let mut unique_positions: HashSet<(usize, usize)> = HashSet::new();

    for (_, positions) in nodes {
        for (prev, curr) in positions.iter().tuple_combinations() {
            let diff = (
                (prev.0 as i32 - curr.0 as i32),
                (prev.1 as i32 - curr.1 as i32),
            );

            let mut p = (curr.0 as i32 + diff.0, curr.1 as i32 + diff.1);
            while (0..size.0).contains(&(p.0 as usize)) && (0..size.1).contains(&(p.1 as usize)) {
                unique_positions.insert((p.0 as usize, p.1 as usize));
                p = (p.0 + diff.0, p.1 + diff.1);
            }
            p = (prev.0 as i32 - diff.0, prev.1 as i32 - diff.1);
            while (0..size.0).contains(&(p.0 as usize)) && (0..size.1).contains(&(p.1 as usize)) {
                unique_positions.insert((p.0 as usize, p.1 as usize));
                p = (p.0 - diff.0, p.1 - diff.1);
                continue;
            }
        }
    }
    Some(unique_positions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
