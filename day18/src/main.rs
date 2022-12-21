use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = read_input();

    let part1: u32 = get_all_unconnected_sides(&input);

    println!("{}", part1);

    let mut water: HashSet<(i8, i8, i8)> = HashSet::new();
    flood_fill((-5, -5, -5), &input, &mut water);

    // find cube sides that connect to water
    let mut count = 0;
    for cube in input {
        for adj in get_adjacent_positions(cube) {
            if water.contains(&adj) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn flood_fill(
    current_position: (i8, i8, i8),
    lava: &HashSet<(i8, i8, i8)>,
    water: &mut HashSet<(i8, i8, i8)>,
) {
    water.insert(current_position);

    for next_position in get_adjacent_positions(current_position) {
        if next_position.0 >= -5
            && next_position.0 <= 25
            && next_position.1 >= -5
            && next_position.1 <= 25
            && next_position.2 >= -5
            && next_position.2 <= 25
            && !lava.contains(&next_position)
            && !water.contains(&next_position)
        {
            flood_fill(next_position, lava, water);
        }
    }
}

fn get_adjacent_positions(position: (i8, i8, i8)) -> Vec<(i8, i8, i8)> {
    return Vec::from([
        (position.0 - 1, position.1, position.2),
        (position.0 + 1, position.1, position.2),
        (position.0, position.1 - 1, position.2),
        (position.0, position.1 + 1, position.2),
        (position.0, position.1, position.2 - 1),
        (position.0, position.1, position.2 + 1),
    ]);
}

fn get_all_unconnected_sides(cubes: &HashSet<(i8, i8, i8)>) -> u32 {
    return cubes
        .iter()
        .map(|cube| get_unconnected_sides(cube, cubes) as u32)
        .sum();
}

fn get_unconnected_sides(cube: &(i8, i8, i8), all_cubes: &HashSet<(i8, i8, i8)>) -> i8 {
    let mut result: i8 = 6;

    let adjacent_positions = get_adjacent_positions(*cube);

    for adjacent_cube in adjacent_positions {
        if all_cubes.contains(&adjacent_cube) {
            result -= 1;
        }
    }

    return result;
}

fn read_input() -> HashSet<(i8, i8, i8)> {
    return include_str!("../resources/input.txt")
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
}
