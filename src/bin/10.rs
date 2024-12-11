use std::collections::HashSet;

use grid::Grid;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(10);

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn parse_into_matrix(input: &str) -> (Vec<(usize, usize)>, Grid<u32>) {
    let mut grid: Grid<u32> = Grid::new(0, 0);
    let mut positions = vec![];
    input.lines().enumerate().for_each(|(row, line)| {
        grid.push_row(
            line.chars()
                .enumerate()
                .map(|(idx, c)| {
                    let num = c.to_digit(10).unwrap();
                    if num == 0 {
                        positions.push((row, idx));
                    }
                    num
                })
                .collect_vec(),
        )
    });
    (positions, grid)
}

fn num_paths_reaching_9(
    position: (usize, usize),
    grid: &Grid<u32>,
    path: &mut Vec<(usize, usize)>,
    all_paths: &mut Vec<Vec<(usize, usize)>>,
    visited_nines: &mut HashSet<(usize, usize)>,
    check_visited_nine: bool,
) {
    path.push(position);
    let current_num = grid[position];
    if current_num == 9 && !visited_nines.contains(&position) {
        all_paths.push(path.clone());
        path.pop();
        if check_visited_nine {
            visited_nines.insert(position);
        }
        return;
    }
    for direction in DIRECTIONS {
        let new_position = (
            position.0 as i32 + direction.0,
            position.1 as i32 + direction.1,
        );
        if let Some(num) = grid.get(new_position.0, new_position.1) {
            if *num == current_num + 1
                && !path.contains(&(new_position.0 as usize, new_position.1 as usize))
            {
                num_paths_reaching_9(
                    (new_position.0 as usize, new_position.1 as usize),
                    grid,
                    path,
                    all_paths,
                    visited_nines,
                    check_visited_nine,
                );
            }
        }
    }
    path.pop();
}

pub fn part_one(input: &str) -> Option<u32> {
    let (zero_points, grid) = parse_into_matrix(input);
    Some(
        zero_points
            .par_iter()
            .map(|p: &(usize, usize)| {
                let mut path: Vec<(usize, usize)> = vec![];
                let mut all_paths: Vec<Vec<(usize, usize)>> = vec![];
                let mut visited_nine: HashSet<(usize, usize)> = HashSet::new();
                num_paths_reaching_9(
                    *p,
                    &grid,
                    &mut path,
                    &mut all_paths,
                    &mut visited_nine,
                    true,
                );
                all_paths.len() as u32
            })
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    let (zero_points, grid) = parse_into_matrix(input);
    Some(
        zero_points
            .par_iter()
            .map(|p: &(usize, usize)| {
                let mut path: Vec<(usize, usize)> = vec![];
                let mut all_paths: Vec<Vec<(usize, usize)>> = vec![];
                let mut visited_nine: HashSet<(usize, usize)> = HashSet::new();
                num_paths_reaching_9(
                    *p,
                    &grid,
                    &mut path,
                    &mut all_paths,
                    &mut visited_nine,
                    false,
                );
                all_paths.len() as u32
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
