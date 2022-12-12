use std::collections::HashSet;

fn main() {
    let input = read_input();

    let end_position = get_position(&input, 'E');

    find_shortest_path_length(
        HashSet::from([get_position(&input, 'S')]),
        end_position,
        &input,
    );

    find_shortest_path_length(get_positions(&input, 'a'), end_position, &input);
}

fn find_shortest_path_length(
    start_positions: HashSet<(usize, usize)>,
    end_position: (usize, usize),
    map: &Vec<Vec<char>>,
) {
    let mut positions_checked: HashSet<(usize, usize)> = HashSet::new();
    let mut positions_to_check = start_positions.clone();
    let mut steps: u16 = 0;

    while !positions_to_check.contains(&end_position) {
        positions_checked.extend(&positions_to_check);
        positions_to_check = do_step(&positions_to_check, &positions_checked, map);

        steps += 1;
    }

    println!("{}", steps);
}

fn read_input() -> Vec<Vec<char>> {
    return include_str!("../resources/input.txt")
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
}

fn get_position(input: &Vec<Vec<char>>, destination: char) -> (usize, usize) {
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == destination {
                return (x, y);
            }
        }
    }

    panic!("start position not found");
}

fn get_positions(input: &Vec<Vec<char>>, destination: char) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == destination {
                result.insert((x, y));
            }
        }
    }

    return result;
}

fn do_step(
    to_check: &HashSet<(usize, usize)>,
    already_checked: &HashSet<(usize, usize)>,
    map: &Vec<Vec<char>>,
) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();

    for position in to_check {
        let char_at_position = map[position.1][position.0];

        for new_pos in [
            (position.0 as i16, position.1 as i16 - 1),
            (position.0 as i16, position.1 as i16 + 1),
            (position.0 as i16 - 1, position.1 as i16),
            (position.0 as i16 + 1, position.1 as i16),
        ] {
            if new_pos.1 >= 0
                && new_pos.1 < map.len() as i16
                && new_pos.0 >= 0
                && new_pos.0 < map[new_pos.1 as usize].len() as i16
                && is_valid_target(
                    char_at_position,
                    map[new_pos.1 as usize][new_pos.0 as usize],
                )
                && !already_checked.contains(&(new_pos.0 as usize, new_pos.1 as usize))
            {
                result.insert((new_pos.0 as usize, new_pos.1 as usize));
            }
        }
    }

    return result;
}

fn is_valid_target(current: char, target: char) -> bool {
    return match current {
        'S' => target == 'a',
        'E' => true,
        'z' => target == 'z' || target == 'E',
        _ => target as u8 >= 'a' as u8 && target as u8 <= current as u8 + 1,
    };
}
