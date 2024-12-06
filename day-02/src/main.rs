use std::fs::read_to_string;

// Too complicated, but I'm too slow. Differing between increasing and decreasing not necessary.
fn ok(levels: Vec<i64>) -> bool {
    let mut diffs: Vec<bool> = vec![];
    let mut signs: Vec<i64> = vec![];
    for xy in levels.windows(2) {
        let x = xy[0];
        let y = xy[1];
        let diff = y - x;
        let abs_diff = diff.abs();
        let sign = if abs_diff != 0 { diff / abs_diff } else { 0 };
        let diff_ok = abs_diff >= 1 && abs_diff <= 3;
        diffs.push(diff_ok);
        signs.push(sign);
    }
    let diffs_ok = diffs.iter().filter(|d| **d).count();
    let increasing = signs.iter().filter(|s| **s > 0i64).count();
    let decreasing = signs.iter().filter(|s| **s < 0i64).count();
    let full = levels.len() - 1;
    increasing == full && diffs_ok == full || (decreasing == full && diffs_ok == full)
}

fn main() {
    let mut safe: usize = 0;
    for line in read_to_string("input").unwrap().lines() {
        let levels: Vec<&str> = line.split_whitespace().collect();
        let levels: Vec<i64> = levels
            .iter()
            .map(|i| {
                let v = i.parse::<i64>().unwrap();
                return v;
            })
            .collect();

        for i in 0..levels.len() {
            let new_levels = levels
                .iter()
                .enumerate()
                .filter_map(|x| if x.0 != i { Some(*x.1) } else { None })
                .collect();
            let ok = ok(new_levels);
            if ok {
                safe += 1;
                break;
            }
        }
    }
    dbg!(safe);
}
