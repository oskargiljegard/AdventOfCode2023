#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn parse_numbers(line: &str) -> Vec<i64> {
    line.split(" ").skip(1).filter(|s| *s != "").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

fn parse_races(input: &str) -> Vec<Race> {
    let times_str = input.lines().nth(0).expect("No first line");
    let dists_str = input.lines().nth(1).expect("No second line");
    let times = parse_numbers(times_str);
    let dists = parse_numbers(dists_str);
    assert_eq!(times.len(), dists.len());
    let races: Vec<_> = times.into_iter().zip(dists).map(|(time, distance)| Race {time, distance}).collect();
    return races;
}

fn get_num_options(race: &Race) -> i64 {
    (0..race.time).map(|press_time| {
        let move_time = race.time - press_time;
        let distance_moved = move_time * press_time;
        distance_moved
    }).filter(|distance_moved| *distance_moved > race.distance).count().try_into().unwrap()
}

fn part1() {
    let input = include_str!("input1.txt");
    let races = parse_races(input);
    let all_num_options = races.iter().map(|r| get_num_options(r)).collect::<Vec<_>>();
    let output = all_num_options.iter().fold(1, |acc, x| acc*x);
    println!("Part 1: {}", output)
}

fn parse_big_race(input: &str) -> Race {
    let time_str = input.lines().nth(0).expect("No first line");
    let dist_str = input.lines().nth(1).expect("No second line");
    let time = time_str.split_whitespace().skip(1).collect::<String>().parse::<i64>().unwrap();
    let distance = dist_str.split_whitespace().skip(1).collect::<String>().parse::<i64>().unwrap();
    Race { time, distance }
}

fn part2() {
    let input = include_str!("input1.txt");
    let race = parse_big_race(input);
    let output = get_num_options(&race);
    println!("Part 2: {}", output)
}

fn main() {
    part1();
    part2();
}
