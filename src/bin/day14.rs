#![feature(iter_advance_by)]

use anyhow::Error;
use std::env;

fn get_input() -> Result<(i32, usize), Error> {
    let inputs: Vec<_> = env::args().collect();

    Ok((
        inputs
            .get(1)
            .ok_or(Error::msg("Missing input argument"))?
            .parse()?,
        inputs[1].len(),
    ))
}
fn main() -> Result<(), Error> {
    let (input, n) = get_input()?;
    let result = solve(input);
    println!("{:010}", result);
    println!("{}", solve2(input, n));
    Ok(())
}

fn scores_iter() -> impl Iterator<Item = usize> {
    let mut scores = vec![3, 7];
    let mut pos1 = 0usize;
    let mut pos2 = 1usize;
    let mut next_to_return = 0usize;

    std::iter::from_fn(move || {
        if scores.len() > next_to_return {
            next_to_return += 1;
            return Some(scores[next_to_return - 1]);
        }

        let combined = scores[pos1] + scores[pos2];
        let first = combined / 10;
        let second = combined % 10;
        if first > 0 {
            scores.push(first);
        }
        scores.push(second);

        update_pos(&scores, &mut pos1);
        update_pos(&scores, &mut pos2);

        next_to_return += 1;
        Some(scores[next_to_return - 1])
    })
}

fn solve(input: i32) -> i64 {
    scores_iter()
        .skip(input as usize)
        .take(10)
        .fold(0i64, |acc, val| 10 * acc + (val as i64))
}

fn solve2(input: i32, n: usize) -> usize {
    let mut value = 0;
    let mod_value = i32::pow(10, (n - 1) as u32);

    for (i, score) in scores_iter().enumerate() {
        value %= mod_value;
        value = value * 10 + (score as i32);
        if value == input {
            return i - (n - 1);
        }
    }
    unreachable!()
}

fn update_pos(scores: &Vec<usize>, pos: &mut usize) {
    *pos = (*pos + 1 + scores[*pos]) % scores.len();
}
