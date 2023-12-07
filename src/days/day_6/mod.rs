use std::collections::HashSet;
use crate::common_libs::files::get_lines_from_file;

pub fn solve(filename: &str, log: bool) {
    let mut lines = get_lines_from_file(filename);

    let mut times: Vec<i128> = vec!();
    let mut records: Vec<i128> = vec!();

    match lines[0].split(":").nth(1) {
        Some(val) => {
            times.append(&mut process_line(val.to_string()));
        },
        None => return
    }

    match lines[1].split(":").nth(1) {
        Some(val) => {
            records.append(&mut process_line(val.to_string()));
        },
        None => return
    }

    println!("{:?}", times);
    println!("{:?}", records);

    assert_eq!(times.len(), records.len());

    let mut result = 1;
    for i in 0..times.len() {
        result *= get_nums_wins(records[i], times[i]);
    }

    println!("Result: {}",result);
}

pub fn process_line(val: String) -> Vec<i128> {
    let mut result = vec!();

    for item in val.trim().split(' ') {
        if item == "" { continue; }

        result.push(str::parse::<i128>(item).unwrap());
    }

    result
}

pub fn get_nums_wins(record: i128, time: i128) -> i128 {
    let win = find_win(record, time);

    let mut i = 0;
    let mut j = time;

    j = win;
    while i < j {
        let mid = i + (j - i) / 2;
        match will_win(mid, record, time - mid) {
            true => {
                j = mid;
            },
            false => {
                i = mid + 1;
            }
        }
    }

    let start = i;

    i = win;
    j = time;

    while i < j {
        let mid = i + (j - i) / 2;
        match will_win(mid, record, time - mid) {
            true => {
                i = mid + 1;
            },
            false => {
                j = mid;
            }
        }
    }

    let end = j;

    return end - start;
}

pub fn will_win(val: i128, record: i128, time: i128) -> bool {
    if val == 0 || time == 0 { return false; }
    val * time > record
}

pub fn find_win(record: i128, time: i128) -> i128 {
    let mut i = 0;
    let mut j = time;

    let mut queue: Vec<i128> = vec!();
    queue.push(i + (j - i) / 2);

    let mut set: HashSet<i128> = HashSet::new();

    while queue.len() > 0 {
        match queue.pop() {
            Some(val) => {
                if set.contains(&val) { continue; }

                if will_win(val, record, time - val) {
                    return val;
                } else {
                    set.insert(val);

                    if val > 0 {
                        queue.push(val - val / 2);
                    }
                    if val + val / 2 < time {
                        queue.push(val + val / 2);
                    }
                }
            },
            None => {
                break;
            }
        }
    }

    0
}