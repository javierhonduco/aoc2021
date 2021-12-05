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
    assert!(bit_count < 32);
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

fn rating(diagnostic: &Diagnostic, bit_count: usize, count_ones: bool) -> u32 {
    let mut num_zeroes = 0;
    let mut num_ones = 0;

    let mut result = diagnostic.clone();
    let mut diagnostics_starting_with_ones = Vec::new();
    let mut diagnostics_starting_with_zeroes = Vec::new();

    for bit_idx in (0..bit_count).rev() {
        num_zeroes = 0;
        num_ones = 0;
        diagnostics_starting_with_ones = Vec::new();
        diagnostics_starting_with_zeroes = Vec::new();

        for number in &result {
            let mask = 1 << bit_idx;
            if (number & mask) == mask {
                num_ones += 1;
                diagnostics_starting_with_ones.push(*number);
            } else {
                num_zeroes += 1;
                diagnostics_starting_with_zeroes.push(*number);
            }
        }

        // not great, but alas
        if count_ones {
            if num_ones >= num_zeroes {
                result = diagnostics_starting_with_ones.clone();
            } else {
                result = diagnostics_starting_with_zeroes.clone();
            }
        } else {
            if num_ones < num_zeroes {
                result = diagnostics_starting_with_ones.clone();
            } else {
                result = diagnostics_starting_with_zeroes.clone();
            }
        }

        if result.len() == 1 {
            return result.pop().unwrap();
        }
    }

    panic!("Should not happen");
}

fn oxygen_support_rating(diagnostic: &Diagnostic, bit_count: usize) -> u32 {
    rating(diagnostic, bit_count, true) * rating(diagnostic, bit_count, false)
}

fn main() -> Result<(), Box<dyn Error>> {
    let diagnostic = parse_diagnostic();
    println!("Result: {:?} ", power_consumption_1(&diagnostic, 12));
    println!("Result: {:?} ", oxygen_support_rating(&diagnostic, 12));

    Ok(())
}
