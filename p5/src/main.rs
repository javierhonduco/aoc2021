use std::error::Error;
use std::fs::File;

use std::io::{self, BufRead};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

type Line = (Point, Point);
type Lines = Vec<Line>;

fn parse_lines() -> Lines {
    let file = File::open("src/input.txt").unwrap();
    let mut diagnostic = Lines::new();
    for line in io::BufReader::new(file).lines() {
        let le_line = line.unwrap();
        let mut splitted = le_line.split(" -> ");

        let (p1, p2) = (splitted.next().unwrap(), splitted.next().unwrap());
        let mut p1_s = p1.split(",");
        let mut p2_s = p2.split(",");

        // figure out how to make this nicer
        diagnostic.push((
            Point {
                x: p1_s.next().unwrap().parse::<usize>().unwrap(),
                y: p1_s.next().unwrap().parse::<usize>().unwrap(),
            },
            Point {
                x: p2_s.next().unwrap().parse::<usize>().unwrap(),
                y: p2_s.next().unwrap().parse::<usize>().unwrap(),
            },
        ));
    }
    diagnostic
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = parse_lines();

    let mut result: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for (point_a, point_b) in &lines {
        if point_a.x == point_b.x {
            // horizontal line
            let x = point_a.x;

            for y in point_a.y..=point_b.y {
                result[y][x] += 1;
            }

            // this is a gross hack as decreasing iterators
            // won't run
            for y in point_b.y..=point_a.y {
                result[y][x] += 1;
            }
        } else if point_a.y == point_b.y {
            // vertical line
            let y = point_a.y;
            for x in point_a.x..=point_b.x {
                result[y][x] += 1;
            }
            for x in point_b.x..=point_a.x {
                result[y][x] += 1;
            }
        } else {
            // ignore diagonals
        }
    }

    let mut count = 0;
    for rows in &result {
        for col in rows {
            if *col > 1 {
                count += 1;
            }
        }
    }

    println!("{:?}", count);
    Ok(())
}
