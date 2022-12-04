fn main() {
    let input: Vec<&str> = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .collect();

    let part1: u32 = input
        .iter()
        .map(|line| (&line[..line.len() / 2], &line[line.len() / 2..]))
        .map(|sack| find_duplicate_item(sack))
        .map(|item| get_priority(item))
        .sum();

    println!("{}", part1);

    let part2: u32 = input
        .chunks(3)
        .map(|chunk| find_badge(chunk))
        .map(|badge| get_priority(badge))
        .sum();

    println!("{}", part2);
}

fn get_priority(item: char) -> u32 {
    let c = item as u32;

    return if c >= 'a' as u32 && c <= 'z' as u32 {
        c - 'a' as u32 + 1
    } else {
        c - 'A' as u32 + 27
    };
}

fn find_duplicate_item(sack: (&str, &str)) -> char {
    for ch in sack.0.chars() {
        if sack.1.contains(ch) {
            return ch;
        }
    }

    panic!("no duplicate item");
}

fn find_badge(group: &[&str]) -> char {
    for ch in group[0].chars() {
        if group[1].contains(ch) && group[2].contains(ch) {
            return ch;
        }
    }

    panic!("no badge found");
}
