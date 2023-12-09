use std::{collections::HashMap, io::empty};

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

fn find_path_length<F>(instr_str: &str, map: HashMap<&str, (&str, &str)>, start_state: &str, is_end_state: F) -> (usize, String)
                        where F: Fn(&str) -> bool {
    let num_instructions = instr_str.chars().count();
    let mut state = start_state;
    let mut i = 0;
    while !is_end_state(state) || i == 0 {
        let action = instr_str.chars().nth(i % num_instructions).unwrap();
        let next = map.get(state).unwrap();
        state = match action {
            'L' => next.0,
            'R' => next.1,
            _ => panic!("Action was not L or R"),
        };
        i += 1;
    }
    return (i, state.to_string());
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn part1() {
    let input = include_str!("input1.txt");
    let (instr_str, map) = parse(input);
    let (num_steps, _) = find_path_length(instr_str, map, "AAA", |s| s=="ZZZ");
    println!("Part 1: {}", num_steps);
}

fn part2() {
    let input = include_str!("input1.txt");
    let (instr_str, map) = parse(input);
    let start_states = map.keys().filter(|k| k.ends_with("A")).collect::<Vec<_>>();
    let mut all_steps: Vec<i64> = vec![];
    for start_state in start_states {
        let (num_steps, final_state) = find_path_length(instr_str, map.clone(), start_state, |s| s.ends_with("Z"));
        let (period, final_state2) = find_path_length(instr_str, map.clone(), &final_state, |s| s.ends_with("Z"));
        assert!(final_state == final_state2);
        println!("Start state: {} Final state: {} Steps: {} Period: {}", start_state, final_state, num_steps, period);
        all_steps.push(num_steps as i64);
    }
    let lcm = all_steps.iter().fold(1, |acc, x| lcm(acc, *x as u64));
    println!("Part 2: {}", lcm);
}


fn main() {
    part1();
    part2();
}
