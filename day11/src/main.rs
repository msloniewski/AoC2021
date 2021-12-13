use std::env;
use std::fs;

#[derive(Debug)]
struct Octopus {
    energy: i32,
    fired_this_turn: bool,
}

#[derive(Debug)]
struct GridModel {
    grid: Vec<Vec<Octopus>>,
    clock: i32,
}

impl GridModel {
    fn new(grid_init: Vec<Vec<i32>>) -> Self {
        let mut oct_grid: Vec<Vec<Octopus>> = Vec::new();

        for row in grid_init {
            let mut oct_row: Vec<Octopus> = Vec::new();
            for cell in row {
                oct_row.push(Octopus {
                    energy: cell,
                    fired_this_turn: false,
                });
            }
            oct_grid.push(oct_row);
        }

        Self {
            grid: oct_grid,
            clock: 0,
        }
    }

    fn is_within_grid(&self, i: i32, j: i32) -> bool {
        if (i < 0) | (j < 0) {
            return false;
        }
        if (j >= self.grid[0].len() as i32) | (i >= self.grid.len() as i32) {
            return false;
        }

        return true;
    }

    fn add_and_fire(&mut self, i: i32, j: i32) {
        if !self.is_within_grid(i, j) {
            return;
        }

        if self.grid[i as usize][j as usize].fired_this_turn {
            return;
        }

        self.grid[i as usize][j as usize].energy += 1;

        if self.grid[i as usize][j as usize].energy > 9 {
            self.grid[i as usize][j as usize].fired_this_turn = true;

            self.add_and_fire(i - 1, j + 1);
            self.add_and_fire(i - 1, j);
            self.add_and_fire(i - 1, j - 1);
            self.add_and_fire(i, j - 1);
            self.add_and_fire(i, j + 1);
            self.add_and_fire(i + 1, j - 1);
            self.add_and_fire(i + 1, j);
            self.add_and_fire(i + 1, j + 1);
        }
    }

    fn finish_round(&mut self) -> i32 {
        let mut result = 0;

        for i in 0..(self.grid.len() as i32) {
            for j in 0..(self.grid[0].len() as i32) {
                if self.grid[i as usize][j as usize].fired_this_turn {
                    self.grid[i as usize][j as usize].energy = 0;
                    self.grid[i as usize][j as usize].fired_this_turn = false;
                    result += 1;
                }
            }
        }
        self.clock += 1;
        result
    }

    fn advance_one_clock(&mut self) -> i32 {
        for i in 0..(self.grid.len() as i32) {
            for j in 0..(self.grid[0].len() as i32) {
                self.add_and_fire(i, j);
            }
        }
        self.finish_round()
    }

    fn get_flashes_after(&mut self, intervals: i32) -> i32 {
        let mut result = 0;

        for _ in 0..intervals {
            result += self.advance_one_clock();
        }

        result
    }
    fn run_until_sync(&mut self) {
        let target = (self.grid.len() * self.grid[0].len()) as i32;
        loop {
            if self.advance_one_clock() == target {
                break;
            }
        }
    }
}

fn main() {
    let mut args = env::args();

    // call name
    args.next();
    let data = fs::read_to_string(args.next().expect("Provide filename to use!"))
        .expect("Unable to read file");

    let mut grid_init: Vec<Vec<i32>> = Vec::new();

    let lines = data
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(str::to_string)
        .collect::<Vec<String>>();

    for line in lines {
        let row: Vec<i32> = line
            .chars()
            .map(|x| x.to_string().parse().unwrap())
            .collect();
        grid_init.push(row);
    }

    let mut model = GridModel::new(grid_init);

    println!("day11 part1 ans= {}", model.get_flashes_after(100));

    model.run_until_sync();
    println!("day11 part2 ans= {}", model.clock);
}
