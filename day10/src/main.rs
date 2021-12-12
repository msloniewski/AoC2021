use std::env;
use std::fs;

fn is_closing(bracket: char) -> bool {
    match bracket {
        '(' => return false,
        '[' => return false,
        '{' => return false,
        '<' => return false,
        _ => return true,
    }
}

fn is_closing_pair(bracket_curr: char, bracket_prev: char) -> bool {
    match bracket_curr {
        ')' => return bracket_prev == '(',
        ']' => return bracket_prev == '[',
        '}' => return bracket_prev == '{',
        '>' => return bracket_prev == '<',
        _ => panic!("not closing bracket!"),
    }
}

fn get_closing_pair(bracket: char) -> char {
    match bracket {
        '(' => return ')',
        '[' => return ']',
        '{' => return '}',
        '<' => return '>',
        _ => panic!("not closing bracket!"),
    }
}

fn get_score_checker(invalid_bracket: char) -> i64 {
    match invalid_bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("no score for this bracket!"),
    }
}

fn get_score_autocomplete(invalid_bracket: char) -> i64 {
    match invalid_bracket {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("no score for this bracket!"),
    }
}

fn get_line_score_checker(line: &String) -> i64 {
    let mut visited_brackets: Vec<char> = Vec::new();

    for bracket in line.chars() {
        if is_closing(bracket) {
            let bracket_prev = visited_brackets.pop().unwrap();
            if !is_closing_pair(bracket, bracket_prev) {
                return get_score_checker(bracket);
            }
        } else {
            visited_brackets.push(bracket);
        }
    }
    return 0;
}

fn get_lines_score_checker(lines: &Vec<String>) -> i64 {
    let mut result: i64 = 0;
    for line in lines {
        result += get_line_score_checker(line);
    }
    result
}

fn get_line_score_autocomplete(line: &String) -> i64 {
    let mut visited_brackets: Vec<char> = Vec::new();

    for bracket in line.chars() {
        if is_closing(bracket) {
            let bracket_prev = visited_brackets.pop().unwrap();
            if !is_closing_pair(bracket, bracket_prev) {
                return 0;
            }
        } else {
            visited_brackets.push(bracket);
        }
    }
    let mut score: i64 = 0;
    for remaining_bracket in visited_brackets.iter().rev() {
        score *= 5;
        score += get_score_autocomplete(get_closing_pair(*remaining_bracket));
    }
    score
}

fn get_lines_score_autocomplete(lines: &Vec<String>) -> i64 {
    let mut result: Vec<i64> = Vec::new();
    for line in lines {
        result.push(get_line_score_autocomplete(line));
    }

    result.retain(|&x| x > 0);
    result.sort();

    result[result.len() / 2]
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

    println!("day10 part1 ans= {}", get_lines_score_checker(&lines));
    println!("day10 part1 ans= {}", get_lines_score_autocomplete(&lines));
}
