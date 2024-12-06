use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let input_file_name = "input";
    let mut entire_text = String::new();
    let mut width = 0 as usize;
    let mut sum = 0 as usize;
    for line in read_to_string(input_file_name).unwrap().lines() {
        let line = line.trim();
        width = line.len();
        entire_text.push_str(line)
    }

    dbg!(width);

    // horizontal fwd
    sum += find_and_count(r"XMAS", &entire_text, width, 1);

    // horizontal bwd
    sum += find_and_count(r"SAMX", &entire_text, width, 1);

    // vertical down
    sum += find_and_count(
        format!("X.{{{}}}M.{{{}}}A.{{{}}}S", width - 1, width - 1, width - 1).as_str(),
        &entire_text,
        width,
        0,
    );
    // horizontal up
    sum += find_and_count(
        format!("S.{{{}}}A.{{{}}}M.{{{}}}X", width - 1, width - 1, width - 1).as_str(),
        &entire_text,
        width,
        0,
    );

    // diagonal tl -> br
    sum += find_and_count(
        format!("X.{{{}}}M.{{{}}}A.{{{}}}S", width, width, width).as_str(),
        &entire_text,
        width,
        1,
    );

    // diagonal tr -> bl
    sum += find_and_count(
        format!("X.{{{}}}M.{{{}}}A.{{{}}}S", width - 2, width - 2, width - 2).as_str(),
        &entire_text,
        width,
        -1,
    );

    // diagonal br -> tl
    sum += find_and_count(
        format!("S.{{{}}}A.{{{}}}M.{{{}}}X", width, width, width).as_str(),
        &entire_text,
        width,
        1,
    );

    // diagonal bl -> tr
    sum += find_and_count(
        format!("S.{{{}}}A.{{{}}}M.{{{}}}X", width - 2, width - 2, width - 2).as_str(),
        &entire_text,
        width,
        -1,
    );

    let mut sum2 = 0 as usize;

    sum2 += find_and_count(
        format!("M.M.{{{}}}.A..{{{}}}S.S", width - 3, width - 3).as_str(),
        &entire_text,
        width,
        1,
    );
    sum2 += find_and_count(
        format!("S.S.{{{}}}.A..{{{}}}M.M", width - 3, width - 3).as_str(),
        &entire_text,
        width,
        1,
    );
    sum2 += find_and_count(
        format!("S.M.{{{}}}.A..{{{}}}S.M", width - 3, width - 3).as_str(),
        &entire_text,
        width,
        1,
    );
    sum2 += find_and_count(
        format!("M.S.{{{}}}.A..{{{}}}M.S", width - 3, width - 3).as_str(),
        &entire_text,
        width,
        1,
    );

    dbg!(sum);
    dbg!(sum2);
}

fn find_and_count(needle: &str, haystack: &str, width: usize, end_to_start_cmp: i32) -> usize {
    dbg!(needle);

    let mut start = 0 as usize;
    let mut found = 0 as usize;

    let regex = Regex::new(needle).unwrap();
    while start < haystack.len() {
        let result = regex.find_at(&haystack, start);
        if let Some(result) = result {
            let rstart = (result.start() % width) as i32;
            let rend = ((result.end() - 1) % width) as i32;
            let diff = rend - rstart;
            if diff == 0 && end_to_start_cmp == 0
                || diff < 0 && end_to_start_cmp < 0
                || diff > 0 && end_to_start_cmp > 0
            {
                found += 1;
            }
            start = result.start() + 1;
        } else {
            start += 1;
        }
    }

    return found;
}
