use std::fmt::format;

use itertools::Itertools;
use utils::read_input;

fn main() {
    let lines = read_input("../day6/input2.txt");
    let rows = lines
        .map(|line| {
            line.unwrap()
                .trim()
                .split(" ")
                .map(|str| str.to_string())
                .filter(|str| !str.is_empty())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<_>>();

    let operators = dbg!(&rows[rows.len() - 1]);

    let response_part1 = part1(&rows, operators);

    println!("RESPONSE: {:?}", response_part1)
}

fn part1(rows: &Vec<Vec<String>>, operators: &Vec<String>) -> usize {
    let convert_to_usize = |row: &Vec<String>| {
        row.iter()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };

    let init_fold = rows.iter().rev().collect::<Vec<_>>()[1];

    let response = rows.iter().rev().skip(2).fold::<Vec<usize>, _>(
        convert_to_usize(&init_fold),
        |acc, row| {
            let res = acc
                .iter()
                .zip(row)
                .zip(operators)
                .map(|c| {
                    let value_row = c.0.1.parse::<usize>().unwrap();
                    let value_acc = c.0.0;
                    match c.1.as_str() {
                        "+" => value_row + value_acc,
                        "-" => value_acc + value_row,
                        "*" => value_acc * value_row,
                        _ => panic!("NOT A VALID OPERATOR"),
                    }
                })
                .collect::<Vec<usize>>();
            res
        },
    );

    response.iter().sum::<usize>()
}

fn part2(rows: &Vec<Vec<String>>, operators: &Vec<String>) -> usize {
    let convert_to_usize = |row: &Vec<&String>| {
        row.iter()
            .map(|c| c.chars().map(|c| c.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    };

    let operators = &operators
        .iter()
        .rev()
        .map(|str| str.to_string())
        .collect::<Vec<_>>();

    let init_fold = rows.iter().collect::<Vec<_>>()[0];

    let init_fold = init_fold.iter().rev().collect::<Vec<_>>();
    println!("INIT FOLD: {:?}", &init_fold);

    let response = rows[..rows.len() - 1]
        .to_vec()
        .iter()
        .skip(1)
        .fold::<Vec<Vec<String>>, _>(convert_to_usize(&init_fold), |acc, row| {
            let row = row.into_iter().rev().collect::<Vec<_>>();

            let res = acc
                .iter()
                .zip(row)
                //.zip(operators)
                .map(|c| {
                    let mut value_acc = c.0.clone();
                    let mut value_row = c.1.chars().map(|c| c.to_string()).collect::<Vec<_>>();

                    println!("ACC: {:?}", value_acc);
                    println!("ROW: {:?}", value_row);

                    let res = value_acc
                        .iter()
                        .zip_longest(&value_row)
                        .map(|res| {
                            let res = [
                                res.as_ref().left().unwrap_or(&&"".to_string()),
                                res.as_ref().right().unwrap_or(&&"".to_string()),
                            ]
                            .to_vec()
                            .iter()
                            .map(|c| c.to_string())
                            .collect::<String>();
                            res
                        })
                        .collect::<Vec<_>>();

                    println!("RESPONSE: {:?}", res);

                    //res.iter().map(|c|vec![c]).collect()
                    res
                })
                .collect::<Vec<Vec<_>>>();
            res
        });
    println!("RES: {:?}", response);

    0
}
