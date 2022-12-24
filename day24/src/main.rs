use std::collections::HashSet;

fn main() {
    let (width, height, mut map) = read_input();
    let start = (1, 0);
    let end = (width - 2, height - 1);

    let mut time = move_to_target(start, end, &mut map, width, height);

    println!("{}", time);

    time += move_to_target(end, start, &mut map, width, height);
    time += move_to_target(start, end, &mut map, width, height);

    println!("{}", time);
}

fn move_to_target(
    start_position: (usize, usize),
    target_position: (usize, usize),
    map: &mut Vec<(char, usize, usize)>,
    width: usize,
    height: usize,
) -> u16 {
    let mut possible_positions = HashSet::from([start_position]);

    let mut minute = 0;
    while !possible_positions.contains(&target_position) {
        update_blizzard_positions(width, height, map);

        let mut next_possible_positions = HashSet::new();

        for p in &possible_positions {
            // down
            if (p.1 < height - 2 || (p.1 == height - 2 && p.0 == width - 2))
                && position_is_open((p.0, p.1 + 1), &map)
            {
                next_possible_positions.insert((p.0, p.1 + 1));
            }

            // right
            if p.0 < width - 2 && p.1 > 0 && position_is_open((p.0 + 1, p.1), &map) {
                next_possible_positions.insert((p.0 + 1, p.1));
            }

            // up
            if (p.1 > 1 || (p.1 == 1 && p.0 == 1)) && position_is_open((p.0, p.1 - 1), &map) {
                next_possible_positions.insert((p.0, p.1 - 1));
            }

            // left
            if p.0 > 1 && p.1 != height - 1 && position_is_open((p.0 - 1, p.1), &map) {
                next_possible_positions.insert((p.0 - 1, p.1));
            }

            // wait
            if position_is_open(*p, &map) {
                next_possible_positions.insert(*p);
            }
        }

        possible_positions = next_possible_positions;
        minute += 1;
    }

    return minute;
}

fn position_is_open(position: (usize, usize), map: &Vec<(char, usize, usize)>) -> bool {
    return !map
        .iter()
        .any(|blizzard| blizzard.1 == position.0 && blizzard.2 == position.1);
}

fn update_blizzard_positions(width: usize, height: usize, map: &mut Vec<(char, usize, usize)>) {
    for blizzard in map {
        match blizzard.0 {
            '>' => {
                blizzard.1 = if blizzard.1 == width - 2 {
                    1
                } else {
                    blizzard.1 + 1
                }
            }
            '<' => {
                blizzard.1 = if blizzard.1 == 1 {
                    width - 2
                } else {
                    blizzard.1 - 1
                }
            }
            '^' => {
                blizzard.2 = if blizzard.2 == 1 {
                    height - 2
                } else {
                    blizzard.2 - 1
                }
            }
            'v' => {
                blizzard.2 = if blizzard.2 == height - 2 {
                    1
                } else {
                    blizzard.2 + 1
                }
            }
            _ => panic!(),
        }
    }
}

fn read_input() -> (usize, usize, Vec<(char, usize, usize)>) {
    let mut result = Vec::new();

    let lines: Vec<&str> = include_str!("../resources/input.txt").lines().collect();

    for y in 0..lines.len() - 1 {
        let chars: Vec<char> = lines[y].chars().collect();

        for x in 0..chars.len() - 1 {
            if chars[x] == '>' || chars[x] == 'v' || chars[x] == '<' || chars[x] == '^' {
                result.push((chars[x], x, y));
            }
        }
    }

    return (lines[0].len(), lines.len(), result);
}
