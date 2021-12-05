use std::error::Error;
use std::fs::File;

use std::io::{self, BufRead};

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

type Commands = Vec<Command>;

fn parse_commands() -> Commands {
    let file = File::open("src/input.txt").unwrap();
    let mut commands = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let the_line = line.unwrap(); // appease the borrow checker
        let mut splitted = the_line.split(" ");
        let (command_string, how_much) = (
            splitted.next().unwrap(),
            splitted.next().unwrap().parse::<i32>().unwrap(),
        );

        let command = match command_string {
            "forward" => Command::Forward(how_much),
            "down" => Command::Down(how_much),
            "up" => Command::Up(how_much),
            _ => {
                panic!("did not expect this command")
            }
        };

        commands.push(command);
    }
    commands
}

fn pilot_submarine_1(commands: &Commands) -> i32 {
    let mut state = (0, 0); // x, y position starting from the top left
    for command in commands {
        match command {
            Command::Forward(how_much) => {
                state.0 += how_much;
            }
            Command::Down(how_much) => state.1 += how_much,
            Command::Up(how_much) => state.1 -= how_much,
        }
    }

    state.0 * state.1
}

fn pilot_submarine_2(commands: &Commands) -> i32 {
    let mut state = (0, 0, 0); // x, y, aim
    for command in commands {
        match command {
            Command::Forward(how_much) => {
                state.0 += how_much;
                state.1 += state.2 * how_much;
            }
            Command::Down(how_much) => state.2 += how_much,
            Command::Up(how_much) => state.2 -= how_much,
        }
    }

    state.0 * state.1
}

fn main() -> Result<(), Box<dyn Error>> {
    let commands = parse_commands();
    println!("Result: {} ", pilot_submarine_1(&commands));
    println!("Result: {} ", pilot_submarine_2(&commands));

    Ok(())
}
