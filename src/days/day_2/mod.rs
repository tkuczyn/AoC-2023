use std::cmp::max;
use std::collections::HashMap;
use crate::common_libs::files::get_lines_from_file;

pub fn solve(filename: &str) -> (i32, i32) {
    let lines = get_lines_from_file(filename);

    let mut valid_games = 0;
    let mut score = 0;

    for line in lines {
        let mut hash_map: HashMap<&str, i32> = HashMap::from([
            ("red", 0),
            ("blue", 0),
            ("green", 0)
        ]);
        let mut parts = line.split(":");

        assert_eq!(parts.clone().count(), 2);

        let mut invalid_hand = false;

        let  hands = parts.clone().last().unwrap().split(";");
        for hand in hands {
            if !process_hand(hand, &mut hash_map) {
                invalid_hand = true;
            }
        }

        score += hash_map["red"] * hash_map["blue"] * hash_map["green"];

        if invalid_hand {
            continue;
        }

        let value: i32 = str::parse(
            parts
                .nth(0)
                .unwrap()
                .split(" ")
                .nth(1)
                .unwrap())
            .unwrap();
        valid_games += value;
    }

    (score, valid_games)
}

fn process_hand<'a>(hand: &'a str, hash_map: &mut HashMap<&'a str, i32>) -> bool {
    let mut result: bool = true;

    for value in hand.split(",") {
        let value = value.trim();
        let mut parts = value.split(" ");

        assert_eq!(2, parts.clone().count());

        let count: i32 = str::parse(parts.next().expect("Invalid count")).unwrap();
        let color = parts.next().expect("Invalid color");

        hash_map.insert(color, max(hash_map[color], count));
        result = match color {
            "red" => {
                count <= 12
            },
            "green" => {
                count <= 13
            },
            "blue" => {
                count <= 14
            },
            _ => true
        };
    }

    result
}