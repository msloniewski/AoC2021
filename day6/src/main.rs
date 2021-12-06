use std::env;
use std::fs;

#[derive(Debug)]
struct LanternFishModel {
    state: Vec<u64>,
}

const FISH_RESET: usize = 6;
const NEW_FISH_RESET: usize = 8;

impl LanternFishModel {
    fn new(fish_init: &Vec<i32>) -> Self {
        let mut state: Vec<u64> = vec![0; (NEW_FISH_RESET + 1) as usize];

        for fish in fish_init {
            assert!((*fish > 0) & (*fish < NEW_FISH_RESET as i32));
            state[*fish as usize] += 1;
        }

        Self { state: state }
    }

    fn spin_day(&mut self) {
        let mut new_state: Vec<u64> = vec![0; self.state.len()];
        for (i, fish) in self.state.iter().enumerate() {
            if i == 0 {
                new_state[NEW_FISH_RESET] = *fish;
                new_state[FISH_RESET] += *fish;
            } else {
                new_state[(i - 1) as usize] += *fish;
            }
        }
        self.state = new_state;
    }

    fn get_number_of_fish(&self) -> u64 {
        self.state.iter().fold(0, |sum, x| sum + x)
    }
}

fn main() {
    let mut args = env::args();

    // call name
    args.next();
    let data = fs::read_to_string(args.next().expect("Provide filename to use!"))
        .expect("Unable to read file");

    let fish_init: Vec<i32> = data.split(",").map(|x| x.trim().parse().unwrap()).collect();

    let mut model = LanternFishModel::new(&fish_init);

    for _i in 0..80 {
        model.spin_day();
    }

    println!("day3 part1 ans= {}", model.get_number_of_fish());

    let mut model = LanternFishModel::new(&fish_init);
    for _i in 0..256 {
        model.spin_day();
    }

    println!("day3 part2 ans= {}", model.get_number_of_fish());
}
