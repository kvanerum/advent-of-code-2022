fn main() {
    let input = read_input();

    let sum: u64 = input.iter().map(|s| snafu_to_decimal(s)).sum();
    println!("{}", sum);

    println!("{}", decimal_to_snafu(sum));
}

fn snafu_to_decimal(snafu: &str) -> u64 {
    let mut result: i64 = 0;
    let mut multiplier = 1;

    let chars: Vec<char> = snafu.chars().rev().collect();

    for c in chars {
        match c {
            '2' => result += multiplier * 2,
            '1' => result += multiplier,
            '0' => {}
            '-' => result -= multiplier,
            '=' => result -= multiplier * 2,
            _ => panic!(),
        }

        multiplier *= 5;
    }

    return result as u64;
}

fn decimal_to_snafu(decimal: u64) -> String {
    let mut multiplier = 1;
    let mut result: Vec<char> = Vec::from(['2']);

    while decimal > 2 * multiplier {
        multiplier *= 5;
        result.push('2');
    }

    let current_snafu: String = result.iter().collect();
    let mut current_value = snafu_to_decimal(&current_snafu);

    for i in 0..result.len() {
        let m = 5_u64.pow((result.len() - 1 - i) as u32);
        let too_much = current_value - decimal;
        let units_to_subtract = too_much / m;

        if units_to_subtract == 1 {
            result[i] = '1';
            current_value -= m;
        } else if units_to_subtract == 2 {
            result[i] = '0';
            current_value -= 2 * m;
        } else if units_to_subtract == 3 {
            result[i] = '-';
            current_value -= 3 * m;
        } else if units_to_subtract == 4 {
            result[i] = '=';
            current_value -= 4 * m;
        }
    }

    return result.iter().collect();
}

fn read_input() -> Vec<String> {
    return include_str!("../resources/input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();
}
