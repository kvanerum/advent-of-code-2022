extern crate core;

use std::collections::HashSet;

fn main() {
    let input = read_input();

    simulate(2, &input);
    simulate(10, &input);
}

fn simulate(num_knots: u8, moves: &Vec<(char, u16)>) {
    let mut rope: Vec<(i32, i32)> = (0..num_knots).map(|_| (0, 0)).collect();
    let mut all_tail_positions: HashSet<(i32, i32)> = HashSet::new();

    for movement in moves {
        for _ in 0..movement.1 {
            move_head(movement.0, &mut rope[0]);
            for i in 1..rope.len() {
                let previous = rope[i - 1].clone();
                move_knot(&mut rope[i], &previous);
            }
            all_tail_positions.insert(*rope.last().unwrap());
        }
    }

    println!("{}", all_tail_positions.len());
}

fn move_head(direction: char, position: &mut (i32, i32)) {
    match direction {
        'U' => position.1 += 1,
        'R' => position.0 += 1,
        'D' => position.1 -= 1,
        'L' => position.0 -= 1,
        _ => panic!("wrong input"),
    }
}

fn move_knot(tail_position: &mut (i32, i32), head_position: &(i32, i32)) {
    if (head_position.0 - tail_position.0).abs() <= 1
        && (head_position.1 - tail_position.1).abs() <= 1
    {
        return;
    }

    if head_position.0 != tail_position.0 {
        tail_position.0 += if tail_position.0 < head_position.0 {
            1
        } else {
            -1
        }
    }

    if head_position.1 != tail_position.1 {
        tail_position.1 += if tail_position.1 < head_position.1 {
            1
        } else {
            -1
        }
    }
}

fn read_input() -> Vec<(char, u16)> {
    let input = include_str!("../resources/input.txt");

    return input
        .trim()
        .split("\n")
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            return (split[0].chars().nth(0).unwrap(), split[1].parse().unwrap());
        })
        .collect();
}
