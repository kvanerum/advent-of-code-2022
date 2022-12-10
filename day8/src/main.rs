fn main() {
    let grid = read_input();

    let mut count: u32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_visible(i, j, &grid) {
                count += 1;
            }
        }
    }

    println!("{}", count);

    let mut max_scenic_score: u32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let scenic_score = calculate_scenic_score(i, j, &grid);

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("{}", max_scenic_score);
}

fn read_input() -> Vec<Vec<u8>> {
    let input = include_str!("../resources/input.txt");

    return input
        .trim()
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
}

fn is_visible(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> bool {
    return is_visible_left(i, j, grid)
        || is_visible_right(i, j, grid)
        || is_visible_top(i, j, grid)
        || is_visible_bottom(i, j, grid);
}

fn is_visible_left(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> bool {
    if j == 0 {
        return true;
    }

    for x in 0..j {
        if grid[i][x] >= grid[i][j] {
            return false;
        }
    }

    return true;
}

fn is_visible_right(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> bool {
    for x in j + 1..grid[i].len() {
        if grid[i][x] >= grid[i][j] {
            return false;
        }
    }

    return true;
}

fn is_visible_top(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> bool {
    if i == 0 {
        return true;
    }

    for y in 0..i {
        if grid[y][j] >= grid[i][j] {
            return false;
        }
    }

    return true;
}

fn is_visible_bottom(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> bool {
    for y in i + 1..grid.len() {
        if grid[y][j] >= grid[i][j] {
            return false;
        }
    }

    return true;
}

fn calculate_scenic_score(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> u32 {
    return scenic_score_left(i, j, grid)
        * scenic_score_right(i, j, grid)
        * scenic_score_top(i, j, grid)
        * scenic_score_bottom(i, j, grid);
}

fn scenic_score_left(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> u32 {
    let mut count = 0;
    let mut x = j as i16 - 1;

    while x >= 0 {
        count += 1;

        if grid[i][x as usize] >= grid[i][j] {
            return count;
        }

        x -= 1;
    }

    return count;
}

fn scenic_score_right(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> u32 {
    let mut count = 0;
    let mut x = j as i16 + 1;

    while x < grid[i].len() as i16 {
        count += 1;

        if grid[i][x as usize] >= grid[i][j] {
            return count;
        }

        x += 1;
    }

    return count;
}

fn scenic_score_top(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> u32 {
    let mut count = 0;
    let mut y = i as i16 - 1;

    while y >= 0 {
        count += 1;

        if grid[y as usize][j] >= grid[i][j] {
            return count;
        }

        y -= 1;
    }

    return count;
}

fn scenic_score_bottom(i: usize, j: usize, grid: &Vec<Vec<u8>>) -> u32 {
    let mut count = 0;
    let mut y = i as i16 + 1;

    while y < grid.len() as i16 {
        count += 1;

        if grid[y as usize][j] >= grid[i][j] {
            return count;
        }

        y += 1;
    }

    return count;
}
