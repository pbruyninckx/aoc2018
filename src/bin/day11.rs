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

fn solve(grid_serial_number: i32) -> (i32, i32) {
    let power_levels: Vec<_> = (1..(GRID_SIZE + 1))
        .map(|y| {
            (1..(GRID_SIZE + 1))
                .map(|x| get_power_level(grid_serial_number, x, y))
                .collect::<Vec<_>>()
        })
        .collect();

    let window = 3;
    let my_sliding_sum = |row| {sliding_sum(row, window)};
    let square_power_levels :Vec<Vec<i32>>= transpose(
        transpose(power_levels.iter().map(my_sliding_sum).collect())
            .iter()
            .map(my_sliding_sum)
            .collect::<Vec<Vec<i32>>>(),
    );
    let (index, _val) = square_power_levels.iter().flatten().enumerate().max_by_key(|(_i,el) | *el).unwrap();
    let new_grid_size = square_power_levels.len() as i32;
    (index as i32 % new_grid_size + 1, index as i32 / new_grid_size + 1)
}

fn sliding_sum(v: &Vec<i32>, window: i32) -> Vec<i32> {
    let first: i32 = v.iter().take(window as usize).sum();
    zip(v.iter(), v.iter().skip(window as usize))
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

fn main() -> Result<(), Error> {
    let grid_serial_number = get_input()?;

    let (row,col) = solve(grid_serial_number);
    println!("{},{}", row, col);
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
        assert_eq!(sliding_sum(&vec![1, 2, 3, 4, 5, 6], 3), vec![6, 9, 12, 15]);
    }

    #[rstest]
    fn test_transpose() {
        assert_eq!(
            transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
    }
}
