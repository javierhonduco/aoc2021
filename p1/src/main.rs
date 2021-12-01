use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("src/input.txt")?;
    let mut prev_value = None;
    let mut increment_count = 0;
    for line in io::BufReader::new(file).lines() {
        let current_value = line.unwrap().parse::<i32>().unwrap();

        match prev_value {
            Some(val) => {
                if current_value > val {
                    increment_count += 1;
                }
            }
            None => {}
        }

        prev_value = Some(current_value);
    }

    println!("result: {}", increment_count);
    Ok(())
}
