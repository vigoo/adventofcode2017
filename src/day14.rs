extern crate multiarray;

use self::multiarray::*;
use day10::knot_hash;

fn row_map(row: u8, hash: &str) -> String {
    let hash = knot_hash(format!("{}-{}", hash, row).as_str());
    let parts: Vec<String> = hash.iter().map(|n| format!("{:8b}", n)).collect();
    parts.as_slice().join("")
}

fn usage_map(hash: &str) -> Array2D<bool> {
    let mut result: Array2D<bool> = Array2D::new([128, 128], false);

    for row in 0..(128 as usize) {
        let row_str = row_map(row as u8, hash);
        for col in 0..(128 as usize) {
            result[[row, col]] = row_str.chars().nth(col).unwrap() == '1';
        }
    }

    result
}

fn traverse_region(visited: &mut Array2D<bool>, map: &Array2D<bool>, row: usize, col: usize) {
    visited[[row, col]] = true;
    if col < 127 && map[[row, col + 1]] && !visited[[row, col + 1]] {
        traverse_region(visited, map, row, col + 1);
    }
    if row < 127 && map[[row + 1, col]] && !visited[[row + 1, col]] {
        traverse_region(visited, map, row + 1, col);
    }
    if col > 0 && map[[row, col - 1]] && !visited[[row, col - 1]] {
        traverse_region(visited, map, row, col - 1);
    }
    if row > 0 && map[[row - 1, col]] && !visited[[row - 1, col]] {
        traverse_region(visited, map, row - 1, col);
    }
}

fn count_regions(hash: &str) -> i32 {
    let map = usage_map(hash);
    let mut visited: Array2D<bool> = Array2D::new([128, 128], false);
    let mut region_count = 0;

    for row in 0..128 {
        for col in 0..128 {
            if !visited[[row, col]] {
                if map[[row, col]] {
                    traverse_region(&mut visited, &map, row, col);
                    region_count = region_count + 1;
                }
            }
        }
    }

    region_count
}

fn usage_count(row: &String) -> usize {
    row.chars().filter(|&ch| ch == '1').count()
}

pub fn run() {
    let input = "hxtvlmkl";
    println!("Day 14 result 1: {}", (0..128).map(|row| usage_count(&row_map(row, input))).sum::<usize>());
    println!("Day 14 result 2: {}", count_regions(input));
}
