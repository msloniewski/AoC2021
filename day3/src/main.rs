use std::env;
use std::fs;

#[derive(Debug)]
struct DiagnosticReport {
    gamma: i32,
    epsilon: i32,
    co2: i32,
    oxy: i32,
}

impl DiagnosticReport {
    fn new(lines: Vec<String>) -> Self {
        let numbers: Vec<String> = lines;

        let word_size = numbers[0].len();

        let mut sum_of_bits: Vec<i32> = vec![0; word_size];

        for number in &numbers {
            for (i, bit) in number.chars().rev().enumerate() {
                if bit == '1' {
                    sum_of_bits[i] += 1;
                }
            }
        }

        let all_numbers = numbers.len() as i32;

        let gamma_bits: Vec<i32> = sum_of_bits
            .iter()
            .map(|&x| if x > all_numbers / 2 { 1 } else { 0 })
            .collect();

        let gamma: i32 = gamma_bits.iter().enumerate().map(|(i, &x)| x << i).sum();

        let epsilon = (!gamma) & ((1 << word_size) - 1);

        // second part

        let mut oxy_numbers = numbers.clone();
        let mut co2_numbers = numbers.clone();

        for i in 0..word_size {
            let ones_num = oxy_numbers.iter().fold(0, |sum, x| {
                sum + if x.as_bytes()[i] == '1' as u8 { 1 } else { 0 }
            });

            if ones_num * 2 < oxy_numbers.len() {
                oxy_numbers.retain(|x| x.as_bytes()[i] == '0' as u8);
            } else {
                oxy_numbers.retain(|x| x.as_bytes()[i] == '1' as u8);
            }

            if oxy_numbers.len() == 1 {
                break;
            }
        }
        for i in 0..word_size {
            let ones_num = co2_numbers.iter().fold(0, |sum, x| {
                sum + if x.as_bytes()[i] == '1' as u8 { 1 } else { 0 }
            });

            if ones_num * 2 >= co2_numbers.len() {
                co2_numbers.retain(|x| x.as_bytes()[i] == '0' as u8);
            } else {
                co2_numbers.retain(|x| x.as_bytes()[i] == '1' as u8);
            }
            if co2_numbers.len() == 1 {
                break;
            }
        }

        let oxy: i32 = oxy_numbers[0]
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &x)| if '1' as u8 == x { 1 } else { 0 } << i)
            .sum();
        let co2: i32 = co2_numbers[0]
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &x)| if '1' as u8 == x { 1 } else { 0 } << i)
            .sum();

        Self {
            gamma: gamma,
            epsilon: epsilon,
            co2: co2,
            oxy: oxy,
        }
    }
}

fn main() {
    let mut args = env::args();

    // call name
    args.next();
    let data = fs::read_to_string(args.next().expect("Provide filename to use!"))
        .expect("Unable to read file");

    let lines = data
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(str::to_string)
        .collect();

    let diagnostic_rep = DiagnosticReport::new(lines);

    println!(
        "day3 part1 ans= {}",
        diagnostic_rep.gamma * diagnostic_rep.epsilon
    );
    println!(
        "day3 part2 ans= {}",
        diagnostic_rep.oxy * diagnostic_rep.co2
    );
}
