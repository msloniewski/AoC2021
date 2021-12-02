use std::env;
use std::fs;

#[derive(Debug)]
enum Command {
    Forwad(i64),
    Down(i64),
    Up(i64),
}

#[derive(Debug)]
struct SubPosition {
    position_x: i64,
    position_y: i64,
    aim: i64,
}

impl SubPosition {
    fn run_command(&mut self, cmd: &Command) {
        match cmd {
            Command::Forwad(val) => self.position_x += val,
            Command::Down(val) => self.position_y += val,
            Command::Up(val) => self.position_y -= val,
        }
    }

    fn run_command_ver2(&mut self, cmd: &Command) {
        match cmd {
            Command::Forwad(val) => {
                self.position_x += val;
                self.position_y += val * self.aim;
            }
            Command::Down(val) => self.aim += val,
            Command::Up(val) => self.aim -= val,
        }
    }

    fn new() -> Self {
        Self {
            position_x: 0,
            position_y: 0,
            aim: 0,
        }
    }
}

fn main() {
    let mut args = env::args();

    // call name
    args.next();
    let data = fs::read_to_string(args.next().expect("Provide filename to use!"))
        .expect("Unable to read file");

    let commands_strings = data.split("\n").filter(|&x| !x.is_empty());

    let mut commands: Vec<Command> = Vec::new();
    for cmd_string in commands_strings {
        let mut cmd_parts = cmd_string.trim().split_whitespace();

        let dir = cmd_parts.next().unwrap();
        let val = cmd_parts.next().unwrap().parse().unwrap();

        let cmd = match dir {
            "forward" => Command::Forwad(val),
            "down" => Command::Down(val),
            "up" => Command::Up(val),
            _ => panic!(),
        };

        commands.push(cmd);
    }

    let mut sub_pos = SubPosition::new();

    for cmd in &commands {
        sub_pos.run_command(&cmd);
    }

    println!(
        "day2 part1 ans= {}",
        sub_pos.position_y.abs() * sub_pos.position_x.abs()
    );

    let mut sub_pos = SubPosition::new();

    for cmd in &commands {
        sub_pos.run_command_ver2(&cmd);
    }

    println!(
        "day2 part2 ans= {}",
        sub_pos.position_y.abs() * sub_pos.position_x.abs()
    );
}
