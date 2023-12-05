use std::ops::Range;

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

fn apply_map_to_range(map: &Map, range: Range<u64>) -> Vec<Range<u64>> {
    let relevant_sections = map.sections.iter().filter_map(|section| {
        if section.source_start < range.end || section.source_start + section.length > range.start {
            return Some((section.source_start..(section.source_start + section.length),
                        section.destination_start..(section.destination_start + section.length)));
        }
        return None;
    }).collect::<Vec<(Range<u64>, Range<u64>)>>();
    let mut clamped_sections = relevant_sections.iter().map(|(source_range, destination_range)| {
        let start = if source_range.start < range.start {
            range.start
        } else {
            source_range.start
        };
        let end = if source_range.end > range.end {
            range.end
        } else {
            source_range.end
        };
        let start_delta = (start as i128) - (source_range.start as i128);
        let end_delta = (end as i128) - (source_range.start as i128);
        let new_destination_start = (destination_range.start as i128) + start_delta;
        let new_destination_end = (destination_range.start as i128) + end_delta;
        (start..end, (new_destination_start as u64)..(new_destination_end as u64))
    }).collect::<Vec<(Range<u64>, Range<u64>)>>();
    clamped_sections.sort_by_key(|(source_range, _)| source_range.start);
    let mut out_ranges: Vec<(Range<u64>, Range<u64>)> = Vec::new();
    let mut last_end = range.start;
    for (source_range, destination_range) in clamped_sections {
        if source_range.start > last_end {
            out_ranges.push((last_end..source_range.start, last_end..source_range.start));
        }
        out_ranges.push((source_range.clone(), destination_range));
        last_end = source_range.end;
    }
    if range.end > last_end {
        out_ranges.push((last_end..range.end, last_end..range.end));
    }
    out_ranges.into_iter().map(|(src, dst)| dst).collect::<Vec<_>>()
}

fn apply_map_to_ranges(map: &Map, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    ranges.into_iter().flat_map(|range| apply_map_to_range(map, range)).collect::<Vec<_>>()
}

fn apply_maps_to_ranges(maps: &Vec<Map>, start_ranges: &Vec<Range<u64>>) -> Vec<Range<u64>> {
    maps.iter().fold(start_ranges.clone(), |ranges, map| apply_map_to_ranges(map, ranges))
}

fn part2() {
    let input = include_str!("./input-mini.txt");
    let parts = input.split("\n\n").collect::<Vec<_>>();
    if parts.len() == 0 {
        panic!("Input is in the wrong format");
    }
    let seeds = parse_seeds(parts[0]);
    let all_seed_ranges = seeds_to_seed_ranges(&seeds).into_iter().map(|range| vec![range]).collect::<Vec<_>>();
    let maps = parts[1..].iter().map(|s| parse_map(s)).collect::<Vec<_>>();
    //dbg!(&all_seed_ranges);
    //dbg!(apply_map_to_range(&maps[0], 0..100));
    let final_ranges = apply_maps_to_ranges(&maps, &all_seed_ranges[0]);
    dbg!(&final_ranges);
}

fn main() {
    part2();
}
