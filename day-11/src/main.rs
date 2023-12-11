fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![vec!['.'; grid.len()]; grid[0].len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            new_grid[j][i] = c;
        }
    }
    new_grid
}

fn get_empty_rows(grid: &Vec<Vec<char>>) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect()
}

fn pretty_print(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn expand_grid_positions(grid: &Vec<Vec<char>>, empty_rows: &Vec<usize>, empty_cols: &Vec<usize>, expandion: usize) -> Vec<(usize, usize)> {
    let galaxy_positions: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(move |(x, _)| (x, y))
        }).collect();
    let expanded_galaxy_positions = galaxy_positions
        .iter()
        .map(|&(x, y)| {
            let new_x = x + empty_cols.iter().filter(|&&col| col < x).count() * (expandion - 1);
            let new_y = y + empty_rows.iter().filter(|&&row| row < y).count() * (expandion - 1);
            (new_x, new_y)
        }).collect::<Vec<_>>();
    expanded_galaxy_positions
}

fn find_all_distances(positions: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut distances: Vec<usize> = vec![];
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let (x1, y1) = positions[i];
            let (x2, y2) = positions[j];
            let dx = (x1 as isize - x2 as isize).abs() as usize;
            let dy = (y1 as isize - y2 as isize).abs() as usize;
            distances.push(dx + dy);
        }
    }
    distances
}

fn part1() {
    let input = include_str!("input1.txt");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let empty_rows = get_empty_rows(&grid);
    let empty_cols = get_empty_rows(&transpose(&grid));
    let expanded_grid_positions = expand_grid_positions(&grid, &empty_rows, &empty_cols, 2);
    let distances = find_all_distances(&expanded_grid_positions);
    println!("Part 1: {:?}", distances.iter().sum::<usize>());
}

fn part2() {
    let input = include_str!("input1.txt");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let empty_rows = get_empty_rows(&grid);
    let empty_cols = get_empty_rows(&transpose(&grid));
    let expanded_grid_positions = expand_grid_positions(&grid, &empty_rows, &empty_cols, 1000000);
    let distances = find_all_distances(&expanded_grid_positions);
    println!("Part 2: {:?}", distances.iter().sum::<usize>());
}

fn main() {
    part1();
    part2();
}
