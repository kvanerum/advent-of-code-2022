use std::collections::HashMap;

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

    let _cube_sides_example = [
        (8, 11, 0, 3),
        (0, 3, 4, 7),
        (4, 7, 4, 7),
        (8, 11, 4, 7),
        (8, 11, 8, 11),
        (12, 15, 8, 11),
    ];

    let cube_sides_input = [
        (50, 99, 0, 49),
        (100, 149, 0, 49),
        (50, 99, 50, 99),
        (0, 49, 100, 149),
        (50, 99, 100, 149),
        (0, 49, 150, 199),
    ];

    let _cube_transitions_example_2d: HashMap<(usize, u8), (usize, u8, u8, bool)> =
        HashMap::from([
            ((0, 1), (3, 1, 3, false)),
            ((1, 0), (2, 0, 2, false)),
            ((1, 1), (1, 1, 3, false)),
            ((2, 0), (3, 0, 2, false)),
            ((2, 1), (2, 1, 3, false)),
            ((3, 0), (1, 0, 2, false)),
        ]);

    let _cube_transitions_example_3d: HashMap<(usize, u8), (usize, u8, u8, bool)> =
        HashMap::from([
            ((0, 1), (3, 1, 3, false)),
            ((1, 0), (2, 0, 2, false)),
            ((2, 3), (0, 0, 2, false)),
            ((3, 0), (5, 1, 3, true)),
            ((4, 1), (1, 3, 1, true)),
            ((5, 2), (4, 2, 0, false)),
        ]);

    let cube_transitions_input_2d: HashMap<(usize, u8), (usize, u8, u8, bool)> = HashMap::from([
        ((0, 0), (1, 0, 2, false)),
        ((0, 1), (2, 1, 3, false)),
        ((0, 2), (1, 2, 0, false)),
        ((0, 3), (4, 3, 1, false)),
        ((1, 0), (0, 0, 2, false)),
        ((1, 1), (1, 1, 3, false)),
        ((1, 2), (0, 2, 0, false)),
        ((1, 3), (1, 3, 1, false)),
        ((2, 0), (2, 0, 2, false)),
        ((2, 1), (4, 1, 3, false)),
        ((2, 2), (2, 2, 0, false)),
        ((2, 3), (0, 3, 1, false)),
        ((3, 0), (4, 0, 2, false)),
        ((3, 1), (5, 1, 3, false)),
        ((3, 2), (4, 2, 0, false)),
        ((3, 3), (5, 3, 1, false)),
        ((4, 0), (3, 0, 2, false)),
        ((4, 1), (0, 1, 3, false)),
        ((4, 2), (3, 2, 0, false)),
        ((4, 3), (2, 3, 1, false)),
        ((5, 0), (5, 0, 2, false)),
        ((5, 1), (3, 1, 3, false)),
        ((5, 2), (5, 2, 0, false)),
        ((5, 3), (3, 3, 1, false)),
    ]);

    let cube_transitions_input_3d: HashMap<(usize, u8), (usize, u8, u8, bool)> = HashMap::from([
        ((0, 0), (1, 0, 2, false)),
        ((0, 1), (2, 1, 3, false)),
        ((0, 2), (3, 0, 2, true)),
        ((0, 3), (5, 0, 2, false)),
        ((1, 0), (4, 2, 0, true)),
        ((1, 1), (2, 2, 0, false)),
        ((1, 2), (0, 2, 0, false)),
        ((1, 3), (5, 3, 1, false)),
        ((2, 0), (1, 3, 1, false)),
        ((2, 1), (4, 1, 3, false)),
        ((2, 2), (3, 1, 3, false)),
        ((2, 3), (0, 3, 1, false)),
        ((3, 0), (4, 0, 2, false)),
        ((3, 1), (5, 1, 3, false)),
        ((3, 2), (0, 0, 2, true)),
        ((3, 3), (2, 0, 2, false)),
        ((4, 0), (1, 2, 0, true)),
        ((4, 1), (5, 2, 0, false)),
        ((4, 2), (3, 2, 0, false)),
        ((4, 3), (2, 3, 1, false)),
        ((5, 0), (4, 3, 1, false)),
        ((5, 1), (1, 1, 3, false)),
        ((5, 2), (0, 1, 3, false)),
        ((5, 3), (3, 3, 1, false)),
    ]);

    run(
        &map,
        &instructions,
        &cube_sides_input,
        &cube_transitions_input_2d,
    );

    run(
        &map,
        &instructions,
        &cube_sides_input,
        &cube_transitions_input_3d,
    );
}

