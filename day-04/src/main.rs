use std::vec;

fn parse_numbers(number_line: &str) -> Vec<i32> {
    number_line
        .split(" ")
        .filter(|s| *s != "")
        .map(|s| s.parse::<i32>().expect(format!("Failed to parse {}", s).as_str()))
        .collect::<Vec<_>>()
}

fn find_matches(input: &str) -> Vec<i32> {
    input.lines().enumerate().map(|(_, line)| {
        let colon_pos = line.find(':').expect("No colon found");
        let (winning_str, owned_str) = &line[(colon_pos+2)..].split_once(" | ").expect("No pipe found");
        let winning = parse_numbers(winning_str);
        let owned = parse_numbers(owned_str);
        let num_matches = owned.iter().filter(|&x| winning.contains(x)).count();
        num_matches as i32
    }).collect::<Vec<_>>()
}

fn to_score(num_matches: i32) -> i32 {
    match num_matches {
        0 => 0,
        n => 2i32.pow((n-1) as u32),
    }
}

fn part1() {
    let input = include_str!("./input1.txt");
    let sum = find_matches(input).into_iter().map(to_score).sum::<i32>();
    println!("Part 1: {}", sum);
}

fn part2() {
    let input = include_str!("./input1.txt");
    let matches = find_matches(input);
    let num_games = input.lines().count();
    let mut instances = vec![1; num_games];
    for (game_index, num_matches) in matches.into_iter().enumerate() {
        let current_instances = instances[game_index];
        for game_index_delta in 1..=num_matches {
            let new_index = game_index + game_index_delta as usize;
            if new_index >= num_games {
                break;
            }
            instances[new_index] += current_instances;
        }
    }
    let num_copies = instances.iter().sum::<i32>();
    println!("Part 2: {}", num_copies);
}

fn main() {
    part1();
    part2();
}
