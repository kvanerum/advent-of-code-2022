use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug)]
struct Blueprint {
    id: u8,
    ore_robot_cost: u8,
    clay_robot_cost: u8,
    obsidian_robot_ore_cost: u8,
    obsidian_robot_clay_cost: u8,
    geode_robot_ore_cost: u8,
    geode_robot_obsidian_cost: u8,
}

#[derive(Debug, Clone)]
struct Status {
    ores: u8,
    ore_robots: u8,
    clay: u8,
    clay_robots: u8,
    obsidian: u8,
    obsidian_robots: u8,
    geodes: u8,
    geode_robots: u8,
    minutes_left: u8,
}

impl Default for Status {
    fn default() -> Self {
        Status {
            ores: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geodes: 0,
            geode_robots: 0,
            minutes_left: 32,
        }
    }
}

fn main() {
    let blueprints = read_input();

    let mut quality_level: u32 = 0;

    for blueprint in blueprints {
        let mut max_geodes = 0;
        let mut max_geode_robots_at_time: HashMap<u8, u8> = HashMap::new();

        simulate(
            &Status::default(),
            &blueprint,
            &mut max_geodes,
            &mut max_geode_robots_at_time,
        );
        println!("blueprint {}: {} geodes", blueprint.id, max_geodes);

        quality_level += blueprint.id as u32 * max_geodes as u32;
    }

    println!("{}", quality_level);
}

fn simulate(
    status: &Status,
    blueprint: &Blueprint,
    max_geodes: &mut u8,
    max_geode_robots_at_time: &mut HashMap<u8, u8>,
) {
    if status.minutes_left == 0 {
        if status.geodes > *max_geodes {
            *max_geodes = status.geodes;
        }

        return;
    }

    let mut machine_built = false;

    // try all robots
    machine_built =
        build_geode_robot(status, blueprint, max_geodes, max_geode_robots_at_time) || machine_built;
    machine_built = build_obsidian_robot(status, blueprint, max_geodes, max_geode_robots_at_time)
        || machine_built;
    machine_built =
        build_clay_robot(status, blueprint, max_geodes, max_geode_robots_at_time) || machine_built;
    machine_built =
        build_ore_robot(status, blueprint, max_geodes, max_geode_robots_at_time) || machine_built;

    if !machine_built {
        // simulate remaining minutes
        let end_status = collect_materials(status, status.minutes_left);

        if end_status.geodes > *max_geodes {
            *max_geodes = end_status.geodes;
        }
    }
}

fn build_ore_robot(
    status: &Status,
    blueprint: &Blueprint,
    max_geodes: &mut u8,
    max_geode_robots_at_time: &mut HashMap<u8, u8>,
) -> bool {
    if max_geode_robots_at_time
        .get(&(status.minutes_left - 1))
        .unwrap_or(&0)
        > &status.geode_robots
    {
        return false;
    }

    if blueprint.ore_robot_cost <= status.ores {
        let mut new_status = collect_materials(&status, 1);
        new_status.ores -= blueprint.ore_robot_cost;
        new_status.ore_robots += 1;
        simulate(&new_status, blueprint, max_geodes, max_geode_robots_at_time);
        return true;
    } else {
        let ores_needed = blueprint.ore_robot_cost - status.ores;
        let time_needed = (ores_needed + status.ore_robots - 1) / status.ore_robots + 1;

        if time_needed <= status.minutes_left {
            if max_geode_robots_at_time
                .get(&(status.minutes_left - time_needed))
                .unwrap_or(&0)
                > &status.geode_robots
            {
                return false;
            }

            let mut new_status = collect_materials(&status, time_needed);
            new_status.ore_robots += 1;
            new_status.ores -= blueprint.ore_robot_cost;
            simulate(&new_status, blueprint, max_geodes, max_geode_robots_at_time);
            return true;
        }
    }

    return false;
}

fn build_clay_robot(
    status: &Status,
    blueprint: &Blueprint,
    max_geodes: &mut u8,
    max_geode_robots_at_time: &mut HashMap<u8, u8>,
) -> bool {
    if max_geode_robots_at_time
        .get(&(status.minutes_left - 1))
        .unwrap_or(&0)
        > &status.geode_robots
    {
        return false;
    }

    if blueprint.clay_robot_cost <= status.ores {
        let mut new_status = collect_materials(&status, 1);
        new_status.ores -= blueprint.clay_robot_cost;
        new_status.clay_robots += 1;
        simulate(&new_status, blueprint, max_geodes, max_geode_robots_at_time);
        return true;
    } else {
        let ores_needed = blueprint.clay_robot_cost - status.ores;
        let time_needed = (ores_needed + status.ore_robots - 1) / status.ore_robots + 1;

        if time_needed < status.minutes_left {
            if max_geode_robots_at_time
                .get(&(status.minutes_left - time_needed))
                .unwrap_or(&0)
                > &status.geode_robots
            {
                return false;
            }

            let mut new_status = collect_materials(&status, time_needed);

            new_status.clay_robots += 1;
            new_status.ores -= blueprint.clay_robot_cost;
            simulate(&new_status, blueprint, max_geodes, max_geode_robots_at_time);
            return true;
        }
    }

    return false;
}

