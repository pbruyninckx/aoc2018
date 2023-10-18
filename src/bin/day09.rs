#![feature(linked_list_cursors)]

use anyhow::Error;
use std::{collections::linked_list::CursorMut, collections::LinkedList, env, process};

#[derive(Debug)]
struct PuzzleInput {
    num_players: i64,
    top_marble: i64,
}
fn parse_input(args: Vec<String>) -> Result<PuzzleInput, Error> {
    let int_args = {
        let parsed_args: Result<Vec<i64>, _> =
            args.iter().skip(1).map(|s| s.parse::<i64>()).collect();
        parsed_args?
    };
    if int_args.len() != 2 {
        Err(Error::msg("Expected 2 arguments"))
    } else {
        Ok(PuzzleInput {
            num_players: int_args[0],
            top_marble: int_args[1],
        })
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    match parse_input(args) {
        Ok(input) => println!("Score: {}", solve(input)),
        Err(e) => {
            println!("Something went wrong: {}", e);
            process::exit(1);
        }
    };
}

fn solve(puzzle_input: PuzzleInput) -> i64 {
    let mut player_scores = vec![0; puzzle_input.num_players as usize];
    let mut marbles: LinkedList<i64> = LinkedList::new();
    let mut cursor = marbles.cursor_front_mut();

    for i in 0..puzzle_input.top_marble + 1 {
        if is_scoring_round(i) {
            for _ in 0..7 {
                cursor.circular_move_prev();
            }
            let current_player = (i % puzzle_input.num_players) as usize;
            player_scores[current_player] += cursor.remove_current().unwrap() + i;
            if cursor.current().is_none() {
                cursor.move_next();
            }
        } else {
            cursor.circular_move_next();
            cursor.insert_after(i);
            cursor.circular_move_next();
        }
    }
    *player_scores.iter().max().unwrap()
}

fn is_scoring_round(round: i64) -> bool {
    round > 0 && (round % 23) == 0
}

pub trait CursorMutExt<T> {
    fn circular_move_next(&mut self);
    fn circular_move_prev(&mut self);
}

impl<'a, T> CursorMutExt<T> for CursorMut<'a, T> {
    fn circular_move_next(&mut self) {
        self.move_next();
        if self.current().is_none() {
            self.move_next();
        }
    }
    fn circular_move_prev(&mut self) {
        self.move_prev();
        if self.current().is_none() {
            self.move_prev();
        }
    }
}
