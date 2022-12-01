use std::fs;

fn main() {
    let input = fs::read_to_string("day1/resources/input.txt").expect("read input file");

    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|elf| {
            elf.split_whitespace()
                .map(|calories| calories.parse::<u32>().expect("parse string"))
                .sum()
        })
        .collect();

    elves.sort();
    elves.reverse();

    println!("{}", elves[0]);
    println!("{}", elves.iter().take(3).sum::<u32>());
}
