extern crate regex;

use common;
use self::regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Vector {
    x: i64,
    y: i64,
    z: i64
}

impl Vector {
    pub fn manhattan_distance(&self, other: &Vector) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as u64
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
struct Particle {
    index: usize,
    position: Vector,
    velocity: Vector,
    acceleration: Vector
}

impl Particle {
    pub fn parse(index: usize, line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"p=<([-, ]?\d+),([-, ]?\d+),([-, ]?\d+)>,\s+v=<([-, ]?\d+),([-, ]?\d+),([-, ]?\d+)>,\s+a=<([-, ]?\d+),([-, ]?\d+),([-, ]?\d+)>"#).unwrap();
        }

        let captures = RE.captures(line).unwrap();
        let px = captures.get(1).unwrap().as_str().trim().parse::<i64>().unwrap();
        let py = captures.get(2).unwrap().as_str().trim().parse::<i64>().unwrap();
        let pz = captures.get(3).unwrap().as_str().trim().parse::<i64>().unwrap();

        let vx = captures.get(4).unwrap().as_str().trim().parse::<i64>().unwrap();
        let vy = captures.get(5).unwrap().as_str().trim().parse::<i64>().unwrap();
        let vz = captures.get(6).unwrap().as_str().trim().parse::<i64>().unwrap();

        let ax = captures.get(7).unwrap().as_str().trim().parse::<i64>().unwrap();
        let ay = captures.get(8).unwrap().as_str().trim().parse::<i64>().unwrap();
        let az = captures.get(9).unwrap().as_str().trim().parse::<i64>().unwrap();

        let position = Vector { x: px, y: py, z: pz };
        let velocity = Vector { x: vx, y: vy, z: vz };
        let acceleration = Vector { x: ax, y: ay, z: az };

        Particle { index, position, velocity, acceleration }
    }

    pub fn step(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    pub fn step_and_check_dist(&mut self) -> bool {
        let origin = Vector { x: 0, y: 0, z: 0 };
        let original_dist = self.position.manhattan_distance(&origin);
        self.step();
        let new_dist = self.position.manhattan_distance(&origin);

        new_dist < original_dist
    }
}

struct ParticleSystem {
    particles: Vec<Particle>
}

impl ParticleSystem {
    pub fn new(particles: Vec<Particle>) -> Self {
        ParticleSystem { particles }
    }

    pub fn run_until_min_dist(&mut self) {
        while self.step_all() > 0 {
        }
    }

    pub fn run_until_collisions(&mut self) {
        let mut steps_since_collision = 0;
        while steps_since_collision < 10000 {
            self.step_all();
            if self.remove_colliding() {
                steps_since_collision = 0;
            } else {
                steps_since_collision += 1;
            }
        }
    }

    pub fn run_more(&mut self, n: usize) {
        for _ in 0..n {
            self.step_all();
        }
    }

    fn step_all(&mut self) -> usize {
        let mut getting_closer: usize = 0;
        for particle in self.particles.iter_mut() {
            if particle.step_and_check_dist() {
                getting_closer += 1;
            }
        }
        getting_closer
    }

    fn remove_colliding(&mut self) -> bool {
        let mut map: HashMap<Vector, HashSet<usize>> = HashMap::new();

        for (idx, particle) in self.particles.iter().enumerate() {
            let set = map.entry(particle.position).or_insert(HashSet::new());
            set.insert(idx);
        }

        let mut colliding_particles: Vec<&usize> =
            map.values()
                .filter(|set| set.len() > 1)
                .flat_map(|set| set)
                .collect();
        colliding_particles.sort_by(|&a, &b| a.cmp(b).reverse());

        for &&idx in colliding_particles.iter() {
            self.particles.remove(idx);
            println!("Removed {}", idx);
        }

        colliding_particles.len() > 0
    }
}

fn compare_particle(a: &Particle, b: &Particle) -> Ordering {
    let origin = Vector { x: 0, y: 0, z: 0 };
    a.position.manhattan_distance(&origin).cmp(&b.position.manhattan_distance(&origin))
}

fn part1(particles: Vec<Particle>) {
    let mut particle_system = ParticleSystem::new(particles);
    particle_system.run_until_min_dist();
    particle_system.run_more(100000);

    let mut sorted_result: Vec<Particle> = particle_system.particles.iter().map(|p| p.clone()).collect();
    sorted_result.sort_by(compare_particle);

    println!("Day 20 result 1: {:?}", sorted_result[0].index)
}

fn part2(particles: Vec<Particle>) {
    let mut particle_system = ParticleSystem::new(particles);
    particle_system.run_until_collisions();
    println!("Day 20 result 2: {}", particle_system.particles.len());
}

pub fn run() {
    let input = common::read_data("day20.txt");
    let particles: Vec<Particle> = input.split("\n").enumerate().map(|(idx, line)| Particle::parse(idx, line)).collect();
    part1(particles.clone());
    part2(particles.clone());
}