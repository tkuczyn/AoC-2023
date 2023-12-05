use std::collections::HashMap;
use std::hash::Hash;

use crate::common_libs::files::get_lines_from_file;

pub fn solve(filename: &str) -> (i32, i128) {
    let mut special: HashMap<(i32, i32), (char, i32, i128)> = HashMap::new();
    let mut numbers: HashMap<(i32, i32), String> = HashMap::new();

    let lines = get_lines_from_file(filename);

    let mut iter = lines.iter();
    let mut current_word: String = String::from("");
    for y in 0..lines.len() {
        let line = iter.next().unwrap();
        let mut chars = line.chars();

        if current_word.len() > 0 {
            numbers.insert(((chars.clone().count() - 1) as i32, (y - 1) as i32), current_word.clone());
            current_word.clear();
        }

        for x in 0..chars.clone().count() {
            let char = chars.next().unwrap();

            if char.is_numeric() {
                current_word.insert(current_word.len(), char);
            } else {
                if current_word.len() > 0 {
                    numbers.insert(((x - 1) as i32, y as i32), current_word.clone());
                }
                current_word.clear();
                if char != '.' {
                    special.insert((x as i32, y as i32), (char, 0, 1));
                }
            }
        }
    }

    let mut sum: i32 = 0;
    let mut gear_ratio: i128 = 0;

    for number in numbers {
        let (x, y) = number.0;

        let val = str::parse::<i32>(number.1.as_str()).unwrap();
        if adj_to_special(x, y, val, number.1.len() as i32, &mut special) {
            sum += val;
        }
    }

    for s in special {
        if s.1.0 == '*' && s.1.1 == 2 {
            gear_ratio += s.1.2;
        }
    }

    (sum, gear_ratio)
}

fn adj_to_special(x: i32, y: i32, number: i32, width: i32, map: &mut HashMap<(i32, i32), (char, i32, i128)>) -> bool {
    let mut found = false;

    if map.contains_key(&(x - width, y)) {
        update_map(x - width, y, number, map);
        found = true;
    }
    if  map.contains_key(&(x + 1, y)) {
        update_map(x + 1, y, number, map);
        found = true;
    }

    if found {
        return found;
    }

    for new_x in (x - width)..=(x + 1) {
        if map.contains_key(&(new_x, y - 1)) {
            update_map(new_x, y - 1, number, map);
            found = true;
        }
        if map.contains_key(&(new_x, y + 1)) {
            update_map(new_x, y + 1, number, map);
            found = true;
        }
    }

    return found;
}

fn update_map(x: i32, y: i32, number: i32, map: &mut HashMap<(i32, i32), (char, i32, i128)>) {
    let value = map.get(&(x, y)).unwrap().to_owned();
    if value.0 == '*' {
        map.insert((x, y), (value.0, value.1 + 1, value.2 * number as i128));
    }
}