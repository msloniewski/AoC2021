use std::cmp;
use std::env;
use std::fs;

#[derive(Debug)]
struct CrabModel {
    crabs: Vec<i32>,
}

impl CrabModel {
    fn new(crab_init: Vec<i32>) -> Self {
        Self { crabs: crab_init }
    }

    fn get_fuel_consumption(&self, align_point: i32) -> i32 {
        let mut consumption = 0;
        for crab in &self.crabs {
            consumption += (*crab - align_point).abs();
        }
        consumption
    }

    fn get_fuel_consumption2(&self, align_point: i32) -> i32 {
        let mut consumption = 0;
        for crab in &self.crabs {
            let distance = (*crab - align_point).abs();
            consumption += distance * (distance + 1) / 2;
        }
        consumption
    }

    fn get_optimal_fuel_consumption2(&self) -> i32 {
        let avg = self.crabs.iter().sum::<i32>() / self.crabs.len() as i32;

        cmp::min(
            self.get_fuel_consumption2(avg),
            self.get_fuel_consumption2(avg + 1),
        )
    }

    fn get_optimal_fuel_consumption(&self) -> i32 {
        if self.crabs.len() % 2 == 1 {
            let mut sorted_vec = self.crabs.clone();
            sorted_vec.sort();
            let median = sorted_vec[sorted_vec.len() / 2];
            self.get_fuel_consumption(median)
        } else {
            let mut sorted_vec = self.crabs.clone();
            sorted_vec.sort();
            let median_1 = sorted_vec[sorted_vec.len() / 2];
            let median_2 = sorted_vec[sorted_vec.len() / 2 + 1];

            cmp::min(
                self.get_fuel_consumption(median_1),
                self.get_fuel_consumption(median_2),
            )
        }
    }
}

fn main() {
    let mut args = env::args();

    // call name
    args.next();
    let data = fs::read_to_string(args.next().expect("Provide filename to use!"))
        .expect("Unable to read file");

    let crab_init: Vec<i32> = data.split(",").map(|x| x.trim().parse().unwrap()).collect();

    let model = CrabModel::new(crab_init);

    println!("day7 part1 ans= {}", model.get_optimal_fuel_consumption());
    println!("day7 part2 ans= {}", model.get_optimal_fuel_consumption2());
}
