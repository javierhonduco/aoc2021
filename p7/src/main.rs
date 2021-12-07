use std::error::Error;
use std::fs::File;

use std::collections::HashMap;
use std::io::{self, BufRead};

type Positions = Vec<i32>;

fn crab_positions() -> Positions {
    let file = File::open("src/input.txt").unwrap();
    let line = io::BufReader::new(file).lines().next().unwrap().unwrap();
    let splitted = line.split(",");
    splitted
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect::<Positions>()
}

fn min_crab_moves(pos: &Positions) -> i32 {
    let max_x = *pos.iter().max().unwrap();
    let mut cache: HashMap<i32, i32> = HashMap::new();

    let mut min_moves: Option<i32> = None;
    for x in 0..=max_x {
        let mut accum = 0;
        for crab_pos in pos {
            let diff = (crab_pos - x).abs();
            accum += diff;
        }

        if min_moves == None || accum < min_moves.unwrap() {
            min_moves = Some(accum);
        }
        accum = 0;
    }

    min_moves.unwrap()
}

fn actual_diff(upto: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    let mut sum = 0;
    for val in (1..=upto).rev() {
        match cache.get(&val) {
            Some(cached) => {
                sum += cached;
                break;
            }
            None => {}
        }
        sum += val;
    }

    cache.insert(upto, sum);
    sum
}
fn min_crab_moves_sum(pos: &Positions) -> i32 {
    let max_x = *pos.iter().max().unwrap();
    let mut cache: HashMap<i32, i32> = HashMap::new();

    let mut min_moves: Option<i32> = None;
    for x in 0..=max_x {
        let mut accum = 0;
        for crab_pos in pos {
            let diff = (crab_pos - x).abs();
            accum += actual_diff(diff, &mut cache);
        }

        if min_moves == None || accum < min_moves.unwrap() {
            min_moves = Some(accum);
        }
        accum = 0;
    }

    min_moves.unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let pos = crab_positions();
    println!("Result: {}", min_crab_moves(&pos));
    println!("Result: {}", min_crab_moves_sum(&pos));

    Ok(())
}
