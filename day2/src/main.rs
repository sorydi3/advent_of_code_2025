use core::panic;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
fn main() -> () {
    let lines = read_input("input.txt");

    let mut writer = write_input("out.txt");
    lines.for_each(|line| {
        let sum = line.unwrap().split(",").fold(0usize, |acc, range| {
            let (init, end) = range.split_once("-").unwrap();
            if init.len() % 2 == 0 && end.len() % 2 == 0 || end.len() > init.len() {
                //println!("VALIDinit: {:?} init_end: {:?}", init, end);
                writer.write_all(format!("VALIDinit: {:?} init_end: {:?} \n", init, end).as_bytes());
                let invalid_id = (init.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap())
                    .filter(|value| {
                        let val_str = value.to_string();
                        let len = val_str.len();
                        if len == 1 {
                            return false;
                        }
                        let chunks = val_str.chars().collect::<Vec<_>>();
                        let res = chunks.chunks(len / 2).collect::<Vec<_>>();
                        //println!("CHUNKS: {:?}. LEN: {:?}", res, len / 2);
                        //writer.write_all(format!("CHUNKS: {:?}. LEN: {:?} \n", res, len / 2).as_bytes());
                        match res.len() {
                            2 => res[0].eq(res[1]),
                            1 => true,
                            _ => false,
                        }
                    })
                    .collect::<Vec<_>>();
                //println!("VALUES: {:?}", invalid_id);

                writer.write_all(format!("VALUES: {:?} \n", invalid_id).as_bytes());

                return acc + invalid_id.iter().sum::<usize>();
            } else {
                writer.write_all(format!("NOT VALIDinit: {:?} init_end: {:?} \n", init, end).as_bytes());

                return acc;
            }
        });

        println!("SUM: {:?}", sum)
    });
}

fn read_input(input_file_name: &str) -> std::io::Lines<BufReader<File>> {
    use std::io::BufReader;
    let file = File::open(input_file_name).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}


fn write_input(input_file_name: &str) -> std::io::BufWriter<File> {
    use std::io::BufWriter;
    let file = File::create(input_file_name).unwrap();
    let writer = BufWriter::new(file);
    writer
}
