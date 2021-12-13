use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
struct MapModel {
    map: Vec<Vec<i32>>,
}

impl MapModel {
    fn new(map_init: Vec<Vec<i32>>) -> Self {
        Self { map: map_init }
    }

    fn is_lower(&self, val: i32, x: i32, y: i32) -> bool {
        if (x < 0) | (y < 0) {
            return true;
        }
        if (x >= self.map[0].len() as i32) | (y >= self.map.len() as i32) {
            return true;
        }

        return val < self.map[y as usize][x as usize] as i32;
    }

    fn is_part_of_basin(&self, x: i32, y: i32) -> bool {
        if (x < 0) | (y < 0) {
            return false;
        }
        if (x >= self.map[0].len() as i32) | (y >= self.map.len() as i32) {
            return false;
        }

        return self.map[y as usize][x as usize] < 9;
    }

    fn get_risk_level(&self, x: i32, y: i32) -> i32 {
        let val = self.map[y as usize][x as usize];
        if self.is_lower(val, x - 1, y)
            & self.is_lower(val, x, y - 1)
            & self.is_lower(val, x + 1, y)
            & self.is_lower(val, x, y + 1)
        {
            return val + 1;
        } else {
            return 0;
        }
    }

    fn get_all_risk_levels(&self) -> i32 {
        let mut result = 0;
        for i in 0..(self.map.len() as i32) {
            for j in 0..(self.map[0].len() as i32) {
                result += self.get_risk_level(j, i);
            }
        }

        result
    }

    fn visit_point(&self, point: (i32, i32)) -> HashSet<(i32, i32)> {
        let (x, y) = point;
        let mut points_visited: HashSet<(i32, i32)> = HashSet::new();

        if self.is_part_of_basin(x - 1, y) {
            points_visited.insert((x - 1, y));
        }
        if self.is_part_of_basin(x, y - 1) {
            points_visited.insert((x, y - 1));
        }
        if self.is_part_of_basin(x + 1, y) {
            points_visited.insert((x + 1, y));
        }
        if self.is_part_of_basin(x, y + 1) {
            points_visited.insert((x, y + 1));
        }
        points_visited
    }

    fn get_basin_size(&self, x: i32, y: i32) -> i32 {
        let mut points_visited: HashSet<(i32, i32)> = HashSet::new();
        let mut points_to_visit: HashSet<(i32, i32)> = HashSet::new();
        points_to_visit.insert((x, y));

        while points_to_visit.len() != 0 {
            let mut new_points_to_visit: HashSet<(i32, i32)> = HashSet::new();

            for point_to_visit in points_to_visit.drain() {
                let new_points = self.visit_point(point_to_visit);
                points_visited.insert(point_to_visit.clone());
                for point in new_points.difference(&points_visited) {
                    new_points_to_visit.insert(point.clone());
                }
            }
            points_to_visit = new_points_to_visit;
        }

        points_visited.len() as i32
    }

    fn get_basins_score(&self) -> i32 {
        let mut results: Vec<i32> = Vec::new();
        for i in 0..(self.map.len() as i32) {
            for j in 0..(self.map[0].len() as i32) {
                if self.get_risk_level(j, i) > 0 {
                    results.push(self.get_basin_size(j, i));
                }
            }
        }

        results.sort();

        let mut result = 1;

        for (i, size) in results.iter().rev().enumerate() {
            if i == 3 {
                break;
            }
            result *= size;
        }
        result
    }
}

fn main() {
    let mut args = env::args();

    // call name
    args.next();
    let data = fs::read_to_string(args.next().expect("Provide filename to use!"))
        .expect("Unable to read file");

    let mut map_init: Vec<Vec<i32>> = Vec::new();

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
        map_init.push(row);
    }

    let model = MapModel::new(map_init);

    println!("day9 part1 ans= {}", model.get_all_risk_levels());
    println!("day9 part2 ans= {}", model.get_basins_score());
}
