use std::vec;

advent_of_code::solution!(4);

pub fn max(i: i32) -> usize {
    if i < 0 {
        usize::MAX
    } else {
        i as usize
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    const XMAS: (Option<&char>, Option<&char>, Option<&char>, Option<&char>) =
        (Some(&'X'), Some(&'M'), Some(&'A'), Some(&'S'));
    let mut xmas_count = 0;
    let mut xmas_matrix: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let line_chars = line.chars().collect();
        xmas_matrix.push(line_chars);
    }
    for rows in 0..xmas_matrix.len() {
        let row = &xmas_matrix[rows];
        let cols = row.len();
        for col in 0..cols {
            // check forwards
            if (
                row.get(col),
                row.get(col + 1),
                row.get(col + 2),
                row.get(col + 3),
            ) == XMAS
            {
                xmas_count += 1
            }
            // check backwards
            if col > 2
                && (
                    row.get(col),
                    row.get(col - 1),
                    row.get(col - 2),
                    row.get(col - 3),
                ) == XMAS
            {
                xmas_count += 1
            }
            // check vertical downwards
            if rows + 3 < xmas_matrix.len()
                && (
                    row.get(col),
                    xmas_matrix.get(rows + 1)?.get(col),
                    xmas_matrix.get(rows + 2)?.get(col),
                    xmas_matrix.get(rows + 3)?.get(col),
                ) == XMAS
            {
                xmas_count += 1
            }
            // check vertical upwards
            if rows > 2
                && (
                    row.get(col),
                    xmas_matrix.get(rows - 1)?.get(col),
                    xmas_matrix.get(rows - 2)?.get(col),
                    xmas_matrix.get(rows - 3)?.get(col),
                ) == XMAS
            {
                xmas_count += 1
            }
            // check diagonal left top
            if rows > 2
                && (
                    row.get(col),
                    xmas_matrix.get(rows - 1)?.get(max(col as i32 - 1)),
                    xmas_matrix.get(rows - 2)?.get(max(col as i32 - 2)),
                    xmas_matrix.get(rows - 3)?.get(max(col as i32 - 3)),
                ) == XMAS
            {
                xmas_count += 1
            }
            // check diagonal left bottom
            if rows + 3 < xmas_matrix.len()
                && (
                    row.get(col),
                    xmas_matrix.get(rows + 1)?.get(max(col as i32 - 1)),
                    xmas_matrix.get(rows + 2)?.get(max(col as i32 - 2)),
                    xmas_matrix.get(rows + 3)?.get(max(col as i32 - 3)),
                ) == XMAS
            {
                xmas_count += 1
            }
            // check diagonal right top
            if rows > 2
                && (
                    row.get(col),
                    xmas_matrix.get(rows - 1)?.get(col + 1),
                    xmas_matrix.get(rows - 2)?.get(col + 2),
                    xmas_matrix.get(rows - 3)?.get(col + 3),
                ) == XMAS
            {
                xmas_count += 1
            }
            if rows + 3 < xmas_matrix.len()
                && (
                    row.get(col),
                    xmas_matrix.get(rows + 1)?.get(col + 1),
                    xmas_matrix.get(rows + 2)?.get(col + 2),
                    xmas_matrix.get(rows + 3)?.get(col + 3),
                ) == XMAS
            {
                xmas_count += 1
            }
        }
    }
    Some(xmas_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut xmas_count = 0;
    let mut xmas_matrix: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let line_chars = line.chars().collect();
        xmas_matrix.push(line_chars);
    }
    for rows in 1..xmas_matrix.len() - 1 {
        let row = &xmas_matrix[rows];
        let cols = row.len();
        for col in 1..cols - 1 {
            if row.get(col) == Some(&'A') {
                match (
                    xmas_matrix
                        .get(max(rows as i32 - 1))?
                        .get(max(col as i32 - 1)), // M
                    xmas_matrix.get(rows + 1)?.get(col + 1), // S
                ) {
                    (Some(&'M'), Some('S')) | (Some(&'S'), Some('M')) => {
                        match (
                            xmas_matrix.get(rows + 1)?.get(max(col as i32 - 1)),
                            xmas_matrix.get(max(rows as i32 - 1))?.get(col + 1),
                        ) {
                            (Some(&'M'), Some('S')) | (Some(&'S'), Some('M')) => xmas_count += 1,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Some(xmas_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
