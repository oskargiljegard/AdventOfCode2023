use std::vec;


fn part1() {
    let input = include_str!("./input1.txt");
    let mut sum = 0;
    for line in input.lines() {
        let colon_pos = line.find(':').expect("No colon found");
        let num_part = &line[(colon_pos+2)..].split(" | ").collect::<Vec<&str>>();
        let winning = num_part[0]
            .split(" ")
            .filter(|s| *s != "")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let owned = num_part[1]
            .split(" ")
            .filter(|s| *s != "")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let num_matches = owned.iter().filter(|&x| winning.contains(x)).count();
        let points = match num_matches {
            0 => 0,
            n => 2i32.pow((n-1) as u32),
        };
        sum += points;
    }
    dbg!(sum);
}

fn part2() {
    let input = include_str!("./input1.txt");
    let num_games = input.lines().count();
    let mut matches: Vec<i32> = vec![0; num_games];
    for (game_index, line) in input.lines().enumerate() {
        let colon_pos = line.find(':').expect("No colon found");
        let num_part = &line[(colon_pos+2)..].split(" | ").collect::<Vec<&str>>();
        let winning = num_part[0]
            .split(" ")
            .filter(|s| *s != "")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let owned = num_part[1]
            .split(" ")
            .filter(|s| *s != "")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let num_matches = owned.iter().filter(|&x| winning.contains(x)).count();
        matches[game_index] = num_matches as i32;
    }
    let mut instances = vec![1; num_games];
    for (game_index, num_matches) in matches.iter().enumerate() {
        let current_instances = instances[game_index];
        for game_index_delta in 1..=*num_matches {
            let new_index = game_index + game_index_delta as usize;
            if new_index >= num_games {
                break;
            }
            instances[new_index] += current_instances;
        }
    }
    let num_copies = instances.iter().sum::<i32>();
    dbg!(num_copies);
}

fn main() {
    part1();
    part2();
}
