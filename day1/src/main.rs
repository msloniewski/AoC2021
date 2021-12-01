use std::env;
use std::fs;

fn main() {
    let mut args = env::args();

    // call name
    args.next();
    let data = fs::read_to_string(args.next().expect("Provide filename to use!"))
        .expect("Unable to read file");

    let depth_measurements: Vec<i32> = data
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    assert!(depth_measurements.len() >= 2);

    let mut num_increases = 0;
    let mut prev_meas = depth_measurements[0];

    for curr_meas in &depth_measurements[1..] {
        if curr_meas - prev_meas > 0 {
            num_increases += 1;
        }
        prev_meas = *curr_meas;
    }

    println!("part 1 ans={}", num_increases);

    // for second part we need only to compare first meas of the prev window
    // and the last of the next window -- meas in between sum up to the same number
    let window_size = 3;
    let mut num_increases_window = 0;

    for i in 0..depth_measurements.len() - window_size {
        if depth_measurements[i + window_size] - depth_measurements[i] > 0 {
            num_increases_window += 1;
        }
    }

    println!("part 2 ans={}", num_increases_window);
}
