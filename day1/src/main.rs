use std::fs::read_to_string;

fn main() {
    let file_path = "/home/gtisdelle/source/repos/aoc_2024/day1/input.txt";
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        let lr: Vec<&str> = line.split("   ").collect();
        let l: i32 = lr[0].parse().expect("Left should be a number");
        let r: i32 = lr[1].parse().expect("Right should be a number");
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        let diff: i32 = l - r;
        sum += diff.abs();
    }

    dbg!(sum);
}
