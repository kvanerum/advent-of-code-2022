fn main() {
    let input: Vec<(char, char)> = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
        .collect();

    let total_score_part_1: u32 = input.iter()
        .map(|entry| calculate_round_score_1(&entry))
        .fold(0u32, |acc, x| acc + x as u32);

    println!("{}", total_score_part_1);

    let total_score_part_2: u32 = input.iter()
        .map(|entry| calculate_round_score_2(entry))
        .fold(0u32, |acc, x| acc + x as u32);

    println!("{}", total_score_part_2);
}

fn calculate_round_score_1(round: &(char, char)) -> u8 {
    return get_own_score(round.1) + get_win_score(round);
}

fn calculate_round_score_2(round: &(char, char)) -> u8 {
    return get_result_score(round.1) + get_own_score_to_lose(round);
}

fn get_own_score(played: char) -> u8 {
    return match played {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0
    };
}

fn get_win_score(round: &(char, char)) -> u8 {
    return match round.0 {
        'A' => match round.1 {
            'X' => 3,
            'Y' => 6,
            'Z' => 0,
            _ => 0
        },
        'B' => match round.1 {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => 0
        },
        'C' => match round.1 {
            'X' => 6,
            'Y' => 0,
            'Z' => 3,
            _ => 0
        },
        _ => 0
    };
}

fn get_result_score(result: char) -> u8 {
    return match result {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0
    };
}

fn get_own_score_to_lose(round: &(char, char)) -> u8 {
    return match round.0 {
        'A' => match round.1 {
            'X' => get_own_score('Z'),
            'Y' => get_own_score('X'),
            'Z' => get_own_score('Y'),
            _ => 0
        },
        'B' => match round.1 {
            'X' => get_own_score('X'),
            'Y' => get_own_score('Y'),
            'Z' => get_own_score('Z'),
            _ => 0
        },
        'C' => match round.1 {
            'X' => get_own_score('Y'),
            'Y' => get_own_score('Z'),
            'Z' => get_own_score('X'),
            _ => 0
        },
        _ => 0
    };
}
