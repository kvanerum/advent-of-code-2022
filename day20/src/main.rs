fn main() {
    let input = read_input();
    let mut result: Vec<&i64> = input.iter().collect();

    mix(&mut result, &input);
    find_coordinates(&result);

    let decryption_key: i64 = 811589153;
    let input_decrypted: Vec<i64> = input.iter().map(|x| *x as i64 * decryption_key).collect();
    result = input_decrypted.iter().collect();

    for _ in 0..10 {
        mix(&mut result, &input_decrypted);
    }

    find_coordinates(&result);
}

fn mix(result: &mut Vec<&i64>, order: &Vec<i64>) {
    let len = order.len() as i64 - 1;
    for i in order {
        let position = result.iter().position(|x| std::ptr::eq(*x, i)).unwrap();
        let mut target_position = position as i64 + *i;

        if target_position < 0 {
            target_position += (target_position.abs() + len - 1) / len * len;
        }

        target_position %= len;

        let popped = result.remove(position);
        result.insert(target_position as usize, popped);
    }
}

fn find_coordinates(result: &Vec<&i64>) {
    let p0 = result.iter().position(|x| **x == 0).unwrap();

    let p1000 = get_at_position_relative_to(p0, 1000, &result);
    let p2000 = get_at_position_relative_to(p0, 2000, &result);
    let p3000 = get_at_position_relative_to(p0, 3000, &result);

    println!("{}", p1000 + p2000 + p3000);
}

fn get_at_position_relative_to(start: usize, offset: i32, list: &Vec<&i64>) -> i64 {
    let position = (start + offset as usize) % list.len();
    return *list[position];
}

fn read_input() -> Vec<i64> {
    return include_str!("../resources/input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
}
