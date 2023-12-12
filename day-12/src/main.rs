#[derive(Debug)]
struct Record {
    springs: String,
    nums: Vec<i32>,
}

fn is_valid(springs: &str, nums: &[i32]) -> bool {
    springs
        .split(".")
        .filter(|s| !s.is_empty())
        .map(|s| s.len() as i32)
        .eq(nums.iter().copied())
}

fn count_options(record: &Record) -> usize {
    let unknown_positions = record
        .springs
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '?')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut count = 0;
    let num_unknowns = unknown_positions.len();
    let last_int = 2i32.pow(num_unknowns as u32);
    for i in 0..last_int {
        let mut springs = record.springs.chars().collect::<Vec<_>>();
        let mask = format!("{:0>width$b}", i, width = num_unknowns);
        for (j, c) in mask.chars().enumerate() {
            let pos = unknown_positions[j];
            let new_char = match c {
                '0' => '.',
                '1' => '#',
                _ => panic!("Invalid mask"),
            };
            springs[pos] = new_char;
        }
        let valid = is_valid(&springs.iter().collect::<String>(), &record.nums);
        //println!("Springs: {:?} valid: {}", springs, valid);
        if valid {
            count += 1;
        }
    }
    count
}

fn part1() {
    let input = include_str!("input.txt");
    let records = input
        .lines()
        .map(|line| {
            let (springs, nums_str) = line.split_once(" ").unwrap();
            let nums = nums_str
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            Record {
                springs: springs.to_string(),
                nums,
            }
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for record in records {
        let count = count_options(&record);
        sum += count;
    }
    println!("Part 1: {}", sum);
}

fn main() {
    part1();
}
