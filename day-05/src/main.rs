use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

fn main() {
    let input_file_name = "input";
    let mut part1 = 0 as i32;
    let mut part2 = 0 as i32;
    let mut rules: Vec<(i32, i32)> = Vec::new();

    for line in read_to_string(input_file_name).unwrap().lines() {
        if line.contains("|") {
            let (x, y) = line.trim().split_once("|").unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            rules.push((x, y));
        }

        if line.contains(",") {
            let pages: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();
            let pages_indexes: HashMap<i32, usize> = pages
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, v)| (v, i))
                .collect();

            // Check if is ordered
            let mut is_ordered = true;
            for (x, y) in &rules {
                if pages_indexes.contains_key(&x) && pages_indexes.contains_key(&y) {
                    let x_index = pages_indexes.get(&x);
                    let y_index = pages_indexes.get(&y);
                    if x_index > y_index {
                        is_ordered = false;
                        break;
                    }
                }
            }

            if is_ordered {
                let center_index = pages.len() / 2;
                let center_value = pages[center_index];
                part1 += center_value;
            } else {
                // order them
                let mut pages_sorted = pages.clone();
                pages_sorted.sort_by(|a, b| {
                    // Find rule for both
                    if rules.contains(&(*a, *b)) {
                        Ordering::Less
                    } else if rules.contains(&(*b, *a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                let center_index = pages_sorted.len() / 2;
                let center_value = pages_sorted[center_index];
                part2 += center_value;
            }
        }
    }
    dbg!(part1);
    dbg!(part2);
}
