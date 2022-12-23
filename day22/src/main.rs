#[derive(Debug)]
struct Instruction {
    walk: Option<u8>,
    turn: Option<char>,
}

// directions:
// right = 0
// down = 1
// left = 2
// up = 3

fn main() {
    let (map, instructions) = read_input();

    let mut position = find_start_position(&map);
    let mut current_direction: u8 = 0;

    for instruction in instructions {
        if let Some(steps) = instruction.walk {
            walk(&mut position, steps, current_direction, &map);
        } else if let Some(turn_to) = instruction.turn {
            turn(&mut current_direction, turn_to);
        }
    }

    println!(
        "{}",
        1000 * (position.1 + 1) + 4 * (position.0 + 1) + current_direction as usize
    )
}

fn walk(position: &mut (usize, usize), steps: u8, direction: u8, map: &Vec<Vec<char>>) {
    let mut steps_left = steps;

    while steps_left > 0 {
        let next_position = get_next_position(&position, direction, map);

        if map[next_position.1][next_position.0] == '#' {
            return;
        }

        *position = next_position;
        steps_left -= 1;
    }
}

fn get_next_position(
    position: &(usize, usize),
    direction: u8,
    map: &Vec<Vec<char>>,
) -> (usize, usize) {
    let mut x = position.0 as i16;
    let mut y = position.1 as i16;

    match direction {
        0 => x = fix_x(x + 1, &map[position.1], direction) as i16,
        1 => y = fix_y(position.0, y + 1, map, direction) as i16,
        2 => x = fix_x(x - 1, &map[position.1], direction) as i16,
        3 => y = fix_y(position.0, y - 1, map, direction) as i16,
        _ => panic!("invalid direction"),
    };

    return (x as usize, y as usize);
}

fn fix_x(x: i16, row: &Vec<char>, direction: u8) -> usize {
    return if x < 0 || (direction == 2 && row[x as usize] == ' ') {
        let mut outer_right = row.len() - 1;

        while row[outer_right] == ' ' {
            outer_right -= 1;
        }

        outer_right
    } else if x as usize >= row.len() || (direction == 0 && row[x as usize] == ' ') {
        let mut outer_left = 0;

        while row[outer_left] == ' ' {
            outer_left += 1;
        }

        outer_left
    } else {
        x as usize
    };
}

fn fix_y(x: usize, y: i16, map: &Vec<Vec<char>>, direction: u8) -> usize {
    return if y < 0 || (direction == 3 && map[y as usize][x] == ' ') {
        let mut outer_bottom = map.len() - 1;

        while map[outer_bottom].len() < x || map[outer_bottom][x] == ' ' {
            outer_bottom -= 1;
        }

        outer_bottom
    } else if y as usize >= map.len()
        || (direction == 1 && (x > map[y as usize].len() || map[y as usize][x] == ' '))
    {
        let mut outer_top = 0;

        while map[outer_top][x] == ' ' {
            outer_top += 1;
        }

        outer_top
    } else {
        y as usize
    };
}

fn turn(direction: &mut u8, turn: char) {
    let mut d = *direction as i8;
    if turn == 'R' {
        d += 1;
    } else {
        d -= 1;
    }

    if d == -1 {
        d = 3;
    }

    if d == 4 {
        d = 0;
    }

    *direction = d as u8;
}

fn find_start_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    let first_open_position = map[0].iter().position(|pos| *pos == '.').unwrap();

    return (first_open_position, 0);
}

fn read_input() -> (Vec<Vec<char>>, Vec<Instruction>) {
    let input = include_str!("../resources/input.txt");

    let mut split = input.split("\n\n");

    let map = read_map(split.nth(0).unwrap());
    let instructions = read_instructions(split.nth(0).unwrap().trim());

    return (map, instructions);
}

fn read_map(input: &str) -> Vec<Vec<char>> {
    return input.lines().map(|line| line.chars().collect()).collect();
}

fn read_instructions(input: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    let mut current_pos = 0;
    let mut buffer: Vec<char> = Vec::new();

    while current_pos < input.len() {
        let c = input.chars().nth(current_pos).unwrap();
        if c == 'R' {
            if !buffer.is_empty() {
                clear_buffer(&mut buffer, &mut result);
            }

            result.push(Instruction {
                turn: Some('R'),
                walk: None,
            });
        } else if c == 'L' {
            if !buffer.is_empty() {
                clear_buffer(&mut buffer, &mut result);
            }

            result.push(Instruction {
                turn: Some('L'),
                walk: None,
            });
        } else {
            buffer.push(c);
        }
        current_pos += 1;
    }

    if !buffer.is_empty() {
        clear_buffer(&mut buffer, &mut result);
    }

    return result;
}

fn clear_buffer(buffer: &mut Vec<char>, instructions: &mut Vec<Instruction>) {
    let s: String = buffer.iter().collect();
    buffer.clear();
    let n: u8 = s.parse().unwrap();

    instructions.push(Instruction {
        turn: None,
        walk: Some(n),
    })
}
