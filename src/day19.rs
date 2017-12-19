extern crate multiarray;

use self::multiarray::*;
use common;

#[derive(Copy, Clone, Debug, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Coord {
    pub fn mov(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.x = self.x - 1,
            Direction::Right => self.x = self.x + 1,
            Direction::Up => self.y = self.y - 1,
            Direction::Down=> self.y = self.y + 1,
        }
    }

    pub fn next(&self, direction: Direction) -> Coord {
        match direction {
            Direction::Left => Coord { x: self.x - 1, y: self.y },
            Direction::Right => Coord { x: self.x + 1, y: self.y },
            Direction::Up => Coord { x: self.x, y: self.y - 1 },
            Direction::Down => Coord { x: self.x, y: self.y + 1 },
        }
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Map {
    map: Array2D<char>,
    starting_point: Coord
}

fn find_starting_point(first_row: &Array1DRef<char>) -> Coord {
    let starting_point_x = first_row
        .clone()
        .position(|&ch| ch == '|')
        .unwrap();

    Coord {
        x: starting_point_x as i32,
        y: 0
    }
}

impl Map {
    pub fn from_string(input: &str) -> Self {
        let lines: Vec<&str> = input.split("\n").collect();
        let max_width = lines.iter().map(|line| line.len()).max().unwrap();

        let mut map = Array2D::new([lines.len(), max_width], ' ');
        for (i, line) in lines.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                map[[i, j]] = ch;
            }
        }

        let starting_point = find_starting_point(&map.eliminated_dim(0, 0));

        Map {
            map,
            starting_point
        }
    }

    pub fn traverse(&self) -> (Vec<char>, usize) {
        let mut result: Vec<char> = vec![];
        let mut position = self.starting_point;
        let mut direction = Direction::Down;
        let mut ended = false;
        let mut steps = 0;

        while !ended && !self.is_outside(&position) {
            position.mov(direction);

            match self.at(&position) {
                Some(next) => {
                    if next.is_alphabetic() {
                        if !result.contains(&next) {
                            result.push(next);
                        }
                    } else if next == '+' {
                        let left = self.at(&position.next(Direction::Left));
                        let right = self.at(&position.next(Direction::Right));
                        let up = self.at(&position.next(Direction::Up));
                        let down = self.at(&position.next(Direction::Down));

                        if (direction == Direction::Up || direction == Direction::Down) &&
                            (left == Some('-') || left.map(|ch| ch.is_alphabetic()).unwrap_or(false)) {
                            direction = Direction::Left;
                        } else if (direction == Direction::Up || direction == Direction::Down) &&
                            (right == Some('-') || right.map(|ch| ch.is_alphabetic()).unwrap_or(false)) {
                            direction = Direction::Right;
                        } else if (direction == Direction::Left || direction == Direction::Right) &&
                            (up == Some('|') || up.map(|ch| ch.is_alphabetic()).unwrap_or(false)) {
                            direction = Direction::Up;
                        } else if (direction == Direction::Left || direction == Direction::Right) &&
                            (down == Some('|') || down.map(|ch| ch.is_alphabetic()).unwrap_or(false)) {
                            direction = Direction::Down;
                        }
                    }
                    else if next == ' ' {
                        ended = true;
                    }

                    steps = steps + 1;
                },
                None => {}
            }
        }

        (result, steps)
    }

    fn at(&self, coord: &Coord) -> Option<char> {
        if self.is_outside(coord) {
            None
        } else {
            Some(self.map[[coord.y as usize, coord.x as usize]])
        }
    }

    fn is_outside(&self, coord: &Coord) -> bool {
        let height = self.map.extents()[0] as i32;
        let width = self.map.extents()[1] as i32;
        coord.x < 0 ||
            coord.x >= width ||
            coord.y < 0 ||
            coord.y >= height
    }
}

pub fn run() {
    let input = common::read_data("day19.txt");
    let map = Map::from_string(input.as_str());

    println!("Starting point: {:?}", map.starting_point);
    let (result, steps) = map.traverse();

    println!("Day 19 part 1: {:?}", result);
    println!("Day 19 part 2: {}", steps);
}