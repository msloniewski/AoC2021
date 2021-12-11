use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
enum TrueSevenSegment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

fn create_mapping_from_letters_to_segments(
    examples: &Vec<HashSet<char>>,
) -> HashMap<TrueSevenSegment, char> {
    let mut mapping: HashMap<TrueSevenSegment, char> = HashMap::new();

    let seven = examples.iter().find(|x| x.len() == 3).unwrap();
    let one = examples.iter().find(|x| x.len() == 2).unwrap();
    let four = examples.iter().find(|x| x.len() == 4).unwrap();
    let eight = examples.iter().find(|x| x.len() == 7).unwrap();
    let zero_nine_six: Vec<HashSet<char>> =
        examples.iter().filter(|x| x.len() == 6).cloned().collect();
    let two_three_five: Vec<HashSet<char>> =
        examples.iter().filter(|x| x.len() == 5).cloned().collect();
    let two_three_five_anded = two_three_five[0]
        .intersection(&two_three_five[1])
        .cloned()
        .collect::<HashSet<char>>()
        .intersection(&two_three_five[2])
        .cloned()
        .collect::<HashSet<char>>();
    let zero_nine_six_anded = zero_nine_six[0]
        .intersection(&zero_nine_six[1])
        .cloned()
        .collect::<HashSet<char>>()
        .intersection(&zero_nine_six[2])
        .cloned()
        .collect::<HashSet<char>>();

    let a = seven.difference(one).next().unwrap();
    mapping.insert(TrueSevenSegment::A, a.clone());

    let d: char = two_three_five_anded
        .intersection(four)
        .cloned()
        .next()
        .unwrap();
    mapping.insert(TrueSevenSegment::D, d.clone());

    let mut b = four.clone();
    b.remove(&d);
    let b = b.difference(one).cloned().next().unwrap();
    mapping.insert(TrueSevenSegment::B, b.clone());

    let mut g = zero_nine_six_anded
        .difference(one)
        .cloned()
        .collect::<HashSet<char>>();
    g.remove(&d);
    g.remove(&a);
    g.remove(&b);
    assert!(g.len() == 1);
    let g = g.iter().next().unwrap();
    mapping.insert(TrueSevenSegment::G, g.clone());

    let mut f = zero_nine_six_anded
        .iter()
        .cloned()
        .collect::<HashSet<char>>();
    f.remove(&a);
    f.remove(&b);
    f.remove(&g);
    assert!(f.len() == 1);
    let f = f.iter().next().unwrap();
    mapping.insert(TrueSevenSegment::F, f.clone());

    let mut c = one.iter().cloned().collect::<HashSet<char>>();
    c.remove(&f);
    assert!(c.len() == 1);
    let c = c.iter().next().unwrap();
    mapping.insert(TrueSevenSegment::C, c.clone());

    let mut e = eight.iter().cloned().collect::<HashSet<char>>();
    e.remove(&a);
    e.remove(&b);
    e.remove(&c);
    e.remove(&d);
    e.remove(&f);
    e.remove(&g);
    assert!(e.len() == 1);
    let e = e.iter().next().unwrap();

    mapping.insert(TrueSevenSegment::E, e.clone());

    //println!("g: {:?}", mapping);

    mapping
}

fn convert_mixed_segment_to_number(
    mixed: &HashSet<char>,
    mapping: &HashMap<TrueSevenSegment, char>,
) -> i64 {
    match mixed.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
            if mixed.contains(mapping.get(&TrueSevenSegment::E).unwrap()) {
                return 2;
            } else {
                if mixed.contains(mapping.get(&TrueSevenSegment::C).unwrap()) {
                    return 3;
                } else {
                    return 5;
                }
            }
        }
        6 => {
            if mixed.contains(mapping.get(&TrueSevenSegment::D).unwrap()) {
                if mixed.contains(mapping.get(&TrueSevenSegment::E).unwrap()) {
                    return 6;
                } else {
                    return 9;
                }
            } else {
                return 0;
            }
        }
        7 => 8,
        _ => panic!("Invalid number in question!"),
    }
}

fn convert_mixed_display_to_number(
    display: &Vec<HashSet<char>>,
    mapping: &HashMap<TrueSevenSegment, char>,
) -> i64 {
    let mut result: i64 = 0;
    let mut multiplier = 1;

    for digit in display.iter().rev() {
        result += convert_mixed_segment_to_number(digit, mapping) * multiplier;
        multiplier *= 10;
    }

    result
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

    let mut examples: Vec<Vec<HashSet<char>>> = Vec::new();
    let mut questions: Vec<Vec<HashSet<char>>> = Vec::new();

    for line in file_lines {
        let mut split_line = line.split("|");
        let example_strings = split_line.next().unwrap();

        let question_strings = split_line.next().unwrap();

        let examples_vec: Vec<String> = example_strings
            .split_whitespace()
            .filter(|&x| !x.is_empty())
            .map(str::to_string)
            .collect::<Vec<String>>();

        let questions_vec: Vec<String> = question_strings
            .split_whitespace()
            .filter(|&x| !x.is_empty())
            .map(str::to_string)
            .collect::<Vec<String>>();

        let mut questions_vec_map: Vec<HashSet<char>> = Vec::new();
        let mut examples_vec_map: Vec<HashSet<char>> = Vec::new();

        for question in questions_vec {
            let mut display: HashSet<char> = HashSet::new();
            for letter in question.chars() {
                display.insert(letter);
            }
            questions_vec_map.push(display);
        }
        questions.push(questions_vec_map);

        for example in examples_vec {
            let mut display: HashSet<char> = HashSet::new();
            for letter in example.chars() {
                display.insert(letter);
            }
            examples_vec_map.push(display);
        }
        examples.push(examples_vec_map);
    }

    let ans = questions.iter().fold(0, |sum, x| {
        sum + x.iter().fold(0, |sum, y| {
            sum + if y.len() == 2 || y.len() == 4 || y.len() == 3 || y.len() == 7 {
                1
            } else {
                0
            }
        })
    });

    println!("day8 part1 ans= {}", ans);

    let mut ans: i64 = 0;
    for (example, question) in examples.iter().zip(questions.iter()) {
        let mapping = create_mapping_from_letters_to_segments(example);
        ans += convert_mixed_display_to_number(question, &mapping);
    }

    println!("day8 part2 ans= {}", ans);
}
