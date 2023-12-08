use std::{ops::Range, cmp::min};

#[derive(Debug)]
struct MapSection {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Debug)]
struct Map {
    sections: Vec<MapSection>,
}

fn parse_seeds(input: &str) -> Vec<u64> {
    input
        .split(" ")
        .into_iter()
        .skip(1)
        .map(|s| s.parse::<u64>().expect(format!("Failed to parse {}", s).as_str()))
        .collect::<Vec<_>>()
}

fn parse_map(input: &str) -> Map {
    let sections = input.lines().skip(1).map(|line| {
        let parts = line.split(" ").collect::<Vec<_>>();
        let destination_start = parts[0].parse::<u64>().expect(format!("Failed to parse destination {}", parts[0]).as_str());
        let source_start = parts[1].parse::<u64>().expect(format!("Failed to parse source {}", parts[1]).as_str());
        let length = parts[2].parse::<u64>().expect(format!("Failed to parse length {}", parts[2]).as_str());
        MapSection {
            destination_start,
            source_start,
            length,
        }
    }).collect::<Vec<_>>();
    Map {
        sections,
    }
}

fn apply_map(map: &Map, key: u64) -> u64 {
    for section in &map.sections {
        if key >= section.source_start && key < section.source_start + section.length {
            return section.destination_start + (key - section.source_start);
        }
    }
    return key;
}

fn apply_maps(maps: &Vec<Map>, key: u64) -> u64 {
    maps.iter().fold(key, |key, map| apply_map(map, key))
}

fn part1() {
    let input = include_str!("./input1.txt");
    let parts = input.split("\n\n").collect::<Vec<_>>();
    if parts.len() == 0 {
        panic!("Input is in the wrong format");
    }
    let seeds = parse_seeds(parts[0]);
    let maps = parts[1..].iter().map(|s| parse_map(s)).collect::<Vec<_>>();
    let soils = seeds.iter().map(|seed| apply_maps(&maps, *seed)).collect::<Vec<_>>();
    println!("Part 1: {}", soils.iter().min().unwrap());
}

fn seeds_to_seed_ranges(seeds: &Vec<u64>) -> Vec<Range<u64>> {
    let mut ranges = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let seed = seeds[i];
        let length = seeds[i + 1];
        ranges.push(seed..(seed + length));
    }
    ranges
}


fn part2() {
    let input = include_str!("./input1.txt");
    let parts = input.split("\n\n").collect::<Vec<_>>();
    if parts.len() == 0 {
        panic!("Input is in the wrong format");
    }
    let seeds = parse_seeds(parts[0]);
    let all_seed_ranges = seeds_to_seed_ranges(&seeds);
    let maps = parts[1..].iter().map(|s| parse_map(s)).collect::<Vec<_>>();
    let mut lowest_output = u64::MAX;
    let total_num_seeds: u64 = all_seed_ranges.iter().map(|range| &range.end - &range.start).sum();
    let mut num_seeds_tried = 0;
    dbg!(total_num_seeds);
    for range in all_seed_ranges.into_iter() {
        for seed in range {
            let output = apply_maps(&maps, seed);
            lowest_output = min(lowest_output, output);
            num_seeds_tried += 1;
        }
        dbg!(num_seeds_tried);
    }
    println!("Part 2: {}", lowest_output);
}

fn main() {
    part2();
}
