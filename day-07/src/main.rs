use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Mul,
    Add,
    Concat,
}

impl Op {
    fn apply(&self, x: i64, y: i64) -> i64 {
        match &self {
            Self::Mul => x * y,
            Self::Add => x + y,
            Self::Concat => format!("{}{}", x, y).parse().unwrap(),
        }
    }
}

fn generate_combinations<'a>(operators: &'a [&Op], n: usize) -> Vec<Vec<&'a Op>> {
    if n == 0 {
        return vec![vec![]];
    }
    let smaller_combinations = generate_combinations(operators, n - 1);
    let mut result = Vec::new();
    for combination in smaller_combinations {
        for operator in operators {
            let mut with_this_op = combination.clone();
            with_this_op.push(&operator);
            result.push(with_this_op);
        }
    }
    result
}

fn main() {
    let file = "input";
    let mut perm_cache: HashMap<usize, Vec<Vec<&Op>>> = HashMap::new();
    let mut part1_result = 0;

    for line in std::fs::read_to_string(file).unwrap().lines() {
        dbg!(line);
        let parts: Vec<&str> = line.split(": ").collect();
        let expected = parts[0].parse::<i64>().unwrap();
        // dbg!(line);
        let numbers: Vec<i64> = parts[1]
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let operators_len = numbers.len() - 1;

        let perm = if let Some(perm) = perm_cache.get(&operators_len) {
            perm
        } else {
            // println!("calc for {}", operators_len);
            // part1:
            // let per = generate_combinations(&[&Op::Add], operators_len);
            // part2:
            let per = generate_combinations(&[&Op::Add, &Op::Mul, &Op::Concat], operators_len);
            // dbg!(&per);
            perm_cache.insert(operators_len, per);
            perm_cache.get(&operators_len).unwrap()
        };

        // dbg!(perm.len());
        for perm in perm {
            // dbg!(perm);
            let mut result = numbers[0];
            // dbg!(&numbers);
            for (&x, &op) in numbers.iter().skip(1).zip(perm) {
                result = op.apply(result, x);
            }
            if result == expected {
                // dbg!(perm);
                // println!("valid");
                part1_result += result;
                break;
            }
        }
    }

    dbg!(part1_result);
}