fn run(
    map: &Vec<Vec<char>>,
    instructions: &Vec<Instruction>,
    cube_sides: &[(usize, usize, usize, usize)],
    cube_transitions: &HashMap<(usize, u8), (usize, u8, u8, bool)>,
) {
    let mut position = find_start_position(&map);
    let mut current_direction: u8 = 0;

    for instruction in instructions {
        if let Some(steps) = instruction.walk {
            walk(
                &mut position,
                steps,
                &mut current_direction,
                &map,
                cube_sides,
                cube_transitions,
            );
        } else if let Some(turn_to) = instruction.turn {
            turn(&mut current_direction, turn_to);
        }
    }

    println!(
        "{}",
        1000 * (position.1 + 1) + 4 * (position.0 + 1) + current_direction as usize
    )
}

fn walk(
    position: &mut (usize, usize),
    steps: u8,
    direction: &mut u8,
    map: &Vec<Vec<char>>,
    cube_sides: &[(usize, usize, usize, usize)],
    cube_transitions: &HashMap<(usize, u8), (usize, u8, u8, bool)>,
) {
    let mut steps_left = steps;

    while steps_left > 0 {
        let (next_x, next_y, next_direction) =
            get_next_position(&position, *direction, cube_sides, cube_transitions);

        if map[next_y][next_x] == '#' {
            return;
        }

        *position = (next_x, next_y);

        *direction = next_direction;
        steps_left -= 1;
    }
}

fn get_next_position(
    position: &(usize, usize),
    direction: u8,
    cube_sides: &[(usize, usize, usize, usize)],
    cube_transitions: &HashMap<(usize, u8), (usize, u8, u8, bool)>,
) -> (usize, usize, u8) {
    let mut x = position.0 as i16;
    let mut y = position.1 as i16;
    let mut new_direction = direction;

    let current_cube_side = get_cube_side(x, y, cube_sides);

    match direction {
        0 => x += 1,
        1 => y += 1,
        2 => x += -1,
        3 => y += -1,
        _ => panic!("invalid direction"),
    };

    let new_cube_side = get_cube_side(x, y, cube_sides);

    if current_cube_side != new_cube_side {
        do_transition(
            current_cube_side.unwrap(),
            &mut x,
            &mut y,
            &mut new_direction,
            cube_sides,
            cube_transitions,
        );
    }

    return (x as usize, y as usize, new_direction);
}

fn do_transition(
    cube_side: usize,
    x: &mut i16,
    y: &mut i16,
    direction: &mut u8,
    cube_sides: &[(usize, usize, usize, usize)],
    cube_transitions: &HashMap<(usize, u8), (usize, u8, u8, bool)>,
) {
    let (new_cube_side, new_direction, map_coordinate_to_side, flip) = cube_transitions
        .get(&(cube_side, *direction))
        .expect("transition fetched");

    let source_offset = match direction {
        1 | 3 => *x - cube_sides[cube_side].0 as i16,
        0 | 2 => *y - cube_sides[cube_side].2 as i16,
        _ => panic!(),
    };

    match map_coordinate_to_side {
        0 => {
            *x = cube_sides[*new_cube_side].1 as i16;
            *y = if *flip {
                cube_sides[*new_cube_side].3 as i16 - source_offset
            } else {
                cube_sides[*new_cube_side].2 as i16 + source_offset
            }
        }
        1 => {
            *x = if *flip {
                cube_sides[*new_cube_side].1 as i16 - source_offset
            } else {
                cube_sides[*new_cube_side].0 as i16 + source_offset
            };
            *y = cube_sides[*new_cube_side].3 as i16
        }
        2 => {
            *x = cube_sides[*new_cube_side].0 as i16;
            *y = if *flip {
                cube_sides[*new_cube_side].3 as i16 - source_offset
            } else {
                cube_sides[*new_cube_side].2 as i16 + source_offset
            }
        }
        3 => {
            *x = if *flip {
                cube_sides[*new_cube_side].1 as i16 - source_offset
            } else {
                cube_sides[*new_cube_side].0 as i16 + source_offset
            };
            *y = cube_sides[*new_cube_side].2 as i16;
        }
        _ => panic!(),
    }

    *direction = *new_direction;
}

fn get_cube_side(x: i16, y: i16, cube_sides: &[(usize, usize, usize, usize)]) -> Option<usize> {
    for i in 0..cube_sides.len() {
        if x >= cube_sides[i].0 as i16
            && x <= cube_sides[i].1 as i16
            && y >= cube_sides[i].2 as i16
            && y <= cube_sides[i].3 as i16
        {
            return Some(i);
        }
    }

    return None;
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
