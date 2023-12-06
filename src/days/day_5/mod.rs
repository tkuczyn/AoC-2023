mod mapping;
mod range;

use std::collections::HashSet;
use std::slice::Iter;
use crate::common_libs::files::get_lines_from_file;
use crate::days::day_5::mapping::Mapping;
use crate::days::day_5::range::Range;

pub fn solve(filename: &str, log: bool) {
    let lines = get_lines_from_file(filename);

    let mut iterator = lines.iter();

    let mut seeds = parse_seeds(iterator.next().unwrap());
    let mut mappings: Vec<(String, Vec<Mapping>)> = vec!();

    parse_chunks(&mut iterator, &mut mappings);

    if log {
        println!("{:?}", seeds);
        println!("{:?}", mappings);
    }

    for mapping in mappings {
        let mut used: HashSet<usize> = HashSet::new();

        for i_mapping in &mapping.1 {
            if log {
                println!("{:?}", i_mapping);
            }
            for i in 0..seeds.len() {
                if used.contains(&i) { continue; }

                let value = seeds[i];
                if value >= i_mapping.source &&
                    value <= i_mapping.source + i_mapping.size - 1 {
                    seeds[i] = seeds[i] + (i_mapping.destination - i_mapping.source);
                    used.insert(i);
                }
            }
        }


        if log {
            println!("{:?}", seeds);
        }
    }

    println!("{}", get_min(seeds));
}

pub fn solve_part_2(filename: &str, log: bool) {

    let lines = get_lines_from_file(filename);

    let mut iterator = lines.iter();

    let mut seeds = parse_seed_ranges(iterator.next().unwrap());
    let mut mappings: Vec<(String, Vec<Mapping>)> = vec!();

    parse_chunks(&mut iterator, &mut mappings);

    if log {
        println!("{:?}", seeds);
        println!("{:?}", mappings);
    }

    for mapping in mappings {
        let mut used: HashSet<usize> = HashSet::new();

        for i_mapping in &mapping.1 {
            if log {
                println!("{:?}", i_mapping);
            }

            let change = i_mapping.destination - i_mapping.source;

            let mut i = 0;
            loop {
                if log {
                    println!("Comparing: {:?} {:?}", i, seeds.len());
                }
                if i >= seeds.len() {
                    break;
                }

                if used.contains(&i) {
                    i += 1;
                    continue;
                }

                let mut overlap: bool = false;
                if i_mapping.source <= seeds[i].start &&
                    i_mapping.source + i_mapping.size - 1 >= seeds[i].stop {
                    overlap = true;
                } else if i_mapping.source >= seeds[i].start &&
                    i_mapping.source + i_mapping.size - 1 < seeds[i].stop {

                    let previous_start = seeds[i].start;
                    let previous_stop = seeds[i].stop;

                    seeds[i].start = i_mapping.source;
                    seeds[i].stop = i_mapping.source + i_mapping.size - 1;

                    if previous_start < i_mapping.source {
                        seeds.insert(seeds.len(), Range { start: previous_start, stop: seeds[i].start - 1 });
                    }
                    if previous_stop > seeds[i].stop {
                        seeds.insert(seeds.len(), Range { start: seeds[i].stop + 1, stop: previous_stop});
                    }

                    overlap = true;
                } else if i_mapping.source <= seeds[i].start &&
                    i_mapping.source + i_mapping.size - 1 >= seeds[i].start {
                    let previous_stop = seeds[i].stop;

                    seeds[i].stop = i_mapping.source + i_mapping.size - 1;

                    seeds.insert(seeds.len(), Range{ start: i_mapping.source + i_mapping.size, stop: previous_stop});

                    overlap = true;
                } else if i_mapping.source <= seeds[i].stop &&
                    i_mapping.source + i_mapping.size - 1 >= seeds[i].stop {

                    let previous_start = seeds[i].start;

                    seeds[i].start = i_mapping.source;

                    seeds.insert(seeds.len(), Range{ start: previous_start, stop: i_mapping.source - 1});

                    overlap = true;
                }

                if overlap {
                    seeds[i].start += change;
                    seeds[i].stop += change;

                    used.insert(i);
                }

                i += 1;

                if log {
                    println!("{:?}", seeds);
                }
            }
        }
    }

    println!("{}", get_min_range(seeds));
}

fn parse_seed_ranges(line: &String) -> Vec<Range> {
    let mut parts = line.split(":");
    assert_eq!(2, parts.clone().count());

    let mut result: Vec<Range> = vec!();

    let mut nums = parts.nth(1).unwrap().trim().split(" ");

    loop {

        let mut start: i128 = 0;

        match nums.next() {
            Some(text) => {
                start = text.parse().unwrap();
            },
            None => {
                break;
            }
        }
        let num: i128 = nums.next().unwrap().parse().unwrap();
        result.push(Range{ start, stop: start + num - 1})
    }

    result
}

fn parse_seeds(line: &String) -> Vec<i128> {
    let mut parts = line.split(":");
    assert_eq!(2, parts.clone().count());

    let mut result: Vec<i128> = vec!();
    for part in parts.nth(1).unwrap().split(" ") {
        match str::parse::<i128>(part) {
            Ok(value) => result.push(value),
            _ => {}
        }
    }

    result
}

fn parse_chunks(iterator: &mut Iter<String>, mappings: &mut Vec<(String, Vec<Mapping>)>) -> () {
    let mut chunks: Vec<String> = vec!();
    loop {
        let next = iterator.next();
        match next {
            Some(line) => {
                if line.trim() == "" {
                    if chunks.len() > 0 {
                        mappings.push(process_chunk(&chunks));
                        chunks.clear();
                    }
                    continue
                }

                chunks.push(line.clone());
            },
            None => {
                if chunks.len() > 0 {
                    mappings.push(process_chunk(&chunks));
                    chunks.clear();
                }
                break;
            }
        }
    }
}

fn process_chunk(lines: &Vec<String>) -> (String, Vec<Mapping>) {
    let header = process_header(lines[0].clone());

    let mut mappings: Vec<Mapping> = vec!();
    for i in 1..lines.len() {
        mappings.push(process_mapping(lines[i].clone()));
    }

    (header, mappings)
}

fn process_header(line: String) -> String {
    let mut parts = line.split( " ");
    match parts.nth(0) {
        Some(val) => {
            val.to_owned()
        },
        None => {
            panic!("Unable to process header");
        }
    }

}

fn process_mapping(line: String) -> Mapping {
    let mut parts = line.split(" ");

    let destination = parts.nth(0);
    let start = parts.nth(0);
    let size = parts.nth(0);

    Mapping {
        destination: destination.unwrap().parse().unwrap(),
        source: start.unwrap().parse().unwrap(),
        size: size.unwrap().parse().unwrap()
    }
}

fn get_min(vals: Vec<i128>) -> i128 {
    let mut min = i128::MAX;

    for val in vals {
        min = val.min(min);
    }

    min
}

fn get_min_range(vals: Vec<Range>) -> i128 {
    let mut min = i128::MAX;

    for val in vals {
        min = min.min(val.start);
    }

    min
}