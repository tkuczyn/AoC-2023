use std::collections::{HashMap, HashSet};
use crate::common_libs::files::get_lines_from_file;

pub fn solve(filename: &str) -> i32 {
    let lines = get_lines_from_file(filename);

    let mut sum = 0;
    for line in lines {
        let val = process_line(line);
        sum += process_nums(val.as_str(), true);
    }

    return sum;
}

pub fn solve_part_2(filename: &str) -> i32 {
    let lines = get_lines_from_file(filename);

    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..lines.len() {
        map.insert(i, 1);
    }

    for i in 0..lines.len() {
        let line = lines.get(i);

        match line {
            Some(text) => {
                let val = process_line(text.clone());
                let winnings = process_nums(val.as_str(), false);

                for j in 1..=winnings {
                    map.insert(i + j as usize, map.get(&i).unwrap() + map.get(&(i + j as usize)).unwrap());
                }
            },
            None => {

            }
        }
    }

    let mut sum = 0;
    for entry in map {
        sum += entry.1 as i32;
    }

    sum
}

pub fn process_line(line: String) -> String {
    let mut parts = line.split(':');
    assert_eq!(2, parts.clone().count());

    parts.nth(1).unwrap().to_owned()
}

pub fn process_nums(nums: &str, multiply: bool) -> i32 {
    let mut parts = nums.split('|');
    assert_eq!(2, parts.clone().count());

    let mut winning_nums: HashSet<&str> = HashSet::new();
    for num in parts.nth(0).unwrap().trim().split(" ") {
        winning_nums.insert(num);
    }

    let mut score = 0;
    for num in parts.nth(0).unwrap().trim().split(" ") {
        if num == "" { continue; }

        if winning_nums.contains(num) {
            if multiply {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            } else {
                score += 1
            }
        }

    }

    score
}