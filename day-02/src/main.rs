#[derive(Debug)]
struct GameSet {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

fn parse_set(input: &str) -> GameSet {
    let mut game_set = GameSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    for part in input.split(", ") {
        let (num, color) = part.split_at(part.find(' ').unwrap());
        match color {
            " red" => game_set.red = num.parse::<i32>().unwrap(),
            " green" => game_set.green = num.parse::<i32>().unwrap(),
            " blue" => game_set.blue = num.parse::<i32>().unwrap(),
            _ => panic!("Unknown color: {}", color),
        }
    }
    game_set
}

fn game_is_legal(game_set: &GameSet) -> bool {
    game_set.red <= 12 && game_set.green <= 13 && game_set.blue <= 14
}

fn part1() {
    let input = include_str!("input1.txt");
    let mut legal_idx_sum = 0;
    for line in input.lines() {
        let colon_pos = line.find(':').unwrap();
        let header = &line[0..colon_pos];
        let rest = &line[colon_pos+2..];
        let idx = header.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i32>().unwrap();
        let set_strings = rest.split("; ").collect::<Vec<&str>>();
        let game_sets = set_strings.iter().map(|s| parse_set(s)).collect::<Vec<GameSet>>();
        if game_sets.iter().all(|s| game_is_legal(s)) {
            legal_idx_sum += idx;
        }
    }
    println!("{:?}", legal_idx_sum);
}

fn min_possible_cubes(game_sets: &Vec<GameSet>) -> GameSet {
    let mut min_set = GameSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    for game_set in game_sets {
        if game_set.red > min_set.red {
            min_set.red = game_set.red;
        }
        if game_set.green > min_set.green {
            min_set.green = game_set.green;
        }
        if game_set.blue > min_set.blue {
            min_set.blue = game_set.blue;
        }
    }
    min_set
}

fn part2() {
    let input = include_str!("input1.txt");
    let mut cube_sum = 0;
    for line in input.lines() {
        let colon_pos = line.find(':').unwrap();
        let header = &line[0..colon_pos];
        let rest = &line[colon_pos+2..];
        let idx = header.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i32>().unwrap();
        let set_strings = rest.split("; ").collect::<Vec<&str>>();
        let game_sets = set_strings.iter().map(|s| parse_set(s)).collect::<Vec<GameSet>>();
        let min_set = min_possible_cubes(&game_sets);
        cube_sum += min_set.red * min_set.green * min_set.blue;
    }
    println!("{:?}", cube_sum);
}


fn main() {
    part2();
}
