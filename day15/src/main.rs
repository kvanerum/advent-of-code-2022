use regex::Regex;
use std::cmp::{max, min};

fn main() {
    let input = read_input();

    let impossible_regions = get_impossible_regions_for_row(2000000, &input);
    println!("{}", count_total_positions(&impossible_regions));

    for row in 0..4000001 {
        let mut impossible_regions = get_impossible_regions_for_row(row, &input);
        clamp_regions(&mut impossible_regions);

        if impossible_regions.len() != 1 {
            // found a gap, check if it is not a known beacon
            let beacon = (impossible_regions[0].1 + 1, row);

            if !input.iter().any(|x| x.1 == beacon) {
                println!("{}", beacon.0 as u64 * 4000000 + beacon.1 as u64);
            }
        }
    }
}

fn clamp_regions(regions: &mut Vec<(i32, i32)>) {
    regions.retain(|region| region.0 <= 4000000 && region.1 >= 0);

    for region in regions {
        if region.0 < 0 {
            region.0 = 0;
        }

        if region.1 > 4000000 {
            region.1 = 4000000;
        }
    }
}

fn get_impossible_regions_for_row(
    row: i32,
    scan: &Vec<((i32, i32), (i32, i32))>,
) -> Vec<(i32, i32)> {
    let mut impossible_regions: Vec<(i32, i32)> = Vec::new();

    for scan in scan {
        let max_distance = manhattan_distance(scan.0, scan.1);
        let y_distance = (scan.0 .1 - row).abs();
        let distance_left = max_distance - y_distance;

        if distance_left >= 0 {
            impossible_regions.push((scan.0 .0 - distance_left, scan.0 .0 + distance_left));
        }
    }

    merge_regions(&mut impossible_regions);
    remove_beacon_positions(&mut impossible_regions, scan, row);

    return impossible_regions;
}

fn merge_regions(regions: &mut Vec<(i32, i32)>) {
    let mut did_simplify = true;

    while did_simplify {
        did_simplify = false;
        'outer: for i in 0..regions.len() {
            for j in i + 1..regions.len() {
                if regions[i].0 <= regions[j].1 && regions[i].1 >= regions[j].0 {
                    //extend i, remove j
                    regions[i].0 = min(regions[i].0, regions[j].0);
                    regions[i].1 = max(regions[i].1, regions[j].1);
                    regions.remove(j);
                    did_simplify = true;
                    break 'outer;
                }
            }
        }
    }
}

fn remove_beacon_positions(
    regions: &mut Vec<(i32, i32)>,
    scan: &Vec<((i32, i32), (i32, i32))>,
    row: i32,
) {
    for scan in scan {
        if scan.1 .1 == row {
            let beacon_x = scan.1 .0;

            if let Some(region_index) = regions
                .iter()
                .position(|r| r.0 <= beacon_x && r.1 >= beacon_x)
            {
                let region = regions.remove(region_index);

                // add part to the left
                if region.0 < beacon_x {
                    regions.push((region.0, beacon_x - 1));
                }

                // add part to the right
                if region.1 > beacon_x {
                    regions.push((beacon_x + 1, region.1));
                }
            }
        }
    }
}

fn count_total_positions(regions: &Vec<(i32, i32)>) -> u32 {
    return regions
        .iter()
        .map(|region| (region.1 - region.0) as u32 + 1)
        .sum();
}

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
}

fn read_input() -> Vec<((i32, i32), (i32, i32))> {
    let regex =
        Regex::new(r"^Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)$").unwrap();

    return include_str!("../resources/input.txt")
        .trim()
        .split("\n")
        .map(|line| parse_line(line, &regex))
        .collect();
}

fn parse_line(line: &str, regex: &Regex) -> ((i32, i32), (i32, i32)) {
    let capture = regex.captures(line).expect("parse input");

    return (
        (capture[1].parse().unwrap(), capture[2].parse().unwrap()),
        (capture[3].parse().unwrap(), capture[4].parse().unwrap()),
    );
}
