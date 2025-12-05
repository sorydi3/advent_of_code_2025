use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read_input(input_file_name: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(input_file_name).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}

pub fn write_input(input_file_name: &str) -> std::io::BufWriter<File> {
    use std::io::BufWriter;
    let file = File::create(input_file_name).unwrap();
    let writer = BufWriter::new(file);
    writer
}
