use std::{collections::HashSet, fs::read_to_string};

fn main() {
    // Read inputs
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    let mut line_number = 0;
    for line in read_to_string("input").unwrap().lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        assert!(cols.len() == 2);
        let l: i64 = cols[0].parse().unwrap();
        let r: i64 = cols[1].parse().unwrap();
        assert!(
            l >= 0 && r >= 0,
            "Assumption that values are >= 0 not given, line {}",
            line_number
        );
        left.push(l);
        right.push(r);
        line_number += 1;
    }

    // Day 1 Part 1
    left.sort();
    right.sort();
    let total_difference: u64 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| return i64::abs_diff(*l, *r))
        .sum();
    dbg!(total_difference);

    // Day 1 Part 2
    let left_set = HashSet::<i64>::from_iter(left.iter().cloned());
    let similarity_score: i64 = left_set
        .iter()
        .map(|current_left| {
            // Inefficient but should be ok for the input size.
            // Improvement: moving cursor as we know that right vector is already sorted.
            let number_of_ocurrences = right
                .iter()
                .filter(|current_right| *current_right == current_left)
                .count();
            number_of_ocurrences as i64 * current_left
        })
        .sum();

    dbg!(similarity_score);
}
