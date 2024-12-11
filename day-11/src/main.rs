use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let nums: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("Part1 = {}", solve(&nums, 25));
    println!("Part2 = {}", solve(&nums, 75));
}

fn solve(nums: &Vec<i64>, iterations: usize) -> i64 {
    let mut cache = HashMap::<(i64, usize), i64>::new();
    return nums.iter().map(|n| dfs(*n, iterations, &mut cache)).sum();
}

fn dfs(x: i64, depth: usize, cache: &mut HashMap<(i64, usize), i64>) -> i64 {
    if let Some(r) = cache.get(&(x, depth)) {
        return *r;
    } else if depth == 0 {
        return 1;
    }
    let result = if x == 0 {
        dfs(1, depth - 1, cache)
    } else {
        let s = x.to_string();
        if s.len() % 2 == 0 {
            let half_length = s.len() / 2;
            let left = &s[0..half_length].parse::<i64>().unwrap();
            let right = &s[half_length..].parse::<i64>().unwrap();
            dfs(*left, depth - 1, cache) + dfs(*right, depth - 1, cache)
        } else {
            dfs(x * 2024, depth - 1, cache)
        }
    };
    cache.insert((x, depth), result);
    return result;
}
