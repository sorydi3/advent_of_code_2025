use core::panic;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
fn main() -> () {
    let lines = read_input("test2.txt");

    let mut writer = write_input("out.txt");
    lines.for_each(|line| {
        let sum = line.unwrap().split(",").fold(0usize, |acc, range| {
            let (init, end) = range.split_once("-").unwrap();

            println!("VALIDinit: {:?} init_end: {:?}", init, end);
            writer.write_all(format!("VALIDinit: {:?} init_end: {:?} \n", init, end).as_bytes());
            let invalid_id = (init.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap())
                .filter(|value| {
                    let val_str = value.to_string();
                    let len = val_str.len();
                    let chunks = val_str.chars().collect::<Vec<_>>();
                    let res = chunks.chunks(len / 2).collect::<Vec<_>>();
                    if len == 1 && res.len() == 1 {
                        return false;
                    }
                    //println!("CHUNKS: {:?}. LEN: {:?}", res, len / 2);
                    writer
                        .write_all(format!("CHUNKS: {:?}. LEN: {:?} \n", res, len / 2).as_bytes());

                    match res.len() {
                        2 => {
                            let probe = res[0];
                            println!(
                                "CHUNKS:{:?}. PROBE: {:?}. LEN/2:{:?} LEN:{:?}",
                                res,
                                probe,
                                len / 2,
                                len
                            );
                            let t = res.iter().all(|c| (*c).eq(probe));
                            //dbg!(t)
                            t
                        }
                        1 => true,
                        _ => false,
                    }
                })
                .collect::<Vec<_>>();
            println!("VALUES: {:?}", invalid_id);

            writer.write_all(format!("VALUES: {:?} \n", invalid_id).as_bytes());

            acc + invalid_id.iter().sum::<usize>()
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
