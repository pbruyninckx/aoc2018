use anyhow::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

const NUM_RULES: i32 = 32;

#[derive(Debug)]
struct Rules {
    produce_sets: Vec<i32>,
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
                    .map(|c| (c == '#') as i32)
                    .fold(0, |acc, el| 2 * acc + el);
                produce_sets[index as usize] = 1;
            }
        }
        Ok(Self { produce_sets })
    }
    fn produce(&self, combo: i32) -> i32 {
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
    state: Vec<i32>,
    index: i32,
}

impl State {
    fn from_string(s: &str) -> Self {
        Self {
            state: s.chars().map(|c| (c == '#') as i32).collect(),
            index: 0,
        }
    }

    fn sum_plant_indices(&self) -> i32 {
        self.state.iter().enumerate().filter(|(_i, val)| val != &&0).map(|(i, _val)| i as i32 + self.index).sum()
    }

    fn next(&self, rules: &Rules) -> Self {
        let numeric = self
            .state
            .iter()
            .chain(vec![0, 0, 0].iter())
            .fold(vec![0], |mut acc, el| {
                acc.push((2 * acc.last().unwrap() + *el) % NUM_RULES);
                acc
            });
        let next_state = numeric
            .iter()
            .map(|v| rules.produce(*v))
            .skip(1)
            .collect();
        State { state: next_state , index: self.index-2}
    }
}

fn parse_input(filename: &Path) -> Result<Input, Error> {
    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let start_state = State::from_string(
        lines
            .next()
            .ok_or(Error::msg("Empty file"))??
            .rsplit_once(" ")
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

fn solve(input: &Input) -> i32 {
    let rules = &input.rules;
    let mut state = input.state.clone();

    for _i in 0..20 {
        state = state.next(&rules);
    }

    state.sum_plant_indices()
}

fn main() -> Result<(), Error> {
    let input = parse_input(Path::new("data/input12.txt"))?;
    println!("{}", solve(&input));
    Ok(())
}
