fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = &args[1];

    let puzzle: Vec<Vec<char>> = std::fs::read_to_string(filepath)
        .expect("File should be read to string")
        .lines()
        .map(|p| p.chars().collect::<Vec<char>>())
        .collect();

    let mut xmas_count = 0;
    for (row_idx, row) in puzzle.iter().enumerate() {
        for (col_idx, letter) in row.iter().enumerate() {
            if *letter != 'X' {
                continue;
            }

            xmas_count += count_matches(&puzzle, row_idx, col_idx);
        }
    }

    dbg!(xmas_count);
}

fn count_matches(puzzle: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u32 {
    let num_rows: i32 = puzzle.len().try_into().unwrap();

    let num_cols: i32 = puzzle[0].len().try_into().unwrap();

    let mut x_words: Vec<Vec<char>> = Vec::new();
    let dirs: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];

    for (row_dir, col_dir) in dirs {
        let mut row: i32 = row_idx.try_into().unwrap();

        let mut col: i32 = col_idx.try_into().unwrap();

        let mut x_word: Vec<char> = Vec::new();
        for _ in 0..4 {
            if row > num_rows - 1 || col > num_cols - 1 || row < 0 || col < 0 {
                break;
            }

            x_word.push(puzzle[row as usize][col as usize]);

            row += row_dir;
            col += col_dir;
        }

        x_words.push(x_word);
    }

    let match_count = x_words
        .iter()
        .filter(|word| word.iter().collect::<String>() == "XMAS")
        .count();

    match_count.try_into().unwrap()
}
