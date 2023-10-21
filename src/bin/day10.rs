use anyhow::Error;
use std::path::PathBuf;
use std::{env, fs};

fn display(points: &[Point]) {
    let min_x = points.iter().map(|p| p.px).min().unwrap();
    let max_x = points.iter().map(|p| p.px).max().unwrap();
    let min_y = points.iter().map(|p| p.py).min().unwrap();
    let max_y = points.iter().map(|p| p.py).max().unwrap();

    let mut m = vec![vec![0; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for point in points.iter() {
        m[(point.py - min_y) as usize][(point.px - min_x) as usize] = 1;
    }
    for line in m.iter() {
        for val in line.iter() {
            print!("{}", if *val == 1 { '#' } else { '.' });
        }
        println!();
    }
}

fn solve(points: &mut [Point]) {
    let mut max_width: i32 = i32::MAX;
    let mut iterations = 0;
    loop {
        for point in points.iter_mut() {
            point.update()
        }
        let new_max_width =
            points.iter().map(|p| p.px).max().unwrap() - points.iter().map(|p| p.px).min().unwrap();
        if new_max_width > max_width {
            break;
        } else {
            iterations += 1;
            max_width = new_max_width;
        }
    }

    for point in points.iter_mut() {
        point.reverse()
    }

    display(points);

    println!("Sec: {}", iterations);
}

fn main() -> Result<(), Error> {
    let mut points = parse_input_data(get_input_data()?)?;
    solve(&mut points);
    Ok(())
}

fn get_input_data() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();
    let file_path: PathBuf = [
        "data".to_string(),
        if args.len() == 1 {
            "input10.txt".to_string()
        } else {
            args[1].clone()
        },
    ]
    .iter()
    .collect();

    Ok(fs::read_to_string(file_path)?)
}

#[derive(Debug)]
struct Point {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Point {
    fn update(&mut self) {
        self.px += self.vx;
        self.py += self.vy;
    }

    fn reverse(&mut self) {
        self.px -= self.vx;
        self.py -= self.vy;
    }
}

fn parse_input_line(line: &str) -> Result<Point, Error> {
    let parsed_numbers: Vec<i32> = line
        .split(|c: char| !(c.is_ascii_digit() || c == '-'))
        .filter(|s| !s.is_empty())
        .map(|d| d.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Point {
        px: parsed_numbers[0],
        py: parsed_numbers[1],
        vx: parsed_numbers[2],
        vy: parsed_numbers[3],
    })
}
fn parse_input_data(input: String) -> Result<Vec<Point>, Error> {
    input.lines().map(parse_input_line).collect()
}
