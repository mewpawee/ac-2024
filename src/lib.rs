use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

pub fn read_lines<P>(file_name: P) -> std::io::Result<Lines<BufReader<File>>> where P: AsRef<Path> {
    // Open the file for reading
    let file = File::open(file_name)?;
    // Create a buffered reader to read the file
    let reader = BufReader::new(file);
    Ok(reader.lines())
}
