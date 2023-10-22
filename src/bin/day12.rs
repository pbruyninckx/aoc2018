use anyhow::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

const NUM_RULES: i64 = 32;

#[derive(Debug)]
struct Rules {
    produce_sets: Vec<i64>,
}

impl Rules {
    fn from_lines(lines: &mut Lines<BufReader<File>>) -> Result<Self, Error> {
        let mut produce_sets = vec![0; NUM_RULES as usize];
        for line_result in lines {
            let line = line_result?;
            if line.ends_with('#') {
                let index = line
                    .chars()
                    .take(5)
                    .map(|c| (c == '#') as i64)
                    .fold(0, |acc, el| 2 * acc + el);
                produce_sets[index as usize] = 1;
            }
        }
        Ok(Self { produce_sets })
    }
    fn produce(&self, combo: i64) -> i64 {
        self.produce_sets[combo as usize]
    }
}

#[derive(Debug)]
struct Input {
    state: State,
    rules: Rules,
}

#[derive(Debug, Clone)]
struct State {
    state: Vec<i64>,
    index: i64,
}

impl State {
    fn from_string(s: &str) -> Self {
        Self {
            state: s.chars().map(|c| (c == '#') as i64).collect(),
            index: 0,
        }
    }

    fn sum_plant_indices(&self) -> i64 {
        self.state
            .iter()
            .enumerate()
            .filter(|(_i, val)| val != &&0)
            .map(|(i, _val)| i as i64 + self.index)
            .sum()
    }

    fn next(&self, rules: &Rules) -> Self {
        let numeric = self
            .state
            .iter()
            .chain([0, 0, 0].iter())
            .fold(vec![0], |mut acc, el| {
                acc.push((2 * acc.last().unwrap() + *el) % NUM_RULES);
                acc
            });
        let next_state_it = numeric.iter().map(|v| rules.produce(*v));
        let num_zeros = next_state_it.clone().take_while(|v| *v == 0).count();
        State {
            state: next_state_it.skip(num_zeros).collect(),
            index: self.index - 3 + (num_zeros as i64),
        }
    }
}

fn parse_input(filename: &Path) -> Result<Input, Error> {
    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let start_state = State::from_string(
        lines
            .next()
            .ok_or(Error::msg("Empty file"))??
            .rsplit_once(' ')
            .ok_or(Error::msg("Invalid input file"))?
            .1,
    );
    lines.next().ok_or(Error::msg("Invalid input file"))??;

    let rules = Rules::from_lines(&mut lines)?;

    let result = Input {
        state: start_state,
        rules,
    };
    Ok(result)
}

fn solve(input: &Input, num_iterations: i64) -> i64 {
    let rules = &input.rules;
    let mut state = input.state.clone();

    for i in 0..num_iterations {
        let old_state = state.clone();
        state = state.next(rules);

        if old_state.state == state.state {
            let sum_diff = state.sum_plant_indices() - old_state.sum_plant_indices();
            return state.sum_plant_indices() + (num_iterations - i - 1) * sum_diff;
        }
    }

    state.sum_plant_indices()
}

fn main() -> Result<(), Error> {
    let input = parse_input(Path::new("data/input12.txt"))?;
    for num_iterations in [20, 50000000000] {
        println!("{}", solve(&input, num_iterations));
    }

    Ok(())
}
