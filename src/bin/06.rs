use core::fmt;
use rayon::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(6);

const NORTH: (i32, i32) = (-1, 0);
const SOUTH: (i32, i32) = (1, 0);
const EAST: (i32, i32) = (0, 1);
const WEST: (i32, i32) = (0, -1);

#[derive(Clone, Copy)]
enum Objects {
    Guard(i32, i32),
    Obstacle,
    Path(bool),
}

impl fmt::Debug for Objects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Objects::Guard(_, _) => '^',
                Objects::Obstacle => '#',
                Objects::Path(b) =>
                    if *b {
                        'X'
                    } else {
                        '.'
                    },
            }
        )
    }
}

fn pretty_print_matrix(matrix: &Vec<Vec<Objects>>) {
    for row in matrix {
        for obj in row {
            print!("{:?} ", obj);
        }
        println!(); // Move to the next line after each row
    }
}

fn add_tuple(u: (usize, usize), i: (i32, i32)) -> Option<(usize, usize)> {
    let left = add(u.0, i.0)?;
    let right = add(u.1, i.1)?;
    Some((left, right))
}

fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

fn parse_into_matrix(input: &str) -> ((usize, usize), Vec<Vec<Objects>>) {
    let mut guard_position = (usize::MAX, usize::MAX);
    let positions = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, i): (usize, char)| match i {
                    '.' => Objects::Path(false),
                    'X' => Objects::Path(true),
                    '#' => Objects::Obstacle,
                    '^' | '<' | '>' | 'v' => {
                        guard_position = (row, col);
                        match i {
                            '^' => Objects::Guard(NORTH.0, NORTH.1),
                            '<' => Objects::Guard(WEST.0, WEST.1),
                            '>' => Objects::Guard(EAST.0, EAST.1),
                            'v' => Objects::Guard(SOUTH.0, SOUTH.1),
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                })
                .collect::<Vec<Objects>>()
        })
        .collect::<Vec<Vec<Objects>>>();
    (guard_position, positions)
}

fn get_next_pos(p: (usize, usize), v: &mut [Vec<Objects>]) -> Option<&mut Objects> {
    if let Some(l) = v.get_mut(p.0) {
        if let Some(g) = l.get_mut(p.1) {
            Some(g)
        } else {
            None
        }
    } else {
        None
    }
}

fn check_if_loop(
    mut matrix: Vec<Vec<Objects>>,
    position: (usize, usize),
    idx: (usize, usize),
) -> (u32, bool) {
    let mut count = 0u32;
    matrix[idx.0][idx.1] = Objects::Obstacle;
    let mut current_position = position;
    let mut current_direction = NORTH;
    let current_obj = &mut matrix[position.0][position.1];
    let mut visited_states = HashSet::new();
    if let Objects::Guard(d1, d2) = current_obj {
        current_direction = (*d1, *d2);
    }
    *current_obj = Objects::Path(true);
    count += 1;
    while let Some(next_position) = add_tuple(current_position, current_direction) {
        if let Some(next_pos_obj) = get_next_pos(next_position, &mut matrix) {
            if !visited_states.insert((current_position, current_direction)) {
                return (count, true);
            }
            match next_pos_obj {
                Objects::Obstacle => match current_direction {
                    NORTH => current_direction = EAST,
                    EAST => current_direction = SOUTH,
                    SOUTH => current_direction = WEST,
                    WEST => current_direction = NORTH,
                    _ => panic!("Direction didn't match anything"),
                },
                Objects::Path(b) => {
                    if !*b {
                        count += 1;
                        *next_pos_obj = Objects::Path(true);
                    }
                    current_position = next_position;
                }
                _ => {
                    pretty_print_matrix(&matrix);
                    panic!("Not a obstacle or a path")
                }
            };
            // dbg!(current_position, current_direction);
        } else {
            break;
        }
    }
    (count, false)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (position, mut matrix) = parse_into_matrix(input);
    let mut count = 0;
    let mut current_position = position;
    let mut current_direction = NORTH;
    let current_obj = &mut matrix[position.0][position.1];
    if let Objects::Guard(d1, d2) = current_obj {
        current_direction = (*d1, *d2);
    }
    *current_obj = Objects::Path(true);
    count += 1;
    let starting_direction = current_direction;
    while let Some(next_position) = add_tuple(current_position, current_direction) {
        if let Some(next_pos_obj) = get_next_pos(next_position, &mut matrix) {
            if current_position == position && starting_direction == current_direction && count != 1
            {
                panic!("Loop");
            }
            match next_pos_obj {
                Objects::Obstacle => match current_direction {
                    NORTH => current_direction = EAST,
                    EAST => current_direction = SOUTH,
                    SOUTH => current_direction = WEST,
                    WEST => current_direction = NORTH,
                    _ => panic!("Direction didn't match anything"),
                },
                Objects::Path(b) => {
                    if !*b {
                        count += 1;
                        *next_pos_obj = Objects::Path(true);
                    }
                    current_position = next_position;
                }
                _ => {
                    pretty_print_matrix(&matrix);
                    panic!("Not a obstacle or a path")
                }
            };
            // dbg!(current_position, current_direction);
        } else {
            break;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (position, matrix) = parse_into_matrix(input);

    let count: u32 = matrix
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.par_iter()
                .enumerate()
                .filter_map(|(col_idx, col)| {
                    if let Objects::Path(false) = col {
                        let (_, loop_exist) =
                            check_if_loop(matrix.clone(), position, (row_idx, col_idx));
                        if loop_exist {
                            Some(1)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum();
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
