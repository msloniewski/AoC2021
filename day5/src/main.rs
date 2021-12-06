use std::cmp;
use std::env;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Line {
    point_a: Point,
    point_b: Point,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Plane {
    plane: Vec<Vec<i32>>,
}

impl Plane {
    fn new(size_x: usize, size_y: usize) -> Self {
        Self {
            plane: vec![vec![0; size_y]; size_x],
        }
    }

    fn add_line(&mut self, line: &Line) {
        if line.is_perpendicular() {
            if line.point_a.x == line.point_b.x {
                let start = cmp::min(line.point_a.y, line.point_b.y);
                let end = cmp::max(line.point_a.y, line.point_b.y);
                for i in start..end + 1 {
                    self.plane[line.point_a.x as usize][i as usize] += 1;
                }
            } else {
                let start = cmp::min(line.point_a.x, line.point_b.x);
                let end = cmp::max(line.point_a.x, line.point_b.x);
                for i in start..end + 1 {
                    self.plane[i as usize][line.point_a.y as usize] += 1;
                }
            }
        } else {
            // assuming lines are always at 45 degrees
            let start = cmp::min(line.point_a.x, line.point_b.x);
            let end = cmp::max(line.point_a.x, line.point_b.x);
            let sign = if (line.point_b.y - line.point_a.y) > 0 {
                1
            } else {
                -1
            } * if (line.point_b.x - line.point_a.x) > 0 {
                1
            } else {
                -1
            };

            let mut start_y = if sign > 0 {
                cmp::min(line.point_b.y, line.point_a.y)
            } else {
                cmp::max(line.point_b.y, line.point_a.y)
            };
            for i in start..end + 1 {
                self.plane[i as usize][start_y as usize] += 1;
                start_y += sign;
            }
        }
    }

    fn get_number_of_crossings(&self) -> i32 {
        self.plane.iter().fold(0, |sum, row| {
            sum + row
                .iter()
                .fold(0, |sum, x| sum + if *x > 1 { 1 } else { 0 })
        })
    }
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for row in &self.plane {
            for cell in row {
                write!(f, "{}\t", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Line {
    fn is_perpendicular(&self) -> bool {
        self.point_a.x == self.point_b.x || self.point_a.y == self.point_b.y
    }
}

fn main() {
    let mut args = env::args();

    // call name
    args.next();
    let data = fs::read_to_string(args.next().expect("Provide filename to use!"))
        .expect("Unable to read file");

    let file_lines = data
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(str::to_string)
        .collect::<Vec<String>>();

    let mut lines: Vec<Line> = Vec::new();

    let mut max_y = 0;
    let mut max_x = 0;

    for line in file_lines {
        let mut point_str = line.split("->");

        let mut point_a = Point { x: 0, y: 0 };
        let mut point_b = Point { x: 0, y: 0 };
        let mut coords_a = point_str.next().unwrap().split(",");
        let mut coords_b = point_str.next().unwrap().split(",");

        point_a.y = coords_a.next().unwrap().trim().parse().unwrap();
        point_a.x = coords_a.next().unwrap().trim().parse().unwrap();

        point_b.y = coords_b.next().unwrap().trim().parse().unwrap();
        point_b.x = coords_b.next().unwrap().trim().parse().unwrap();

        max_x = cmp::max(max_x, cmp::max(point_a.x, point_b.x));
        max_y = cmp::max(max_y, cmp::max(point_a.y, point_b.y));

        lines.push(Line {
            point_a: point_a,
            point_b: point_b,
        });
    }

    let mut plane = Plane::new((max_x + 1) as usize, (max_y + 1) as usize);

    for line in &lines {
        if line.is_perpendicular() {
            plane.add_line(&line);
        }
    }

    println!("day5 part1 ans= {}", plane.get_number_of_crossings());

    let mut plane = Plane::new((max_x + 1) as usize, (max_y + 1) as usize);
    for line in lines {
        plane.add_line(&line);
    }

    println!("day5 part1 ans= {}", plane.get_number_of_crossings());
}
