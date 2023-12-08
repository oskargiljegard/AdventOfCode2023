use std::collections::HashMap;

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (instr_str, nodes_str) = input.split_once("\n\n").unwrap();
    let map: HashMap<&str, (&str, &str)> = nodes_str.lines().map(|l| {
        let (curr, next) = l.split_once(" = ").unwrap();
        let (left, right) = next.split_once(", ").unwrap();
        let tuples = (curr, (&left[1..], &right[..right.len()-1]));
        tuples
    }).collect();
    return (instr_str, map);
}

fn find_path_length<F>(instr_str: &str, map: HashMap<&str, (&str, &str)>, start_state: &str, is_end_state: F) -> usize 
                        where F: Fn(&str) -> bool {
    let num_instructions = instr_str.chars().count();
    let mut state = start_state;
    let mut i = 0;
    while !is_end_state(state) {
        let action = instr_str.chars().nth(i % num_instructions).unwrap();
        let next = map.get(state).unwrap();
        state = match action {
            'L' => next.0,
            'R' => next.1,
            _ => panic!("Action was not L or R"),
        };
        i += 1;
    }
    return i;
}

fn part1() {
    let input = include_str!("input1.txt");
    let (instr_str, map) = parse(input);
    let num_steps = find_path_length(instr_str, map, "AAA", |s| s=="ZZZ");
    println!("Part 1: {}", num_steps);
}

fn part2() {
    let input = include_str!("input1.txt");
    let (instr_str, map) = parse(input);
    let num_steps = find_path_length(instr_str, map, "AAA", |s| s=="ZZZ");
    println!("Part 1: {}", num_steps);
}


fn main() {
    part1();
}
