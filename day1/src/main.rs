use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];

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

    part_2(left, right);
}

fn part_2(left: Vec<i32>, right: Vec<i32>) {
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for location_id in right {
        if right_map.contains_key(&location_id) {
            let cur = right_map.get(&location_id).unwrap();
            right_map.insert(location_id, cur + 1);
        } else {
            right_map.insert(location_id, 1);
        }
    }

    let mut similarity_score = 0;
    for location_id in left {
        if right_map.contains_key(&location_id) {
            similarity_score += location_id * right_map.get(&location_id).unwrap();
        }
    }

    dbg!(similarity_score);
}
