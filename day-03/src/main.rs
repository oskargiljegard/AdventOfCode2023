use regex::Regex;
use std::collections::HashMap;

fn part1() {
    let input = include_str!("input1.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let matrix = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        let y = y as i32;
        let re = Regex::new(r"\d+").unwrap();
        // find index of all matches
        let indices = re.find_iter(line).map(|m| (m.start() as i32, m.end() as i32)).collect::<Vec<_>>();
        for (start, end) in indices.iter() {
            let num = &line[*start as usize..*end as usize];
            let adj_over = (start-1..=*end).map(|x| (x as i32, (y-1) as i32)).collect::<Vec<_>>();
            let adj_under = (start-1..=*end).map(|x| (x as i32, (y+1) as i32)).collect::<Vec<_>>();
            let sides = vec![(start-1, y as i32), (*end, y as i32)];
            let adj = [adj_over, adj_under, sides].concat();
            adj.iter()
                .filter(|(x, y)| x >= &0 && y >= &0 && x < &(line.len() as i32) && y < &(lines.len() as i32))
                .any(|(x, y)| matrix[*y as usize][*x as usize] != '.' && !matrix[*y as usize][*x as usize].is_numeric())
                .then(|| {
                    sum += num.parse::<i32>().unwrap();
                });
        }
    }
    println!("{:?}", sum);
}

fn part2() {
    let input = include_str!("input1.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let matrix = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let star_positions = matrix.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().filter(|(_, c)| **c == '*').map(|(x, _)| (x as i32, y as i32)).collect::<Vec<_>>()
    }).collect::<Vec<_>>().concat();
    let mut star_map: HashMap<(i32, i32), Vec<i32>> = HashMap::from_iter(star_positions.iter().map(|(x, y)| ((*x, *y), vec![])));
    for (y, line) in lines.iter().enumerate() {
        let y = y as i32;
        let re = Regex::new(r"\d+").unwrap();
        // find index of all matches
        let indices = re.find_iter(line).map(|m| (m.start() as i32, m.end() as i32)).collect::<Vec<_>>();
        for (start, end) in indices.iter() {
            let num = &line[*start as usize..*end as usize];
            let adj_over = (start-1..=*end).map(|x| (x as i32, (y-1) as i32)).collect::<Vec<_>>();
            let adj_under = (start-1..=*end).map(|x| (x as i32, (y+1) as i32)).collect::<Vec<_>>();
            let sides = vec![(start-1, y as i32), (*end, y as i32)];
            let adj = [adj_over, adj_under, sides].concat();
            adj.iter()
                .filter(|(x, y)| x >= &0 && y >= &0 && x < &(line.len() as i32) && y < &(lines.len() as i32))
                .for_each(|(x, y)| {
                    if star_map.contains_key(&(*x, *y)) {
                        star_map.get_mut(&(*x, *y)).unwrap().push(num.parse::<i32>().unwrap());
                    }
                });
        }
    }
    let gear_sum: i32 = star_map.iter()
        .filter(|(_, adj_nums)| adj_nums.len() == 2)
        .map(|(_, adj_nums)| adj_nums.iter().fold(1, |acc, x| acc * x))
        .sum();
    println!("{:?}", gear_sum);
}

fn main() {
    part2();
}
