use crate::ListEntry::{Int, Sublist};
use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::Greater;

#[derive(Debug)]
enum ListEntry {
    Sublist(Vec<ListEntry>),
    Int(u8),
}

fn main() {
    let input = read_input();

    let part1 = input
        .iter()
        .positions(|x| are_in_right_order(&(&x.0, &x.1)).unwrap())
        .map(|x| x as u32 + 1)
        .sum::<u32>();

    println!("{}", part1);

    let mut all_packets: Vec<&ListEntry> = input
        .iter()
        .map(|pair| Vec::from([&pair.0, &pair.1]))
        .flatten()
        .collect();

    let packet_2 = Sublist(Vec::from([Sublist(Vec::from([Int(2)]))]));
    let packet_6 = Sublist(Vec::from([Sublist(Vec::from([Int(6)]))]));

    all_packets.push(&packet_2);
    all_packets.push(&packet_6);

    let part2: u16 = all_packets
        .iter()
        .sorted_by(|a, b| {
            if are_in_right_order(&(a, b)) == Some(true) {
                Ordering::Less
            } else {
                Greater
            }
        })
        .positions(|x| {
            are_in_right_order(&(x, &packet_2)).is_none()
                || are_in_right_order(&(x, &packet_6)).is_none()
        })
        .map(|x| x as u16 + 1)
        .product();

    println!("{:?}", part2);
}

fn read_input() -> Vec<(ListEntry, ListEntry)> {
    return include_str!("../resources/input.txt")
        .trim()
        .split("\n\n")
        .map(|x| parse_pair(x))
        .collect();
}

fn parse_pair(input: &str) -> (ListEntry, ListEntry) {
    let split: Vec<&str> = input.split("\n").collect();

    return (
        parse(split[0], &mut (0 as usize)),
        parse(split[1], &mut (0 as usize)),
    );
}

fn parse(input: &str, current_pos: &mut usize) -> ListEntry {
    let mut current_list = Vec::new();
    let mut in_list = false;

    while *current_pos < input.len() {
        let current_char = input.chars().nth(*current_pos).unwrap();
        if current_char == '[' {
            if !in_list {
                in_list = true;
                *current_pos += 1;
            } else {
                current_list.push(parse(input, current_pos));
            }
        } else if current_char == ',' {
            *current_pos += 1;
        } else if current_char == ']' {
            *current_pos += 1;
            return Sublist(current_list);
        } else {
            let end = input
                .chars()
                .skip(*current_pos)
                .position(|x| !x.is_digit(10))
                .unwrap();

            let item = input
                .chars()
                .skip(*current_pos)
                .take(end)
                .collect::<String>()
                .parse::<u8>()
                .unwrap();

            *current_pos += 1;
            current_list.push(Int(item));
        }
    }

    panic!("invalid input");
}

fn are_in_right_order(pair: &(&ListEntry, &ListEntry)) -> Option<bool> {
    if let (Int(l), Int(r)) = (&pair.0, &pair.1) {
        return if l < r {
            Some(true)
        } else if l > r {
            Some(false)
        } else {
            None
        };
    } else if let (Sublist(l), Sublist(r)) = (&pair.0, &pair.1) {
        let mut index: usize = 0;

        loop {
            let left_item = l.get(index);
            let right_item = r.get(index);

            if let (Some(ll), Some(rr)) = (left_item, right_item) {
                let cmp = are_in_right_order(&(ll, rr));
                if cmp.is_some() {
                    return cmp;
                }
                index += 1;
            } else if left_item.is_none() && right_item.is_none() {
                return None;
            } else if left_item.is_none() {
                return Some(true);
            } else {
                return Some(false);
            }
        }
    } else if let (Int(l), Sublist(_)) = (&pair.0, &pair.1) {
        // panic!("left is int, convert to list");
        return are_in_right_order(&(&Sublist(Vec::from([Int(*l)])), pair.1));
    } else if let (Sublist(_), Int(r)) = (&pair.0, &pair.1) {
        // panic!("right is int, convert to list");
        return are_in_right_order(&(pair.0, &Sublist(Vec::from([Int(*r)]))));
    }

    panic!("invalid input");
}
