use std::{collections::HashSet, ops::RangeInclusive};
use utils::read_input;

fn main() {
    let lines = read_input("../day5/input.txt");

    let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();

    let mut ids: Vec<usize> = Vec::new();

    lines.for_each(|line| {
        if let Ok(line) = line {
            match line.split_once("-") {
                Some((init, end)) => {
                    let init_value = init.parse::<usize>().unwrap();
                    let end_value = end.parse::<usize>().unwrap();
                    ranges.push(RangeInclusive::new(init_value, end_value));
                }
                None => {
                    if !line.is_empty() {
                        let id = line.parse::<usize>().expect("Failed to parse line!!");
                        ids.push(id);
                    }
                }
            }
        }
    });

    //Part 1

    let response = ids.iter().fold::<usize, _>(0, |acc, id| {
        let res = ranges.iter().any(|range| range.contains(id));
        if res { acc + 1 } else { acc }
    });

    // Part 2
    let collection = ranges
        .into_iter()
        .map(|range| range.collect::<Vec<_>>())
        .flatten()
        .collect::<HashSet<_>>();
}
