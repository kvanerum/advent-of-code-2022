use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Input {
    flow_rate_reduction: u8,
    connections: HashMap<String, u8>,
}

fn main() {
    let input = read_input();
    let reduced_input = reduce_input(&input);
    let valves_to_open: HashSet<String> = reduced_input
        .keys()
        .filter(|key| key != &"AA")
        .map(|s| s.to_string())
        .collect();
    let mut solutions: HashMap<String, u16> = HashMap::new();

    run(
        "AA",
        30,
        0,
        HashSet::new(),
        valves_to_open.clone(),
        &reduced_input,
        &mut solutions,
    );

    println!("{}", solutions.values().max().unwrap());

    solutions.clear();
    run(
        "AA",
        26,
        0,
        HashSet::new(),
        valves_to_open.clone(),
        &reduced_input,
        &mut solutions,
    );

    // find distinct combination with highest flow reduction
    println!("{}", find_best_distinct_combination(&solutions));
}

fn to_key(set: &HashSet<String>) -> String {
    let mut sorted: Vec<String> = set.iter().map(|x| x.to_string()).collect();
    sorted.sort();
    return sorted.join(",");
}

fn from_key(key: &String) -> HashSet<String> {
    return key.split(",").map(|x| x.to_string()).collect();
}

fn find_best_distinct_combination(solutions: &HashMap<String, u16>) -> u16 {
    let mut max_reduction = 0;
    for combination in solutions.iter().combinations(2) {
        let human = combination[0];
        let elephant = combination[1];

        if from_key(human.0).is_disjoint(&from_key(elephant.0)) {
            if human.1 + elephant.1 > max_reduction {
                max_reduction = human.1 + elephant.1;
            }
        }
    }

    return max_reduction;
}

fn run(
    current_position: &str,
    minutes_left: u8,
    total_flow_reduction: u16,
    valves_open: HashSet<String>,
    valves_to_open: HashSet<String>,
    configuration: &HashMap<String, Input>,
    solutions: &mut HashMap<String, u16>,
) {
    let key = to_key(&valves_open);
    if !solutions.contains_key(&key) || total_flow_reduction > *solutions.get(&key).unwrap() {
        solutions.insert(to_key(&valves_open), total_flow_reduction);
    }

    // try all valves from current position
    for next_valve in &valves_to_open {
        let travel_time = configuration
            .get(current_position)
            .unwrap()
            .connections
            .get(&*next_valve)
            .unwrap();

        if *travel_time < minutes_left - 1 {
            // travel and open valve
            let next_minutes_left = minutes_left - travel_time - 1;
            let next_flow_reduction =
                configuration.get(&*next_valve).unwrap().flow_rate_reduction as u16;
            let next_total_flow_reduction =
                total_flow_reduction + (next_minutes_left as u16 * next_flow_reduction);
            let mut next_valves_to_open = valves_to_open.clone();
            next_valves_to_open.remove(next_valve);
            let mut next_valves_open = valves_open.clone();
            next_valves_open.insert(next_valve.clone());

            run(
                &*next_valve,
                next_minutes_left,
                next_total_flow_reduction,
                next_valves_open,
                next_valves_to_open,
                configuration,
                solutions,
            );
        }
    }
}

fn reduce_input(input: &HashMap<String, Input>) -> HashMap<String, Input> {
    let useful_valves: HashSet<&String> = input
        .iter()
        .filter(|valve| valve.1.flow_rate_reduction > 0 || valve.0 == "AA")
        .map(|valve| valve.0)
        .collect();

    return useful_valves
        .iter()
        .map(|valve| {
            (
                valve.to_string(),
                Input {
                    flow_rate_reduction: input.get(*valve).unwrap().flow_rate_reduction,
                    connections: find_shortest_distances(valve, &useful_valves, input),
                },
            )
        })
        .collect();
}

fn find_shortest_distances(
    start: &String,
    destinations: &HashSet<&String>,
    configuration: &HashMap<String, Input>,
) -> HashMap<String, u8> {
    let mut stack = HashSet::from([start]);
    let mut current_travel_time = 0;
    let mut result = HashMap::new();

    while result.len() < destinations.len() - 1 {
        current_travel_time += 1;
        let mut next_stack: HashSet<&String> = HashSet::new();

        for valve in &stack {
            for connection in &configuration.get(&valve.to_string()).unwrap().connections {
                let next_valve = connection.0;
                if next_valve != start
                    && !result.contains_key(next_valve)
                    && destinations.contains(next_valve)
                {
                    result.insert(next_valve.to_string(), current_travel_time);
                }

                next_stack.insert(next_valve);
            }
        }

        stack = next_stack;
    }

    return result;
}

fn read_input() -> HashMap<String, Input> {
    let mut result = HashMap::new();
    let regex =
        Regex::new(r"^Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$").unwrap();

    include_str!("../resources/input.txt")
        .lines()
        .for_each(|line| {
            let capture = regex.captures(line).expect("parse input");
            let flow_reduction: u8 = capture[2].parse().unwrap();

            result.insert(
                capture[1].to_string(),
                Input {
                    flow_rate_reduction: flow_reduction,
                    connections: parse_connections(&capture[3]),
                },
            );
        });

    return result;
}

fn parse_connections(connections: &str) -> HashMap<String, u8> {
    return connections
        .split(", ")
        .map(|c| (c.to_string(), 1))
        .collect();
}
