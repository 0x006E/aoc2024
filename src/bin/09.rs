use core::fmt;
use std::collections::HashSet;

advent_of_code::solution!(9);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Objects {
    File(u32, u32),
    FreeSpace(u32),
}

impl fmt::Debug for Objects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Objects::File(id, num) => format!("{}", id).repeat(*num as usize),
                Objects::FreeSpace(num) => ".".repeat(*num as usize),
            }
        )
    }
}
impl fmt::Display for Objects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Objects {
    pub fn checksum(&self, position: usize) -> usize {
        match self {
            Objects::File(id, _) => position * (*id as usize),
            Objects::FreeSpace(_) => 0,
        }
    }
}

fn parse_into_vec(input: &str, pack_as_one: bool) -> Vec<Objects> {
    let mut file_index = 0;
    let mut v = vec![];
    for (idx, char) in input.chars().enumerate() {
        if let Some(num) = char.to_digit(10) {
            if idx % 2 == 0 {
                if pack_as_one {
                    for _ in 0..num {
                        v.push(Objects::File(file_index, 1));
                    }
                } else {
                    v.push(Objects::File(file_index, num))
                }
                file_index += 1;
            } else if pack_as_one {
                for _ in 0..num {
                    v.push(Objects::FreeSpace(1));
                }
            } else {
                v.push(Objects::FreeSpace(num));
            }
        }
    }
    v
}

fn convert_to_ones(input: &Vec<Objects>) -> Vec<Objects> {
    let mut v = vec![];
    for i in input {
        match i {
            Objects::File(id, num) => {
                for _ in 0..*num {
                    v.push(Objects::File(*id, 1));
                }
            }
            Objects::FreeSpace(num) => {
                for _ in 0..*num {
                    v.push(Objects::FreeSpace(1));
                }
            }
        }
    }
    v
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut vec = parse_into_vec(input, true);
    let (mut i, mut j) = (0, vec.len() - 1);
    while i <= j {
        if let Objects::File(_, _) = vec[i] {
            i += 1;
            continue;
        }
        if let Objects::FreeSpace(_) = vec[j] {
            j -= 1;
            continue;
        }
        vec.swap(i, j);
        i += 1;
        j -= 1;
    }
    Some(
        vec.iter()
            .enumerate()
            .map(|(idx, v)| v.checksum(idx))
            .sum::<usize>() as u64,
    )
}

fn try_swapping(vec: &mut Vec<Objects>, moved_objects: &mut HashSet<Objects>) -> bool {
    let mut swap_count = 0;
    for i in 0..vec.len() {
        if let Objects::FreeSpace(numi) = vec[i] {
            for j in (i..vec.len()).rev() {
                if let Objects::File(_, numj) = vec[j] {
                    if moved_objects.contains(&vec[j]) {
                        continue;
                    }
                    match numi.cmp(&numj) {
                        std::cmp::Ordering::Equal => {
                            vec.swap(i, j);
                            moved_objects.insert(vec[j]);
                            swap_count += 1;
                            break;
                        }
                        std::cmp::Ordering::Greater => {
                            let diff = numi - numj;
                            vec.swap(i, j);
                            swap_count += 1;
                            moved_objects.insert(vec[j]);
                            vec[j] = Objects::FreeSpace(numj);
                            vec.insert(i + 1, Objects::FreeSpace(diff));
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    swap_count >= 1
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut vec = parse_into_vec(input, false);
    let mut moved_objects: HashSet<Objects> = HashSet::new();
    loop {
        if !try_swapping(&mut vec, &mut moved_objects) {
            break;
        }
    }
    Some(
        convert_to_ones(&vec)
            .iter()
            .enumerate()
            .map(|(idx, v)| v.checksum(idx))
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
