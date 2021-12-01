use std::error::Error;
use std::fs::File;
use std::io::Seek;
use std::io::{self, BufRead};
fn count(f: &File) -> i32 {
    let mut prev_value = None;
    let mut increment_count = 0;
    for line in io::BufReader::new(f).lines() {
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

    increment_count
}

fn window_count(f: &File) -> i32 {
    let mut running_count: Vec<i32> = Vec::new();
    let mut increment_count = 0;
    let mut last_sum = None;
    for line in io::BufReader::new(f).lines() {
        let current_value = line.unwrap().parse::<i32>().unwrap();
        if running_count.len() == 3 {
            running_count.remove(0);
        }

        running_count.push(current_value);

        if running_count.len() == 3 {
            let sum = running_count.iter().sum::<i32>();
            if last_sum.is_none() {}

            match last_sum {
                Some(val) => {
                    if sum > val {
                        increment_count += 1;
                    }
                }
                None => {}
            }

            last_sum = Some(sum);
        }
    }
    increment_count
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("src/input.txt")?;
    println!("count: {}", count(&file));
    file.rewind()?;
    println!("window_count: {}", window_count(&file));
    Ok(())
}
