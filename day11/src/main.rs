use crate::monkey::Monkey;
use regex::Regex;

mod monkey;

fn main() {
    let input = read_input();

    run_part(&input, 20, None);

    let divider = input.iter().map(|x| x.0.divisible_by as u32).product();
    run_part(&input, 10000, Some(divider));
}

fn run_part(input: &Vec<(Monkey, Vec<u64>)>, rounds: u32, divider: Option<u32>) {
    let monkeys: Vec<&Monkey> = input.iter().map(|x| &(x.0)).collect();
    let mut items: Vec<Vec<u64>> = input.iter().map(|x| x.1.clone()).collect();
    let mut inspections: Vec<u32> = input.iter().map(|_| 0).collect();

    for _ in 0..rounds {
        play_round(&monkeys, &mut items, &mut inspections, divider);
    }

    inspections.sort();
    let result =
        inspections[inspections.len() - 1] as u64 * inspections[inspections.len() - 2] as u64;
    println!("{}", result);
}

fn play_round(
    monkeys: &Vec<&Monkey>,
    items: &mut Vec<Vec<u64>>,
    inspections: &mut Vec<u32>,
    divider: Option<u32>,
) {
    for monkey_index in 0..monkeys.len() {
        let monkey = &monkeys[monkey_index];
        for item in items[monkey_index].clone() {
            inspections[monkey_index] += 1;
            let mut worry_level: u64 = (monkey.operation)(item);

            if divider.is_none() {
                worry_level /= 3;
            } else {
                worry_level %= divider.unwrap() as u64;
            }

            if worry_level % monkey.divisible_by as u64 == 0 {
                items[monkey.true_destination as usize].push(worry_level);
            } else {
                items[monkey.false_destination as usize].push(worry_level);
            }
        }

        items[monkey_index].clear();
    }
}

fn read_input() -> Vec<(Monkey, Vec<u64>)> {
    let regex = Regex::new(
        r"^Monkey \d:
  Starting items: (.+)
  Operation: (.+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d)
    If false: throw to monkey (\d)$",
    )
    .unwrap();

    return include_str!("../resources/input.txt")
        .trim()
        .split("\n\n")
        .map(|m| parse_monkey(m, regex.clone()))
        .collect();
}

fn parse_monkey(input: &str, regex: Regex) -> (Monkey, Vec<u64>) {
    let capture = regex.captures(input).expect("parse monkey");

    return (
        Monkey {
            operation: parse_operation(capture[2].to_string()),
            divisible_by: capture[3].parse().unwrap(),
            true_destination: capture[4].parse().unwrap(),
            false_destination: capture[5].parse().unwrap(),
        },
        capture[1].split(", ").map(|x| x.parse().unwrap()).collect(),
    );
}

fn parse_operation(op: String) -> Box<dyn Fn(u64) -> u64> {
    let split: Vec<&str> = (&op[10..]).split_whitespace().collect();

    if split[0] == "+" {
        let arg = split[1].parse::<u64>().unwrap();
        return Box::new(move |old| old + arg);
    } else if split[0] == "*" {
        return if split[1] == "old" {
            Box::new(|old| old * old)
        } else {
            let arg = split[1].parse::<u64>().unwrap();
            Box::new(move |old| old * arg)
        };
    }

    panic!("cannot parse operation: {}", op);
}