fn build_obsidian_robot(
    status: &Status,
    blueprint: &Blueprint,
    max_geodes: &mut u8,
    max_geode_robots_at_time: &mut HashMap<u8, u8>,
) -> bool {
    if max_geode_robots_at_time
        .get(&(status.minutes_left - 1))
        .unwrap_or(&0)
        > &status.geode_robots
    {
        return false;
    }

    if blueprint.obsidian_robot_ore_cost <= status.ores
        && blueprint.obsidian_robot_clay_cost <= status.clay
    {
        let mut new_status = collect_materials(&status, 1);
        new_status.ores -= blueprint.obsidian_robot_ore_cost;
        new_status.clay -= blueprint.obsidian_robot_clay_cost;
        new_status.obsidian_robots += 1;
        simulate(&new_status, blueprint, max_geodes, max_geode_robots_at_time);
        return true;
    } else if status.clay_robots > 0 {
        let ore_time_needed = if blueprint.obsidian_robot_ore_cost <= status.ores {
            0
        } else {
            let ores_needed = blueprint.obsidian_robot_ore_cost - status.ores;
            (ores_needed + status.ore_robots - 1) / status.ore_robots
        };

        let clay_time_needed = if blueprint.obsidian_robot_clay_cost <= status.clay {
            0
        } else {
            let clay_needed = blueprint.obsidian_robot_clay_cost - status.clay;
            (clay_needed + status.clay_robots - 1) / status.clay_robots
        };

        let time_needed = max(ore_time_needed, clay_time_needed) + 1;

        if time_needed < status.minutes_left {
            if max_geode_robots_at_time
                .get(&(status.minutes_left - time_needed))
                .unwrap_or(&0)
                > &status.geode_robots
            {
                return false;
            }

            let mut new_status = collect_materials(&status, time_needed);
            new_status.obsidian_robots += 1;
            new_status.ores -= blueprint.obsidian_robot_ore_cost;
            new_status.clay -= blueprint.obsidian_robot_clay_cost;
            simulate(&new_status, blueprint, max_geodes, max_geode_robots_at_time);
            return true;
        }
    }

    return false;
}

fn build_geode_robot(
    status: &Status,
    blueprint: &Blueprint,
    max_geodes: &mut u8,
    max_geode_robots_at_time: &mut HashMap<u8, u8>,
) -> bool {
    if blueprint.geode_robot_ore_cost <= status.ores
        && blueprint.geode_robot_obsidian_cost <= status.obsidian
    {
        let mut new_status = collect_materials(&status, 1);
        new_status.ores -= blueprint.geode_robot_ore_cost;
        new_status.obsidian -= blueprint.geode_robot_obsidian_cost;
        new_status.geode_robots += 1;

        if max_geode_robots_at_time
            .get(&new_status.minutes_left)
            .unwrap_or(&0)
            < &new_status.geode_robots
        {
            max_geode_robots_at_time.insert(new_status.minutes_left, new_status.geode_robots);
        }

        simulate(&new_status, blueprint, max_geodes, max_geode_robots_at_time);

        return true;
    } else if status.obsidian_robots > 0 {
        let ore_time_needed = if blueprint.geode_robot_ore_cost <= status.ores {
            0
        } else {
            let ores_needed = blueprint.geode_robot_ore_cost - status.ores;
            (ores_needed + status.ore_robots - 1) / status.ore_robots
        };

        let obsidian_time_needed = if blueprint.geode_robot_obsidian_cost <= status.obsidian {
            0
        } else {
            let obsidian_needed = blueprint.geode_robot_obsidian_cost - status.obsidian;
            (obsidian_needed + status.obsidian_robots - 1) / status.obsidian_robots
        };

        let time_needed = max(ore_time_needed, obsidian_time_needed) + 1;

        if time_needed < status.minutes_left {
            let mut new_status = collect_materials(&status, time_needed);
            new_status.geode_robots += 1;
            new_status.ores -= blueprint.geode_robot_ore_cost;
            new_status.obsidian -= blueprint.geode_robot_obsidian_cost;

            if max_geode_robots_at_time
                .get(&new_status.minutes_left)
                .unwrap_or(&0)
                < &new_status.geode_robots
            {
                max_geode_robots_at_time.insert(new_status.minutes_left, new_status.geode_robots);
            }

            simulate(&new_status, blueprint, max_geodes, max_geode_robots_at_time);
            return true;
        }
    }

    return false;
}

fn collect_materials(status: &Status, minutes: u8) -> Status {
    Status {
        ore_robots: status.ore_robots,
        clay_robots: status.clay_robots,
        obsidian_robots: status.obsidian_robots,
        geode_robots: status.geode_robots,
        ores: status.ores + status.ore_robots * minutes,
        clay: status.clay + status.clay_robots * minutes,
        obsidian: status.obsidian + status.obsidian_robots * minutes,
        geodes: status.geodes + status.geode_robots * minutes,
        minutes_left: status.minutes_left - minutes,
    }
}

fn read_input() -> Vec<Blueprint> {
    let regex =
        Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();

    return include_str!("../resources/input.txt")
        .lines()
        .map(|line| {
            let capture = regex.captures(line).unwrap();

            return Blueprint {
                id: capture[1].parse().unwrap(),
                ore_robot_cost: capture[2].parse().unwrap(),
                clay_robot_cost: capture[3].parse().unwrap(),
                obsidian_robot_ore_cost: capture[4].parse().unwrap(),
                obsidian_robot_clay_cost: capture[5].parse().unwrap(),
                geode_robot_ore_cost: capture[6].parse().unwrap(),
                geode_robot_obsidian_cost: capture[7].parse().unwrap(),
            };
        })
        .collect();
}
