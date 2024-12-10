use std::collections::HashMap;

struct Grid {
    grid: Vec<Vec<i32>>,
}

// TODOs
// impl iter for grid to go over all positions
// overload plus for vectors
// Make grid generic

impl Grid {
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        if self.height() > 0 {
            self.grid[0].len()
        } else {
            0
        }
    }

    fn in_grid(&self, y: i32, x: i32) -> bool {
        y >= 0 && y < self.height() as i32 && x >= 0 && x < self.width() as i32
    }

    fn get_checked(&self, y: i32, x: i32) -> Option<i32> {
        if !self.in_grid(y, x) {
            None
        } else {
            Some(self.grid[y as usize][x as usize])
        }
    }
    fn get(&self, y: usize, x: usize) -> i32 {
        self.grid[y as usize][x as usize]
    }

    fn from_str(s: &str) -> Grid {
        let mut grid = Vec::new();
        for row in s.lines() {
            let mut row_elements = Vec::new();
            for col in row.chars() {
                let digit = col.to_string().parse::<i32>().unwrap();
                row_elements.push(digit);
            }
            grid.push(row_elements);
        }
        Grid { grid }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn get_vec(&self) -> Vec2 {
        match &self {
            Self::Left => Vec2 { y: 0, x: -1 },
            Self::Right => Vec2 { y: 0, x: 1 },
            Self::Up => Vec2 { y: -1, x: 0 },
            Self::Down => Vec2 { y: 1, x: 0 },
        }
    }
}

fn main() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    let input = std::fs::read_to_string("input").unwrap();
    let grid = Grid::from_str(input.as_str());

    // let grid = Grid::from_str(input);

    // For each starting point

    // Impl an iterator for this
    let mut starting_positions = Vec::<Vec2>::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let current = grid.get(y, x);
            if current == 0 {
                starting_positions.push(Vec2 {
                    y: y as i32,
                    x: x as i32,
                })
            }
        }
    }

    let mut part1 = 0;
    for starting_pos in starting_positions {
        // dbg!(starting_pos);
        let mut path = Vec::new();
        let mut reaching = HashMap::<Vec2, Vec<Vec<Vec2>>>::new();
        find_path(&grid, starting_pos, 1, &mut path, &mut reaching);
        // dbg!(reaching.len());
        for (_, reaching) in reaching.iter() {
            part1 += reaching.len();
        }
    }
    dbg!(part1);
}

// We can remember what we already visited
fn find_path(
    grid: &Grid,
    start: Vec2,
    allowed_diff: i32,
    path: &mut Vec<Vec2>,
    reaching: &mut HashMap<Vec2, Vec<Vec<Vec2>>>,
) {
    path.push(start);
    let dirs = [Dir::Left, Dir::Right, Dir::Up, Dir::Down];
    let current_val = grid.get(start.y as usize, start.x as usize);
    if current_val == 9 {
        if let Some(paths) = reaching.get_mut(&start) {
            paths.push(path.clone());
        } else {
            reaching.insert(start, vec![path.clone()]);
        }
    }
    for dir in dirs {
        let dir = dir.get_vec();
        let new_pos = Vec2 {
            y: start.y + dir.y,
            x: start.x + dir.x,
        };
        if let Some(value) = grid.get_checked(new_pos.y, new_pos.x) {
            if current_val + allowed_diff == value {
                // also go in there!
                let mut path = path.clone();
                find_path(&grid, new_pos, allowed_diff, &mut path, reaching);
            }
        }
    }
}
