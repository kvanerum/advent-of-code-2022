use crate::input::Input;
use crate::input::Move;
use regex::Regex;

mod input;

fn main() {
    let input = read_input();

    let mut crates_part_1 = input.stacks.clone();
    for m in &input.moves {
        for _ in 0..m.amount {
            let c = crates_part_1[(m.from - 1) as usize].pop().unwrap();
            crates_part_1[(m.to - 1) as usize].push(c);
        }
    }

    println!("{}", get_result(crates_part_1));

    let mut crates_part_2 = input.stacks.clone();
    for m in &input.moves {
        let crate_from = &mut crates_part_2[(m.from - 1) as usize];
        let new_size = crate_from.len() - m.amount as usize;
        let pick_up: Vec<char> = crate_from.drain(new_size..).collect();
        crate_from.truncate(new_size);
        crates_part_2[(m.to - 1) as usize].extend(pick_up);
    }

    println!("{}", get_result(crates_part_2));
}

fn get_result(crates: Vec<Vec<char>>) -> String {
    return crates.iter().map(|stack| stack.last().unwrap()).collect();
}

fn read_input() -> Input {
    let input = include_str!("../resources/input.txt");
    let parts: Vec<&str> = input.split("\n\n").collect();

    return Input {
        stacks: read_stacks(parts[0]),
        moves: read_moves(parts[1]),
    };
}

fn read_stacks(input: &str) -> Vec<Vec<char>> {
    // positions: 1-5-9...
    let lines: Vec<&str> = input.split("\n").collect();

    return (1..lines.last().unwrap().len())
        .step_by(4)
        .into_iter()
        .map(|index| read_stack(index as u8, &lines))
        .collect();
}

fn read_stack(index: u8, lines: &Vec<&str>) -> Vec<char> {
    return lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| line.chars().nth(index as usize).unwrap_or(' '))
        .filter(|c| *c != ' ')
        .rev()
        .collect();
}

fn read_moves(input: &str) -> Vec<Move> {
    let regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    return input
        .trim_end()
        .split("\n")
        .map(|line| {
            let captures = regex.captures(line).expect("parse line");

            return Move {
                amount: captures[1].parse().unwrap(),
                from: captures[2].parse().unwrap(),
                to: captures[3].parse().unwrap(),
            };
        })
        .collect();
}
