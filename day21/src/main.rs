use std::collections::HashMap;

fn main() {
    let input = read_input();

    let part1 = get_value("root", &input, false).unwrap();

    println!("{}", part1);

    let root = input.get("root").unwrap();
    let mut split = root.split(" ");
    let left_str = split.nth(0).unwrap();
    let right_str = split.nth(1).unwrap();
    let left = get_value(left_str, &input, true);
    let right = get_value(right_str, &input, true);

    let part2 = if left.is_none() {
        resolve(right.unwrap(), left_str, &input)
    } else {
        resolve(left.unwrap(), right_str, &input)
    };

    println!("{}", part2);
}

fn get_value(monkey: &str, input: &HashMap<String, String>, part2: bool) -> Option<i64> {
    if part2 && monkey == "humn" {
        return None;
    }

    let value = input.get(monkey).unwrap();
    let number = value.parse::<i64>();

    if number.is_ok() {
        return Some(number.unwrap());
    }

    let mut split = value.split(" ");
    let left_op = get_value(split.nth(0).unwrap(), input, part2);
    let op = split.nth(0).unwrap();
    let right_op = get_value(split.nth(0).unwrap(), input, part2);

    if left_op.is_none() || right_op.is_none() {
        return None;
    }

    return match op {
        "+" => Some(left_op.unwrap() + right_op.unwrap()),
        "-" => Some(left_op.unwrap() - right_op.unwrap()),
        "*" => Some(left_op.unwrap() * right_op.unwrap()),
        "/" => Some(left_op.unwrap() / right_op.unwrap()),
        &_ => panic!("invalid op {}", op),
    };
}

fn resolve(target: i64, monkey: &str, input: &HashMap<String, String>) -> i64 {
    if monkey == "humn" {
        return target;
    }

    let value = input.get(monkey).unwrap();

    let mut split = value.split(" ");
    let left_str = split.nth(0).unwrap();
    let op = split.nth(0).unwrap();
    let right_str = split.nth(0).unwrap();
    let left = get_value(left_str, &input, true);
    let right = get_value(right_str, &input, true);

    return if left.is_none() {
        let next_target = match op {
            "+" => target - right.unwrap(),
            "-" => target + right.unwrap(),
            "*" => target / right.unwrap(),
            "/" => target * right.unwrap(),
            &_ => panic!("invalid op {}", op),
        };

        resolve(next_target, left_str, input)
    } else {
        let next_target = match op {
            "+" => target - left.unwrap(),
            "-" => left.unwrap() - target,
            "*" => target / left.unwrap(),
            "/" => left.unwrap() / target,
            &_ => panic!("invalid op {}", op),
        };

        resolve(next_target, right_str, input)
    };
}

fn read_input() -> HashMap<String, String> {
    return include_str!("../resources/input.txt")
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            (
                split.nth(0).unwrap().to_string(),
                split.nth(0).unwrap().to_string(),
            )
        })
        .collect();
}
