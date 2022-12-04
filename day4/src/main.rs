use regex::Regex;

fn main() {
    let regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    let input: Vec<((u8, u8), (u8, u8))> = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|line| parse_line(line, &regex))
        .collect();

    let part1 = input.iter().filter(|pair| fully_contains(pair)).count();
    println!("{}", part1);

    let part2 = input.iter().filter(|pair| have_overlap(pair)).count();
    println!("{}", part2);
}

fn parse_line(line: &str, regex: &Regex) -> ((u8, u8), (u8, u8)) {
    let capture = regex.captures(line).expect("parse line");

    return (
        (capture[1].parse().unwrap(), capture[2].parse().unwrap()),
        (capture[3].parse().unwrap(), capture[4].parse().unwrap()),
    );
}

fn fully_contains(pair: &((u8, u8), (u8, u8))) -> bool {
    return (pair.0 .0 <= pair.1 .0 && pair.0 .1 >= pair.1 .1)
        || (pair.0 .0 >= pair.1 .0 && pair.0 .1 <= pair.1 .1);
}

fn have_overlap(pair: &((u8, u8), (u8, u8))) -> bool {
    return pair.0 .0 <= pair.1 .1 && pair.0 .1 >= pair.1 .0;
}
