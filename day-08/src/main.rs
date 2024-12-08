use regex::Regex;

fn main() {
    let file_name = "input";
    let s = std::fs::read_to_string(file_name).unwrap();
    let part1 = part1(&mut read_grid(&s));
    let part2 = part2(&mut read_grid(&s));
    dbg!(part1);
    dbg!(part2);
}

struct Field {
    antinode: bool,
    freq: Option<char>,
}

type Grid = Vec<Vec<Field>>;

fn part1(grid: &mut Grid) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    for y in 0..rows {
        for x in 0..cols {
            let field = &grid[y as usize][x as usize];
            if let Some(freq) = field.freq {
                for y2 in 0..rows {
                    for x2 in 0..cols {
                        if y2 == y && x2 == x {
                            continue;
                        }
                        let other_field = &grid[y2 as usize][x2 as usize];
                        if let Some(other_freq) = other_field.freq {
                            // part 1
                            if freq == other_freq {
                                // Found pair of same frequency
                                let diff_x = x2 - x;
                                let diff_y = y2 - y;
                                // Create new postions for antinodes
                                let new_y = y2 + diff_y;
                                let new_x = x2 + diff_x;
                                if (0..rows).contains(&new_y) && (0..cols).contains(&new_x) {
                                    grid[new_y as usize][new_x as usize].antinode = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    count_antinodes(&grid)
}

fn part2(grid: &mut Grid) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    for y in 0..rows {
        for x in 0..cols {
            let field = &grid[y as usize][x as usize];
            if let Some(freq) = field.freq {
                // Bruteforce for now, we can start after the current element if we want
                for y2 in 0..rows {
                    for x2 in 0..cols {
                        if y2 == y && x2 == x {
                            continue;
                        }
                        let other_field = &grid[y2 as usize][x2 as usize];
                        if let Some(other_freq) = other_field.freq {
                            if freq == other_freq {
                                // Found pair of same values
                                // Go into the direction as often as possible
                                // with length = distance
                                let diff_x = x2 - x;
                                let diff_y = y2 - y;
                                let (mut y3, mut x3) = (y2, x2);
                                loop {
                                    let valid_row = (0..rows).contains(&y3);
                                    let valid_col = (0..cols).contains(&x3);
                                    let outside_of_grid = !valid_row || !valid_col;
                                    if outside_of_grid {
                                        break;
                                    }
                                    grid[y3 as usize][x3 as usize].antinode = true;
                                    y3 += diff_y;
                                    x3 += diff_x;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    count_antinodes(&grid)
}

fn read_grid(s: &str) -> Grid {
    let mut grid = Vec::new();
    let freq_regex = Regex::new("[a-zA-Z0-9]").unwrap();
    for line in s.lines() {
        if line.is_empty() {
            continue;
        }
        let mut row = Vec::new();
        for char in line.chars() {
            if let Some(_captures) = freq_regex.captures(&format!("{}", char)) {
                row.push(Field {
                    antinode: false,
                    freq: Some(char),
                });
            } else if char == '.' {
                row.push(Field {
                    antinode: false,
                    freq: None,
                });
            } else {
                panic!("Invalid char {}", char);
            }
        }
        grid.push(row);
    }
    grid
}

fn count_antinodes(grid: &Grid) -> usize {
    let mut antinodes = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for y in 0..rows as usize {
        for x in 0..cols as usize {
            if grid[y][x].antinode {
                antinodes += 1;
            }
        }
    }
    antinodes
}
