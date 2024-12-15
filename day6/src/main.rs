use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Guard {
    position: Position,
    direction: Direction,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let puzzle = std::fs::read_to_string(path).expect("file should be read to a string");

    part_1(puzzle);
}

fn part_1(puzzle: String) {
    let puzzle = to_matrix(puzzle);
    let mut unique_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut guard = find_guard(&puzzle).expect("guard should be in puzzle");

    // dbg!(&puzzle);

    while is_within_map(&puzzle, &guard.position) {
        // dbg!(&guard);
        unique_positions.insert((guard.position.x, guard.position.y));
        guard = travel(&puzzle, guard);
    }

    dbg!(unique_positions.len());
}

fn is_within_map(puzzle: &Vec<Vec<char>>, position: &Position) -> bool {
    let height = puzzle.len();
    let width = puzzle[0].len();

    position.y >= 0
        && position.x >= 0
        && (position.y as usize) < height
        && (position.x as usize) < width
}

fn is_obsticle(puzzle: &Vec<Vec<char>>, position: &Position) -> bool {
    is_within_map(puzzle, position) && puzzle[position.y as usize][position.x as usize] == '#'
}

fn travel(puzzle: &Vec<Vec<char>>, guard: Guard) -> Guard {
    let new_dir = match guard.direction {
        Direction::Up => {
            let pos_above = Position {
                x: guard.position.x,
                y: guard.position.y - 1,
            };
            if is_obsticle(puzzle, &pos_above) {
                rotate_direction(Direction::Up)
            } else {
                Direction::Up
            }
        }
        Direction::Right => {
            let pos_above = Position {
                x: guard.position.x + 1,
                y: guard.position.y,
            };
            if is_obsticle(puzzle, &pos_above) {
                rotate_direction(Direction::Right)
            } else {
                Direction::Right
            }
        }
        Direction::Down => {
            let pos_above = Position {
                x: guard.position.x,
                y: guard.position.y + 1,
            };
            if is_obsticle(puzzle, &pos_above) {
                rotate_direction(Direction::Down)
            } else {
                Direction::Down
            }
        }
        Direction::Left => {
            let pos_above = Position {
                x: guard.position.x - 1,
                y: guard.position.y,
            };
            if is_obsticle(puzzle, &pos_above) {
                rotate_direction(Direction::Left)
            } else {
                Direction::Left
            }
        }
    };

    match new_dir {
        Direction::Up => Guard {
            position: Position {
                x: guard.position.x,
                y: guard.position.y - 1,
            },
            direction: new_dir,
        },
        Direction::Right => Guard {
            position: Position {
                x: guard.position.x + 1,
                y: guard.position.y,
            },
            direction: new_dir,
        },
        Direction::Down => Guard {
            position: Position {
                x: guard.position.x,
                y: guard.position.y + 1,
            },
            direction: new_dir,
        },
        Direction::Left => Guard {
            position: Position {
                x: guard.position.x - 1,
                y: guard.position.y,
            },
            direction: new_dir,
        },
    }
}

fn rotate_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn find_guard(puzzle: &Vec<Vec<char>>) -> Option<Guard> {
    for (y, row) in puzzle.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            match item {
                '^' => {
                    println!("found guard at x={x} y={y}");
                    return Some(Guard {
                        position: Position {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        },
                        direction: Direction::Up,
                    });
                }
                '>' => {
                    return Some(Guard {
                        position: Position {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        },
                        direction: Direction::Right,
                    })
                }
                'v' => {
                    return Some(Guard {
                        position: Position {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        },
                        direction: Direction::Down,
                    })
                }
                '<' => {
                    return Some(Guard {
                        position: Position {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        },
                        direction: Direction::Left,
                    })
                }
                _ => continue,
            }
        }
    }

    None
}

fn to_matrix(puzzle: String) -> Vec<Vec<char>> {
    puzzle.lines().map(|line| line.chars().collect()).collect()
}
