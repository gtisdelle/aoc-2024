use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let mut safe = 0;
    for report in read_to_string(filepath).unwrap().lines() {
        let levels: Vec<i32> = report
            .split(' ')
            .map(|level| level.parse().unwrap())
            .collect();

        if levels[1] == levels[0] || (levels[1] - levels[0]).abs() > 3 {
            continue;
        }

        let asc = levels[1] > levels[0];
        let mut is_record_safe = true;
        for i in 1..levels.len() - 1 {
            let diff = levels[i + 1] - levels[i];
            if (asc && diff <= 0) || (!asc && diff >= 0) || diff.abs() > 3 {
                is_record_safe = false;
                break;
            }
        }

        if is_record_safe {
            safe += 1;
        }
    }

    dbg!(safe);
}
