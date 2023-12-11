use std::collections::{HashMap, HashSet};
use crate::common_libs::files::get_lines_from_file;

pub fn solve(filename: &str) {
    let (commands, map) = get_commands_and_map(filename);

    let mut steps = 0;
    let mut curr_key = "AAA";
    loop {
        for i in 0..commands.len() {
            if curr_key == "ZZZ" { break; }

            steps += 1;

            let curr_mapping = map.get(curr_key).unwrap();
            let command = &commands[i..i+1];

            match command {
                "L" => {
                    curr_key = &*curr_mapping.0;
                }
                "R" => {
                    curr_key = &*curr_mapping.1;
                }
                _ => {
                    panic!("Hello There!");
                }
            }
        }

        if curr_key == "ZZZ" { break; }
    }

    println!("Num steps: {}", steps);
}

pub(crate) fn solve_part_2(filename: &str, logging: bool) {
    let (commands, map) = get_commands_and_map(filename);

    let mut curr_keys: Vec<String> = get_keys(&map);
    if logging { println!("{:?}", curr_keys); }

    let mut repeats = vec!();
    for j in 0..curr_keys.len() {
        let curr_mapping = map.get(&curr_keys[j]).unwrap();
        let mut visited: HashSet<String> = HashSet::new();

        let mut steps = 0;

        loop {
            let mut command= "";
            for i in 0..commands.len() {
                if logging { println!("{:?}", curr_keys); }

                steps += 1;
                command = &commands[i..i + 1];

                if visited.contains(&format!("{}:{}", command, curr_keys[j])) {
                    break;
                } else {
                    visited.insert(format!("{}:{}", command, curr_keys[j]).to_owned());
                }

                if logging { println!("Command: {}", command); }
                    if logging { println!("Command: {}", command); }
                    if logging { println!("Mapping for: {} => {:?}", &curr_keys[j], curr_mapping); }
                    match command {
                        "L" => {
                            curr_keys[j] = curr_mapping.0.clone();
                        }
                        "R" => {
                            curr_keys[j] = curr_mapping.1.clone();
                        }
                        _ => {
                            panic!("Hello There!");
                        }
                    }
                }
            if visited.contains(&format!("{}:{}", command, curr_keys[j])) {
                break;
            }
        }
        repeats.push(steps);
    }

    println!("{:?}", repeats);
}

fn get_commands_and_map(filename: &str) -> (String, HashMap<String, (String, String)>) {
    let lines = get_lines_from_file(filename);

    let commands: String = lines[0].clone();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for i in 2..lines.len() {
        let line = lines[i].clone();

        let mut parts = line.split("=");
        let key = parts.nth(0).unwrap().trim();
        let directions = parts.nth(0)
            .unwrap()
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')');

        let mut dir_split = directions.split(", ");
        map.insert(String::from(key), (
            dir_split.nth(0).unwrap().to_owned(),
            dir_split.nth(0).unwrap().to_owned()
        )
        );
    }

    (commands, map)
}

fn get_keys(map: &HashMap<String, (String, String)>) -> Vec<String>{
    map.keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
}

fn keys_end_with_z(keys: &Vec<String>) -> bool {
    keys.iter().all(|x| x.ends_with('Z'))
}