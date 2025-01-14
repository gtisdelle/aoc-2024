use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Coordinates {
    x: usize,
    y: usize,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = &args[1];

    let map: Vec<Vec<char>> = read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut antenna_index: HashMap<char, Vec<Coordinates>> = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, letter) in row.iter().enumerate() {
            if *letter != '.' {
                if antenna_index.contains_key(letter) {
                    antenna_index
                        .get_mut(letter)
                        .unwrap()
                        .push(Coordinates { x, y });
                } else {
                    let mut new_list: Vec<Coordinates> = Vec::new();
                    new_list.push(Coordinates { x, y });
                    antenna_index.insert(*letter, new_list);
                }
            }
        }
    }

    let mut antinodes: HashSet<Coordinates> = HashSet::new();
    for (_, antennas) in antenna_index {
        for coordinates in antennas.iter() {
            for other in antennas.iter().filter(|x| *x != coordinates) {
                // let antinode = calculate_antinode(&map, coordinates, other);
                // if let Some(a) = antinode {
                //     antinodes.insert(a);
                // }
                let resonating = calculate_antinodes_with_resonance(&map, coordinates, other);
                antinodes.extend(resonating);
            }
        }
    }

    dbg!(/*&antinodes,*/ antinodes.len());
}

fn calculate_antinodes_with_resonance(
    map: &Vec<Vec<char>>,
    a: &Coordinates,
    b: &Coordinates,
) -> Vec<Coordinates> {
    let mut result = Vec::new();
    result.push(a.clone());
    let mut prev = a.clone();
    let mut cur = calculate_antinode(map, a, b);
    while cur.is_some() {
        result.push(cur.clone().unwrap());
        let next = calculate_antinode(map, &cur.clone().unwrap(), &prev);
        prev = cur.clone().unwrap();
        cur = next;
    }

    result
}

fn calculate_antinode(
    map: &Vec<Vec<char>>,
    a: &Coordinates,
    b: &Coordinates,
) -> Option<Coordinates> {
    let delta_x: i32 = a.x as i32 - b.x as i32;
    let delta_y: i32 = a.y as i32 - b.y as i32;

    let x: i32 = a.x as i32 + delta_x;
    let y: i32 = a.y as i32 + delta_y;

    if !is_within_map(map, x, y) {
        return None;
    }

    Some(Coordinates {
        x: x as usize,
        y: y as usize,
    })
}

fn is_within_map(map: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    let right_bound: i32 = map.len().try_into().unwrap();
    let bottom_bound: i32 = map.first().unwrap().len().try_into().unwrap();

    x < right_bound && y < bottom_bound && x >= 0 && y >= 0
}
