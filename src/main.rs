use std::collections::HashMap;

use crate::days::*;
use day_7::hand_type::HandType;

pub mod common_libs;
pub mod days;
pub mod tests;

fn main() {
    let map: HashMap<&str, bool> = HashMap::from([
        ("1", false),
        ("2", false),
        ("3", false),
        ("4", false),
        ("5", false),
        ("6", false),
        ("7", true)
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

    if *map.get("5").unwrap() {
        day_5::solve("resources/day5/day5.test.txt", false);
        day_5::solve("resources/day5/day5.prod.txt", false);
        day_5::solve_part_2("resources/day5/day5.test.txt", false);
        day_5::solve_part_2("resources/day5/day5.prod.txt", false);
    }

    if *map.get("6").unwrap() {
        assert_eq!(4, day_6::get_nums_wins(9, 7));
        assert_eq!(8, day_6::get_nums_wins(40, 15));
        assert_eq!(9, day_6::get_nums_wins(200, 30));

        day_6::solve("resources/day6/day6.test.txt", false);
        day_6::solve("resources/day6/day6.prod.txt", false);


        //part 2
        println!("---- PART 2 ----");
        println!("{}", day_6::get_nums_wins(940200, 71530));
        println!("{}", day_6::get_nums_wins(390110311121360, 48989083));
    }

    if *map.get("7").unwrap() {
        day_7::solve("resources/day7/day7.test.txt");
        day_7::solve("resources/day7/day7.prod.txt");
    }
}
