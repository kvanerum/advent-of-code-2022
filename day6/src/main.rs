use std::collections::HashSet;

fn main() {
    let input = include_str!("../resources/input.txt");

    let mut index: u16 = 4;
    while !is_marker(input, index as usize, 4) {
        index += 1;
    }

    println!("{}", index);

    index = 16;
    while !is_marker(input, index as usize, 14) {
        index += 1;
    }

    println!("{}", index);
}

fn is_marker(str: &str, index: usize, different_chars: usize) -> bool {
    let char_set: HashSet<char> = str
        .chars()
        .skip(index - different_chars)
        .take(different_chars)
        .collect();

    return char_set.len() == different_chars;
}
