use core::panic;

fn main() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    let input = std::fs::read_to_string("input").unwrap();
    let grid_chars = input.trim().split("\n\n").collect::<Vec<&str>>()[0];
    let moves = input.split("\n\n").collect::<Vec<&str>>()[1];
    let moves = moves.replace("\n", "");
    let moves = moves.trim();
    let size = grid_chars.split_whitespace().collect::<Vec<&str>>()[0].len();
    dbg!(moves);
    let mut grid = Vec::<Vec<char>>::new();
    let mut robot = Vec2::default();
    let mut y = 0;
    for row in grid_chars.lines() {
        let mut row_vec = Vec::new();
        let mut x = 0;
        for col in row.chars() {
            row_vec.push(col);
            if col == '@' {
                robot = Vec2::new(x, y);
            }
            x += 1;
        }
        grid.push(row_vec);
        y += 1;
    }

    dbg!(size);
    for move_dir in moves.chars().map(|c| Dir::from_char(c)) {
        dbg!(robot, move_dir);
        if let Some(moves) = check(&grid, robot, move_dir, Vec::new()) {
            for (from, to) in moves {
                grid[from.y as usize][from.x as usize] = '.';
                grid[to.y as usize][to.x as usize] = 'O';
            }
            let from = robot;
            let to = robot + move_dir.to_vec();
            grid[from.y as usize][from.x as usize] = '.';
            grid[to.y as usize][to.x as usize] = '@';
            robot = to;
        } else {
            // println!("\tNo move!")
        }
        // print_grid(&grid);
    }
    dbg!(gps(&grid));
}
fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
    println!();
}

fn gps(grid: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x] == 'O' {
                sum += 100 * y + x;
            }
        }
    }
    return sum;
}

/// Empty vec --> No movement possible
/// non empty vec, all that elements can be moved
fn check(
    grid: &Vec<Vec<char>>,
    pos: Vec2,
    dir: Dir,
    to_be_moved: Vec<(Vec2, Vec2)>,
) -> Option<Vec<(Vec2, Vec2)>> {
    let predicted = pos + dir.to_vec();
    let predicted_char = grid[predicted.y as usize][predicted.x as usize];
    if predicted_char == '#' {
        return None;
    } else if predicted_char == 'O' {
        let r = check(grid, predicted, dir, to_be_moved);
        if let Some(x) = r {
            // Append This to next
            let possible_move = (pos, predicted);
            let mut y = x.clone();
            y.push(possible_move);
            return Some(y);
        } else {
            return None;
        }
    } else {
        let mut x = Vec::new();
        let possible_move = (pos, predicted);
        x.push(possible_move);
        return Some(x);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Vec2 {
    fn new(x: i64, y: i64) -> Vec2 {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn to_vec(&self) -> Vec2 {
        match self {
            Self::Left => Vec2::new(-1, 0),
            Self::Up => Vec2::new(0, -1),
            Self::Right => Vec2::new(1, 0),
            Self::Down => Vec2::new(0, 1),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            _ => panic!("Invalid dir '{}'", c),
        }
    }

    fn all() -> [Dir; 4] {
        [Self::Left, Self::Right, Self::Up, Self::Down]
    }
}
