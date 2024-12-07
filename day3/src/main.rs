use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = &args[1];

    let instructions = read_to_string(filepath).expect("file should open");

    let mut sum = 0;
    let mut i: usize = 0;
    while i < instructions.len() {
        if instructions.chars().nth(i).unwrap() != 'm' {
            i += 1;
            continue;
        }

        let candidate = &instructions[i..]
            .chars()
            .take_while(|c| *c != ')')
            .collect::<Vec<char>>();

        sum += parse_mul(candidate);

        i += 1;
    }

    dbg!(sum);
}

fn parse_mul(candidate: &Vec<char>) -> i32 {
    let name = candidate
        .clone()
        .into_iter()
        .take_while(|c| *c != '(')
        .collect::<String>();

    if name != "mul" {
        return 0;
    }

    let first = candidate
        .clone()
        .into_iter()
        .skip(name.len() + 1)
        .take_while(|c| *c != ',')
        .collect::<String>();

    let second = candidate
        .clone()
        .into_iter()
        .skip(name.len() + 1 + first.len() + 1)
        .collect::<String>();

    // dbg!(candidate, &first, &second);

    let first = match first.parse::<i32>() {
        Ok(x) => x,
        Err(_) => return 0,
    };

    let second = match second.parse::<i32>() {
        Ok(x) => x,
        Err(_) => return 0,
    };

    first * second
}
