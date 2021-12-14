use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn is_small_cave(cave: &String) -> bool {
    for letter in cave.chars() {
        if letter.is_uppercase() {
            return false;
        }
    }
    true
}

fn number_of_this_caves(route: &Vec<String>, cave: &String) -> i32 {
    let mut result = 0;
    for el in route {
        if el == cave {
            result += 1;
        }
    }
    result
}

#[derive(Debug)]
struct Graph {
    model: HashMap<String, Vec<String>>,
    small_caves: Vec<String>,
}

impl Graph {
    fn new(connections: Vec<String>) -> Self {
        let mut model: HashMap<String, Vec<String>> = HashMap::new();
        let mut small_caves: Vec<String> = Vec::new();

        for conn in &connections {
            let nodes = conn.split("-");

            for node in nodes {
                let node = node.trim().to_string();
                if is_small_cave(&node) & (node != "start".to_string()) {
                    if !small_caves.contains(&node) {
                        small_caves.push(node.clone());
                    }
                }
                model.insert(node, Vec::new());
            }
        }

        for conn in &connections {
            let mut nodes = conn.split("-");
            let left = nodes.next().unwrap().trim().to_string();
            let right = nodes.next().unwrap().trim().to_string();

            model.get_mut(&left).unwrap().push(right.clone());
            model.get_mut(&right).unwrap().push(left);
        }

        Self {
            model: model,
            small_caves: small_caves,
        }
    }

    fn step_in_route_or_finish(&self, route: Vec<String>) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
        let mut unfinished_routes: Vec<Vec<String>> = Vec::new();
        let mut finished_routes: Vec<Vec<String>> = Vec::new();

        let options = self.model[route.last().unwrap()].clone();

        for option in options {
            let mut new_route = route.clone();

            if is_small_cave(&option) {
                if route.contains(&option) {
                    continue;
                }

                new_route.push(option.clone());
                if option == "end".to_string() {
                    finished_routes.push(new_route);
                } else {
                    unfinished_routes.push(new_route);
                }
            } else {
                new_route.push(option.clone());
                unfinished_routes.push(new_route)
            }
        }

        (unfinished_routes, finished_routes)
    }

    fn step_in_route_or_finish_v2(
        &self,
        route: Vec<String>,
        double_small_cave: &String,
    ) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
        let mut unfinished_routes: Vec<Vec<String>> = Vec::new();
        let mut finished_routes: Vec<Vec<String>> = Vec::new();

        let options = self.model[route.last().unwrap()].clone();

        for option in options {
            let mut new_route = route.clone();

            if is_small_cave(&option) {
                if route.contains(&option) {
                    if *double_small_cave == option {
                        if number_of_this_caves(&route, double_small_cave) == 2 {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                new_route.push(option.clone());
                if option == "end".to_string() {
                    finished_routes.push(new_route);
                } else {
                    unfinished_routes.push(new_route);
                }
            } else {
                new_route.push(option.clone());
                unfinished_routes.push(new_route)
            }
        }

        (unfinished_routes, finished_routes)
    }

    fn get_number_of_possible_routes(&self) -> i32 {
        let beggining: Vec<String> = vec!["start".to_string()];

        let mut unfinished_routes: Vec<Vec<String>> = vec![beggining];
        let mut result = 0;

        while unfinished_routes.len() > 0 {
            let (new_unfinished, new_finished) =
                self.step_in_route_or_finish(unfinished_routes.pop().unwrap());
            result += new_finished.len() as i32;
            for new in new_unfinished {
                unfinished_routes.push(new);
            }
        }
        result
    }

    fn get_number_of_possible_routes_v2(&self) -> i32 {
        let mut finished_routes: HashSet<Vec<String>> = HashSet::new();
        for double_small_cave in &self.small_caves {
            let beggining: Vec<String> = vec!["start".to_string()];

            let mut unfinished_routes: Vec<Vec<String>> = vec![beggining];

            while unfinished_routes.len() > 0 {
                let (new_unfinished, new_finished) = self.step_in_route_or_finish_v2(
                    unfinished_routes.pop().unwrap(),
                    double_small_cave,
                );
                for new in new_unfinished {
                    unfinished_routes.push(new);
                }
                for new in new_finished {
                    finished_routes.insert(new);
                }
            }
        }

        finished_routes.len() as i32
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
        .collect::<Vec<String>>();

    let graph = Graph::new(lines);

    println!("day12 part1 ans= {}", graph.get_number_of_possible_routes());
    println!(
        "day12 part2 ans= {}",
        graph.get_number_of_possible_routes_v2()
    );
}
