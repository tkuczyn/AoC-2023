use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file)
                          .lines()
                          .map( |l| l.expect("Unable to parse line"))
                          .collect();

    Ok(lines)
}

pub fn get_lines_from_file<P>(filename: P) -> Vec<String> 
where P: AsRef<Path> {
    match read_lines(filename) {
        Ok(files) => {
            return files;
        },
        Err(error) => {
            panic!("Unable to read lines from file: {}", error)
        }
    }
}