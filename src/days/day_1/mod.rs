use std::collections::HashMap;
use crate::common_libs::files;

pub fn solve(filename: &str, include_map: bool, log: bool) -> i32 {
    let lines = files::get_lines_from_file(filename);

    let mut sum = 0;

    for line in lines {
        sum += process_line(line, include_map, log);
    }

    sum
}

fn process_line(line: String, include_map: bool, log: bool) -> i32 {
    let map: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut result = String::from("");

    let mut chars = line.chars();

    let mut current_word = String::from("");
    for _i in 0..line.len() {
        match chars.next() {
            Some(c) => {
                if c.is_numeric() {
                    result += c.to_string().as_str();
                    break;
                } else if include_map {
                    current_word.insert(current_word.len(), c);

                    let mut found = false;
                    for key in map.keys() {
                        if current_word.ends_with(&*key) {
                            result += map[key];
                            found = true;
                            break;
                        }
                    }

                    if found {
                        break;
                    }
                }
            },
            None => ()
        }
    }

    let mut current_word = String::from("");
    chars = line.chars();
    let mut iter = chars.into_iter().rev();
    for _i in 0..line.len() {
        match iter.next() {
            Some(c) => {
                if c.is_numeric() {
                    result += c.to_string().as_str();
                    break;
                } else if include_map {
                    current_word.insert(0, c);

                    let mut found = false;
                    for key in map.keys() {
                        if current_word.starts_with(&*key) {
                            result += map[key];
                            found = true;
                            break;
                        }
                    }

                    if found {
                        break;
                    }
                }
            },
            None => ()
        }
    }

    if log {
        println!("The result for the current line is: {}", result);
    }

    match result.parse::<i32>() {
        Ok(n) => return n,
        Err(e) => panic!("Unable to parse number: {}", e)
    }
}