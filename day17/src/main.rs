use std::collections::HashSet;

fn main() {
    let jets = read_input();

    // blocks 0,0 = bottom-left
    let blocks: Vec<HashSet<(i32, i32)>> = vec![
        HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0)]),
        HashSet::from([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        HashSet::from([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3)]),
        HashSet::from([(0, 0), (1, 0), (0, 1), (1, 1)]),
    ];

    let num_blocks: u64 = 100000;
    let highest_rock_positions = run(&blocks, &jets, num_blocks as usize);

    println!("{}", highest_rock_positions[2021]);

    let pattern_size = find_pattern(&highest_rock_positions, 10);
    let pattern_difference = highest_rock_positions[num_blocks as usize - 1]
        - highest_rock_positions[(num_blocks - pattern_size) as usize - 1];

    // extrapolate
    let target: u64 = 1000000000000;
    let num_skips = (target - num_blocks) / pattern_size + 1;
    let go_back = (num_blocks + num_skips * pattern_size) % target;

    println!(
        "{}",
        highest_rock_positions[(num_blocks - go_back) as usize - 1] as u64
            + num_skips * pattern_difference as u64
    );
}

fn find_pattern(highest_rock_positions: &Vec<i32>, occurrences: u8) -> u64 {
    for pattern_size in 1..highest_rock_positions.len() / 5 {
        if verify_pattern(pattern_size as i32, &highest_rock_positions, occurrences) {
            return pattern_size as u64;
        }
    }

    panic!("no pattern found")
}

fn verify_pattern(pattern_size: i32, highest_rock_positions: &Vec<i32>, occurrences: u8) -> bool {
    let mut i: i32 = highest_rock_positions.len() as i32 - 1;
    let difference =
        highest_rock_positions[i as usize] - highest_rock_positions[(i - pattern_size) as usize];
    i -= pattern_size;

    for _ in 0..occurrences {
        if i < 0
            || highest_rock_positions[(i + pattern_size) as usize]
                - highest_rock_positions[i as usize]
                != difference
        {
            return false;
        }

        i -= pattern_size;
    }

    return true;
}

fn run(blocks: &Vec<HashSet<(i32, i32)>>, jets: &Vec<char>, num_blocks: usize) -> Vec<i32> {
    let mut jet_index: u16 = 0;
    let mut rock_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut highest_rock_positions = Vec::new();

    for block_index in 0..num_blocks {
        let block = &blocks[block_index % blocks.len()];
        let mut current_position: (i32, i32) = (2, get_highest_rock_position(&rock_positions) + 3);
        let mut current_absolute_positions = get_absolute_block_positions(block, current_position);

        while !are_colliding(&current_absolute_positions, &rock_positions)
            && !reached_floor(&current_absolute_positions)
        {
            jet_push(
                &mut current_position,
                block,
                jets[jet_index as usize % jets.len()],
                &rock_positions,
            );
            jet_index = (jet_index + 1) % jets.len() as u16;

            current_position.1 -= 1;
            current_absolute_positions = get_absolute_block_positions(block, current_position);
        }

        // go up and consolidate
        current_position.1 += 1;
        current_absolute_positions = get_absolute_block_positions(block, current_position);
        rock_positions.extend(current_absolute_positions);
        cleanup_rock_positions(&mut rock_positions);
        highest_rock_positions.push(get_highest_rock_position(&rock_positions));
    }

    return highest_rock_positions;
}

fn jet_push(
    current_position: &mut (i32, i32),
    block: &HashSet<(i32, i32)>,
    direction: char,
    rock_positions: &HashSet<(i32, i32)>,
) {
    let new_x = current_position.0 + if direction == '<' { -1 } else { 1 };
    let absolute_block_positions =
        get_absolute_block_positions(&block, (new_x, current_position.1));

    if is_between_walls(&absolute_block_positions)
        && !are_colliding(&absolute_block_positions, rock_positions)
    {
        current_position.0 = new_x;
    }
}

fn get_absolute_block_positions(
    block: &HashSet<(i32, i32)>,
    offset: (i32, i32),
) -> HashSet<(i32, i32)> {
    return block
        .iter()
        .map(|b| (offset.0 + b.0, offset.1 + b.1))
        .collect();
}

fn get_highest_rock_position(rock_positions: &HashSet<(i32, i32)>) -> i32 {
    return rock_positions
        .iter()
        .map(|position| position.1 + 1)
        .max()
        .unwrap_or(0);
}

fn is_between_walls(rocks: &HashSet<(i32, i32)>) -> bool {
    return rocks.iter().map(|rock| rock.0).all(|x| x >= 0 && x < 7);
}

fn are_colliding(rock1: &HashSet<(i32, i32)>, rock2: &HashSet<(i32, i32)>) -> bool {
    return !rock1.is_disjoint(rock2);
}

fn reached_floor(rocks: &HashSet<(i32, i32)>) -> bool {
    return rocks.iter().any(|rock| rock.1 < 0);
}

fn cleanup_rock_positions(rock_positions: &mut HashSet<(i32, i32)>) {
    let mut x0: Vec<i32> = rock_positions
        .iter()
        .filter(|pos| pos.0 == 0)
        .map(|pos| pos.1)
        .collect();
    x0.sort();
    x0.reverse();

    for y in x0 {
        if let Some(chain) = create_chain(y, rock_positions) {
            // remove everything under the chain
            rock_positions.retain(|pos| pos.1 >= chain[pos.0 as usize]);

            return;
        }
    }
}

fn create_chain(y0: i32, rock_positions: &HashSet<(i32, i32)>) -> Option<Vec<i32>> {
    let mut result = Vec::new();
    result.push(y0);

    for x in 1..7 {
        let last = result.last().unwrap();
        if rock_positions.contains(&(x, last + 1)) {
            result.push(last + 1);
        } else if rock_positions.contains(&(x, *last)) {
            result.push(*last);
        } else if rock_positions.contains(&(x, last - 1)) {
            result.push(last - 1);
        } else {
            return None;
        }
    }

    return Some(result);
}

fn read_input() -> Vec<char> {
    return include_str!("../resources/input.txt")
        .trim()
        .chars()
        .collect();
}
