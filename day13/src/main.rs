use std::cmp;
use std::env;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Paper {
    dots: Vec<Vec<bool>>,
}

#[derive(Debug)]
enum Instruction {
    FoldUp(usize),
    FoldLeft(usize),
}

fn create_instruction(instr_string: &String) -> Instruction {
    let mut instr_split = instr_string.split_whitespace();

    instr_split.next();
    instr_split.next();

    let instr_split = instr_split.next().unwrap().to_string();
    let mut instr_split = instr_split.split("=");

    let axis = instr_split.next().unwrap().to_string();
    let value: usize = instr_split.next().unwrap().to_string().parse().unwrap();
    if axis == "x" {
        return Instruction::FoldLeft(value);
    } else {
        return Instruction::FoldUp(value);
    }
}

impl Paper {
    fn new(dots_init: Vec<String>) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        for row in &dots_init {
            let mut coord = row.split(",");
            let x = coord.next().unwrap().to_string().parse::<usize>().unwrap();
            let y = coord.next().unwrap().to_string().parse::<usize>().unwrap();
            max_x = cmp::max(x, max_x);
            max_y = cmp::max(y, max_y);
        }
        let mut dots: Vec<Vec<bool>> = vec![vec![false; max_x + 1]; max_y + 1];

        for row in &dots_init {
            let mut coord = row.split(",");
            let x = coord.next().unwrap().to_string().parse::<usize>().unwrap();
            let y = coord.next().unwrap().to_string().parse::<usize>().unwrap();
            dots[y][x] = true;
        }

        Self { dots: dots }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::FoldUp(axis) => {
                let y_len = self.dots.len();
                for i in 0..(y_len - axis - 1) {
                    for j in 0..self.dots[0].len() {
                        self.dots[i][j] |= self.dots[y_len - 1 - i][j];
                    }
                }
                for _ in 0..(axis + 1) {
                    self.dots.pop();
                }
            }
            Instruction::FoldLeft(axis) => {
                let x_len = self.dots[0].len();
                for i in 0..self.dots.len() {
                    for j in 0..(x_len - axis - 1) {
                        self.dots[i][j] |= self.dots[i][x_len - 1 - j];
                    }
                }
                for row in &mut self.dots {
                    for _ in 0..(axis + 1) {
                        row.pop();
                    }
                }
            }
        }
    }

    fn get_dots_count(&self) -> i32 {
        let mut result = 0;
        for row in &self.dots {
            for dot in row {
                if *dot {
                    result += 1;
                }
            }
        }
        result
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for row in &self.dots {
            for dot in row {
                if *dot {
                    write!(f, "⯃")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut args = env::args();

    // call name
    args.next();
    let data = fs::read_to_string(args.next().expect("Provide filename to use!"))
        .expect("Unable to read file");

    let mut lines = data.split("\n\n").filter(|&x| !x.is_empty());
    let dots = lines
        .next()
        .unwrap()
        .to_string()
        .split("\n")
        .map(str::to_string)
        .collect::<Vec<String>>();
    let instructions = lines
        .next()
        .unwrap()
        .to_string()
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(str::to_string)
        .collect::<Vec<String>>();

    let mut paper = Paper::new(dots);

    let instructions = instructions
        .iter()
        .map(create_instruction)
        .collect::<Vec<Instruction>>();

    for (i, instr) in instructions.iter().enumerate() {
        paper.run_instruction(instr);
        if i == 0 {
            println!("day13 part1 ans= {}", paper.get_dots_count());
        }
    }

    println!("day13 part2 ans= ");
    println!("{}", paper);
}
