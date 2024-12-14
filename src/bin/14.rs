use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};
use std::{str::FromStr, thread::sleep, time::Duration};
advent_of_code::solution!(14);

type Res<T, U> = IResult<T, U, nom::error::Error<T>>;
#[derive(Debug)]
pub struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

pub fn parse_numbers(input: &str) -> IResult<&str, i32> {
    map_res(recognize(preceded(opt(tag("-")), digit1)), i32::from_str)(input)
}

impl Robot {
    fn parse(input: &str) -> Res<&str, Robot> {
        let position = pair(
            tag("p="),
            separated_pair(parse_numbers, char(','), parse_numbers),
        );
        let velocity = pair(
            tag("v="),
            separated_pair(parse_numbers, char(','), parse_numbers),
        );

        let line = separated_pair(position, char(' '), velocity);
        map_res(line, |((_, pos), (_, vel))| {
            // Correctly return a Result
            Ok::<Robot, nom::error::Error<&str>>(Robot {
                position: (pos.1, pos.0),
                velocity: (vel.1, vel.0),
            })
        })(input)
    }

    fn move_unit_by(&mut self, times: u32, max: (i32, i32)) {
        let x = (self.position.0 + (self.velocity.0 * times as i32)).rem_euclid(max.0);
        let y = (self.position.1 + (self.velocity.1 * times as i32)).rem_euclid(max.1);
        self.position = (x, y);
    }
}

/// Parse the whole Advent of Code day 5 text file.
pub fn parse_input(s: &str) -> Vec<Robot> {
    let (remaining_input, lines) = separated_list1(line_ending, Robot::parse)(s).unwrap();
    // assert!(remaining_input.is_empty());
    lines
}

pub fn pretty_print_robots(robots: &[Robot], max: (i32, i32)) {
    // Create a 2D vector initialized with zeroes for the grid
    let mut grid = vec![vec![0; max.1 as usize]; max.0 as usize];

    // Increment the count for each robot's position, adjusted for the grid bounds
    for robot in robots {
        let x = robot.position.0 as usize % max.0 as usize;
        let y = robot.position.1 as usize % max.1 as usize;
        grid[x][y] += 1;
    }

    // Print the grid
    for row in grid {
        for cell in row {
            if cell == 0 {
                print!(".");
            } else {
                print!("{}", cell);
            }
        }
        println!(); // Move to the next line for the grid
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut robots = parse_input(input);

    let min_y = robots.iter().map(|r| r.position.0).min().unwrap_or(0);
    let max_y = robots.iter().map(|r| r.position.0).max().unwrap_or(0);
    let min_x = robots.iter().map(|r| r.position.1).min().unwrap_or(0);
    let max_x = robots.iter().map(|r| r.position.1).max().unwrap_or(0);

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mid_x = height / 2;
    let mid_y = width / 2;
    let (mut first, mut second, mut third, mut fourth) = (0, 0, 0, 0);
    for robot in &mut robots {
        robot.move_unit_by(100, (height, width));

        if robot.position.0 == mid_x || robot.position.1 == mid_y {
            continue;
        }

        if robot.position.0 < mid_x && robot.position.1 < mid_y {
            first += 1;
        }
        if robot.position.0 < mid_x && robot.position.1 > mid_y {
            third += 1;
        }
        if robot.position.0 > mid_x && robot.position.1 < mid_y {
            fourth += 1;
        }
        if robot.position.0 > mid_x && robot.position.1 > mid_y {
            second += 1;
        }
    }
    pretty_print_robots(&robots, (height, width));
    Some(first * second * third * fourth)
}

fn tree_test(robots: &[Robot]) -> bool {
    // courtesy of https://github.com/ChristopherBiscardi/advent-of-code/blob/9e7b55b8f7a872901aedfeda538e53340f666557/2024/rust/day-14/src/part2.rs
    robots
        .iter()
        .map(|Robot { position, .. }| position)
        .all_unique()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = parse_input(input);

    let min_y = robots.iter().map(|r| r.position.0).min().unwrap_or(0);
    let max_y = robots.iter().map(|r| r.position.0).max().unwrap_or(0);
    let min_x = robots.iter().map(|r| r.position.1).min().unwrap_or(0);
    let max_x = robots.iter().map(|r| r.position.1).max().unwrap_or(0);

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mid_x = height / 2;
    let mid_y = width / 2;
    let (mut first, mut second, mut third, mut fourth) = (0, 0, 0, 0);
    let mut i = 0;
    loop {
        for robot in &mut robots {
            robot.move_unit_by(1, (height, width));

            if robot.position.0 == mid_x || robot.position.1 == mid_y {
                continue;
            }

            if robot.position.0 < mid_x && robot.position.1 < mid_y {
                first += 1;
            }
            if robot.position.0 < mid_x && robot.position.1 > mid_y {
                third += 1;
            }
            if robot.position.0 > mid_x && robot.position.1 < mid_y {
                fourth += 1;
            }
            if robot.position.0 > mid_x && robot.position.1 > mid_y {
                second += 1;
            }
        }
        i += 1;
        if tree_test(&robots) {
            pretty_print_robots(&robots, (height, width));
            println!(
                "\nSafety factor: {}, i: {}",
                first * second * third * fourth,
                i
            );
            break;
        }
    }
    Some(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
