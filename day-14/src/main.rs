use std::{collections::HashMap, ops, thread::sleep, time::Duration};

#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: i64,
    y: i64,
}

/// Overload plus operator for vector 2
impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    // let input = "p=2,4 v=2,-3";
    let file = "input";
    let input = std::fs::read_to_string(&file).unwrap();

    let width = 101 as i64;
    let height = 103 as i64;
    let steps = 100;
    let mut qs = HashMap::<usize, i64>::new();
    let mut states = Vec::new();

    for line in input.replace("p=", "").replace("v=", "").lines() {
        // dbg!(line);
        let pv: Vec<&str> = line.split(" ").collect();
        let p: Vec<i64> = pv[0]
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let v: Vec<i64> = pv[1]
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let p = Vec2 { x: p[0], y: p[1] };
        let v = Vec2 { x: v[0], y: v[1] };
        states.push((p, v));
        // dbg!(p, v);
        // Part 1
        // let propagated = propagate(p, v, width, height, steps);
        // // dbg!(propagated);
        // if let Some(q) = q(propagated, width, height) {
        //     // dbg!(q);
        //     if let Some(sum) = qs.get(&q) {
        //         qs.insert(q, *sum + 1);
        //     } else {
        //         qs.insert(q, 1);
        //     }
        // }
    }

    // Part 2
    for i in 0..10_000 {
        for (pos, vel) in states.iter_mut() {
            let propagated = propagate(*pos, *vel, width, height, 1);
            *pos = propagated;
        }
        if i > 6575 {
            // print!("{}[2J", 27 as char);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Step: {}", i);
            print_grid(&states, width, height);
            sleep(Duration::from_millis(1000));
        }
    }

    // Part 1
    // let mut s = 1;
    // dbg!(&qs);
    // for x in qs.into_values() {
    //     s *= x
    // }
    // dbg!(s);
}

fn print_grid(states: &Vec<(Vec2, Vec2)>, width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            // get number
            let count = states
                .iter()
                .filter(|(Vec2 { x: px, y: py }, _)| x == *px && y == *py)
                .count();
            if count == 0 {
                print!(" ");
            } else {
                print!("{}", count);
            }
        }
        println!();
    }
}

fn propagate(p: Vec2, v: Vec2, width: i64, height: i64, steps: usize) -> Vec2 {
    let mut p_updated = p.clone();
    let wrap = |x: &mut i64, max: i64| {
        if *x < 0 {
            *x = max + *x;
        } else {
            *x = *x % max;
        }
    };
    for i in 0..steps {
        p_updated = p_updated + v;
        wrap(&mut p_updated.x, width);
        wrap(&mut p_updated.y, height);
    }
    p_updated
}

fn q(p: Vec2, width: i64, height: i64) -> Option<usize> {
    let hw = width / 2;
    let hh = height / 2;
    if p.x < hw && p.y < hh {
        Some(0)
    } else if p.x > hw && p.y < hh {
        Some(1)
    } else if p.x > hw && p.y > hh {
        Some(3)
    } else if p.x < hw && p.y > hh {
        Some(2)
    } else {
        None
    }
}
