use std::error::Error;
use std::fs::File;

use std::collections::HashMap;
use std::io::{self, BufRead};

type Heightmap = Vec<Vec<u8>>;

fn heighmap() -> Heightmap {
    let mut heightmap: Vec<Vec<u8>> = vec![vec![0; 0]; 0];

    let file = File::open("src/input.txt").unwrap();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let splitted = line.split("");
        let row = line
            .chars()
            .map(|x| -> u8 { x.to_digit(10).unwrap().try_into().unwrap() })
            .collect::<Vec<u8>>();
        heightmap.push(row);
    }

    heightmap
}

fn is_lowpoint(heighmap: &Heightmap, x: usize, y: usize) -> bool {
    let mut coords = Vec::new();

    let center = heighmap[x][y];
    let rows = heighmap.len();
    let cols = heighmap[0].len();

    // check above
    if x >= 1 {
        coords.push((x - 1, y));
    }
    // check below
    if x < rows - 1 {
        coords.push((x + 1, y));
    }
    // check left
    if y >= 1 {
        coords.push((x, y - 1));
    }
    // check below
    if y < cols - 1 {
        coords.push((x, y + 1));
    }

    for (x, y) in coords {
        let this = heighmap[x][y];
        if this <= center {
            return false;
        }
    }

    true
}
fn find_lowpoints(heighmap: &Heightmap) -> i32 {
    let rows = heighmap.len();
    let cols = heighmap[0].len();

    let mut result: i32 = 0;

    for x in 0..rows {
        for y in 0..cols {
            let curr: i32 = heighmap[x][y].try_into().unwrap();
            if is_lowpoint(heighmap, x, y) {
                result += curr + 1;
            }
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let map = heighmap();
    println!("Result: {}", find_lowpoints(&map));

    Ok(())
}
