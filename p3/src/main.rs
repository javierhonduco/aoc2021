use std::error::Error;
use std::fs::File;

use std::io::{self, BufRead};

type Diagnostic = Vec<u32>;

fn parse_diagnostic() -> Diagnostic {
    let file = File::open("src/input.txt").unwrap();
    let mut diagnostic = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let number = u32::from_str_radix(&line.unwrap(), 2).unwrap();
        diagnostic.push(number);
    }
    diagnostic
}

fn power_consumption_1(diagnostic: &Diagnostic, bit_count: usize) -> u64 {
    assert!(bit_count <= 32);
    let mut tally: Vec<u32> = vec![0; bit_count];
    for number in diagnostic {
        for bit_idx in 0..bit_count {
            let mask = 1 << bit_idx;
            tally[bit_idx] += ((number & mask) != mask) as u32;
        }
    }

    let mut gamma = 0u64;
    let mut epsilon = 0u64;

    for (i, ones_count) in tally.iter().enumerate() {
        let zeros_count = diagnostic.len() as u32 - ones_count;
        println!("zeroes {} ones {}", zeros_count, ones_count);

        if ones_count > &zeros_count {
            // more 1s

            gamma = gamma | 1 << i;
        } else {
            // more 0s

            epsilon = epsilon | 1 << i;
        }
    }
    gamma * epsilon
}

fn main() -> Result<(), Box<dyn Error>> {
    let diagnostic = parse_diagnostic();
    println!("Result: {:?} ", power_consumption_1(&diagnostic, 12));

    Ok(())
}
