use std::error::Error;
use std::fs::File;

use std::collections::HashSet;
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

fn find_basin(heighmap: &Heightmap, x: usize, y: usize, basin: &mut HashSet<(usize, usize)>) {
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

    // insert the initial points
    basin.insert((x, y));

    for (x, y) in coords {
        let this = heighmap[x][y];
        if basin.contains(&(x, y)) {
            continue;
        }
        if this != 9 {
            // add
            // recurse
            basin.insert((x, y));
            find_basin(heighmap, x, y, basin);
        }
    }
}

fn three_largests_basins(heighmap: &Heightmap) -> i32 {
    // 1. Find all lowpoints
    // 2. For each lowpoint
    // 3.   For each neighbour, if they are not 9
    //      yes => expand basin and recurse
    //      no  => stop

    let rows = heighmap.len();
    let cols = heighmap[0].len();

    let mut lowpoints = Vec::new();

    for x in 0..rows {
        for y in 0..cols {
            let curr: i32 = heighmap[x][y].try_into().unwrap();
            if is_lowpoint(heighmap, x, y) {
                lowpoints.push((x, y));
            }
        }
    }

    let mut sizes = Vec::new();

    for lowpoint in lowpoints {
        let mut basin_coordinates: HashSet<(usize, usize)> = HashSet::new();
        find_basin(heighmap, lowpoint.0, lowpoint.1, &mut basin_coordinates);
        sizes.push(basin_coordinates.len());
    }

    sizes.sort();
    sizes.reverse();

    sizes[0..3].iter().fold(1, |a, i| a * i) as i32
}

fn main() -> Result<(), Box<dyn Error>> {
    let map = heighmap();
    println!("Result: {}", find_lowpoints(&map));
    println!("Result: {}", three_largests_basins(&map));
    Ok(())
}
