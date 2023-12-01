use regex::Regex;

fn main() {
    part2();
}

fn part1() {
    let x = include_str!("./input1.txt");
    let sum = x.lines()
        .map(|line| {
            let re = Regex::new(r"[A-Za-z]").unwrap();
            re.replace_all(line, "")
        })
        .map(|line| vec![line.chars().nth(0).unwrap(), line.chars().last().unwrap()].into_iter().collect::<String>())
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>();
    println!("{}", sum);
}

fn find_first_num(line: String, mappings: Vec<(String, i32)>) -> char {

    let dig_index = *line.find(char::is_numeric).get_or_insert(9999);
    let binding = mappings;
    let num_indices: Vec<_> = binding.iter()
        .map(|(word, num)| {
            (*num, *line.find(word).get_or_insert(99999))
        })
        .collect();
    let min_num_index = num_indices.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    if min_num_index.1 < dig_index.clone() {
        return min_num_index.0.to_string().chars().nth(0).unwrap();
    }
    return line[dig_index..dig_index+1].chars().nth(0).unwrap();
}

fn part2() {
    let x = include_str!("./input1.txt");
    let mappings = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].iter().map(|(word, num)| (word.to_string(), *num)).collect::<Vec<_>>();
    let mappings_reversed 
        = mappings.iter().map(
            |(word, num)| 
            (word.chars().rev().collect::<String>(), *num)
        ).collect::<Vec<_>>();

    let sum = x.lines()
        .map(|line| {
            let s = line.to_string();
            let s_rev = s.chars().rev().collect::<String>();
            let first_num = find_first_num(s, mappings.clone());
            let last_num = find_first_num(s_rev, mappings_reversed.clone());
            vec![first_num, last_num].into_iter().collect::<String>().parse::<i32>().unwrap()
        })
        .sum::<i32>();
    println!("{}", sum);
}