extern crate core;

use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input();

    let mut result = input.clone();

    for round in 0..10 {
        let (n, _) = do_round(&result, round);
        result = n;
    }

    println!("{}", count_empty(&result));

    let mut round = 0;
    let mut elf_moved = true;
    result = input.clone();

    while elf_moved {
        let (n, m) = do_round(&result, round);
        result = n;
        elf_moved = m;

        round += 1;
    }

    println!("{}", round);
}

fn count_empty(input: &HashSet<(i32, i32)>) -> u16 {
    let min_x = input.iter().map(|p| p.0).min().unwrap();
    let max_x = input.iter().map(|p| p.0).max().unwrap();
    let min_y = input.iter().map(|p| p.1).min().unwrap();
    let max_y = input.iter().map(|p| p.1).max().unwrap();

    let mut count = 0;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !input.contains(&(x, y)) {
                count += 1;
            }
        }
    }

    return count;
}

fn do_round(input: &HashSet<(i32, i32)>, round_number: u16) -> (HashSet<(i32, i32)>, bool) {
    let mut result = HashSet::new();
    let mut proposals: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut elf_moved = false;

    for elf in input {
        if !has_neighbors(elf, input) {
            result.insert(*elf);
            continue;
        }

        // propose a direction
        if let Some(proposal) = propose(elf, input, round_number) {
            proposals.insert(*elf, proposal);
        } else {
            result.insert(*elf);
        }
    }

    for proposal in &proposals {
        if proposals
            .iter()
            .any(|other_proposal| proposal.0 != other_proposal.0 && proposal.1 == other_proposal.1)
        {
            // don't move
            result.insert(*proposal.0);
        } else {
            // move
            result.insert(*proposal.1);
            elf_moved = true;
        }
    }

    return (result, elf_moved);
}

fn has_neighbors(elf: &(i32, i32), all_elves: &HashSet<(i32, i32)>) -> bool {
    for y in elf.1 - 1..elf.1 + 2 {
        for x in elf.0 - 1..elf.0 + 2 {
            if !(x == elf.0 && y == elf.1) && all_elves.contains(&(x, y)) {
                return true;
            }
        }
    }

    return false;
}

fn propose(
    position: &(i32, i32),
    all_elves: &HashSet<(i32, i32)>,
    round_number: u16,
) -> Option<(i32, i32)> {
    for i in round_number..round_number + 4 {
        let proposal_method = match i % 4 {
            0 => propose_north,
            1 => propose_south,
            2 => propose_west,
            3 => propose_east,
            _ => panic!(),
        };

        if let Some(proposal) = proposal_method(position, all_elves) {
            return Some(proposal);
        }
    }

    return None;
}

fn propose_north(position: &(i32, i32), all_elves: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    if all_elves.contains(&(position.0 - 1, position.1 - 1))
        || all_elves.contains(&(position.0, position.1 - 1))
        || all_elves.contains(&(position.0 + 1, position.1 - 1))
    {
        return None;
    }

    return Some((position.0, position.1 - 1));
}

fn propose_south(position: &(i32, i32), all_elves: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    if all_elves.contains(&(position.0 - 1, position.1 + 1))
        || all_elves.contains(&(position.0, position.1 + 1))
        || all_elves.contains(&(position.0 + 1, position.1 + 1))
    {
        return None;
    }

    return Some((position.0, position.1 + 1));
}

fn propose_east(position: &(i32, i32), all_elves: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    if all_elves.contains(&(position.0 + 1, position.1 - 1))
        || all_elves.contains(&(position.0 + 1, position.1))
        || all_elves.contains(&(position.0 + 1, position.1 + 1))
    {
        return None;
    }

    return Some((position.0 + 1, position.1));
}

fn propose_west(position: &(i32, i32), all_elves: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    if all_elves.contains(&(position.0 - 1, position.1 - 1))
        || all_elves.contains(&(position.0 - 1, position.1))
        || all_elves.contains(&(position.0 - 1, position.1 + 1))
    {
        return None;
    }

    return Some((position.0 - 1, position.1));
}

fn read_input() -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();
    let lines: Vec<&str> = include_str!("../resources/input.txt").lines().collect();

    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();
        for x in 0..line.len() {
            if line[x] == '#' {
                result.insert((x as i32, y as i32));
            }
        }
    }

    return result;
}
