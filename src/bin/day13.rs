use anyhow::Error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Track {
    None,
    Vertical,
    Horizontal,
    CurveForward,
    CurveBackward,
    Intersection,
}

impl Track {
    fn from_char(c: char) -> Result<Self, Error> {
        Ok(match c {
            ' ' => Track::None,
            '|' | 'v' | '^' => Track::Vertical,
            '-' | '>' | '<' => Track::Horizontal,
            '/' => Track::CurveForward,
            '\\' => Track::CurveBackward,
            '+' => Track::Intersection,
            _ => Err(Error::msg("Invalid input data"))?,
        })
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(self) -> Self {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        Some(match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => None?,
        })
    }
}

#[derive(Clone)]
struct Cart {
    location: (i32, i32),
    direction: Direction,
    next_turn: Turn,
}

impl Cart {
    fn turn(&mut self) {
        match self.next_turn {
            Turn::Straight => {}
            Turn::Left => {
                self.direction = match self.direction {
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                }
            }
            Turn::Right => {
                self.direction = match self.direction {
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                }
            }
        }
        self.next_turn = self.next_turn.next();
    }

    fn _step_forward(&mut self) {
        let (x, y) = self.location;
        self.location = match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
    }

    fn step(&mut self, map: &Map) {
        self._step_forward();
        let (x, y) = self.location;
        match (map[y as usize][x as usize], &self.direction) {
            (Track::Horizontal, _) | (Track::Vertical, _) => {}
            (Track::None, _) => panic!("Programming error"),
            (Track::CurveForward, Direction::Down) => self.direction = Direction::Left,
            (Track::CurveForward, Direction::Left) => self.direction = Direction::Down,
            (Track::CurveForward, Direction::Up) => self.direction = Direction::Right,
            (Track::CurveForward, Direction::Right) => self.direction = Direction::Up,
            (Track::CurveBackward, Direction::Down) => self.direction = Direction::Right,
            (Track::CurveBackward, Direction::Right) => self.direction = Direction::Down,
            (Track::CurveBackward, Direction::Up) => self.direction = Direction::Left,
            (Track::CurveBackward, Direction::Left) => self.direction = Direction::Up,
            (Track::Intersection, _) => self.turn(),
        }
    }
}

fn get_crashed_cart(carts: &[Cart], moved_index: usize) -> Option<usize> {
    carts
        .iter()
        .enumerate()
        .filter(|(i, c)| *i != moved_index && c.location == carts[moved_index].location)
        .map(|(i, _c)| i)
        .next()
}

fn solve1(map: &Map, mut carts: Vec<Cart>) -> (i32, i32) {
    loop {
        carts.sort_by_key(|c| c.location);
        for cart_index in 0..carts.len() {
            carts[cart_index].step(map);
            if get_crashed_cart(&carts, cart_index).is_some() {
                return carts[cart_index].location;
            }
        }
    }
}

fn solve2(map: &Map, mut carts: Vec<Cart>) -> (i32, i32) {
    loop {
        carts.sort_by_key(|c| c.location);
        let mut cart_index = 0;
        while cart_index < carts.len() {
            carts[cart_index].step(map);
            if let Some(crashed_index) = get_crashed_cart(&carts, cart_index) {
                if crashed_index < cart_index {
                    carts.remove(cart_index);
                    carts.remove(crashed_index);
                    cart_index -= 1;
                } else {
                    carts.remove(crashed_index);
                    carts.remove(cart_index);
                }
                if carts.len() == 1 {
                    return carts[0].location;
                }
            } else {
                cart_index += 1
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let (map, carts) = parse_input(&read_to_string(Path::new("data/input13.txt"))?)?;
    let (x, y) = solve1(&map, carts.clone());
    println!("{},{}", x, y);
    let (x, y) = solve2(&map, carts);
    println!("{},{}", x, y);
    Ok(())
}

type Map = Vec<Vec<Track>>;

fn parse_carts(input: &str) -> Vec<Cart> {
    fn parse_line(line: &str, line_number: usize) -> Vec<Cart> {
        line.chars()
            .map(Direction::from_char)
            .enumerate()
            .filter(|(_i, d)| d.is_some())
            .map(|(i, d)| Cart {
                location: (i as i32, line_number as i32),
                direction: d.unwrap(),
                next_turn: Turn::Left,
            })
            .collect()
    }

    input
        .lines()
        .enumerate()
        .flat_map(|(line_number, line)| parse_line(line, line_number))
        .collect()
}

fn parse_map(input: &str) -> Result<Map, Error> {
    fn parse_line(line: &str) -> Result<Vec<Track>, Error> {
        line.chars()
            .map(Track::from_char)
            .collect::<Result<Vec<Track>, _>>()
    }

    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()
}
fn parse_input(input: &str) -> Result<(Map, Vec<Cart>), Error> {
    let map = parse_map(input)?;
    let carts = parse_carts(input);

    Ok((map, carts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        let input = " |- \n/\\+ \n<>v^";
        let expected = [
            [Track::None, Track::Vertical, Track::Horizontal, Track::None],
            [
                Track::CurveForward,
                Track::CurveBackward,
                Track::Intersection,
                Track::None,
            ],
            [
                Track::Horizontal,
                Track::Horizontal,
                Track::Vertical,
                Track::Vertical,
            ],
        ];
        let parsed = parse_map(input).unwrap();
        assert_eq!(parsed, expected);
    }
}
