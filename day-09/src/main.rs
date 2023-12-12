fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn extrapolate(numbers: &[i32]) -> (i32, i32) {
    let mut rows: Vec<Vec<i32>> = vec![];
    let mut current_row: Vec<i32> = numbers.to_vec();
    while current_row.iter().any(|&num| num != 0) {
        let mut next_row = vec![];
        for i in 0..current_row.len() - 1 {
            next_row.push(current_row[i + 1] - current_row[i]);
        }
        rows.push(current_row.clone());
        current_row = next_row;
    }
    let mut right_num = 0;
    for i in (0..rows.len()).rev() {
        let row = &rows[i];
        right_num = row.last().unwrap() + right_num;
    }
    let mut left_num = 0;
    for i in (0..rows.len()).rev() {
        let row = &rows[i];
        left_num = row.first().unwrap() - left_num;
    }
    (left_num, right_num)
}

fn main() {
    let input = include_str!("input1.txt");
    let numbers = parse_input(input);
    let exs = numbers
        .iter()
        .map(|row| extrapolate(row))
        .collect::<Vec<_>>();
    let right_sum = exs.iter().map(|&(_, right)| right).sum::<i32>();
    let left_sum = exs.iter().map(|&(left, _)| left).sum::<i32>();
    println!("Part 1: {}", right_sum);
    println!("Part 2: {}", left_sum);
}
