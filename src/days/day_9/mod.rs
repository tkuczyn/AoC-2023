use crate::common_libs::files::get_lines_from_file;


pub(crate) fn solve(filename: &str) {
    let lines = get_lines_from_file(filename);
    let converted_lines = process_lines(lines);

    let mut sum = 0;
    let mut previous_sum = 0;

    for line in converted_lines {
        sum += find_next_element(line.clone());
        previous_sum += find_previous_element(line.clone());
    }

    println!("Result: {}", sum);
    println!("Result Previous: {}", previous_sum);
}

fn find_next_element(nums: Vec<i32>) -> i32 {
    if nums.iter().all(|x| *x == 0) {
        return 0;
    }

    let new_nums: Vec<i32> = get_all_diffs(&nums);
    find_next_element(new_nums) + nums.last().unwrap()
}

fn find_previous_element(nums: Vec<i32>) -> i32 {
    if nums.iter().all(|x| *x == 0) {
        return 0;
    }

    let new_nums: Vec<i32> = get_all_diffs(&nums);
    nums.first().unwrap() - find_previous_element(new_nums)
}

fn get_all_diffs(nums: &Vec<i32>) -> Vec<i32> {
    let mut new_nums: Vec<i32> = vec!();

    for i in 0..nums.len() - 1 {
        new_nums.push(nums[i + 1] - nums[i]);
    }

    new_nums
}

fn process_lines(lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut result = vec!();

    for line in lines {
        let mut current = vec!();

        for char in line.split(" ") {
            current.push(str::parse::<i32>(char).unwrap());
        }

        result.push(current);
    }

    result
}