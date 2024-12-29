use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let num_safe = read_to_string(filepath)
        .expect("File should be read")
        .lines()
        .map(|report| report.split(' ').map(|level| level.parse::<i32>().unwrap()))
        .filter(|report| is_report_safe(&report.clone().collect()))
        .count();

    dbg!(num_safe);

    part_2(filepath);
}

fn part_2(filepath: &String) {
    let num_safe = read_to_string(filepath)
        .expect("File should be read")
        .lines()
        .map(|report| report.split(' ').map(|level| level.parse::<i32>().unwrap()))
        .filter(|report| is_dampenable_report_safe(&report.clone().collect()))
        .count();

    dbg!(num_safe);
}

fn is_dampenable_report_safe(report: &Vec<i32>) -> bool {
    if is_report_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut dampened_report = report.clone();
        dampened_report.remove(i);
        if is_report_safe(&dampened_report) {
            return true;
        }
    }

    false
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    if report[1] == report[0] || (report[1] - report[0]).abs() > 3 {
        return false;
    }

    let asc = report[1] > report[0];
    for i in 1..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if (asc && diff <= 0) || (!asc && diff >= 0) || diff.abs() > 3 {
            return false;
        }
    }

    true
}
