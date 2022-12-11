use std::collections::HashSet;

fn main() {
    let input = read_input();

    let mut register: i32 = 1;
    let mut register_values = Vec::new();

    for op in input {
        let do_cycles: u8 = if op.0 == "noop" { 1 } else { 2 };

        for _ in 0..do_cycles {
            register_values.push(register);
        }

        if op.1.is_some() {
            register += op.1.unwrap();
        }
    }

    let sum = HashSet::from([20, 60, 100, 140, 180, 220])
        .into_iter()
        .map(|c| c - 1)
        .map(|c| (c + 1) as i32 * register_values[c as usize])
        .sum::<i32>();

    println!("{}\n\n", sum);

    let mut index: i32 = 0;
    for register_value in register_values {
        let c = if index % 40 >= register_value - 1 && index % 40 <= register_value + 1 {
            '#'
        } else {
            '.'
        };

        print!("{}", c);
        index += 1;

        if index % 40 == 0 {
            println!();
        }
    }
}

fn read_input() -> Vec<(String, Option<i32>)> {
    return include_str!("../resources/input.txt")
        .trim()
        .split("\n")
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();

            return (
                split[0].to_string(),
                split.get(1).map(|x| x.parse().unwrap()),
            );
        })
        .collect();
}
