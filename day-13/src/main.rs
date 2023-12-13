fn pretty_print(grid: &Vec<Vec<char>>) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![vec!['.'; grid.len()]; grid[0].len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            new_grid[j][i] = *col;
        }
    }
    new_grid
}

fn find_reflection_row(grid: &Vec<Vec<char>>) -> Option<i32> {
    for i in 0..grid.len() - 1 {
        let rows_before = &grid[0..i + 1];
        let rows_after = &grid[i + 1..];
        let min_len = std::cmp::min(rows_before.len(), rows_after.len());
        let up = &rows_before
            .iter()
            .rev()
            .take(min_len)
            .collect::<Vec<&Vec<char>>>();
        let down = &rows_after.iter().take(min_len).collect::<Vec<&Vec<char>>>();
        let is_equal = up.iter().zip(down.iter()).all(|(u, d)| u == d);
        if is_equal {
            return Some((i+1) as i32);
        }
    }
    None
}

fn part1() {
    let input = include_str!("input.txt");
    let grids: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .map(|grid_str| {
            grid_str
                .lines()
                .map(|line| line.chars().collect())
                .collect()
        })
        .collect();
    let score = grids
        .iter()
        .map(|grid| {
            let row = find_reflection_row(grid);
            let col = find_reflection_row(&transpose(grid));
            if (row.is_some() && col.is_some()) || (row.is_none() && col.is_none()) {
                panic!("Invalid grid");
            }
            if let Some(row) = find_reflection_row(grid) {
                return 100 * row;
            }
            if let Some(col) = find_reflection_row(&transpose(grid)) {
                return col;
            }
            panic!("No reflection found");
        })
        .sum::<i32>();
    println!("Part 1: {}", score);
}

fn main() {
    part1();
}
