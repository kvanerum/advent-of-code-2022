use std::collections::HashSet;

fn main() {
    let rocks = read_input();

    let mut sand: HashSet<(u16, u16)> = HashSet::new();

    let max_y = rocks.iter().map(|r| r.1).max().unwrap();

    let mut count: u16 = 0;
    while let Some(s) = drop_sand(&rocks, &sand, max_y, false) {
        sand.insert(s);
        count += 1;
    }

    println!("{}", count);

    count = 0;
    sand.clear();

    loop {
        let s = drop_sand(&rocks, &sand, max_y, true);
        sand.insert(s.unwrap());
        count += 1;

        if s == Some((500, 0)) {
            break;
        }
    }

    println!("{}", count);
}

fn drop_sand(
    rocks: &HashSet<(u16, u16)>,
    sand: &HashSet<(u16, u16)>,
    max_y: u16,
    has_floor: bool,
) -> Option<(u16, u16)> {
    let mut x: u16 = 500;
    let mut y: u16 = 0;

    loop {
        if !has_floor && y > max_y {
            return None;
        } else if check_position((x, y + 1), rocks, sand, max_y, has_floor) {
            y += 1;
        } else if check_position((x - 1, y + 1), rocks, sand, max_y, has_floor) {
            y += 1;
            x -= 1;
        } else if check_position((x + 1, y + 1), rocks, sand, max_y, has_floor) {
            y += 1;
            x += 1;
        } else {
            break;
        }
    }

    return Some((x, y));
}

fn check_position(
    new_position: (u16, u16),
    rocks: &HashSet<(u16, u16)>,
    sand: &HashSet<(u16, u16)>,
    max_y: u16,
    has_floor: bool,
) -> bool {
    if rocks.contains(&new_position) || sand.contains(&new_position) {
        return false;
    }

    if has_floor && new_position.1 == max_y + 2 {
        return false;
    }

    return true;
}

fn read_input() -> HashSet<(u16, u16)> {
    let mut result = HashSet::new();

    include_str!("../resources/input.txt")
        .trim()
        .split("\n")
        .for_each(|line| {
            result.extend(get_rock_path(line));
        });

    return result;
}

fn get_rock_path(line: &str) -> HashSet<(u16, u16)> {
    let lines = parse_line(line);
    let mut current_line: usize = 0;
    let mut current_point = lines[0];
    let mut all_points = HashSet::new();
    all_points.insert(current_point);

    while current_line < lines.len() - 1 {
        let destination = lines[current_line + 1];
        while current_point != destination {
            let mut new_x = current_point.0;
            let mut new_y = current_point.1;

            if current_point.0 < destination.0 {
                new_x += 1;
            } else if current_point.0 > destination.0 {
                new_x -= 1;
            }

            if current_point.1 < destination.1 {
                new_y += 1;
            } else if current_point.1 > destination.1 {
                new_y -= 1;
            }

            current_point = (new_x, new_y);
            all_points.insert(current_point);
        }

        current_line += 1;
    }

    return all_points;
}

fn parse_line(line: &str) -> Vec<(u16, u16)> {
    return line
        .split(" -> ")
        .map(|point| {
            let mut point_split = point.split(",");

            (
                point_split.nth(0).unwrap().parse().unwrap(),
                point_split.nth(0).unwrap().parse().unwrap(),
            )
        })
        .collect();
}
