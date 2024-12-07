use std::{collections::HashSet, fs::read_to_string};

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    dir: Dir,
    pos: Vec2,
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Default for Dir {
    fn default() -> Self {
        Dir::Up
    }
}

impl Dir {
    fn vec2(&self) -> Vec2 {
        match &self {
            Self::Left => Vec2 { y: 0, x: -1 },
            Self::Right => Vec2 { y: 0, x: 1 },
            Self::Up => Vec2 { y: -1, x: 0 },
            Self::Down => Vec2 { y: 1, x: 0 },
        }
    }
    fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Self::Right),
            '<' => Some(Self::Left),
            '^' => Some(Self::Up),
            'v' => Some(Self::Down),
            _ => None,
        }
    }
}

fn main() {
    let input_file_name = "input";
    let content = read_to_string(input_file_name).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;
    let mut initial_grid = vec![vec![' '; width as usize]; height as usize];
    let mut initial_state = State::default();

    for (row, line) in lines.iter().copied().enumerate() {
        for (col, val) in line.chars().enumerate() {
            initial_grid[row][col] = val;

            if let Some(dir) = Dir::from_char(val) {
                initial_state.dir = dir;
                initial_state.pos = Vec2 {
                    y: row as i32,
                    x: col as i32,
                }
            }
        }
    }

    let (number_of_steps_until_leaving_map_without_modifying_map, _, initial_path) =
        propagate_and_check_for_infinite_loops(&mut initial_grid, &initial_state);

    dbg!(number_of_steps_until_leaving_map_without_modifying_map);

    // Do propagation and infinite loop detection for every field

    let mut max_possible_infininite_loops = 0usize;
    for Vec2 { x, y } in initial_path {
        // Ignore initial position
        if x == initial_state.pos.x && y == initial_state.pos.y {
            continue;
        } else {
            let mut grid = &mut initial_grid;
            let before = grid[y as usize][x as usize];
            grid[y as usize][x as usize] = '#';
            let (_positions, is_infinite_loop, _) =
                propagate_and_check_for_infinite_loops(&mut grid, &initial_state);
            if is_infinite_loop {
                max_possible_infininite_loops += 1;
            }
            grid[y as usize][x as usize] = before;
        }
    }
    dbg!(max_possible_infininite_loops);
}

/// Returns the steps that the guard makes until either going out of the map
/// or until identifying the exact same state.
fn propagate_and_check_for_infinite_loops(
    initial_grid: &mut Vec<Vec<char>>,
    initial_state: &State,
) -> (usize, bool, HashSet<Vec2>) {
    let mut distinct_positions = HashSet::<Vec2>::new();
    let mut distinct_states = HashSet::<State>::new();
    let mut state = initial_state.clone();
    let height = initial_grid.len() as i32;
    let width = initial_grid[0].len() as i32;
    loop {
        // array[state.pos.y as usize][state.pos.x as usize] = 'X';
        distinct_positions.insert(state.pos.clone());

        if distinct_states.contains(&state) {
            // Infinite loop, stuck!
            return (distinct_positions.len(), true, distinct_positions);
        } else {
            distinct_states.insert(state.clone());
        }

        // print_grid(&array, &state);

        // Move

        let next_x = state.pos.x + state.dir.vec2().x;
        let next_y = state.pos.y + state.dir.vec2().y;

        // dbg!(next_x, next_y);

        let in_grid = (0..width).contains(&next_x) && (0..height).contains(&next_y);

        // dbg!(in_grid);

        if !in_grid {
            break;
        }

        let next_val = initial_grid[next_y as usize][next_x as usize];

        if next_val == '#' {
            // Turn right
            let next_dir = match state.dir {
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
            };
            state.dir = next_dir;
        } else {
            state.pos = Vec2 {
                x: next_x,
                y: next_y,
            };
        }
    }

    // println!("{}", distinct_positions.len());

    return (distinct_positions.len(), false, distinct_positions);
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>, state: &State) {
    for row in grid {
        for col in row {
            print!("{}", col)
        }
        println!();
    }
    println!("{:?}", state);
}
