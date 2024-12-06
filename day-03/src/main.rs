use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let input_file_name = "input";
    let mut sum = 0;
    let mut enabled = true;
    for line in read_to_string(input_file_name).unwrap().lines() {
        let mut last_end: usize = 0;
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        for capture in re.captures_iter(line) {
            let capture_start = capture.get(0).unwrap().start();
            let in_between = &line[last_end..capture_start];
            if in_between.contains("do()") {
                enabled = true;
            }
            if in_between.contains("don't()") {
                enabled = false;
            }
            if enabled {
                let mul1: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
                let mul2: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
                sum += mul1 * mul2;
            }
            last_end = capture.get(0).unwrap().end();
        }
    }
    dbg!(sum);
}
