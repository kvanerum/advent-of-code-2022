use std::collections::HashMap;

fn main() {
    let files = read_input();

    let directory_sizes: HashMap<String, u32> = files
        .iter()
        .filter(|(_, filesize)| filesize.is_none())
        .map(|(path, _)| (path.clone(), calculate_dir_size(path, &files)))
        .collect();

    let part1: u32 = directory_sizes
        .iter()
        .filter(|(_, size)| *size <= &100000)
        .map(|(_, size)| size)
        .sum();

    println!("{}", part1);

    let unused_space = 70000000 - calculate_dir_size("/", &files);
    let must_delete = 30000000 - unused_space;

    let part2 = directory_sizes
        .iter()
        .filter(|(_, size)| *size >= &must_delete)
        .min_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .1;
    println!("{}", part2);
}

fn read_input() -> HashMap<String, Option<u32>> {
    let input = include_str!("../resources/input.txt");

    let mut files: HashMap<String, Option<u32>> = HashMap::new();
    files.insert("/".parse().unwrap(), None);
    let mut current_path = Vec::new();

    for command in input.split("$ ").filter(|c| !c.is_empty()) {
        if command.starts_with("cd") {
            cd_command(command, &mut current_path);
        } else {
            ls_command(
                command,
                if current_path.len() > 0 {
                    "/".to_owned() + &current_path.join("/")
                } else {
                    "".to_string()
                },
                &mut files,
            );
        }
    }

    return files;
}

fn cd_command<'a>(command: &'a str, current_path: &mut Vec<&'a str>) {
    let destination = command.split(" ").nth(1).expect("destination").trim();
    match destination {
        "/" => current_path.clear(),
        ".." => {
            current_path.pop();
        }
        _ => current_path.push(destination),
    }
}

fn ls_command(command: &str, current_path: String, files: &mut HashMap<String, Option<u32>>) {
    for entry in command.trim_end().split("\n").skip(1) {
        let split: Vec<&str> = entry.trim().split(" ").collect();

        if entry.starts_with("dir") {
            files.insert(current_path.clone() + "/" + split[1], None);
        } else {
            files.insert(
                current_path.clone() + "/" + split[1],
                Some(split[0].parse().unwrap()),
            );
        }
    }
}

fn calculate_dir_size(dir: &str, files: &HashMap<String, Option<u32>>) -> u32 {
    return files
        .iter()
        .filter(|(k, _)| k.starts_with(dir))
        .filter(|(_, v)| v.is_some())
        .map(|(_, v)| v.unwrap())
        .sum();
}
