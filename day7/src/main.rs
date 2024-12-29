use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Test {
    target: u64,
    numbers: Vec<u64>,
}

impl FromStr for Test {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp = s.split(':').collect::<Vec<&str>>();
        let target = sp[0].parse::<u64>().unwrap();
        let numbers = sp[1]
            .split(' ')
            .skip(1)
            .map(|num| num.parse().unwrap())
            .collect::<Vec<u64>>();

        Ok(Test { target, numbers })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let tests: Vec<Test> = std::fs::read_to_string(path)
        .expect("file should be read to a string")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    part_1(&tests);
}

fn part_1(tests: &Vec<Test>) {
    let passing_tests: Vec<&Test> = tests
        .iter()
        .filter(|test| can_eval_to_target(test.target, &test.numbers))
        .collect();

    //dbg!(&passing_tests);

    let sum: u64 = passing_tests.iter().map(|test| test.target).sum();

    dbg!(sum);
}

fn can_eval_to_target(target: u64, numbers: &Vec<u64>) -> bool {
    let mut results: Vec<Vec<Operator>> = Vec::new();
    can_eval_rec(target, numbers, 1, Vec::new(), &mut results);

    //dbg!(&results);

    results.len() > 0
}

fn can_eval_rec(
    target: u64,
    numbers: &Vec<u64>,
    i: usize,
    acc: Vec<Operator>,
    results: &mut Vec<Vec<Operator>>,
) {
    //println!("checking cur_eval...");
    let cur_eval = eval_acc(&acc, numbers);
    //println!("\tat a depth of {i} and currenty evaluating to: {cur_eval}");
    if i == numbers.len() && eval_acc(&acc, numbers) == target {
        //dbg!("result found", &acc);
        results.push(acc);
        return;
    }

    if i >= numbers.len() {
        return;
    }

    //println!("checking next mul...");
    let next_mul = eval(cur_eval, numbers[i], &Operator::Multiply);
    if next_mul <= target {
        let mut next = acc.clone();
        next.push(Operator::Multiply);
        can_eval_rec(target, numbers, i + 1, next, results);
    }

    //println!("checking next add...");
    let next_add = eval(cur_eval, numbers[i], &Operator::Add);
    if next_add <= target {
        let mut next = acc.clone();
        next.push(Operator::Add);
        can_eval_rec(target, numbers, i + 1, next, results);
    }
}

fn eval_acc(acc: &[Operator], numbers: &Vec<u64>) -> u64 {
    //println!("\tin eval_acc");
    let mut result = numbers[0];
    for (i, op) in acc.iter().enumerate() {
        result = eval(result, numbers[i + 1], op)
    }

    result
}

fn eval(a: u64, b: u64, op: &Operator) -> u64 {
    //println!("\tCombining {a} and {b} using {:?}", op);
    match op {
        Operator::Add => a + b,
        Operator::Multiply => a * b,
    }
}
