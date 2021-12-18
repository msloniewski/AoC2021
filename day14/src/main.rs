use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct Polymer {
    template: HashMap<char, HashMap<char, char>>,
    score: HashMap<char, i32>,
    polymer: Vec<char>,
}

impl Polymer {
    fn new(polymer_init: String, templates: Vec<String>) -> Self {
        let mut polymer: Vec<char> = Vec::new();
        let mut template: HashMap<char, HashMap<char, char>> = HashMap::new();
        let mut score: HashMap<char, i32> = HashMap::new();

        for letter in polymer_init.chars() {
            polymer.push(letter);
        }

        for template_str in templates {
            let mut template_split = template_str.split(" -> ");
            let substrates = template_split.next().unwrap().to_string();
            let product = template_split
                .next()
                .unwrap()
                .to_string()
                .parse::<char>()
                .unwrap();

            let substrates = substrates.chars().collect::<Vec<char>>();

            if template.contains_key(&substrates[0]) {
                template
                    .get_mut(&substrates[0])
                    .unwrap()
                    .insert(substrates[1], product);
            } else {
                template.insert(substrates[0], HashMap::from([(substrates[1], product)]));
            }

            score.insert(substrates[0], 0);
            score.insert(substrates[1], 0);
            score.insert(product, 0);
        }

        Self {
            template: template,
            polymer: polymer,
            score: score,
        }
    }

    fn step_and_add_score(&mut self, sub1: &char, sub2: &char, iterations: i32) {
    if iterations > 20 {
    	println!("{} {} {}", sub1, sub2,iterations);
	}
        if iterations == 0 {
            return;
        }
        let new_element: char = self.template[sub1][sub2].clone();

        self.score.insert(new_element, self.score[&new_element] + 1);

        self.step_and_add_score(sub1, &new_element, iterations - 1);
        self.step_and_add_score(&new_element, sub2, iterations - 1);
    }

    fn run_iters(&mut self, interations: i32) {
        for el in &self.polymer {
            self.score.insert(*el, self.score[el] + 1);
        }
        for i in 0..self.polymer.len() - 1 {
            let sub1 = self.polymer[i].clone();
            let sub2 = self.polymer[i + 1].clone();
            self.step_and_add_score(&sub1, &sub2, interations);
        }
    }

    fn restart(&mut self) {
        for (_, val) in self.score.iter_mut() {
            *val = 0;
        }
    }

    fn get_answer(&self) -> i32 {
        let top = self.score.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let bottom = self.score.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
        top.1 - bottom.1
    }
}

fn main() {
    let mut args = env::args();

    // call name
    args.next();
    let data = fs::read_to_string(args.next().expect("Provide filename to use!"))
        .expect("Unable to read file");

    let mut lines = data.split("\n\n").filter(|&x| !x.is_empty());
    let polymer = lines.next().unwrap().trim().to_string();
    let templates = lines
        .next()
        .unwrap()
        .to_string()
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(str::to_string)
        .collect::<Vec<String>>();

    let mut polymer = Polymer::new(polymer, templates);
    polymer.run_iters(10);
    println!("day14 part1 ans= {}", polymer.get_answer());
    polymer.restart();
    polymer.run_iters(40);
    println!("day14 part2 ans= {}", polymer.get_answer());
}
