use regex::Regex;
fn main() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279
";
    let input = std::fs::read_to_string("input").unwrap();

    let mut sum = 0;
    let mut a = (0, 0);
    let mut b = (0, 0);
    let mut p = (0, 0);
    for lin in input.lines() {
        let r = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        if let Some(caps) = r.captures(lin) {
            let x: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let y: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
            a = (x, y);
        }
        let r = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        if let Some(caps) = r.captures(lin) {
            let x: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let y: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
            b = (x, y);
        }
        let r = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        if let Some(caps) = r.captures(lin) {
            let x: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let y: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
            let x = x + 10000000000000;
            let y = y + 10000000000000;

            println!("a = {:?}, b = {:?}, p = {:?}", a, b, p);

            // Brute force, slow solution
            // let r = solve(a, b, p);
            // sum += r.unwrap_or(0);

            // Solve using linear equations using cramers method
            let tmp = solve_linear_system(
                a.0 as f64, b.0 as f64, a.1 as f64, b.1 as f64, x as f64, y as f64,
            );

            if let Some((a, b)) = tmp {
                let exact_match = a.round() == a && b.round() == b;
                if exact_match {
                    let costs = 3 * a as i64 + b as i64;
                    println!("a = {}, b = {}, costs = {}", a, b, costs);
                    sum += costs;
                }
            }
        }
    }
    dbg!(sum);
}

fn solve_linear_system(x0: f64, x1: f64, y0: f64, y1: f64, X: f64, Y: f64) -> Option<(f64, f64)> {
    let det = x0 * y1 - x1 * y0;
    if det == 0.0 {
        return None;
    }
    let a = (X * y1 - x1 * Y) / det;
    let b = (x0 * Y - y0 * X) / det;
    Some((a, b))
}

fn objective_function(a: f64, b: f64) -> f64 {
    3.0 * a + b
}

fn solve(a: (i64, i64), b: (i64, i64), r: (i64, i64)) -> Option<i64> {
    let mut pressed_a = 1e12 as i64;
    let mut overflow = false;
    let mut gmin = None;
    loop {
        let mut pressed_b = 1e12 as i64;
        loop {
            let x = pressed_a * a.0 + pressed_b * b.0;
            let y = pressed_a * a.1 + pressed_b * b.1;
            if x == r.0 && y == r.1 {
                let cost = pressed_a * 3 + pressed_b;
                println!(
                    "Found a = {}, b = {}, cost = {}",
                    pressed_a, pressed_b, cost
                );
                gmin = Some(gmin.map_or(cost, |m| i64::min(m, cost)));
                break;
            } else if x > r.0 || y > r.1 {
                overflow = true;
                break;
            } else {
                pressed_b += 1;
            }
        }
        // println!("outer");
        if pressed_b == 0 && overflow {
            break;
        }
        pressed_a += 1;
    }
    dbg!(gmin);
    gmin
}
