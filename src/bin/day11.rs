use anyhow::Error;
use std::env;
use std::iter::zip;

const GRID_SIZE: i32 = 300;

fn get_input() -> Result<i32, Error> {
    let mut args = env::args();
    if args.len() < 2 {
        Err(Error::msg("Missing puzzle input argument"))
    } else {
        Ok(args.nth(1).unwrap().parse::<i32>()?)
    }
}

fn get_power_level(grid_serial_number: i32, x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    rack_id * (rack_id * y + grid_serial_number) / 100 % 10 - 5
}

fn solve_for_window(grid_serial_number: i32, window: i32) -> ((i32, i32), i32) {
    let power_levels: Vec<_> = (1..(GRID_SIZE + 1))
        .map(|y| {
            (1..(GRID_SIZE + 1))
                .map(|x| get_power_level(grid_serial_number, x, y))
                .collect::<Vec<_>>()
        })
        .collect();

    let my_sliding_sum = |row: &Vec<i32>| sliding_sum(row.iter(), window);
    let square_power_levels: Vec<Vec<i32>> = transpose(
        transpose(power_levels.iter().map(my_sliding_sum).collect())
            .iter()
            .map(my_sliding_sum)
            .collect::<Vec<Vec<i32>>>(),
    );
    let (index, val) = square_power_levels
        .iter()
        .flatten()
        .enumerate()
        .max_by_key(|(_i, el)| *el)
        .unwrap();
    let new_grid_size = square_power_levels.len() as i32;
    (
        (
            index as i32 % new_grid_size + 1,
            index as i32 / new_grid_size + 1,
        ),
        *val,
    )
}

fn sliding_sum<'a, I>(v: I, window: i32) -> Vec<i32>
where
    I: Iterator<Item = &'a i32> + Clone,
{
    let first: i32 = v.clone().take(window as usize).sum();
    zip(v.clone(), v.clone().skip(window as usize))
        .map(|(e1, e2)| e2 - e1)
        .fold(vec![first], |mut acc, el| {
            acc.push(acc.last().unwrap() + el);
            acc
        })
}

fn transpose(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let num_rows = mat.len();
    let num_cols = mat[0].len();

    (0..num_cols)
        .map(|c| (0..num_rows).map(|r| mat[r][c]).collect())
        .collect()
}

fn solve_part_one(grid_serial_number: i32) -> (i32, i32) {
    solve_for_window(grid_serial_number, 3).0
}

fn solve_part_two(grid_serial_number: i32) -> (i32, i32, i32) {
    (1..GRID_SIZE + 1)
        .map(|window| {
            let ((row, col), value) = solve_for_window(grid_serial_number, window);
            ((row, col, window), value)
        })
        .max_by_key(|(_, val)| *val)
        .unwrap()
        .0
}

fn main() -> Result<(), Error> {
    let grid_serial_number = get_input()?;
    {
        let (row, col) = solve_part_one(grid_serial_number);
        println!("{},{}", row, col);
    }
    {
        let (row, col, window) = solve_part_two(grid_serial_number);
        println!("{},{},{}", row, col, window);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(8, 3, 5, 4)]
    #[case(57, 122, 79, -5)]
    #[case(39, 217, 196, 0)]
    #[case(71, 101, 153, 4)]
    fn test_power_level(
        #[case] grid_serial_numer: i32,
        #[case] x: i32,
        #[case] y: i32,
        #[case] expected: i32,
    ) {
        assert_eq!(get_power_level(grid_serial_numer, x, y), expected);
    }

    #[rstest]
    fn test_sliding_sum() {
        assert_eq!(
            sliding_sum(vec![1, 2, 3, 4, 5, 6].iter(), 3),
            vec![6, 9, 12, 15]
        );
    }

    #[rstest]
    fn test_transpose() {
        assert_eq!(
            transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
    }

    #[rstest]
    #[case(18, (90,269,16))]
    #[case(42, (232,251,12))]
    fn test_part_two(#[case] grid_serial_numer: i32, #[case] solution: (i32, i32, i32)) {
        assert_eq!(solve_part_two(grid_serial_numer), solution)
    }
}
