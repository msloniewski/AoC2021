use std::env;
use std::fs;

#[derive(Debug)]
struct BingoField {
    number: i32,
    marked: bool,
}

#[derive(Debug)]
struct BingoSheet {
    winning: bool,
    sheet: Vec<Vec<BingoField>>,
    size: usize,
    winning_num: i32,
}

impl BingoSheet {
    fn new(numbers: Vec<Vec<i32>>) -> Self {
        assert!(numbers.len() != 0);
        let size = numbers[0].len();

        let mut sheet: Vec<Vec<BingoField>> = Vec::new();
        for row in &numbers {
            assert!(row.len() == size);

            let mut field_row: Vec<BingoField> = Vec::new();
            for number in row {
                field_row.push(BingoField {
                    number: *number,
                    marked: false,
                });
            }
            sheet.push(field_row);
        }

        Self {
            sheet: sheet,
            winning: false,
            size: size,
            winning_num: 0,
        }
    }
    fn mark_number(&mut self, number: i32) {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.sheet[i][j].number == number {
                    self.sheet[i][j].marked = true;
                    self.check_winning(i, j, number);
                }
            }
        }
    }

    fn check_winning(&mut self, i: usize, j: usize, number: i32) {
        if self.winning {
            return;
        }

        let mut winning = true;

        for row in 0..self.size {
            if !self.sheet[row][j].marked {
                winning = false;
                break;
            }
        }
        if winning {
            self.winning = true;
            self.winning_num = number;
            return;
        }

        winning = true;
        for column in 0..self.size {
            if !self.sheet[i][column].marked {
                winning = false;
                break;
            }
        }
        if winning {
            self.winning = true;
            self.winning_num = number;
            return;
        }
    }

    fn get_score(&self) -> i32 {
        let score = self.sheet.iter().fold(0, |sum, row| {
            sum + row
                .iter()
                .fold(0, |sum, x| sum + if !x.marked { x.number } else { 0 })
        });

        return score * self.winning_num;
    }

    fn reset(&mut self) {
        for row in &mut self.sheet {
            for el in row {
                el.marked = false;
            }
        }
        self.winning = false;
        self.winning_num = 0;
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
        .map(str::to_string)
        .collect::<Vec<String>>();

    let mut lines = lines.iter();

    let guesses: Vec<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    lines.next(); //empty line

    let mut boards: Vec<BingoSheet> = Vec::new();

    let mut sheet: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        if line.is_empty() {
            boards.push(BingoSheet::new(sheet));
            sheet = Vec::new();
        } else {
            let row: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            sheet.push(row);
        }
    }

    'guess_loop: for guess in &guesses {
        for board in &mut boards {
            board.mark_number(*guess);
            if board.winning {
                println!("day4 part1 ans= {}", board.get_score());
                break 'guess_loop;
            }
        }
    }
    for board in &mut boards {
        board.reset();
    }

    'guess_loop2: for guess in guesses {
        for board in &mut boards {
            board.mark_number(guess);
        }

        if boards.len() > 1 {
            boards.retain(|x| !x.winning);
        }
        if (boards.len() == 1) & boards[0].winning {
            break 'guess_loop2;
        }
    }

    println!("day3 part2 ans= {}", boards[0].get_score());
}
