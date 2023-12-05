use std::collections::HashMap;
use crate::days::*;

pub mod common_libs;
pub mod days;

fn main() {
    let map: HashMap<&str, bool> = HashMap::from([
        ("1", false),
        ("2", false),
        ("3", false),
        ("4", true)
    ]);

    if *map.get("1").unwrap() {
        println!("{}", day_1::solve("resources/day1/day1.test.txt", false, false));
        println!("{}", day_1::solve("resources/day1/day1.prod.txt", false, false));

        println!("{}", day_1::solve("resources/day1/day1.test.txt", true, false));
        println!("{}", day_1::solve("resources/day1/day1.test.2.txt", true, false));
        println!("{}", day_1::solve("resources/day1/day1.prod.txt", true, false));
    }

    if *map.get("2").unwrap() {
        let result = day_2::solve("resources/day2/day2.test.txt");
        println!("Score: {}, Result: {}", result.0, result.1);

        let result = day_2::solve("resources/day2/day2.prod.txt");
        println!("Score: {}, Result: {}", result.0, result.1);
    }

    if *map.get("3").unwrap() {
        let result = day_3::solve("resources/day3/day3.test.txt");
        println!("Sum: {}, Gear Ratio: {}", result.0, result.1);

        let result = day_3::solve("resources/day3/day3.prod.txt");
        println!("Sum: {}, Gear Ratio: {}", result.0, result.1);
    }

    if *map.get("4").unwrap() {
        println!("{}", day_4::solve("resources/day4/day4.test.txt"));
        println!("{}", day_4::solve("resources/day4/day4.prod.txt"));

        println!("{}", day_4::solve_part_2("resources/day4/day4.test.txt"));
        println!("{}", day_4::solve_part_2("resources/day4/day4.prod.txt"));
    }
}
