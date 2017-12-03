
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Coord {
    pub fn manhattan_distance(&self, other: &Coord) -> u32 {
        return ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32;
    }

    pub fn left(&self) -> Coord {
        return Coord { x: self.x - 1, y: self.y };
    }

    pub fn right(&self) -> Coord {
        return Coord { x: self.x + 1, y: self.y };
    }

    pub fn up(&self) -> Coord {
        return Coord { x: self.x, y: self.y - 1 };
    }

    pub fn down(&self) -> Coord {
        return Coord { x: self.x, y: self.y + 1};
    }
}



struct CoordinateSystem {
    cache: HashMap<u32, Coord>
}

impl CoordinateSystem {
    pub fn new() -> CoordinateSystem {
        return CoordinateSystem {
            cache: HashMap::new()
        }
    }

    pub fn coords_of(&mut self, n: u32) -> Coord {
        let result: &Coord = self.cache.entry(n.to_owned()).or_insert_with(|| { return CoordinateSystem::calc_coords_of(n) });
        return *result;
    }

    fn calc_coords_of(n: u32) -> Coord {
        assert_ne!(n, 0);
        let mut from = 2;
        let mut level = 1;
        let mut result = Coord { x: 0, y: 0 };

        if n > 1 {
            loop {
                let width = (level * 2) + 1;
                let capacity = 4 * (width - 1);
                let next = from + capacity;

                if n < next {
                    let idx = n - from;
                    let segment = idx  / (width - 1);
                    let pos = (idx % (width - 1)) as i32;
                    let l = level as i32;

                    result =
                        match segment {
                            0 => Coord { x: l, y: l - 1 - pos },
                            1 => Coord { x: l - 1 - pos, y: -l },
                            2 => Coord { x: -l, y: -l + 1 + pos },
                            3 => Coord { x: -l + 1 + pos, y: l },
                            _ => panic!("Segment was {}", segment)
                        };

                    break;
                } else {
                    from = next;
                    level += 1;
                }
            }
        }
        return result;
    }
}

fn calc_value(squares: &HashMap<Coord, u32>, coord: &Coord) -> u32 {
    return squares.get(&coord.left()).unwrap_or(&0) +
           squares.get(&coord.right()).unwrap_or(&0) +
           squares.get(&coord.up()).unwrap_or(&0) +
           squares.get(&coord.down()).unwrap_or(&0) +
           squares.get(&coord.left().up()).unwrap_or(&0) +
           squares.get(&coord.right().up()).unwrap_or(&0) +
           squares.get(&coord.left().down()).unwrap_or(&0) +
           squares.get(&coord.right().down()).unwrap_or(&0);
}

fn find_first_larger_cell(csys: &mut CoordinateSystem, than: u32) -> u32 {
    let mut squares: HashMap<Coord, u32> = HashMap::new();
    let mut n: u32 = 1;
    let mut coord: Coord = csys.coords_of(n);
    let mut value: u32 = 1;
    squares.insert(coord, value);

    while value < than {
        n = n + 1;
        coord = csys.coords_of(n);
        value = calc_value(&squares, &coord);
        squares.insert(coord, value);
    }

    return value;
}

fn debug(csys: &mut CoordinateSystem, n: u32) {
    println!("{}: {:?} d: {}", n, csys.coords_of(n), csys.coords_of(n).manhattan_distance(&Coord { x: 0, y: 0 }));
}

pub fn run() {
    let mut csys = CoordinateSystem::new();
    debug(&mut csys, 1);
    debug(&mut csys, 12);
    debug(&mut csys, 23);
    debug(&mut csys, 1024);
    debug(&mut csys, 277678);
    println!("Day 3 result 1: {}", csys.coords_of(277678).manhattan_distance(&Coord { x: 0, y: 0 }));
    println!("Day 3 result 2: {}", find_first_larger_cell(&mut csys, 277678));
}