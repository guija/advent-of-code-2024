use std::collections::{HashMap, HashSet};

type Grid = Vec<Vec<char>>;
type Vec2 = (i32, i32);

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let mut regions = initial_regions(&grid);
    merge(&mut regions, &grid);
    let regions: Vec<HashSet<Vec2>> = regions.values().cloned().collect();
    let result1 = part1(&regions);
    dbg!(result1);
    assert_eq!(1363682, result1);
    let result2 = part2(&regions, &grid);
    dbg!(result2);
    assert_eq!(787680, result2);
    return;
}

fn part1(regions: &Vec<HashSet<Vec2>>) -> usize {
    let mut sum = 0;
    for region in regions {
        let mut perimeter = 0;
        let area = region.len();
        for pos in region.iter() {
            for dir in Dir::all() {
                if !region.contains(&dir.neighbour(*pos)) {
                    perimeter += 1;
                }
            }
        }
        sum += perimeter * area;
    }
    sum
}

fn part2(regions: &Vec<HashSet<Vec2>>, grid: &Grid) -> usize {
    let mut sum = 0;
    for region in regions {
        let mut fences = HashSet::<(Vec2, Dir)>::new();
        for pos in region.iter() {
            for dir in Dir::all() {
                let n = dir.neighbour(*pos);
                let same_region = region.contains(&n);
                if !same_region {
                    fences.insert((*pos, dir));
                }
            }
        }
        let mut sides = 0;
        sides += count_fences_lines_in_same_dir(&fences, grid.len(), Dir::Up, |(y, x)| (y, x));
        sides += count_fences_lines_in_same_dir(&fences, grid.len(), Dir::Down, |(y, x)| (y, x));
        sides += count_fences_lines_in_same_dir(&fences, grid.len(), Dir::Left, |(y, x)| (x, y));
        sides += count_fences_lines_in_same_dir(&fences, grid.len(), Dir::Right, |(y, x)| (x, y));
        sum += sides * region.len();
    }
    sum
}

fn count_fences_lines_in_same_dir(
    fences: &HashSet<(Vec2, Dir)>,
    grid_size: usize,
    dir: Dir,
    order: fn((i32, i32)) -> (i32, i32),
) -> usize {
    let mut sides = 0;
    for i in 0..grid_size as i32 {
        let mut side = false;
        for j in 0..grid_size as i32 {
            let x = (order((i, j)), dir);
            if fences.contains(&x) {
                if !side {
                    sides += 1;
                }
                side = true;
            } else {
                side = false;
            }
        }
    }
    sides
}

fn initial_regions(grid: &Grid) -> HashMap<Vec2, HashSet<Vec2>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut regions: HashMap<Vec2, HashSet<Vec2>> = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let value = grid[y][x];
            let mut assigned = false;
            for neighbour in neighbours(&grid, (y as i32, x as i32)) {
                let neighbour_value = grid[neighbour.0 as usize][neighbour.1 as usize];
                for region in regions.iter_mut() {
                    if region.1.contains(&neighbour) && neighbour_value == value {
                        region.1.insert((y as i32, x as i32));
                        assigned = true;
                    }
                }
            }
            if !assigned {
                let mut new_group = HashSet::new();
                new_group.insert((y as i32, x as i32));
                regions.insert((y as i32, x as i32), new_group);
            }
        }
    }
    regions
}

fn merge(regions: &mut HashMap<Vec2, HashSet<Vec2>>, grid: &Grid) {
    loop {
        let mut merged = false;
        // For each group check whether there is another group with the same position
        let keys: Vec<Vec2> = regions.keys().cloned().collect();
        for key in &keys {
            for key2 in &keys {
                if key == key2 {
                    continue;
                }
                if let (Some(elements), Some(elements2)) = (regions.get(&key), regions.get(&key2)) {
                    let first = elements.iter().last().unwrap();
                    let second = elements2.iter().last().unwrap();
                    let same = grid[first.0 as usize][first.1 as usize]
                        == grid[second.0 as usize][second.1 as usize];
                    let overlap = elements.intersection(&elements2).count() > 0 && same;
                    if overlap {
                        merged = true;
                        let other_values: Vec<Vec2> =
                            regions.get(&key2).unwrap().iter().cloned().collect();
                        regions.get_mut(&key).unwrap().extend(other_values);
                        regions.remove(&key2);
                    }
                }
            }
        }
        if !merged {
            break;
        }
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
            Self::Left => (0, -1),
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
        }
    }
    fn neighbour(&self, vec: Vec2) -> Vec2 {
        (vec.0 + self.to_vec().0, vec.1 + self.to_vec().1)
    }
    fn all() -> [Dir; 4] {
        [Self::Left, Self::Right, Self::Up, Self::Down]
    }
}

fn neighbours(grid: &Grid, pos: Vec2) -> Vec<Vec2> {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    return Dir::all()
        .iter()
        .filter_map(|dir| {
            let n = dir.neighbour(pos);
            if n.1 < 0 || n.1 >= width || n.0 < 0 || n.0 >= height {
                None
            } else {
                Some(n)
            }
        })
        .collect();
}
